from .Console import Console
import pytesseract as tesser
from PIL import Image
from .GameStateEnum import GameState
from .Screen import Screen


class Statemachine:
    def __init__(self, console: Console, tesseract_path: str = r"C:\Program Files (x86)\Tesseract-OCR\tesseract"):
        tesser.pytesseract.tesseract_cmd = tesseract_path
        self.console: Console = console

    @staticmethod
    def check_victory_state():
        image: Image = Screen.screen_grab([700, 120, 515, 110], "gold")
        text: str = tesser.image_to_string(image, config="--psm 6", nice=1)
        text = ''.join([c for c in text.upper() if c in "VICTORY"])
        if "VICTORY" in text:
            return True

    @staticmethod
    def check_defeat_state():
        image: Image = Screen.screen_grab([723, 288, 473, 117], "red")
        text: str = tesser.image_to_string(image, config="--psm 6", nice=1)
        text = ''.join([c for c in text.upper() if c in "DEFEAT"])
        if "DEFEAT" in text:
            return True

    @staticmethod
    def check_pause_state():
        image: Image = Screen.screen_grab([870, 15, 175, 65])
        text: str = tesser.image_to_string(image, config="--psm 6", nice=1)
        text = ''.join([c for c in text.upper() if c in "PAUSE"])
        if "PAUSE" in text:
            return True

    @staticmethod
    def check_leveled_up_state():
        image: Image = Screen.screen_grab([827, 534, 276, 105])
        text: str = tesser.image_to_string(image, config="--psm 6", nice=1)
        text = ''.join([c for c in text.upper() if c in "LEVEL UP!"])
        if "LEVEL" in text:
            return True

    @staticmethod
    def check_insta_state():
        image: Image = Screen.screen_grab([770, 648, 377, 52], "yellow")
        text: str = tesser.image_to_string(image, config="--psm 6", nice=1)
        text = ''.join([c for c in text.upper() if c in "INSTA-MONKEY!"])
        if "INSTA" in text:
            return True

    @staticmethod
    def check_current_round():
        image: Image = Screen.screen_grab([1380, 28, 180, 45])
        text: str = tesser.image_to_string(image, config=f"-c tessedit_char_whitelist=0123456789/ --psm 6", nice=1)
        text = ''.join([c for c in text if c in "0123456789/"])

        return text.split('/')[0] if '/' in text else text if len(str(text)) > 0 else None

    @staticmethod
    def check_current_round_freeplay():
        image: Image = Screen.screen_grab([1483, 28, 79, 45])
        text: str = tesser.image_to_string(image, config="--psm 6", nice=1)
        text = ''.join([c for c in text if c in "0123456789"])

        return text if len(str(text)) > 0 else None

    @staticmethod
    def check_game_over_freeplay():
        image: Image = Screen.screen_grab([595, 140, 725, 117], "red")
        text: str = tesser.image_to_string(image, config="--psm 6", nice=1)
        text = ''.join([c for c in text if c in "GAME OVER"])
        if "GAMEOVER" in text:
            image.show()

        return True if "GAMEOVER" in text else False

    @staticmethod
    def check_current_money():
        image: Image = Screen.screen_grab([345, 23, 133, 43])
        text: str = tesser.image_to_string(image, config=f"-c tessedit_char_whitelist=0123456789/ --psm 6", nice=1)
        text = ''.join([c for c in text if c in "0123456789"])

        return text if len(str(text)) > 0 else None

    def check_current_state(self):
        if self.check_victory_state():
            return GameState.VICTORY
        if self.check_defeat_state():
            return GameState.DEFEAT
        if self.check_pause_state():
            return GameState.PAUSED
        if self.check_insta_state():
            return GameState.INSTA
        if self.check_leveled_up_state():
            return GameState.LEVELUP
        if self.check_game_over_freeplay():
            return GameState.GAMEOVER
