use crate::args::{
    SetLongBreakTimeCommand, SetPomodorosToLongBreakCommand, SetShortBreakTimeCommand,
    SetWorkTimeCommand, ToSeconds,
};
use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Config {
    pub work_time: u16,
    pub short_break_time: u16,
    pub long_break_time: u16,
    pub pomodoros_to_long_break: u8,
}

impl Config {
    pub fn save(config: &Config, config_path: &str) -> Result<(), Error> {
        let contents = toml::to_string(config)?;
        fs::write(config_path, contents.as_str())?;
        Ok(())
    }

    pub fn load(config_path: &str) -> Result<Config, Error> {
        let contents = fs::read_to_string(config_path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    // TODO: Do we complain if the user sets the number to just 0? Or do we let them do it? Do we set it to a default value in that case and print an error?
    pub fn set_work_time(self, command: SetWorkTimeCommand) -> Config {
        Config {
            work_time: command.to_seconds(),
            ..self
        }
    }

    pub fn set_short_break_time(self, command: SetShortBreakTimeCommand) -> Config {
        Config {
            short_break_time: command.to_seconds(),
            ..self
        }
    }

    pub fn set_long_break_time(self, command: SetLongBreakTimeCommand) -> Config {
        Config {
            long_break_time: command.to_seconds(),
            ..self
        }
    }

    pub fn set_pomodoros_to_long_break(self, command: SetPomodorosToLongBreakCommand) -> Config {
        Config {
            pomodoros_to_long_break: command.pomodoros_to_long_break,
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
        }
    }
}
