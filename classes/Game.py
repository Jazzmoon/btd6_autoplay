from pyautogui import click, press, moveTo, locateOnScreen, center, size
from time import sleep

class Game:
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
