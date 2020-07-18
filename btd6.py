from pyautogui import click, press, moveTo, locateOnScreen, center, size
from time import sleep
from os import name, system
from json import load

global console, game, settings
with open('settings.json', 'r') as data: settings = load(data)

#! Classes
class Tower:
    def __init__(self, hotkey, image, conf, name):
        self.hotkey = hotkey
        self.pos = self.getPos(image, conf)
        self.name = name
        self.path = ['0', '0', '0']

    def getPos(self, image, conf=0.3):
        location = locateOnScreen(image, confidence=conf)
        if location == None:
            return None
        locationCenter = center(location)
        return locationCenter

    def place(self):
        print(f'{self.name}: Place')
        press(self.hotkey)
        sleep(0.1)
        moveTo(self.pos)
        click()

    def select(self):
        print(f'{self.name}: Select')
        moveTo(self.pos)
        click()
        sleep(0.2)

    def upgrade(self, path, times=1):
        if path == ',':
            self.path[0] = str(int(self.path[0]) + times)
        elif path == '.':
            self.path[1] = str(int(self.path[1]) + times)
        elif path == '/':
            self.path[2] = str(int(self.path[2]) + times)
        print(f"{self.name}: Upgrade | {'-'.join(self.path)}")
        moveTo(self.pos)
        click()
        sleep(0.2)
        for i in range(times):
            press(path)
            sleep(0.2)
class Game:
    def getPosOf(self, image, conf=0.3):
        location = locateOnScreen(image, confidence=conf)
        if location == None:
            return None
        locationCenter = center(location)
        return locationCenter
    def start(self):
        sleep(0.5)
        press('space')
        sleep(0.1)
        press('space')
    def moveToRest(self, delay=27):
        sW, sH = size()
        moveTo(sW / 2, sH / 2)
        for i in range(delay):
            click()
            sleep(0.2)
            click()
            press('1')
            sleep(0.8)
    def restartGame(self, console):
        counter = 0
        while True:
            sleep(2)
            counter += 1
            locationOfNext = self.getPosOf('./images/restart/next.png')
            if locationOfNext != None:
                click(locationOfNext)  # Next
                sleep(0.5)
                click(self.getPosOf('./images/restart/freeplay.png'))  # Freeplay
                sleep(0.7)
                click(self.getPosOf('./images/restart/ok.png'))  # Okay
                sleep(1.3)
                click(self.getPosOf('./images/restart/menu.png'))  # Menu
                sleep(0.5)
                click(self.getPosOf('./images/restart/restart.png'))  # Restart
                sleep(0.5)
                click(settings['restartConfirmButtonLocation']['x'],
                      settings['restartConfirmButtonLocation']['y'])  # Confirm
                sleep(1)
                console.increase('wins', 1)
                console.increase('gamesPlayed', 1)
                self.darkCastle(console)
                break
            if counter >= 60:
                click(self.getPosOf('./images/restart/menu.png'))  # Menu
                sleep(0.5)
                click(self.getPosOf('./images/restart/restart.png'))  # Restart
                sleep(0.5)
                click(settings['restartConfirmButtonLocation']['x'],
                      settings['restartConfirmButtonLocation']['y'])  # Confirm
                sleep(1)
                console.increase('loss', 1)
                console.increase('gamesPlayed', 1)
                self.darkCastle(console)
                break
    def startUp(self, console, preset='darkCastle'):
        sleep(3)
        click(self.getPosOf('./images/main/play.png'))  # Play
        sleep(0.7)
        if preset == 'darkCastle':
            click(self.getPosOf('./images/main/expert.png'))  # Expert
            sleep(0.3)
            # Dark castle
            click(self.getPosOf('./images/maps/dark_castle/playCard.png'))
            sleep(0.3)
            click(self.getPosOf('./images/main/easy.png'))  # Easy
            sleep(0.3)
            click(self.getPosOf('./images/main/standard.png', 0.4))  # Standard
            sleep(5)
            self.darkCastle(console)
        return True
    def darkCastle(self, console):
        console.inGameScreen()
        sleep(1)
        obyn = Tower('u', './images/maps/dark_castle/obynLocation.png',
                     settings['tolerances']['obyn'], "Obyn")
        dart = Tower('q', './images/maps/dark_castle/dartLocation.png',
                     settings['tolerances']['dart'], "Dart")
        sub = Tower('x', './images/maps/dark_castle/subLocation.png',
                    settings['tolerances']['sub'], "Sub")
        # Start Game
        game.start()
        # Start Game Done
        # Obyn
        obyn.place()
        # Obyn Done
        sleep(settings['delays']['obynToDart'])
        # Dart Monkey
        dart.place()
        # Dart Done
        sleep(settings['delays']['dartToDartUpgrade'])
        # Upgrade Dart: 0-0-2
        dart.upgrade('/', 2)
        # Upgrade Done
        game.moveToRest(settings['delays']['dartUpgradeToSub'])
        # Sub
        sub.place()
        # Sub Done
        game.moveToRest(settings['delays']['subToSubUpgrade.2-0-0'])
        # Upgrade Sub: 2-0-0
        sub.upgrade(',', 2)
        # Upgrade Done
        game.moveToRest(settings['delays']['subToSubUpgrade.2-0-1'])
        # Upgrade Sub: 2-0-1
        sub.upgrade('/')
        # Upgrade Done
        game.moveToRest(settings['delays']['subToSubUpgrade.2-0-2'])
        # Upgrade Sub: 2-0-2
        sub.upgrade('/')
        # Upgrade Done
        game.moveToRest(settings['delays']['subToSubUpgrade.2-0-3'])
        # Upgrade Sub: 2-0-3
        sub.upgrade('/')
        # Upgrade Done
        game.moveToRest(settings['delays']['subToSubUpgrade.2-0-4'])
        # Upgrade Sub 2-0-4
        sub.upgrade('/')
        # Upgrade Done
        if settings['optionalUpgrades']['dart0-2-3']:
            game.moveToRest(settings['delays']['optionals']
                            ['sub.2-0-4ToDartUpgrade.0-2-3'])
            # Upgrade Dart 0-2-3
            dart.upgrade('.', 2)
            sleep(0.5)
            dart.upgrade('/')
            # Upgrade Done
            if settings['optionalUpgrades']['dart0-2-4']:
                game.moveToRest(settings['delays']['optionals']
                                ['dart.0-2-3ToDartUpgrade.0-2-4'])
                # Upgrade Dart 0-2-4
                dart.upgrade('/')
                # Upgrade Done
        game.moveToRest(settings['delays']['waitTillEnd'])
        # End Game
        self.restartGame(console)
class Console:
    def __init__(self, wins=0, loss=0, gamesPlayed=0):
        self.wins = wins
        self.loss = loss
        self.gamesPlayed = gamesPlayed

    def inGameScreen(self):
        system('cls' if name == 'nt' else "printf '\033c'")
        print(
            f'BTD6 Auto Play. Created by Team Jazzmoon | Use Ctrl + C then exit() to quit the script\nWins: {self.wins}\nLosses: {self.loss}\nGames Played: {self.gamesPlayed}\n------------------\nCurrent Game Logs\n------------------')

    def introScreen(self, game):
        system('cls' if name == 'nt' else "printf '\033c'")
        print('BTD6 Auto Play. Created by Team Jazzmoon | Use Ctrl + C then exit() to quit the script')
        while (key := input('Press 1 for startUp, and 2 for darkCastle\n------------------\n>>>')):
            if (key == '1'):
                game.startUp(self)
                break
            elif (key == '2'):
                game.darkCastle(self)
                break
            else:
                system('cls' if name == 'nt' else "printf '\033c'")
                print(
                    'BTD6 Auto Play. Created by Team Jazzmoon | Use Ctrl + C then exit() to quit the script')

    def increase(self, category, count):
        if category == 'wins':
            self.wins += count
        elif category == 'loss':
            self.loss += count
        elif category == 'gamesPlayed':
            self.gamesPlayed += count

console = Console(wins=0, loss=0, gamesPlayed=0) ; game = Game()

if not(settings['optionalUpgrades']['dart0-2-3']):
    settings['delays']['waitTillEnd'] += settings['delays']['optionals']['sub.2-0-4ToDartUpgrade0-2-2']
    settings['delays']['waitTillEnd'] += settings['delays']['optionals']['dart.0-2-3ToDartUpgrade0-2-4']
elif not(settings['optionalUpgrades']['dart0-2-4']):
    settings['delays']['waitTillEnd'] += settings['delays']['optionals']['dart.0-2-3ToDartUpgrade0-2-4']

settings['delays']['waitTillEnd'] += settings['delays']['optionals']['sub.2-0-4ToDartUpgrade0-2-2'] if not(settings['optionalUpgrades']['dart0-2-3']) else 0

# Execute the code upon being loaded
console.introScreen(game)
