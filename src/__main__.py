import threading
from json import load
from time import sleep
import os
import argparse as ag
import sys
import time

from utils.GameStateEnum import GameState
from utils.Console import Console
from utils.InstructionHandler import InstructionHandler
from utils.Statemachine import Statemachine

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

    info = {
        "stop": False,
        "paused": False,
        "victory": False,
        "defeat": False,
        "insta": False,
        "levelup": False,
        "mapsettings": None,
        "isFreeplay": False,
        "currentRound": 0,
        "gameover": False,
        "waves": 0
    }


def instructions():
    info['instructionHandler'].start()

    while not info["stop"]:
        if not info["paused"] and not info["victory"] and not info["defeat"] and not info["insta"] and not info["levelup"]:
            info['instructionHandler'].check_for_instruction(info)
        if info["victory"] and not info["isFreeplay"]:
            console.wins += 1
            info['instructionHandler'].restart_game()
            info['instructionHandler'].start()
        if info["victory"] and info["isFreeplay"]:
            info['instructionHandler'].start_freeplay()
        if info["levelup"]:
            info['instructionHandler'].leveled_up()
        if info["defeat"]:
            console.loss += 1
            info['instructionHandler'].restart_defeat()
            info['instructionHandler'].start()
        if info["gameover"]:
            console.loss += 1
            info['instructionHandler'].restart_gameover()
            info['instructionHandler'].start()
        if info["insta"]:
            console.instasGained += 1
            console.wins += 1
            console.instasPerHour = round(60.0 / ((round((console.currentTime - console.startTime) / 1000) / console.instasGained) / 60), 2)
            info['instructionHandler'].restart_after_freeplay()
            info['instructionHandler'].start()

        sleep(0.2)


def state_machine():
    while True:
        state = info["statemachine"].check_current_state()

        if state == GameState.PAUSED:
            console.print_string(f"Paused script...")
            info["paused"] = True
        else:
            info["paused"] = False

        if state == GameState.VICTORY:
            console.print_string(f"Victory")
            info["victory"] = True
        else:
            info["victory"] = False

        if state == GameState.DEFEAT:
            console.print_string(f"Defeat")
            info["defeat"] = True
        else:
            info["defeat"] = False

        if state == GameState.INSTA:
            console.print_string(f"Got insta monkey!")
            info["insta"] = True
        else:
            info["insta"] = False

        if state == GameState.GAMEOVER:
            console.print_string(f"Game over:(")
            info["gameover"] = True
        else:
            info["gameover"] = False

        if state == GameState.LEVELUP:
            console.print_string(f"Congratulations! You have leveled up:)")
            info["levelup"] = True
        else:
            info["levelup"] = False


def money_state_machine():
    while True:
        # current_money = info["statemachine"].check_current_money()
        # if current_money:
        #     info["money"] = current_money

        info["currentRound"] = info["statemachine"].check_current_round()
        is_freeplay()

        sleep(0.1)


def is_freeplay():
    mode = info["mapsettings"]["rules"]["gamemode"]
    mode = 40 if mode == "easy" else 60 if mode == "medium" else 80 if mode == "hard" else 100
    info["waves"] = mode if not info["isFreeplay"] else info["mapsettings"]["rules"]["waves"] if mode < info["mapsettings"]["rules"]["waves"] else 0

    if info["currentRound"] and not info["isFreeplay"]:
        info["isFreeplay"] = True if mode < info["mapsettings"]["rules"]["waves"] and int(info["currentRound"]) >= int(mode) else False


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

    # Give the user some time to click the btd6 screen
    sleep(2)

    # Print information in the console
    console.welcome_screen()
    console.show_stats()
    console.print_new_lines(2)

    # Create the statemachine and instruction handler
    if settings["tesseract_path"] != "":
        statemachine = Statemachine(console, settings["tesseract_path"])
    else:
        statemachine = Statemachine(console)
    instructionHandler = InstructionHandler(settings, map_settings, console)

    # Set values in the info dictionary
    info["mapsettings"] = map_settings
    info['statemachine'] = statemachine
    info['instructionHandler'] = instructionHandler

    # Set waves and freeplay
    is_freeplay()

    # Create threads for specific tasks
    state_machine_thread = threading.Thread(target=state_machine, args=())
    instruction_thread = threading.Thread(target=instructions, args=())
    money_state_machine_thread = threading.Thread(target=money_state_machine, args=())

    # Start the threads
    state_machine_thread.start()
    instruction_thread.start()
    money_state_machine_thread.start()


except KeyboardInterrupt:
    exit()

except Exception as e:
    print(f"An error has occured:")
    raise e
