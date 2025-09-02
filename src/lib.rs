use std::env;
use std::path::PathBuf;

pub fn get_cache_dir() -> PathBuf {
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

    cache_dir.push("deliver");
    cache_dir
}
