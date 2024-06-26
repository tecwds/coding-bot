use std::fs;

use crate::config::settings::Settings;

pub fn load_settings() -> Settings {
    serde_json::from_str(&fs::read_to_string("./resources/config/settings.json").unwrap()).unwrap()
}
