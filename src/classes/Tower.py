from pyautogui import click, press, moveTo, center, size
from time import sleep
from os import get_terminal_size as gts
from typing import Optional, Union

class Tower:
    def __init__(self,
            name: str,
            hotkey: str,
            coords: Optional[Union[list, tuple]] = None) -> None:
        if type(name) != str:
            return TypeError("The name of a Tower must be a string.")
        if type(hotkey) != str:
            return TypeError("The hotkey must be a string.")
        if coords != None and \
                (type(coords) not in (list, tuple) or len(coords) != 2):
            return TypeError("The coordinates of a Tower must be list or tuple of length 2.")
        self.name: str = name
        self.hotkey: str = hotkey
        self.path: tuple = (0, 0, 0)
        if coords:
            self.pos: tuple[Union[int, float]] = (coords[0], coords[1])
        else:
            sW, sH = size()
            self.pos: tuple[Union[int, float]] = (sW / 4, sH / 4)

    def print(self, text: str):
        print(f'{" " * (gts().columns - 5)}\r\033[A{text}\n')
        return self

    def get_coords(self):
        return self.coords

    def place(self):
        return self

    def select(self):
        return self

    def deselect(self):
        return self

    def upgrade(self, path, times=1):
        return self

    def ability(self, ability):
        return self
