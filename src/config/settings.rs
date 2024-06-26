use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub service_url: String,
}

impl Settings {
    pub fn from_config() -> Settings {
        utils::load_config::load_settings()
    }
}

lazy_static! {
    pub static ref SETTING: Settings = Settings::from_config();
}
