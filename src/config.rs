use serde::{
    Serialize,
    Deserialize
};
use std::fs;
use crate::args::ToSeconds;

use crate::args::{
    SetWorkTimeCommand,
    SetShortBreakTimeCommand,
    SetLongBreakTimeCommand,
    SetPomodorosToLongBreakCommand,
};

// TODO: Do we want the config to be aware of its own path?
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Config {
    pub work_time: u16,
    pub short_break_time: u16,
    pub long_break_time: u16,
    pub pomodoros_to_long_break: u8,
}

impl Config {
    // TODO: Should saving and loading be a method or a static function?
    // TODO: Have this return errors OR just panic?
    pub fn save(config: &Config, config_path: &str) {
        let contents = toml::to_string(config)
            .expect("Could not parse config as string.");
        fs::write(config_path, contents.as_str())
            .expect(format!("Could not write to file {}", config_path).as_str());
    }

    // TODO: Have this handle errors gracefully or just have it panic if things aren't right?
    // To be fair, if we have errors reading or parsing we don't /want/ the program to continue.
    pub fn load(config_path: &str) -> Config {
        let contents = fs::read_to_string(config_path)
            .expect(format!("Could not read file {}", config_path).as_str());
        let config: Config = toml::from_str(&contents)
            .unwrap();
        config
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
        // TODO: Is there any validation we want to do on this? It should handle itself I think?
        Config {
            pomodoros_to_long_break: command.pomodoros_to_long_break,
            ..self
        }
    }
}