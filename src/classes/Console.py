from os import name, system, listdir, get_terminal_size
from os.path import isfile, join

class termColor:
    PURPLE = '\033[95m'
    BLUE = '\033[94m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    RED = '\033[91m'
    DEFAULT = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

class Console:
    def __init__(self, wins=0, loss=0, gamesPlayed=0):
        self.wins = wins
        self.loss = loss
        self.gamesPlayed = gamesPlayed
        self.files = [f.replace('.py', '') for f in listdir('./src/maps') if isfile(join('./src/maps', f)) and f != "__init__.py" and f != "example.py"]

    def clear(self):
        system('cls' if name == 'nt' else "printf '\033c'")

    def inGameScreen(self):
        self.clear()
        print(f'BTD6 Auto Play. Created by Team Jazzmoon | Use Ctrl + C then exit() to quit the script\nWins: {self.wins}\nLosses: {self.loss}\nGames Played: {self.gamesPlayed}\n{"-"*18}\nCurrent Game Logs\n{"-"*18}\n')

    def introScreen(self):
        self.clear()
        print('BTD6 Auto Play. Created by Team Jazzmoon | Use Ctrl + C then exit() to quit the script')
        while (key := input(f'Please type the name of the map file\n{"-"*18}\n>>>')):
            if (key.lower() in self.files):
                return key
            else:
                self.clear()
                print('BTD6 Auto Play. Created by Team Jazzmoon | Use Ctrl + C then exit() to quit the script')

    def increase(self, category, count):
        if category == 'wins':
            self.wins += count
        elif category == 'loss':
            self.loss += count
        elif category == 'gamesPlayed':
            self.gamesPlayed += count


    def progressBar(self, completed, total, settings):
        size = round(get_terminal_size().columns / 2)
        percentage = round((completed / total) * 100)
        suffix = settings['suffix'].replace('[PERCENT]', str(percentage))
        barSize = size - len(settings['prefix']) - (len(suffix))
        completedBar = int(percentage / 100 * barSize)
        nullBar = barSize - completedBar
        bar = f'{termColor.GREEN}{(settings["completed"] * completedBar)}{termColor.DEFAULT}{(settings["null"] * nullBar)}'
        print(f'{settings["prefix"]}{bar}{suffix}', end='\r')
