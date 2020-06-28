# Bloons TD 6 Auto Play

## Usage

This requires games to be played in fullscreen with auto-start to work.
Additionally, this is designed to work for those who have upgrades including:

- Free Dart Monkey
- Additional $200 starting cash
- Half of 1st Military Monkey
- Military Monkey Upgrade Discounts

In addition to Python3.7 or higher, you will need to run the following code in a terminal:

```bash
pip install pyautogui opencv-python
```

- To begin the script, open the game to the main screen and select Obyn as your hero.
- Next, open a terminal and use `cd` to navigate to the script.
- In terminal, run `run.bat` or `run.sh` depening on your os.
- Once in python interactive terminal, follow on screen instructions.

## Settings

We have a JSON file now being read with all of the timings and settings for the game loop. We have added this for ease of use and hope you all enjoy and can follo along with the names of the variables.

### Notice

If an error occurs, don't worry. We have it running interactive consoles so you can run any function from the console of your choice.

```python3
    startUp()      # starts dark castle map and runs game loop
    darkCastle()   # runs game loop
```
