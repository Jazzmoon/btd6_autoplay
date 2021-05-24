from typing import Union, Optional

from os import name, system, listdir, get_terminal_size

class termColor:
    PURPLE: str = "\033[95m"
    BLUE: str = "\033[94m"
    GREEN: str = "\033[92m"
    YELLOW: str = "\033[93m"
    RED: str = "\033[91m"
    DEFAULT: str = "\033[0m"
    BOLD: str = "\033[1m"
    UNDERLINE: str = "\033[4m"

class progressBarSettings:
    prefix: str = "Progress: ["
    suffix: str = "] [PERCENT]%"
    completed: str = '#'
    null: str = '-'

class Console:
    def __init__(self):
        self.__wins: int = 0
        self.__loss: int = 0
        self.__gamesPlayed: int = 0


    @property
    def wins(self) -> int:
        return self.__wins

    @wins.setter
    def wins(self, value: int):
        self.__wins = value

    @property
    def loss(self) -> int:
        return self.__loss

    @loss.setter
    def loss(self, value: int):
        self.__loss = value

    @property
    def gamesPlayed(self) -> int:
        return self.__gamesPlayed

    @gamesPlayed.setter
    def gamesPlayed(self, value: int):
        self.__gamesPlayed = value

    def clear_screen(self):
        system("cls" if name == "nt" else "printf \"\033c\"")

    def welcome_screen(self):
        self.clear_screen()

        print(f"{termColor.BLUE} ____ _______ _____    __    ____   ____ _______ \n|  _ \__   __|  __ \  / /   |  _ \ / __ \__   __|\n| |_) | | |  | |  | |/ /_   | |_) | |  | | | |   \n|  _ <  | |  | |  | | '_ \  |  _ <| |  | | | |   \n| |_) | | |  | |__| | (_) | | |_) | |__| | | |   \n|____/  |_|  |_____/ \___/  |____/ \____/  |_|")

        print(f"\n{termColor.DEFAULT}BTD6 Auto Play. Created by Team Jazzmoon | Use Ctrl + C to quit the script\n{self.screen_bar()}")


    def show_stats(self):
        print(f"Wins: {termColor.GREEN}{self.wins}\n{termColor.DEFAULT}Losses: {termColor.RED}{self.loss}\n{termColor.DEFAULT}Games Played: {termColor.YELLOW}{self.gamesPlayed}\n{self.screen_bar()}")

    def game_logs_below(self):
        print("Current Game Logs:")

    def print_new_lines(self, amount: int = 1):
        print(("\n" * amount))


    def progress_bar(self, completed: int, total: int):
        size = round(get_terminal_size().columns / 2)
        percentage = round((completed / total) * 100)
        suffix = progressBarSettings.suffix.replace("[PERCENT]", str(percentage))
        barSize = size - len(progressBarSettings.prefix) - (len(suffix))
        completedBar = int(percentage / 100 * barSize)
        nullBar = barSize - completedBar
        bar = f"{termColor.GREEN}{(progressBarSettings.completed * completedBar)}{termColor.DEFAULT}{(progressBarSettings.null * nullBar)}"
        print(f"\r{progressBarSettings.prefix}{bar}{suffix}", end='')

    def screen_bar(self) -> str:
        return termColor.PURPLE + ('-' * (round(get_terminal_size().columns) - 1)) + termColor.DEFAULT

