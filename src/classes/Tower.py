from pyautogui import click, press, moveTo, locateOnScreen, center, size
from time import sleep
from os import get_terminal_size as gts

class Tower:
    def __init__(self, hotkey, image, conf, name):
        self.hotkey = hotkey
        if isinstance(image, str):
            self.pos = self.getPos(image, conf)
        elif type(image) in [list, tuple] and len(image) == 2:
            self.pos = [image[0], image[1]]
        else:
            sW, sH = size()
            self.pos = [sW / 4, sH / 4]
        self.name = name
        self.path = ['0', '0', '0']

    def print(self, text):
        print(f'{" " * (gts().columns - 5)}\r\033[A{text}\n')
        return self

    def getPos(self, image, conf=0.3):
        location = locateOnScreen(image, confidence=conf)
        if location == None:
            return None
        locationCenter = center(location)
        return locationCenter

    def place(self):
        self.print(f'{self.name}: Place')
        press(self.hotkey)
        sleep(0.1)
        moveTo(self.pos)
        click()
        return self

    def select(self):
        self.print(f'{self.name}: Select')
        moveTo(self.pos)
        click()
        sleep(0.2)
        return self

    def deselect(self):
        sW, sH = size()
        moveTo(sW / 2, sH / 2)
        click()
        return self

    def upgrade(self, path, times=1):
        if path == ',':
            self.path[0] = str(int(self.path[0]) + times)
        elif path == '.':
            self.path[1] = str(int(self.path[1]) + times)
        elif path == '/':
            self.path[2] = str(int(self.path[2]) + times)
        self.print(f'{self.name}: Upgrade | {"-".join(self.path)}')
        moveTo(self.pos)
        click()
        sleep(0.2)
        for _ in range(times):
            press(path)
            sleep(0.2)
        self.deselect()
        return self

    def ability(self, ability):
        self.print(f'{self.name}: Use Ability | {ability}')
        press(ability)
        sleep(0.2)
        return self