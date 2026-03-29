use crate::args::{
    SetLogToDBArgs, SetLongBreakTimeArgs, SetPomodorosToLongBreakArgs, SetShortBreakTimeArgs,
    SetWorkTimeArgs, ToSeconds,
};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Config {
    pub work_time: u16,
    pub short_break_time: u16,
    pub long_break_time: u16,
    pub pomodoros_to_long_break: u8,
    pub log_to_db: bool,
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

    pub fn set_pomodoros_to_long_break(self, args: SetPomodorosToLongBreakArgs) -> Result<Config> {
        if args.pomodoros_to_long_break == 0 {
            return Err(Error::ConfigError(
                "Cannot set 'pomodoros to long break' to 0!".to_string(),
            ));
        }

        Ok(Config {
            pomodoros_to_long_break: args.pomodoros_to_long_break,
            ..self
        })
    }

    pub fn set_log_to_db(self, command: SetLogToDBArgs) -> Config {
        Config {
            log_to_db: command.log_to_db,
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
            log_to_db: true,
        }
    }
}
