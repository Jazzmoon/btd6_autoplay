from classes.Tower import Tower

def mapLoop(game, settings):
    if not(settings['optionalUpgrades']['dart.x-x-3']):
        settings['delays']['waitTillEnd'] += settings['delays']['optionals']['sub.2-0-4ToDartUpgrade.x-x-3']
        settings['delays']['waitTillEnd'] += settings['delays']['optionals']['dart.x-x-3ToDartUpgrade.x-x-4']
    elif not(settings['optionalUpgrades']['dart.x-x-4']):
        settings['delays']['waitTillEnd'] += settings['delays']['optionals']['dart.x-x-3ToDartUpgrade.x-x-4']

    settings['delays']['waitTillEnd'] += settings['delays']['optionals']['sub.2-0-4ToDartUpgrade.x-x-3'] if not(settings['optionalUpgrades']['dart.x-x-3']) else 0

    game.moveToRest(1)
    obyn = Tower('u', './images/maps/dark_castle/obynLocation.png', settings['tolerances']['obyn'], "Obyn")
    dart = Tower('q', './images/maps/dark_castle/dartLocation.png', settings['tolerances']['dart'], "Dart")
    sub = Tower('x', './images/maps/dark_castle/subLocation.png', settings['tolerances']['sub'], "Sub")
    # Start Game
    game.start()
    # Start Game Done
    # Obyn
    obyn.place()
    # Obyn Done
    game.moveToRest(settings['delays']['obynToDart'])
    # Dart Monkey
    dart.place()
    # Dart Done
    game.moveToRest(settings['delays']['dartToDartUpgrade.0-0-2'])
    # Upgrade Dart
    dart.upgrade('/', 2)
    # Upgrade Done
    game.moveToRest(settings['delays']['dartUpgradeToSub'])
    # Sub
    sub.place()
    # Sub Done
    game.moveToRest(settings['delays']['subToSubUpgrade.2-0-0'])
    # Upgrade Sub
    sub.upgrade(',', 2)
    # Upgrade Done
    game.moveToRest(settings['delays']['subToSubUpgrade.2-0-1'])
    # Upgrade Sub
    sub.upgrade('/')
    # Upgrade Done
    game.moveToRest(settings['delays']['subToSubUpgrade.2-0-2'])
    # Upgrade Sub
    sub.upgrade('/')
    # Upgrade Done
    game.moveToRest(settings['delays']['subToSubUpgrade.2-0-3'])
    # Upgrade Sub
    sub.upgrade('/')
     # Upgrade Done
    game.moveToRest(settings['delays']['subToSubUpgrade.2-0-4'])
    # Upgrade Sub
    sub.upgrade('/')
    # Upgrade Done
    if settings['optionalUpgrades']['dart.x-x-3']:
        game.moveToRest(settings['delays']['optionals']['sub.2-0-4ToDartUpgrade.x-x-3'])
        # Upgrade Dart
        dart.upgrade(',', 2)
        game.moveToRest(1)
        dart.upgrade('/')
        # Upgrade Done
        if settings['optionalUpgrades']['dart.x-x-4']:
            game.moveToRest(settings['delays']['optionals']['dart.x-x-3ToDartUpgrade.x-x-4'])
            # Upgrade Dart
            dart.upgrade('/')
            # Upgrade Done
    game.moveToRest(settings['delays']['waitTillEnd'])
    # End Game