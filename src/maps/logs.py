from classes.Tower import Tower

def mapLoop(game, settings):
    game.moveToRest(1)
    obyn = Tower('u', settings['positions']['obyn'], 0, "Obyn")
    dart = Tower('q', settings['positions']['dart'], 0, "Dart")
    ninja = Tower('d', settings['positions']['ninja'], 0, "Ninja")
    # Start Game
    game.start()
    # Start Game Done
    obyn.place()
    game.moveToRest(settings['delays']['obynToDart'])
    dart.place()
    game.moveToRest(settings['delays']['dartUpgrade.0-0-2'])
    dart.upgrade('/', 2)
    game.moveToRest(settings['delays']['dartUpgradeToNinja'])
    ninja.place()
    game.moveToRest(settings['delays']['ninjaUpgrade.0-0-1'])
    ninja.upgrade('/')
    game.moveToRest(settings['delays']['ninjaUpgrade.1-0-1'])
    ninja.upgrade(',')
    game.moveToRest(settings['delays']['ninjaUpgrade.2-0-1'])
    ninja.upgrade(',')
    game.moveToRest(settings['delays']['ninjaUpgrade.3-0-1'])
    ninja.upgrade(',')
    game.moveToRest(settings['delays']['ninjaUpgrade.4-0-1'])
    ninja.upgrade(',')
    game.moveToRest(settings['delays']['waitTillEnd'])
    # End Game
