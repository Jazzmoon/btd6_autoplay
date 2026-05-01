use enigo::{
    Button::Left,
    Coordinate::Abs,
    Direction::{Click, Press, Release},
    Enigo, InputResult, Key, Keyboard, Mouse,
};

use super::global::ENIGO_SETTINGS;
use crate::models::{coords::Coords, hotkeys::Hotkey};

pub fn move_cursor(coords: Coords, delay: Option<u64>) -> InputResult<()> {
    if delay.is_some() {
        std::thread::sleep(std::time::Duration::from_millis(delay.unwrap()));
    }
    // Move cursor to coords
    let enigo_settings_read_lock = ENIGO_SETTINGS.read().unwrap();
    let enigo_settings = enigo_settings_read_lock.as_ref().unwrap();

    let mut enigo = Enigo::new(enigo_settings).unwrap();

    let res = enigo.move_mouse(coords.x, coords.y, Abs);
    let _ = res;
    Ok(())
}

pub fn click(coords: Coords, delay: Option<u64>) -> InputResult<()> {
    if delay.is_some() {
        std::thread::sleep(std::time::Duration::from_millis(delay.unwrap()));
    }
    // Click at coords
    let enigo_settings_read_lock = ENIGO_SETTINGS.read().unwrap();
    let enigo_settings = enigo_settings_read_lock.as_ref().unwrap();

    let mut enigo = Enigo::new(enigo_settings).unwrap();

    let _ = move_cursor(coords, None);

    let res = enigo.button(Left, Click);
    let _ = res;
    Ok(())
}

pub fn press_key(key: Hotkey, delay: Option<u64>) -> InputResult<()> {
    if delay.is_some() {
        std::thread::sleep(std::time::Duration::from_millis(delay.unwrap()));
    }
    // Press key
    let enigo_settings_read_lock = ENIGO_SETTINGS.read().unwrap();
    let enigo_settings = enigo_settings_read_lock.as_ref().unwrap();

    let mut enigo = Enigo::new(enigo_settings).unwrap();

    // Convert Hotkey to Key using into() function
    let converted: Vec<Key> = key.into();

    if converted.len() == 1 {
        let res = enigo.key(converted[0], Click);
        let _ = res;
        return Ok(());
    } else {
        // Press and hold all but the last key
        for k in converted[..converted.len() - 1].iter() {
            let _ = enigo.key(k.clone(), Press);
        }

        // Click the last key
        let _ = enigo.key(converted[converted.len() - 1], Click);

        // Release all but the last key
        for k in converted[..converted.len() - 1].iter() {
            let _ = enigo.key(k.clone(), Release);
        }

        return Ok(());
    }
}
