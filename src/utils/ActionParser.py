import pydirectinput

from .Console import Console
from .Tower import Tower
from time import sleep
import pyautogui as pygui

from typing import Union


class ActionParser:
    def __init__(self, towers: dict, console: Console, settings: dict):
        self.console = console
        self.towers = towers
        self.settings = settings

    def do_action_from_string(self, string: str):
        stringArray = [str(s).lower() for s in string.split(' ')]
        if stringArray[0] == "start":
            self.press_keys(self.settings["game"]["startHotkey"], self.settings["game"]["startHotkey"])
        elif stringArray[0] == "place":
            self.place_tower(stringArray[1])
        elif stringArray[0] == "upgrade":
            self.upgrade_tower(stringArray[1], stringArray[2])
        elif stringArray[0] == "sell" or stringArray[0] == "remove":
            self.sell_tower(stringArray[1])
        elif stringArray[0] == "ability":
            self.use_ability(stringArray[1:])
        elif stringArray[0] == "obstacle" or stringArray[0] == "clear":
            self.clear_obstacle(stringArray[1], stringArray[2])
        elif stringArray[0] == "click":
            self.move_mouse(stringArray[1], stringArray[2], click=True)
        elif stringArray[0] == "hover":
            self.move_mouse(stringArray[1], stringArray[2], click=False)
        elif stringArray[0] == "sleep":
            sleep(float(stringArray[1]))
        elif stringArray[0] == "target":
            self.change_target(stringArray[1], stringArray[2])
        else:
            raise ValueError(f"Unknown instruction \"{stringArray[0]}\"")

    def find_tower(self, towerName: str) -> Tower:
        return self.towers[towerName]

    def change_target(self, towerName: str, amountOfTabsRequired: str):
        pygui.moveTo(self.find_tower(towerName).coords)
        pygui.click()
        for i in range(int(amountOfTabsRequired)):
            pydirectinput.press('tab')
        pygui.moveTo(self.find_tower(towerName).coords)
        pygui.click()

    def place_tower(self, towerName: str):
        self.find_tower(towerName).place()

    def upgrade_tower(self, towerName: str, path: str):
        self.find_tower(towerName).upgrade(path)

    def sell_tower(self, towerName: str):
        self.find_tower(towerName).sell()

    def use_ability(self, ability: list[str]):
        if len(ability) == 1:
            self.press_keys(ability[0])
        elif len(ability) == 3:
            self.press_keys(ability[0])
            self.move_mouse(ability[1], ability[2], click=True)
        elif len(ability) == 5:
            self.press_keys(ability[0])
            self.move_mouse(ability[1], ability[2], click=True)
            self.move_mouse(ability[4], ability[5], click=True)
        else:
            # Print warning for missuse of ability function
            print(f"error: ability should have 1, 3, or 5 words, not {len(ability)}...")
            self.press_keys(ability[0])

    @staticmethod
    def clear_obstacle(obstacle: str, confirm: str):
        oCoords = [float(coord) for coord in obstacle.split(',')]
        cCoords = [float(coord) for coord in confirm.split(',')]
        pygui.moveTo(oCoords)
        sleep(0.2)
        pygui.click()
        sleep(0.2)
        pygui.moveTo(cCoords)
        sleep(0.2)
        pygui.click()

    def move_mouse(self, action: str, location: str, click: bool):
        if action == "tower":
            self.find_tower(location).select(click=click)
        else:
            self.move_mouse([float(s) for s in location.split(',')], click=click)

    @staticmethod
    def press_keys(*argv: Union[str, list[str]]):
        for arg in argv:
            pydirectinput.press(arg)
            sleep(0.25)
