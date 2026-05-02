use enigo::{
    Button::Left,
    Coordinate::Abs,
    Direction::{Click, Press, Release},
    Enigo, InputResult, Key, Keyboard, Mouse,
};
use image::DynamicImage;

use super::global::{CURRENT_WINDOW, ENIGO_SETTINGS};
use crate::models::{coords::Coords, hotkeys::Hotkey};
use crate::utils::screenshot::find_text_on_screen;

/// Controls where within (or relative to) a detected text bounding box
/// the mouse click is placed.
#[derive(Debug, Clone, Copy)]
pub enum ClickTextMode {
    /// Click the horizontal and vertical center of the bounding box.
    /// Use this for label-style buttons such as "NEXT" and "CONTINUE".
    Center,
    /// Click horizontally centered but one full text-height *above* the top
    /// edge of the bounding box.  Use this for icon-style menu items such as
    /// "RESTART", "FREEPLAY", and "HOME" whose interactive region sits above
    /// the rendered text.
    Above,
}

/// Find `target_text` on screen using OCR and click the appropriate spot.
///
/// Returns `Ok(())` on a successful click, or `Err(String)` if the text could
/// not be located after the supplied delay.
pub fn click_text(
    screenshot: &DynamicImage,
    target_text: &str,
    lang: &str,
    mode: ClickTextMode,
    delay: Option<u64>,
    debug: bool,
) -> Result<(), String> {
    let bbox = match find_text_on_screen(screenshot, target_text, lang, debug) {
        Some(b) => b,
        None => return Err(format!("Text '{}' not found on screen", target_text)),
    };

    let (click_x, click_y) = match mode {
        ClickTextMode::Center => (bbox.x + bbox.w / 2, bbox.y + bbox.h / 2),
        // One full glyph-height above the top of the bounding box puts the
        // click squarely in the button icon / highlight area.
        ClickTextMode::Above => (bbox.x + bbox.w / 2, bbox.y - bbox.h),
    };

    let abs_coords = {
        let current_window_read_lock = CURRENT_WINDOW.read().unwrap();
        let current_window = current_window_read_lock.as_ref().unwrap().get();
        Coords {
            x: click_x,
            y: click_y,
        }
        .relative_to_window(current_window)
    };

    click(abs_coords, delay).map_err(|e| e.to_string())
}

pub fn move_cursor(coords: Coords, delay: Option<u64>) -> InputResult<()> {
    std::thread::sleep(std::time::Duration::from_millis(delay.unwrap_or(100)));
    // Move cursor to coords
    let enigo_settings_read_lock = ENIGO_SETTINGS.read().unwrap();
    let enigo_settings = enigo_settings_read_lock.as_ref().unwrap();
    let mut enigo = Enigo::new(enigo_settings).unwrap();
    let res = enigo.move_mouse(coords.x, coords.y, Abs);
    let _ = res;
    Ok(())
}

pub fn click(coords: Coords, delay: Option<u64>) -> InputResult<()> {
    std::thread::sleep(std::time::Duration::from_millis(delay.unwrap_or(100)));
    // Click at coords
    let enigo_settings_read_lock = ENIGO_SETTINGS.read().unwrap();
    let enigo_settings = enigo_settings_read_lock.as_ref().unwrap();
    let mut enigo = Enigo::new(enigo_settings).unwrap();
    let _ = move_cursor(coords.clone(), None);
    let _ = enigo.button(Left, Click);
    println!("Clicked at coords ({}, {})", coords.x, coords.y);
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
