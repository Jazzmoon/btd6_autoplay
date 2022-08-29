# Bloons TD 6 Auto Play

## Disclaimer

Please be aware, Ninja Kiwi doesn't support the use of internal or external modding. Under Ninja Kiwi's Terms of Service, scripts such as this one can lead to a ban on your account. Using this, or any other script like it, means that you accept this risk and agree we (Jazzmoon) are not held accountable for any actions taken against your account.

Note: This autoplayer was created by [Team Jazzmoon](https://github.com/Jazzmoon/btd6_autoplay) and built upon further by developer [remyvanlis](https://github.com/remyvanlis). I, remyvanlis, am not associated with them and do not have permissions to edit their repository. Since I wanted to improve the bot, I mostly rebuilt it and created a repository where people could actually contribute to it, available [here](https://github.com/remyvanlis/btd6-bot).

## Usage

This requires games to be played in fullscreen with auto-start to work.
Additionally, this is designed to work for those who have upgrades including:

- Free Dart Monkey
- Additional $200 starting cash
- Half of 1st Military Monkey
- Military Monkey Upgrade Discounts

You don't need these upgrades if you change the timing.

In addition to Python 3.8 (due to our use of the `:=` operator), you will need to run the following code in a terminal:

```bash
pip install -r requirements.txt # Install requirements for the script
```

Additionally, you will need to install Pytesseract's `Tesseract` dependancy in order to use Pytesseract. Simply download the 32-bit installer, install it should simply work. If it does not work, open Statemachine.py and Screen.py, and change the path to yours there. This is a requirement for anything that isn't windows.

We recommend using the latest version of Python, version 3.10.* at the moment of writing this, as Pytesseract seems to have gotten a bit faster in terms of processing the images for text. This is not a requirement though, and should run in Python 3.8.

- To begin the Dark Castle game loop, open the game to the main screen and select Obyn as your hero.
- Next, open a terminal and use `cd` to navigate to the script locations.
- In terminal, run `scripts/run.*`, where the extension is dependant on your os. A `.sh`, `.bat`, and `.ps1` script are privided for your convenience.
- Once in python interactive terminal, follow on screen instructions. Make sure you type name of map as found in maps folder. (`.py` extension is not needed, and should be excluded)

## Settings

We have a JSON file now being read with all of the timings and settings for the game loop. We have added this for ease of use and hope you all enjoy and can follow along with the names of the variables.

## Custom Maps

See `src/config/maps/darkcastle/easy.json` for example game map and basic instructions.

All instructions can be found in `src/utils/ActionParser.py`.
