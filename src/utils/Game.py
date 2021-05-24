import sys
from time import sleep
from typing import Union, Optional

from .Console import Console
from .Tower import Tower
from .ActionParser import ActionParser

import pyautogui as pygui
import pytesseract as tesser
from PIL import Image

# Import ImageGrab if possible, might fail on Linux
global use_grab
try:
    from PIL import ImageGrab, ImageOps, ImageEnhance
    use_grab: bool = True
except Exception as e:
    # Some older versions of pillow don't support ImageGrab on Linux
    # In which case we will use XLib
    if (sys.platform == "linux"):
        from Xlib import display, X
        use_grab: bool = False
    else:
        print(f"PIL unavailable for your system.")
        raise e

class Game:
    def __init__(self, settings: dict, mapSettings: dict, console: Console):
        tesser.pytesseract.tesseract_cmd = settings["tesseractPath"]
        self.settings: dict = settings
        self.map: dict = mapSettings
        self.console: Console = console
        self.towers: dict = {}
        self.make_towers()

    def make_towers(self) -> dict:
        self.towers: dict = {}
        for tower in self.map["towers"]:
            self.towers[tower] = Tower(self.map["towers"][tower], self.settings)
        return self.towers

    def start(self) -> bool:
        actionParser: ActionParser = ActionParser(self.towers, self.console, self.settings)
        currRound: int = 0
        restartInst: bool = False
        while restartInst == False:
            currentRounds = self.get_round()
            if currentRounds != [] and currRound != currentRounds[0]:
                currentRound = currRound = currentRounds[0]
                if currentRound in self.map["instructions"]:
                    instructions: list[str] = self.map["instructions"][currentRound]
                    for instruction in instructions:
                        #print(instruction)
                        if instruction == "restart":
                            restartInst: bool = True
                            break
                        actionParser.do_action_from_string(instruction)
                self.console.progress_bar(int(currentRound), int(currentRounds[1]))
            sleep(1)
        return self.restart_game()

    def restart_game(self) -> bool:
        while (result := (self.get_text(True), self.get_text(False))) not in ("VICTORY", "GAME OVER"):
            sleep(5)
        pygui.click(self.settings["game"]["next"])
        sleep(0.1)
        pygui.click(self.settings["game"]["freeplay"])
        sleep(0.1)
        pygui.click(self.settings["game"]["freeplayOk"])
        sleep(0.1)
        pygui.press(self.settings["game"]["menuHotkey"])
        sleep(0.1)
        pygui.click(self.settings["game"]["restartGame"])
        sleep(0.1)
        pygui.click(self.settings["game"]["confirm"])
        return result == "VICTORY"

    ### GET ROUND USING PYTESSERACT ###
    def get_round(self) -> str:
        image: Image = self.screen_grab(self.settings["game"]["roundCounter"])
        text: str = tesser.image_to_string(image, config=f"-c tessedit_char_whitelist=0123456789/ --psm 6", nice=1)
        text = ''.join([c for c in text if c in "0123456789/"])
        return text.split('/') if text != '' else []

    def get_text(self, victory: bool = True) -> str:
        if victory:
            image: Image = self.screen_grab(self.settings["game"]["victoryBanner"], "gold")
        else:
            image: Image = self.screen_grab(self.settings["game"]["defeatBanner"], "red")
        text: str = tesser.image_to_string(image, config="--psm 6", nice=1)
        text = ''.join([c for c in text.upper() if c in "VICTORY GAME"])
        return text

    def screen_grab(self, rectangle: list[Union[float, int]], colorFilter: str = 'white'):
        global use_grab
        x, y, width, height = rectangle

        if (use_grab):
            image = ImageGrab.grab(bbox=[x, y, x+width, y+height])

            if colorFilter == 'gold':
                image_data = image.getdata()
                new_image_data = []

                for data in image_data:
                    if data[0] == 255 and data[1] in range(212, 256) and data[2] == 0:
                        new_image_data.append((255, 255, 255))
                    else:
                        new_image_data.append((0,0,0))
                image.putdata(new_image_data)

            elif colorFilter == 'red':
                image_data = image.getdata()
                new_image_data = []

                for data in image_data:
                    if data[0] == 255:
                        new_image_data.append((255, 255, 255))
                    else:
                        new_image_data.append((0,0,0))
                image.putdata(new_image_data)

            enhancer = ImageEnhance.Contrast(image)
            image = enhancer.enhance(2)
            image = ImageOps.invert(image)
            image = image.point(lambda x: 0 if x < 10 else 255)
            image = image.convert('L')
        else:
            # ImageGrab can be missing under Linux
            dsp = display.Display()
            root = dsp.screen().root
            raw_image = root.get_image(x, y, width, height, X.ZPixmap, 0xffffffff)
            image = Image.frombuffer(
                "RGB", (width, height), raw_image.data, "raw", "BGRX", 0, 1)

            # TODO: If on linux edit the image to have less noise
        # | Debug Image
        # image.save(f"C:\\Users\\User\\Documents\\temp.png", "PNG")
        return image
