import pyautogui as pg
import time, sys

sW, sH = pg.size()

def press(key):
    pg.press(key)

def moveToRest(delay=27):
    pg.moveTo(sW / 2, sH / 2)
    for i in range(delay):
        pg.click()
        pg.click()
        press('1')
        time.sleep(1)

def getPosOf(image, conf=0.3):
    location = pg.locateOnScreen(image, confidence=conf)
    if location == None: return None
    locationCenter = pg.center(location)
    return locationCenter

def startUp(preset='darkCastle'):
    time.sleep(3)
    pg.click(getPosOf('./images/main/play.png')) # Play
    time.sleep(0.7)
    if preset == 'darkCastle':
        pg.click(getPosOf('./images/main/expert.png')) # Expert
        time.sleep(0.3)
        pg.click(getPosOf('./images/maps/dark_castle/playCard.png')) # Dark castle
        time.sleep(0.3)
        pg.click(getPosOf('./images/main/easy.png')) # Easy
        time.sleep(0.3)
        pg.click(getPosOf('./images/main/standard.png')) # Standard
        time.sleep(3)
        darkCastle()
    return True

def restartGame():
    counter = 0
    while True:
        time.sleep(2)
        counter += 1
        locationOfNext = getPosOf('./images/restart/next.png')

        if  locationOfNext != None:
            pg.click(locationOfNext) # Next
            time.sleep(0.5)
            pg.click(getPosOf('./images/restart/freeplay.png')) # Freeplay
            time.sleep(0.7)
            pg.click(getPosOf('./images/restart/ok.png')) # Okay
            time.sleep(1.3)
            pg.click(getPosOf('./images/restart/menu.png')) # Menu
            time.sleep(0.5)
            pg.click(getPosOf('./images/restart/restart.png'), 0.5) # Restart
            time.sleep(0.5)
            pg.click(getPosOf('./images/restart/confirm.png')) # Confirm
            time.sleep(5)
            darkCastle()
            break

        if counter >= 60:
            pg.click(getPosOf('./images/restart/menu.png')) # Menu
            time.sleep(0.5)
            pg.click(('./images/restart/restart.png')) # Restart
            time.sleep(0.5)
            pg.click(getPosOf('./images/restart/confirm.png')) # Confirm
            time.sleep(5)
            darkCastle()
            break

def darkCastle():
    time.sleep(2.2)
    obynBox = getPosOf('./images/maps/dark_castle/obynLocation.png', 0.5)
    dartBox = getPosOf('./images/maps/dark_castle/dartLocation.png', 0.5)
    subBox = getPosOf('./images/maps/dark_castle/subLocation.png', 0.4)

    obynLocation = [obynBox[0], obynBox[1]]
    dartLocation = [dartBox[0], dartBox[1]]
    subLocation = [subBox[0], subBox[1]]

    time.sleep(0.5)
    press('space')
    time.sleep(0.1)
    press('space')

    # Obyn
    press('u')
    time.sleep(0.1)
    pg.moveTo(obynLocation)
    pg.click()
    # Obyn Done

    time.sleep(0.1)

    # Dart Monkey
    press('q')
    time.sleep(0.1)
    pg.moveTo(dartLocation)
    pg.click()
    # Dart Done

    time.sleep(0.1)

    # Upgrade Dart: 0-0-2
    pg.click()
    time.sleep(0.2)
    press('/')
    press('/')
    time.sleep(0.2)
    # Upgrade Done

    moveToRest(10)

    # Sub
    press('x')
    pg.moveTo(subLocation)
    pg.click()
    # Sub Done

    moveToRest(40)

    # Upgrade Sub: 2-0-0
    pg.moveTo(subLocation)
    pg.click()
    time.sleep(0.2)
    press(',')
    press(',')
    time.sleep(0.2)
    # Upgrade Done

    moveToRest(27)

    # Upgrade Sub: 2-0-1
    pg.moveTo(subLocation)
    pg.click()
    time.sleep(0.2)
    press('/')
    time.sleep(0.2)
    # Upgrade Done

    moveToRest(45)

    # Upgrade Sub: 2-0-2
    pg.moveTo(subLocation)
    pg.click()
    time.sleep(0.2)
    press('/')
    time.sleep(0.2)
    # Upgrade Done

    moveToRest(52)

    # Upgrade Sub: 2-0-3
    pg.moveTo(subLocation)
    pg.click()
    time.sleep(0.2)
    press('/')
    time.sleep(0.2)
    # Upgrade Done

    moveToRest(63)

    # Upgrade Sub 2-0-4
    pg.moveTo(subLocation)
    pg.click()
    time.sleep(0.2)
    press('/')
    time.sleep(0.2)
    # Upgrade Done

    moveToRest(140)

    # End Game
    restartGame()
