from classes.Tower import Tower
from classes.Game import Game

from pyautogui import click, press, moveTo, locateOnScreen, center, size
from os import system, name
from time import sleep
from json import load

global wins, loss, gamesPlayed, settings, game
wins = 0 ; loss = 0 ; gamesPlayed = 0 ; game = Game()

with open('settings.json', 'r') as data: settings = load(data)

if not(settings['optionalUpgrades']['dart0-2-3']):
    settings['delays']['waitTillEnd'] += settings['delays']['optionals']['sub.2-0-4ToDartUpgrade0-2-2']
    settings['delays']['waitTillEnd'] += settings['delays']['optionals']['dart.0-2-3ToDartUpgrade0-2-4']
elif not(settings['optionalUpgrades']['dart0-2-4']):
    settings['delays']['waitTillEnd'] += settings['delays']['optionals']['dart.0-2-3ToDartUpgrade0-2-4']

settings['delays']['waitTillEnd'] += settings['delays']['optionals']['sub.2-0-4ToDartUpgrade0-2-2'] if not(settings['optionalUpgrades']['dart0-2-3']) else 0

# Console Functions
def inGameScreen():
        system('cls' if name == 'nt' else "printf '\033c'")
        print(
            f'BTD6 Auto Play. Created by Team Jazzmoon | Use Ctrl + C then exit() to quit the script\nWins: {wins}\nLosses: {loss}\nGames Played: {gamesPlayed}\n------------------\nCurrent Game Logs\n------------------')

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
            print(
                'BTD6 Auto Play. Created by Team Jazzmoon | Use Ctrl + C then exit() to quit the script')

# Auxiliary Function for restartGame
def getPosOf(image, conf=0.3):
    location = locateOnScreen(image, confidence=conf)
    if location == None:
        return None
    locationCenter = center(location)
    return locationCenter

# Automatically load the map from start screen
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
        sleep(5)
        darkCastle()
    return True

# Restart the game automatically using image location
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

# The Dark Castle Game Loop
def darkCastle():
    inGameScreen()
    sleep(1)
    obyn = Tower('u', './images/maps/dark_castle/obynLocation.png', settings['tolerances']['obyn'], "Obyn")
    dart = Tower('q', './images/maps/dark_castle/dartLocation.png', settings['tolerances']['dart'], "Dart")
    sub = Tower('x', './images/maps/dark_castle/subLocation.png', settings['tolerances']['sub'], "Sub")
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
    dart.upgrade('/')
    dart.upgrade('/')
    # Upgrade Done
    game.moveToRest(settings['delays']['dartUpgradeToSub'])
    # Sub
    sub.place()
    # Sub Done
    game.moveToRest(settings['delays']['subToSubUpgrade.2-0-0'])
    # Upgrade Sub: 2-0-0
    sub.upgrade(',')
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
        game.moveToRest(settings['delays']['optionals']['sub.2-0-4ToDartUpgrade.0-2-3'])
        # Upgrade Dart 0-2-3
        dart.upgrade('.')
        dart.upgrade('.')
        dart.upgrade('/')
        # Upgrade Done
        if settings['optionalUpgrades']['dart0-2-4']:
            game.moveToRest(settings['delays']['optionals']['dart.0-2-3ToDartUpgrade.0-2-4'])
            # Upgrade Dart 0-2-3
            dart.upgrade('/')
            # Upgrade Done
    game.moveToRest(settings['delays']['waitTillEnd'])
    # End Game
    restartGame()

# Execute the code upon being loaded
introScreen()