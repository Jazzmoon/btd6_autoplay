from typing import Union, Optional
from time import sleep
import pyautogui as pygui
import pydirectinput as pydirectinput


class Tower:
    def __init__(self, mapSettings: dict, settings: dict):
        if type(mapSettings["name"]) != str:
            raise TypeError("Tower: Name should be a string.")
        if type(mapSettings["hotkey"]) not in (str, list):
            raise TypeError("Tower: Hotkey should be a string or list of strings.")
        if type(mapSettings["coords"]) not in (list, tuple) or len(mapSettings["coords"]) != 2:
            raise TypeError("Tower: Coords should be a list or tuple of length 2.")
        self.__name: str = mapSettings["name"]
        self.__hotkey: Union[str, list[str]] = mapSettings["hotkey"]
        self.__coords: Union[list[Union[int, float]],
                             tuple[Union[int, float]]] = mapSettings["coords"]
        self.__path: tuple[int, int, int] = (0, 0, 0)
        self.__settings: dict = settings

    @property
    def name(self) -> str:
        return self.__name

    @property
    def hotkey(self) -> str:
        return self.__hotkey

    @property
    def coords(self) -> Union[list[Union[int, float]], tuple[Union[int, float]]]:
        return self.__coords

    @property
    def path(self) -> tuple[int, int, int]:
        return self.__path

    @path.setter
    def path(self, value: tuple[int]):
        self.__path: tuple[int] = value

    @property
    def settings(self) -> dict:
        return self.__settings

    def place(self):
        pygui.moveTo(self.__coords)
        sleep(0.2)
        pydirectinput.press(self.__hotkey)
        sleep(0.2)
        pygui.click()
        sleep(0.2)
        self.deselect()
        return self

    def select(self, click: Optional[bool]=True):
        pygui.moveTo(self.__coords)
        sleep(0.1)
        if click:
            pygui.click()
            sleep(0.1)
        return self

    def deselect(self):
        sW, sH = pydirectinput.size()
        pygui.moveTo(sW / 2, sH / 2)
        pygui.click()
        return self

    def upgrade(self, path: str):
        self.select()
        newPath = tuple([int(path_item) for path_item in path.split('-')])
        upgradeDiff = tuple([new - old for (new, old) in zip(newPath, self.__path)])
        for _ in range(upgradeDiff[0]):
            pydirectinput.press(self.__settings["game"]["upgradeTopPathHotkey"])
            sleep(0.2)
        for _ in range(upgradeDiff[1]):
            pydirectinput.press(self.__settings["game"]["upgradeMiddlePathHotkey"])
            sleep(0.2)
        for _ in range(upgradeDiff[2]):
            pydirectinput.press(self.__settings["game"]["upgradeBottomPathHotkey"])
            sleep(0.2)
        self.__path = newPath
        self.deselect()
        return self

    def sell(self):
        self.select()
        sleep(0.25)
        pydirectinput.press(self.__settings["game"]["sellHotkey"])
        self.deselect()
        return self
