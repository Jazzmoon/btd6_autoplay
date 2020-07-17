from os import system, name
from pyautogui import click, press, moveTo, locateOnScreen, center, size
from time import sleep
from json import load

global wins, loss, gamesPlayed, settings
wins = 0 ; loss = 0 ; gamesPlayed = 0

with open('settings.json', 'r') as data: settings = load(data)

if not(settings['optionalUpgrades']['dart0-2-3']):
    settings['delays']['waitTillEnd'] += settings['delays']['optionals']['sub.2-0-4ToDartUpgrade0-2-2']
    settings['delays']['waitTillEnd'] += settings['delays']['optionals']['dart.0-2-3ToDartUpgrade0-2-4']
elif not(settings['optionalUpgrades']['dart0-2-4']):
    settings['delays']['waitTillEnd'] += settings['delays']['optionals']['dart.0-2-3ToDartUpgrade0-2-4']

settings['delays']['waitTillEnd'] += settings['delays']['optionals']['sub.2-0-4ToDartUpgrade0-2-2'] if not(settings['optionalUpgrades']['dart0-2-3']) else 0

def inGameScreen():
    system('cls' if name == 'nt' else "printf '\033c'")
    print(f'BTD6 Auto Play. Created by Team Jazzmoon | Use Ctrl + C then exit() to quit the script\nWins: {wins}\nLosses: {loss}\nGames Played: {gamesPlayed}\n------------------\nCurrent Game Logs\n------------------')

def moveToRest(delay=27):
    sW, sH = size()
    moveTo(sW / 2, sH / 2)
    for i in range(delay):
        click()
        sleep(0.2)
        click()
        press('1')
        sleep(0.8)

def getPosOf(image, conf=0.3):
    location = locateOnScreen(image, confidence=conf)
    if location == None: return None
    locationCenter = center(location)
    return locationCenter

def startUp(preset='darkCastle'):
    sleep(3)
    click(getPosOf('./images/main/play.png')) # Play
    sleep(0.7)
    if preset == 'darkCastle':
        click(getPosOf('./images/main/expert.png')) # Expert
        sleep(0.3)
        click(getPosOf('./images/maps/dark_castle/playCard.png')) # Dark castle
        sleep(0.3)
        click(getPosOf('./images/main/easy.png')) # Easy
        sleep(0.3)
        click(getPosOf('./images/main/standard.png', 0.4)) # Standard
        sleep(3)
        darkCastle()
    return True

def restartGame():
    global wins, loss, gamesPlayed
    counter = 0
    while True:
        sleep(2)
        counter += 1 ; locationOfNext = getPosOf('./images/restart/next.png')
        if  locationOfNext != None:
            click(locationOfNext) # Next
            sleep(0.5)
            click(getPosOf('./images/restart/freeplay.png')) # Freeplay
            sleep(0.7)
            click(getPosOf('./images/restart/ok.png')) # Okay
            sleep(1.3)
            click(getPosOf('./images/restart/menu.png')) # Menu
            sleep(0.5)
            click(getPosOf('./images/restart/restart.png')) # Restart
            sleep(0.5)
            click(settings['restartConfirmButtonLocation']['x'], settings['restartConfirmButtonLocation']['y']) # Confirm
            sleep(1)
            wins += 1 ; gamesPlayed += 1
            darkCastle()
            break

        if counter >= 60:
            click(getPosOf('./images/restart/menu.png')) # Menu
            sleep(0.5)
            click(getPosOf('./images/restart/restart.png')) # Restart
            sleep(0.5)
            click(settings['restartConfirmButtonLocation']['x'], settings['restartConfirmButtonLocation']['y']) # Confirm
            sleep(1)
            loss += 1 ; gamesPlayed += 1
            darkCastle()
            break

def darkCastle():
    inGameScreen()
    sleep(1)
    obynBox = getPosOf('./images/maps/dark_castle/obynLocation.png', settings['tolerances']['obyn']) ; obynLocation = [obynBox[0], obynBox[1]]
    dartBox = getPosOf('./images/maps/dark_castle/dartLocation.png', settings['tolerances']['dart']) ; dartLocation = [dartBox[0], dartBox[1]]
    subBox = getPosOf('./images/maps/dark_castle/subLocation.png', settings['tolerances']['sub']) ; subLocation = [subBox[0], subBox[1]]
    # Start Game
    sleep(0.5)
    press('space')
    sleep(0.1)
    press('space')
    # Start Game Done
    # Obyn
    print('Obyn: Place')
    press('u')
    sleep(0.1)
    moveTo(obynLocation)
    click()
    # Obyn Done
    sleep(settings['delays']['obynToDart'])
    # Dart Monkey
    print('Dart: Place')
    press('q')
    sleep(0.1)
    moveTo(dartLocation)
    click()
    # Dart Done
    sleep(settings['delays']['dartToDartUpgrade'])
    # Upgrade Dart: 0-0-2
    print('Dart: Upgrade | 0-0-2')
    click()
    sleep(0.2)
    press('/')
    press('/')
    sleep(0.2)
    # Upgrade Done
    moveToRest(settings['delays']['dartUpgradeToSub'])
    # Sub
    print('Sub: Place')
    press('x')
    moveTo(subLocation)
    click()
    # Sub Done
    moveToRest(settings['delays']['subToSubUpgrade.2-0-0'])
    # Upgrade Sub: 2-0-0
    print('Sub: Upgrade | 2-0-0')
    moveTo(subLocation)
    click()
    sleep(0.2)
    press(',')
    press(',')
    sleep(0.2)
    # Upgrade Done
    moveToRest(settings['delays']['subToSubUpgrade.2-0-1'])
    # Upgrade Sub: 2-0-1
    print('Sub: Upgrade | 2-0-1')
    moveTo(subLocation)
    click()
    sleep(0.2)
    press('/')
    sleep(0.2)
    # Upgrade Done
    moveToRest(settings['delays']['subToSubUpgrade.2-0-2'])
    # Upgrade Sub: 2-0-2
    print('Sub: Upgrade | 2-0-2')
    moveTo(subLocation)
    click()
    sleep(0.2)
    press('/')
    sleep(0.2)
    # Upgrade Done
    moveToRest(settings['delays']['subToSubUpgrade.2-0-3'])
    # Upgrade Sub: 2-0-3
    print('Sub: Upgrade | 2-0-3')
    moveTo(subLocation)
    click()
    sleep(0.2)
    press('/')
    sleep(0.2)
    # Upgrade Done
    moveToRest(settings['delays']['subToSubUpgrade.2-0-4'])
    # Upgrade Sub 2-0-4
    print('Sub: Upgrade | 2-0-4')
    moveTo(subLocation)
    click()
    sleep(0.2)
    press('/')
    sleep(0.2)
    # Upgrade Done
    if settings['optionalUpgrades']['dart0-2-3']:
        moveToRest(settings['delays']['optionals']['sub.2-0-4ToDartUpgrade.0-2-3'])
        # Upgrade Dart 0-2-3
        print('Dart: Upgrade | 0-2-3')
        moveTo(dartLocation)
        click()
        sleep(0.2)
        press('.')
        press('.')
        press('/')
        sleep(0.2)
        # Upgrade Done
        if settings['optionalUpgrades']['dart0-2-4']:
            moveToRest(settings['delays']['optionals']['dart.0-2-3ToDartUpgrade.0-2-4'])
            # Upgrade Dart 0-2-3
            print('Dart: Upgrade | 0-2-4')
            moveTo(dartLocation)
            click()
            sleep(0.2)
            press('/')
            sleep(0.2)
            # Upgrade Done
    moveToRest(settings['delays']['waitTillEnd'])
    # End Game
    restartGame()

def introScreen():
    system('cls' if name == 'nt' else "printf '\033c'")
    print('BTD6 Auto Play. Created by Team Jazzmoon | Use Ctrl + C then exit() to quit the script')
    while (key := input('Press 1 for startUp, and 2 for darkCastle\n------------------\n>>>')):
        if (key == '1'):
            startUp()
            break
        elif (key == '2'):
            darkCastle()
            break
        else:
            system('cls' if name == 'nt' else "printf '\033c'")
            print('BTD6 Auto Play. Created by Team Jazzmoon | Use Ctrl + C then exit() to quit the script')

introScreen()