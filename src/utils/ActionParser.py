from .Console import Console
from .Tower import Tower
from time import sleep
import pyautogui as pygui

from typing import Union, Optional

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
        elif stringArray[0] == "sell":
            self.sell_tower(stringArray[1])
        elif stringArray[0] == "ability":
            self.use_ability(stringArray[1])
        elif stringArray[0] == "click":
            self.move_mouse(stringArray[1], stringArray[2], click=True)
        elif stringArray[0] == "hover":
            self.move_mouse(stringArray[1], stringArray[2], click=False)
        elif stringArray[0] == "sleep":
            sleep(float(stringArray[1]))
        else:
            raise ValueError(f"Unknown instruction \"{stringArray[0]}\"")

    def find_tower(self, towerName: str) -> Tower:
        return self.towers[towerName]

    def place_tower(self, towerName: str):
        self.find_tower(towerName).place()

    def upgrade_tower(self, towerName: str, path: str):
        self.find_tower(towerName).upgrade(path)

    def sell_tower(self, towerName: str):
        self.find_tower(towerName).sell()
        pass

    def use_ability(self, ability: str):
        self.game.press_keys(ability)

    def move_mouse(self, action: str, location: str, click: bool):
        if action == "tower":
            self.find_tower(location).select(click=click)
        else:
            self.mouse_to([float(s) for s in location.split(',')], click=click)

    def press_keys(self, *argv: Union[str, list[str]]):
        for arg in argv:
            pygui.press(arg)
            sleep(0.25)

    def move_mouse(self, position: tuple[float], click: Optional[bool] = False):
        pygui.moveTo(position)
        if click:
            pygui.click()
