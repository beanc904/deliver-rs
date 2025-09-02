use std::collections::VecDeque;
use std::fs;

use dialoguer::Select;
use serde::{Deserialize, Serialize};

use deliver::get_cache_dir;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddrCache {
    history: VecDeque<String>,
}

impl AddrCache {
    fn new() -> Self {
        Self {
            history: VecDeque::new(),
        }
    }

    pub fn load() -> Self {
        let mut path = get_cache_dir();
        path.push("addr_cache.json");

        log::debug!("Loading address cache from {:?}", path);

        if let Ok(data) = fs::read_to_string(&path) {
            serde_json::from_str(&data).unwrap_or_else(|_| AddrCache::new())
        } else {
            AddrCache::new()
        }
    }

    pub fn save(&self) {
        let mut path = get_cache_dir();

        // create the directory if it does not exist
        if let Err(e) = fs::create_dir_all(&path) {
            log::error!("Failed to create cache directory: {}", e);

            return;
        }

        path.push("addr_cache.json");

        log::debug!("Saving address cache to {:?}", path);

        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, data);
        }
    }

    /// Add an IP address to the history.
    /// If the IP address already exists, move it to the most recent position.
    /// If the history is full (5 addresses), remove the oldest one.
    pub fn add_addr(&mut self, ip: String) {
        // Remove if already exists
        if let Some(pos) = self.history.iter().position(|x| *x == ip) {
            self.history.remove(pos);
        }

        // Remove the oldest if full
        if self.history.len() == 5 {
            self.history.pop_front();
        }

        self.history.push_back(ip);
    }

    pub fn select_addr(&mut self) -> Option<String> {
        if self.history.is_empty() {
            return None;
        }

        let mut selections: Vec<&String> = self.history.iter().collect();
        let other = String::from("Enter a new IP address");
        selections.push(&other);
        let selection = Select::new()
            .with_prompt("Select a previously used IP address")
            .items(&selections)
            .default(selections.len() - 2)
            .interact()
            .unwrap();

        if selection == selections.len() - 1 {
            return None;
        }
        Some(selections[selection].to_string())
    }
}
