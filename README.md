# Bloons TD 6 Auto Play

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

Additionally, you will need to install Pytesseract's `Tesseract` dependancy in order to use Pytesseract. This is pretty irritating to do, so I will link some guides that better explain it than I ever could:

Using Conda: [here](https://pythonforundergradengineers.com/how-to-install-pytesseract.html)
Tesseract Source Code: [here](https://github.com/tesseract-ocr/tesseract)

We recommend using the latest version of Python, version 3.9.5 at the moment of writing this, as Pytesseract seems to have gotten a bit faster in terms of processing the images for text. This is not a requirement though, and should run in Python 3.8.

- To begin the Dark Castle game loop, open the game to the main screen and select Obyn as your hero.
- Next, open a terminal and use `cd` to navigate to the script locations.
- In terminal, run `scripts/run.*`, where the extension depending on your os.
- Once in python interactive terminal, follow on screen instructions. Make sure you type name of map as found in maps folder. (.py extension is not needed)

## Settings

We have a JSON file now being read with all of the timings and settings for the game loop. We have added this for ease of use and hope you all enjoy and can follow along with the names of the variables.

## Custom Maps

See `logs.json` for example game map.
