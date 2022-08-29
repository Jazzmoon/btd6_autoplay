import time
import datetime

from os import name, system, get_terminal_size


class TermColor:
    PURPLE: str = "\033[95m"
    BLUE: str = "\033[94m"
    GREEN: str = "\033[92m"
    YELLOW: str = "\033[93m"
    RED: str = "\033[91m"
    DEFAULT: str = "\033[0m"
    BOLD: str = "\033[1m"
    UNDERLINE: str = "\033[4m"


class ProgressBarSettings:
    prefix: str = "Progress: ["
    suffix: str = "] [PERCENT]%"
    completed: str = '#'
    null: str = '-'


class Console:
    def __init__(self):
        self.__wins: int = 0
        self.__loss: int = 0
        self.__gamesPlayed: int
        self.__levelsGained: int = 0
        self.__startTime: int = round(time.time() * 1000)
        self.__currentTime: int = round(time.time() * 1000)
        self.__instasGained: int = 0
        self.__instasPerHour: float = 0.0


    @property
    def startTime(self) -> int:
        return self.__startTime

    @property
    def wins(self) -> int:
        return self.__wins

    @wins.setter
    def wins(self, value: int):
        self.__wins = value

    @property
    def instasPerHour(self) -> float:
        return self.__instasPerHour

    @instasPerHour.setter
    def instasPerHour(self, value: float):
        self.__instasPerHour = value

    @property
    def instasGained(self) -> int:
        return self.__instasGained

    @instasGained.setter
    def instasGained(self, value: int):
        self.__instasGained = value

    @property
    def levels_gained(self) -> int:
        return self.__levelsGained

    @levels_gained.setter
    def levels_gained(self, value: int):
        self.__levelsGained = value

    @property
    def currentTime(self) -> int:
        return self.__currentTime

    @currentTime.setter
    def currentTime(self, value: int):
        self.__currentTime = value

    @property
    def loss(self) -> int:
        return self.__loss

    @loss.setter
    def loss(self, value: int):
        self.__loss = value

    @staticmethod
    def clear_screen():
        system("cls" if name == "nt" else "printf \"\033c\"")

    def welcome_screen(self):
        self.clear_screen()
        print(f"{TermColor.PURPLE}               (                             \n   (    *   ) )\ )  (        (           )  \n ( )\ ` )  /((()/(  )\ )   ( )\       ( /(  \n )((_) ( )(_))/(_))(()/(   )((_)  (   )\()) \n((_)_ (_(_())(_))_  /(_)) ((_)_   )\ (_))/  \n | _ )|_   _| |   \(_) /   | _ ) ((_)| |_   \n | _ \  | |   | |) |/ _ \  | _ \/ _ \|  _|  \n |___/  |_|   |___/ \___/  |___/\___/ \__|  \n")
        print(f"\n{TermColor.RED}BTD6 Bot. Created by: Remy van Lis\n{self.screen_bar()}")

    def print_string(self, string: str):
        self.print_new_lines()
        print(f"{string}")

    @staticmethod
    def print_time_of_last_action(string: str):
        print(f"[{str(datetime.timedelta(milliseconds=round(time.time() * 1000))).split(' ')[2].split('.')[0]}] Action: {string}")

    def show_stats(self):
        print(f"Wins: {TermColor.GREEN}{self.wins}\n{TermColor.DEFAULT}Losses: {TermColor.RED}{self.loss}\n{TermColor.DEFAULT}Games Played: {TermColor.YELLOW}{self.__wins + self.__loss}\n{TermColor.DEFAULT}Levels gained: {TermColor.BLUE}{self.levels_gained}\n{TermColor.DEFAULT}Insta monkeys gained: {TermColor.PURPLE}{self.instasGained}\n{TermColor.DEFAULT}Insta monkeys per hour: {TermColor.BOLD}{self.instasPerHour}\n{TermColor.DEFAULT}Running for: {TermColor.DEFAULT}{(self.currentTime - self.__startTime) / 1000}s\n{self.screen_bar()}")

    @staticmethod
    def game_logs_below():
        print("Current Game Logs:")

    @staticmethod
    def print_new_lines(amount: int = 1):
        print(("\n" * amount))

    @staticmethod
    def progress_bar(completed: int, total: int):
        size = round(get_terminal_size().columns / 2)
        percentage = round((completed / total) * 100)
        suffix = ProgressBarSettings.suffix.replace("[PERCENT]", str(percentage))
        barSize = size - len(ProgressBarSettings.prefix) - (len(suffix))
        completedBar = int(percentage / 100 * barSize)
        nullBar = barSize - completedBar
        bar = f"{TermColor.GREEN}{(ProgressBarSettings.completed * completedBar)}{TermColor.DEFAULT}{(ProgressBarSettings.null * nullBar)}"
        print(f"\r{ProgressBarSettings.prefix}{bar}{suffix}", end='')

    @staticmethod
    def screen_bar() -> str:
        return TermColor.PURPLE + ('-' * (round(get_terminal_size().columns) - 1)) + TermColor.DEFAULT


