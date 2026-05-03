use device_query::{DeviceEvents, DeviceEventsHandler, Keycode};
use enigo::{Enigo, Mouse, Settings as EnigoSettings};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::time::Duration;

use crate::models::coords::CoordsArea;

use super::global::ENIGO_SETTINGS;
use super::logger::Logger;

#[derive(Debug, Clone, Copy)]
pub enum LocationFinderMode {
    SinglePoint,
    AreaSelection,
}

pub fn location_finder(window_x: i32, window_y: i32) {
    // This is a tool that acts as an infinite loop that prints the current mouse position when you left click
    // When you press the letter 'q', the program will exit
    // Pressing 'p' will pause the program so you can interact with your system without logging mouse positions
    // Pressing 'p' again will resume the program
    // Pressing 'm' changes the mode from single click to area selection
    // Print help dialog for the functionality of the program
    Logger::notice("Welcome to the location finder utility!");
    Logger::info("Press 'q' to exit the program");
    Logger::info("Press 'p' to pause/resume the program");
    Logger::info("Press 'm' to change the mode from single point to area selection mode\n");

    let paused = Arc::new(AtomicBool::new(false));
    let mode = Arc::new(Mutex::new(LocationFinderMode::SinglePoint));
    let area_mode_first_coordinate: Arc<Mutex<Option<(i32, i32)>>> = Arc::new(Mutex::new(None));
    let device_events = DeviceEventsHandler::new(Duration::from_millis(10)).unwrap();

    let paused_clone = Arc::clone(&paused);
    let mode_clone = Arc::clone(&mode);

    let _guard = device_events.on_key_up(move |key| {
        if key.eq(&Keycode::Q) {
            Logger::notice("Exiting program");
            std::process::exit(0);
        } else if key.eq(&Keycode::P) {
            let paused = paused_clone.load(Ordering::SeqCst);
            paused_clone.store(!paused, Ordering::SeqCst);
            if !paused {
                Logger::notice("Program paused");
            } else {
                Logger::notice("Program resumed");
            }
        } else if key.eq(&Keycode::M) {
            let paused = paused_clone.load(Ordering::SeqCst);
            if !paused {
                let mut mode_lock = mode_clone.lock().unwrap();
                *mode_lock = match *mode_lock {
                    LocationFinderMode::SinglePoint => LocationFinderMode::AreaSelection,
                    LocationFinderMode::AreaSelection => LocationFinderMode::SinglePoint,
                };
                Logger::notice(format!("Mode changed to {:?}", *mode_lock));
            }
        }
    });

    let paused_clone = Arc::clone(&paused);
    let mode_clone = Arc::clone(&mode);
    let area_mode_first_coordinate_clone = Arc::clone(&area_mode_first_coordinate);
    let _guard = device_events.on_mouse_up(move |button| {
        if *button != 1 {
            return;
        }
        if paused_clone.load(Ordering::SeqCst) {
            return;
        }
        let enigo_settings_lock = ENIGO_SETTINGS.read().unwrap();
        let coords = match enigo_settings_lock.as_ref() {
            Some(enigo_settings) => Enigo::new(enigo_settings)
                .unwrap()
                .location()
                .unwrap_or((0, 0)),
            None => {
                let default_settings = EnigoSettings::default();
                Enigo::new(&default_settings)
                    .unwrap()
                    .location()
                    .unwrap_or((0, 0))
            }
        };
        let mode_lock = mode_clone.lock().unwrap();
        match *mode_lock {
            LocationFinderMode::SinglePoint => {
                // Print two messages: Absolute coordinates and relative coordinates to window x and y
                let coords_s = (coords.0 - window_x, coords.1 - window_y);
                Logger::info(format!("Relative to Game Window: {:?}", coords_s));
            }
            LocationFinderMode::AreaSelection => {
                let mut area_mode_first_coordinate =
                    area_mode_first_coordinate_clone.lock().unwrap();
                match *area_mode_first_coordinate {
                    Some(area_mode_first_coordinate_val) => {
                        let first_x = area_mode_first_coordinate_val.0 - window_x;
                        let first_y = area_mode_first_coordinate_val.1 - window_y;

                        let second_x = coords.0 - window_x;
                        let second_y = coords.1 - window_y;

                        let x: i32;
                        let y: i32;
                        let w: i32;
                        let h: i32;

                        // First point is either top-left, top-right, bottom-left, or bottom-right
                        // But the coords area ALWAYS needs x and y to be the top-left corner

                        if first_x < second_x && first_y < second_y {
                            // Top-Left
                            x = first_x;
                            y = first_y;
                            w = second_x - first_x;
                            h = second_y - first_y;
                        } else if first_x >= second_x && first_y < second_y {
                            // Top-Right
                            x = second_x;
                            y = first_y;
                            w = first_x - second_x;
                            h = second_y - first_y;
                        } else if first_x < second_x && first_y > second_y {
                            // Bottom-Left
                            x = first_x;
                            y = second_y;
                            w = second_x - first_x;
                            h = first_y - second_y;
                        } else {
                            // Bottom-Right
                            x = second_x;
                            y = second_y;
                            w = first_x - second_x;
                            h = first_y - second_y;
                        }

                        let coords_area = CoordsArea { x, y, w, h };
                        Logger::info(format!("{:?}", coords_area));
                        *area_mode_first_coordinate = None;
                    }
                    None => {
                        *area_mode_first_coordinate = Some((coords.0, coords.1));
                    }
                }
            }
        }
    });

    Logger::notice("Starting coordinate finder in Single Point mode...");
    loop {}
}
