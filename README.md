# Bloons TD 6 Auto Play

## Usage

This requires games to be played in fullscreen with auto-start to work.
Additionally, this is designed to work for those who have upgrades including:

- Free Dart Monkey
- Additional $200 starting cash
- Half of 1st Military Monkey
- Military Monkey Upgrade Discounts

You don't need these upgrades if you change the timing.

In addition to Python 3.8 or higher (due to our use of the := operator in Console.py on line 37), you will need to run the following code in a terminal:

```bash
pip install -r requirements.txt # Install requirements for the script
```

- To begin the Dark Castle game loop, open the game to the main screen and select Obyn as your hero.
- Next, open a terminal and use `cd` to navigate to the script locations.
- In terminal, run `run.bat` or `run.sh` depending on your os.
- Once in python interactive terminal, follow on screen instructions. Make sure you type name of map as found in maps folder. (.py extension is not needed)

## Settings

We have a JSON file now being read with all of the timings and settings for the game loop. We have added this for ease of use and hope you all enjoy and can follow along with the names of the variables.

## Custom Maps

We made the new class system to make sure that anyone can make their own maps. Simply import the Tower class using `from classes.Tower import Tower` and follow the `darkcastle.py` example we have already made.

New Tower Example Code:

```python3
hero = Tower(hotkey='u', image='./images/maps/dark_castle/obynLocation.png', conf=settings['tolerances']['obyn'], name="Obyn")
dart = Tower(hotkey='q', image='./images/maps/dark_castle/dartLocation.png', conf=settings['tolerances']['dart'], name="Dart")
```

If you have made a reliable map, feel free to make a pull request and we will review it and potentially add it to the maps folder. Hopefully, every map in the game will have a loop at some point.
