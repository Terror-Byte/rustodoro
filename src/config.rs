use crate::args::{
    SetDesktopNotificationsArgs, SetLogToDBArgs, SetLongBreakTimeArgs, SetPomodorosToLongBreakArgs,
    SetShortBreakTimeArgs, SetWorkTimeArgs, ToSeconds,
};
use crate::error::{Error, Result};
#[cfg(all(not(debug_assertions), not(feature = "portable")))]
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
#[cfg(any(debug_assertions, feature = "portable"))]
use std::env;
use std::fs;

const CONFIG_NAME: &str = "config.toml";

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Config {
    pub work_time: u16,
    pub short_break_time: u16,
    pub long_break_time: u16,
    pub pomodoros_to_long_break: u8,
    #[serde(default)]
    pub log_to_db: bool,
    #[serde(default)]
    pub desktop_notifications: bool,
}

impl Config {
    pub fn save(config: &Config, config_path: &str) -> Result<()> {
        let contents = toml::to_string(config)?;
        fs::write(config_path, contents.as_str())?;
        Ok(())
    }

    pub fn load(config_path: &str) -> Result<Config> {
        if let Ok(contents) = fs::read_to_string(config_path) {
            let config = toml::from_str(&contents)?;
            return Ok(config);
        }

        Ok(Config::default())
    }

    pub fn set_work_time(self, args: SetWorkTimeArgs) -> Result<Config> {
        let work_time = args.to_seconds();
        if work_time == 0 {
            return Err(Error::ConfigError(
                "Cannot set work timer duration to 0 seconds!".to_string(),
            ));
        }

        Ok(Config { work_time, ..self })
    }

    pub fn set_short_break_time(self, args: SetShortBreakTimeArgs) -> Result<Config> {
        let short_break_time = args.to_seconds();
        if short_break_time == 0 {
            return Err(Error::ConfigError(
                "Cannot set short break time duration to 0 seconds!".to_string(),
            ));
        }

        Ok(Config {
            short_break_time,
            ..self
        })
    }

    pub fn set_long_break_time(self, args: SetLongBreakTimeArgs) -> Result<Config> {
        let long_break_time = args.to_seconds();
        if long_break_time == 0 {
            return Err(Error::ConfigError(
                "Cannot set long break time duration to 0 seconds!".to_string(),
            ));
        }

        Ok(Config {
            long_break_time,
            ..self
        })
    }

    pub fn set_pomodoros_to_long_break(self, args: SetPomodorosToLongBreakArgs) -> Config {
        Config {
            pomodoros_to_long_break: args.pomodoros_to_long_break,
            ..self
        }
    }

    pub fn set_log_to_db(self, args: SetLogToDBArgs) -> Config {
        Config {
            log_to_db: args.log_to_db,
            ..self
        }
    }

    pub fn set_desktop_notifications(self, args: SetDesktopNotificationsArgs) -> Config {
        Config {
            desktop_notifications: args.desktop_notifications,
            ..self
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            work_time: 1500,
            short_break_time: 300,
            long_break_time: 900,
            pomodoros_to_long_break: 4,
            log_to_db: false,
            desktop_notifications: false,
        }
    }
}

#[cfg(all(not(debug_assertions), not(feature = "portable")))]
pub fn get_config_path() -> Option<String> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "TerrorByte", "Rustodoro") {
        let mut config_dir = proj_dirs.config_dir().to_path_buf();
        config_dir.push(CONFIG_NAME);
        if let Some(config_path) = config_dir.to_str() {
            return Some(config_path.to_string());
        }
    }

    None
}

#[cfg(all(not(debug_assertions), feature = "portable"))]
pub fn get_config_path() -> Option<String> {
    if let Ok(mut exe_dir) = env::current_exe() {
        exe_dir.pop();
        exe_dir.push(CONFIG_NAME);
        if let Some(config_path) = exe_dir.to_str() {
            return Some(config_path.to_string());
        }
    }

    None
}

#[cfg(all(debug_assertions, not(feature = "portable")))]
pub fn get_config_path() -> Option<String> {
    if let Ok(mut pwd) = env::current_dir() {
        pwd.push(CONFIG_NAME);
        if let Some(config_path) = pwd.to_str() {
            return Some(config_path.to_string());
        }
    }

    None
}
