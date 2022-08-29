from enum import Enum


class GameState(Enum):
    VICTORY = 1
    INSTA = 2
    DEFEAT = 3
    PAUSED = 4
    LEVELUP = 5
    GAMEOVER = 6
