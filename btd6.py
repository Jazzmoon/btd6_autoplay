import pyautogui as pg
import time, sys

sW, sH = pg.size()

def press(key):
    pg.press(key)
    pass

def moveToRest(delay=27):
    pg.moveTo(sW / 2, sH / 2)
    for i in range(delay):
        levelUpCheck()
        pg.click()
        press('1')
        time.sleep(1)
    pass

def getPosOf(image):
    location = pg.locateOnScreen(image, confidence=0.3)

    if location == None return None

    locationCenter = pg.center(location)

    return locationCenter

def startUp(preset='darkCastle'):
    pg.click(getPosOf('./images/main/play.png')) # Play
    time.sleep(0.3)
    if preset == 'darkCastle':
        pg.click(getPosOf('./images/main/expert.png')) # Expert
        time.sleep(0.3)
        pg.click(getPosOf('./images/maps/dark_castle/playCard.png')) # Dark castle
        time.sleep(0.3)
        pg.click(getPosOf('./images/main/easy.png')) # Easy
        time.sleep(0.3)
        pg.click(getPosOf('./images/main/standard.png')) # Standard
    time.sleep(5.2)
    return True

def restartGame():
    while True:
        time.sleep(2)
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
            pg.click(getPosOf('./images/restart/restart.png')) # Restart
            time.sleep(0.5)
            pg.click(getPosOf('./images/restart/confirm.png')) # Confirm
            time.sleep(5)
            darkCastle()
            break
    pass

def levelUpCheck():
    levelUp = pg.locateOnScreen('./images/levelUp.png', confidence=0.3)

    if levelUp != None:
        pg.click()
        time.sleep(2)
        pg.click()
        time.sleep(1)
        press('space')
    pass

def darkCastle():

    obynLocation = getPosOf('./images/maps/dark_castle/obynLocation.png')
    dartLocation = getPosOf('./images/maps/dark_castle/dartLocation.png')
    subLocation = getPosOf('./images/maps/dark_castle/subLocation.png')
    time.sleep(0.5)
    press('space')
    time.sleep(0.1)
    press('space')

    # Obyn
    levelUpCheck()
    press('u')
    levelUpCheck()
    time.sleep(0.1)
    levelUpCheck()
    pg.moveTo(obynLocation)
    levelUpCheck()
    pg.click()
    levelUpCheck()
    # Obyn Done

    time.sleep(0.1)
    levelUpCheck()

    # Dart Monkey
    press('q')
    levelUpCheck()
    time.sleep(0.1)
    levelUpCheck()
    pg.moveTo(dartLocation)
    levelUpCheck()
    pg.click()
    levelUpCheck()
    # Dart Done

    time.sleep(0.1)
    levelUpCheck()

    # Upgrade Dart: 0-0-2
    pg.click()
    levelUpCheck()
    time.sleep(0.2)
    levelUpCheck()
    press('/')
    levelUpCheck()
    press('/')
    levelUpCheck()
    time.sleep(0.2)
    levelUpCheck()
    # Upgrade Done

    moveToRest(10)
    levelUpCheck()

    # Sub
    press('x')
    levelUpCheck()
    pg.moveTo(subLocation)
    levelUpCheck()
    pg.click()
    levelUpCheck()

    # Sub Done

    moveToRest(40)
    levelUpCheck()

    # Upgrade Sub: 2-0-0
    pg.moveTo(subLocation)
    levelUpCheck()
    pg.click()
    levelUpCheck()
    time.sleep(0.2)
    levelUpCheck()
    press(',')
    levelUpCheck()
    press(',')
    levelUpCheck()
    time.sleep(0.2)
    levelUpCheck()
    # Upgrade Done

    moveToRest(27)
    levelUpCheck()

    # Upgrade Sub: 2-0-1
    pg.moveTo(subLocation)
    levelUpCheck()
    pg.click()
    levelUpCheck()
    time.sleep(0.2)
    levelUpCheck()
    press('/')
    levelUpCheck()
    time.sleep(0.2)
    levelUpCheck()
    # Upgrade Done

    moveToRest(45)
    levelUpCheck()

    # Upgrade Sub: 2-0-2
    pg.moveTo(subLocation)
    levelUpCheck()
    pg.click()
    levelUpCheck()
    time.sleep(0.2)
    levelUpCheck()
    press('/')
    levelUpCheck()
    time.sleep(0.2)
    levelUpCheck()
    # Upgrade Done

    moveToRest(52)
    levelUpCheck()

    # Upgrade Sub: 2-0-3
    pg.moveTo(subLocation)
    levelUpCheck()
    pg.click()
    levelUpCheck()
    time.sleep(0.2)
    levelUpCheck()
    press('/')
    levelUpCheck()
    time.sleep(0.2)
    levelUpCheck()
    # Upgrade Done

    moveToRest(63)
    levelUpCheck()

    # Upgrade Sub 2-0-4
    pg.moveTo(subLocation)
    levelUpCheck()
    pg.click()
    levelUpCheck()
    time.sleep(0.2)
    levelUpCheck()
    press('/')
    levelUpCheck()
    time.sleep(0.2)
    levelUpCheck()
    # Upgrade Done

    moveToRest(179)
    levelUpCheck()

    # End Game
    restartGame()
    pass