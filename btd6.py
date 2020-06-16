import pyautogui as pg
import time, sys

sW, sH = pg.size()

xPosResting, yPosResting = 1500, 1000

def press(key):
    pg.press(key)
    pass

def moveToRest(delay=27):
    pg.moveTo(xPosResting, yPosResting)
    for i in range(delay):
        levelUpCheck()
        pg.click()
        press('1')
        time.sleep(1)
    pass

def startUp(preset='darkCastle'):
    pg.click(800, 900) # Play
    time.sleep(0.5)
    if preset == 'darkCastle':
        pg.click(1400, 950) # Expert
        time.sleep(0.5)
        pg.click(1500, 600) # Dark castle
        time.sleep(0.5)
        pg.click(550, 400) # Easy
        time.sleep(0.5)
        pg.click(550, 600) # Standard
    time.sleep(5.2)
    return True

def restartGame():
    while True:
        time.sleep(2)
        locationOfNext = pg.locateOnScreen('./images/next.png', confidence=0.3)

        if  locationOfNext != None:
            pg.click(960, 911) # Next
            time.sleep(0.5)
            pg.click(1150, 911) # Freeplay
            time.sleep(0.7)
            pg.click(980, 770) # Okay
            time.sleep(1.3)
            pg.click(1585, 45) # Menu
            time.sleep(0.5)
            pg.click(1100, 911) # Restart
            time.sleep(0.5)
            pg.click(1180, 760) # Confirm
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
    pg.moveTo(549, 617)
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
    pg.moveTo(549, 489)
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
    pg.moveTo(1085, 409)
    levelUpCheck()
    pg.click()
    levelUpCheck()

    # Sub Done

    moveToRest(40)
    levelUpCheck()

    # Upgrade Sub: 2-0-0
    pg.moveTo(1085, 409)
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
    pg.moveTo(1085, 409)
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
    pg.moveTo(1085, 409)
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
    pg.moveTo(1085, 409)
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
    pg.moveTo(1085, 409)
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

startUp(preset='darkCastle')