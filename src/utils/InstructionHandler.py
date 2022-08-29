import time
import pydirectinput
import pyautogui as pygui

from .Console import Console
from .Tower import Tower
from .ActionParser import ActionParser
from time import sleep


class InstructionHandler:
    def __init__(self, settings: dict, map_settings: dict, console: Console):
        self.current_time = None
        self.restartInst = None
        self.curr_round = None
        self.actionParser = None
        self.settings: dict = settings
        self.map: dict = map_settings
        self.console: Console = console
        self.towers: dict = {}
        self.make_towers()
        self.previousRound = 0
        self.executedInstructions = []

    def make_towers(self) -> dict:
        self.towers: dict = {}
        for tower in self.map["towers"]:
            self.towers[tower] = Tower(self.map["towers"][tower], self.settings)
        return self.towers

    def leveled_up(self):
        self.console.levels_gained += 1
        pygui.moveTo(1920/2, 1080/2)
        pygui.click()
        sleep(0.25)
        pygui.click()
        sleep(0.25)
        pydirectinput.press('space')
        sleep(0.25)
        pydirectinput.press('space')
        self.console.print_string('You seem to have leveled up! congratulations:)')

    def start(self):
        self.actionParser: ActionParser = ActionParser(self.towers, self.console, self.settings)
        self.curr_round: int = 0
        self.restartInst: bool = False
        self.current_time: int = round(time.time() * 1000)
        self.executedInstructions = []
        self.make_towers()

    def check_for_instruction(self, args):
        current_round = args["currentRound"]
        Console.currentTime = self.current_time

        if current_round is not None:
            self.current_time = round(time.time() * 1000)

            if current_round in self.map["instructions"]:
                instructions: list[str] = self.map["instructions"][current_round]

                for instruction in instructions:
                    if instruction not in self.executedInstructions:
                        self.console.welcome_screen()
                        self.console.show_stats()
                        self.console.print_time_of_last_action(instruction)

                        self.actionParser.do_action_from_string(instruction)

                        self.executedInstructions.append(instruction)

                        if isinstance(current_round, str) and len(current_round) > 0 and int(current_round) > 0:
                            self.console.progress_bar(int(current_round), args["waves"])

            if isinstance(current_round, str) and len(current_round) > 0 and int(current_round) > 0:
                self.console.progress_bar(int(current_round), args["waves"])

    def start_freeplay(self) -> bool:
        pygui.moveTo(self.settings["game"]["nextButton"])
        pygui.click()
        sleep(0.25)
        pygui.moveTo(self.settings["game"]["freeplay"])
        pygui.click()
        sleep(2)
        pygui.moveTo(self.settings["game"]["confirm"])
        pygui.click()
        sleep(1)
        pydirectinput.press('space')
        return True

    def restart_after_freeplay(self) -> bool:
        pygui.click()
        pygui.click()
        sleep(2)
        pygui.click()
        sleep(0.25)
        pydirectinput.press('esc')
        sleep(1)
        pygui.moveTo(self.settings["game"]["restartGame"])
        pygui.click()
        sleep(1)
        pygui.moveTo(self.settings["game"]["confirm"])
        pygui.click()
        sleep(1)
        return True

    def restart_game(self) -> bool:
        pygui.moveTo(self.settings["game"]["nextButton"])
        pygui.click()
        sleep(0.25)
        pygui.moveTo(self.settings["game"]["freeplay"])
        pygui.click()
        sleep(2)
        pygui.press(self.settings["game"]["menuHotkey"])
        sleep(1)
        pygui.press(self.settings["game"]["menuHotkey"])
        sleep(1)
        pygui.moveTo(self.settings["game"]["restartGame"])
        pygui.click()
        sleep(1)
        pygui.moveTo(self.settings["game"]["confirm"])
        pygui.click()
        sleep(1)
        return True

    def restart_defeat(self) -> bool:
        pygui.moveTo(self.settings["game"]["defeatNextButton"])
        pygui.click()
        sleep(1)
        pygui.moveTo(self.settings["game"]["defeatRestartButton"])
        pygui.click()
        return True

    def restart_gameover(self) -> bool:
        pygui.moveTo(self.settings["game"]["gameoverNextButton"])
        pygui.click()
        sleep(1)
        pygui.moveTo(self.settings["game"]["gameoverRestartButton"])
        pygui.click()
        return True

