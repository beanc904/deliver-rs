use std::fs;

use serde::{Deserialize, Serialize};

use crate::pkg_info::PkgInfo;

#[derive(Serialize, Deserialize, Debug)]
pub struct Cfg {
    default_port: u16,
    max_history: usize,
}

impl Cfg {
    fn new() -> Self {
        Self {
            default_port: 9000,
            max_history: 5,
        }
    }

    pub fn load() -> Self {
        let mut path = PkgInfo::new().get_config_dir();
        path.push("sender.toml");

        log::debug!("Loading config from {:?}", path);

        if let Ok(data) = fs::read_to_string(&path) {
            toml::from_str(&data).unwrap_or_else(|_| {
                log::error!("Failed to parse config file, using default config");
                Cfg::new()
            })
        } else {
            log::debug!("Config file not found, using default config");

            Cfg::new()
        }
    }

    pub fn get_port(&self) -> u16 {
        self.default_port
    }

    pub fn get_history_size(&self) -> usize {
        self.max_history
    }

    pub fn set_port(&mut self, port: u16) {
        self.default_port = port;
    }

    pub fn set_history_size(&mut self, size: usize) {
        self.max_history = size;
    }

    pub fn save(&self) {
        let mut path = PkgInfo::new().get_config_dir();

        // create the directory if it does not exist
        if let Err(e) = fs::create_dir_all(&path) {
            log::error!("Failed to create config directory: {}", e);

            return;
        }

        path.push("sender.toml");

        let data = toml::to_string_pretty(self).unwrap();

        if let Err(e) = fs::write(&path, data) {
            log::error!("Failed to write config file: {}", e);
        }
    }
}
