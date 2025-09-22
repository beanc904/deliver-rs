use std::env;
use std::path::PathBuf;

/// Struct to hold package information.
/// Includes methods to retrieve package name, version, authors,
/// cache directory, and config directory.
/// # Examples
/// ```
/// let pi = PkgInfo::new();
/// println!("Package Name: {}", pi.get_pkg_name());
/// println!("Package Version: {}", pi.get_pkg_version());
/// println!("Package Authors: {}", pi.get_pkg_authors());
/// ```
pub struct PkgInfo {
    pkg_name: &'static str,
    pkg_version: &'static str,
    pkg_authors: &'static str,
}

impl PkgInfo {
    pub fn new() -> Self {
        Self {
            pkg_name: env!("CARGO_PKG_NAME"),
            pkg_version: env!("CARGO_PKG_VERSION"),
            pkg_authors: env!("CARGO_PKG_AUTHORS"),
        }
    }

    pub fn get_pkg_name(&self) -> &str {
        &self.pkg_name
    }

    pub fn get_pkg_version(&self) -> &str {
        &self.pkg_version
    }

    pub fn get_pkg_authors(&self) -> &str {
        &self.pkg_authors
    }

    pub fn get_cache_dir(&self) -> PathBuf {
        let mut cache_dir = if cfg!(target_os = "windows") {
            let local_appdata = env::var("LOCALAPPDATA").expect("Cannot find $env:LOCALAPPDATA");
            let cache_dir = PathBuf::from(local_appdata);

            log::debug!("Windows OS detected, using LOCALAPPDATA: {:?}", cache_dir);

            cache_dir
        } else {
            if let Ok(xdg_cache_home) = env::var("XDG_CACHE_HOME") {
                let cache_dir = PathBuf::from(xdg_cache_home);

                log::debug!("POSIX detected, using XDG_CACHE_HOME: {:?}", cache_dir);

                cache_dir
            } else {
                let home = env::var("HOME").expect("Cannot find $HOME");
                let cache_dir = PathBuf::from(home).join(".cache");

                log::debug!("POSIX detected, using default: {:?}", cache_dir);

                cache_dir
            }
        };

        cache_dir.push(&self.pkg_name);
        cache_dir
    }

    pub fn get_config_dir(&self) -> PathBuf {
        let mut config_dir = if cfg!(target_os = "windows") {
            let appdata = env::var("APPDATA").expect("Cannot find $env:APPDATA");
            let config_dir = PathBuf::from(appdata);

            log::debug!("Windows OS detected, using APPDATA: {:?}", config_dir);

            config_dir
        } else {
            if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") {
                let config_dir = PathBuf::from(xdg_config_home);

                log::debug!("POSIX detected, using XDG_CONFIG_HOME: {:?}", config_dir);

                config_dir
            } else {
                let home = env::var("HOME").expect("Cannot find $HOME");
                let config_dir = PathBuf::from(home).join(".config");

                log::debug!("POSIX detected, using default: {:?}", config_dir);

                config_dir
            }
        };

        config_dir.push(&self.pkg_name);
        config_dir
    }
}