mod lib {
    pub mod global;
}
mod models;

use std::fs::File;
use std::io::BufReader;
use lib::global::SETTINGS;
use models::Settings::Settings;
use models::Hotkeys::Hotkeys;

fn main() {
    // Load the config/Settings.1080p.yaml file into the SETTINGS global variable
    {
        let file = File::open("../../config/Settings.1080p.yaml").unwrap();
        let reader = BufReader::new(file);
        let settings: Settings = serde_yaml::from_reader(reader).unwrap();
        *SETTINGS.lock().unwrap() = Option::Some(settings);
    }

    // Print the following information to test loading:
    // Settigns is an Option<Settings> and Hotkeys is an Option<Hotkeys>
    {
        let settings = SETTINGS.lock().unwrap().as_ref().unwrap();
        println!("The VictoryBanner x coordinate is: {}", settings.game.victory_banner.x);
        println!("The VictoryBanner y coordinate is: {}", settings.game.victory_banner.y);
    }
}