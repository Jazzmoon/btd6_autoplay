use enigo::{
    Button::Left,
    Coordinate::Abs,
    Direction::{Click, Press, Release},
    Enigo, InputResult, Key, Keyboard, Mouse,
};

use super::global::ENIGO_SETTINGS;
use crate::models::{coords::Coords, hotkeys::Hotkey};
use crate::utils::global::CURRENT_WINDOW;

pub fn move_cursor(coords: Coords, delay: Option<u64>) -> InputResult<()> {
    std::thread::sleep(std::time::Duration::from_millis(delay.unwrap_or(100)));
    // Get the current window for relative coords
    let current_window_read_lock = CURRENT_WINDOW.read().unwrap();
    let current_window = current_window_read_lock.as_ref().unwrap().get();
    let rel_coords = coords.relative_to_window(current_window);
    // Move cursor to coords
    let enigo_settings_read_lock = ENIGO_SETTINGS.read().unwrap();
    let enigo_settings = enigo_settings_read_lock.as_ref().unwrap();
    let mut enigo = Enigo::new(enigo_settings).unwrap();
    let res = enigo.move_mouse(rel_coords.x, rel_coords.y, Abs);
    let _ = res;
    Ok(())
}

pub fn click(coords: Coords, delay: Option<u64>) -> InputResult<()> {
    std::thread::sleep(std::time::Duration::from_millis(delay.unwrap_or(100)));
    // Get the current window for relative coords
    let current_window_read_lock = CURRENT_WINDOW.read().unwrap();
    let current_window = current_window_read_lock.as_ref().unwrap().get();
    let rel_coords = coords.relative_to_window(current_window);
    // Click at coords
    let enigo_settings_read_lock = ENIGO_SETTINGS.read().unwrap();
    let enigo_settings = enigo_settings_read_lock.as_ref().unwrap();
    let mut enigo = Enigo::new(enigo_settings).unwrap();
    let _ = move_cursor(rel_coords.clone(), None);
    let _ = enigo.button(Left, Click);
    Ok(())
}

pub fn press_key(key: Hotkey, delay: Option<u64>) -> InputResult<()> {
    std::thread::sleep(std::time::Duration::from_millis(delay.unwrap_or(100)));

    // Press key
    let enigo_settings_read_lock = ENIGO_SETTINGS.read().unwrap();
    let enigo_settings = enigo_settings_read_lock.as_ref().unwrap();

    let mut enigo = Enigo::new(enigo_settings).unwrap();

    // Convert Hotkey to Key using into() function
    let converted: Vec<Key> = key.into();

    if converted.len() == 1 {
        let _ = enigo.key(converted[0], Press);
        std::thread::sleep(std::time::Duration::from_millis(100));
        let _ = enigo.key(converted[0], Release);
        return Ok(());
    } else {
        // Press and hold all but the last key
        for k in converted[..converted.len() - 1].iter() {
            let _ = enigo.key(k.clone(), Press);
        }

        // Click the last key
        let _ = enigo.key(converted[converted.len() - 1], Press);
        std::thread::sleep(std::time::Duration::from_millis(100));
        let _ = enigo.key(converted[converted.len() - 1], Release);

        // Release all but the last key
        for k in converted[..converted.len() - 1].iter() {
            let _ = enigo.key(k.clone(), Release);
        }

        return Ok(());
    }
}
