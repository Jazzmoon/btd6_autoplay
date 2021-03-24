# Hello mark

from json import load
from classes.Game import Game
from classes.Console import Console
with open('settings.json', 'r') as data: settings = load(data)

console = Console()
gameMap = console.introScreen()
game = Game(gameMap, settings, console)
