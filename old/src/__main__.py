from json import load
from time import sleep
import os
import argparse as ag
import sys

from utils.Console import Console
from utils.Game import Game

global OS_PATH
global DELIMETER
OS_PATH = os.path.dirname(os.path.realpath(__file__))
if os.name in ['nt']:
    DELIMETER = '\\'
else:
    DELIMETER = '/'

# Fetch command line arguments for map
parser = ag.ArgumentParser()
parser.add_argument("-map", "--map")
parser.add_argument("-gm", "--gamemode")
args = parser.parse_args(sys.argv[1:])

# Create Console
console = Console()

# Load settings file
with open(os.path.join(OS_PATH, "config/settings.json"), 'r') as json_settings:
    settings = load(json_settings)

try:
    console.welcome_screen()

    if not args.map:
        mapName = input("Please choose the map you want to load the configs for >>> ")
    else:
        mapName = args.map

    if not args.gamemode:
        gamemode = input("Please choose the gamemode (no .json extension) >>> ")
    else:
        gamemode = args.gamemode

    mapName = mapName.lower()
    gamemode = gamemode.lower()
    print(f"Loading Config: {os.path.join(OS_PATH, f'config{DELIMETER}maps{DELIMETER}{mapName}{DELIMETER}{gamemode}.json')}")

    with open(os.path.join(OS_PATH, f"config{DELIMETER}maps{DELIMETER}{mapName}{DELIMETER}{gamemode}.json"), 'r') as map_json:
        map_settings = load(map_json)

    sleep(2)

    while True:
        console.welcome_screen()
        console.show_stats()
        console.print_new_lines(2)
        game = Game(settings, map_settings, console)

        gameResult = game.start()

        if gameResult == True:
            console.wins += 1
            console.moneyGenerated += map_settings["moneyPerGame"]
        else:
            console.loss += 1
        console.gamesPlayed += 1

except KeyboardInterrupt:
    exit()

except Exception as e:
    print(f"An error has occured:")
    raise e
    exit()
