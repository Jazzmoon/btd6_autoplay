from pyautogui import click, press, moveTo, locateOnScreen, center, size
from time import sleep
from importlib import import_module as im


class Game:
    def __init__(self, gameMap, settings, console):
        self.gameMap = gameMap
        self.console = console
        self.settings = settings
        moduleName = f'maps.{self.gameMap.lower()}'
        mapLoop = im(moduleName, 'mapLoop')
        self.mapLoop = mapLoop.mapLoop
        self.loadMapLoop()

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
        i = 0
        while i < delay:
            self.console.progressBar(i, delay, self.settings['progressBar'])
            click()
            sleep(0.2)
            click()
            press('1')
            sleep(0.8)
            i += 1
        self.console.progressBar(delay, delay, self.settings['progressBar'])
        sleep(0.2)

    def restartGame(self):
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
                click(self.settings['restartConfirmButtonLocation']['x'],
                      self.settings['restartConfirmButtonLocation']['y'])  # Confirm
                sleep(1)
                self.console.increase('wins', 1)
                self.console.increase('gamesPlayed', 1)
                self.loadMapLoop()
                break
            if counter >= 60:
                click(self.getPosOf('./images/restart/menu.png'))  # Menu
                sleep(0.5)
                click(self.getPosOf('./images/restart/restart.png'))  # Restart
                sleep(0.5)
                click(self.settings['restartConfirmButtonLocation']['x'],
                      self.settings['restartConfirmButtonLocation']['y'])  # Confirm
                sleep(1)
                self.console.increase('loss', 1)
                self.console.increase('gamesPlayed', 1)
                self.loadMapLoop()
                break

    def loadMapLoop(self):
        self.console.inGameScreen()
        self.mapLoop(self, self.settings[self.gameMap.lower()])
        self.restartGame()
