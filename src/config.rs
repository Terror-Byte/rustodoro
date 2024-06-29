use serde::{
    Serialize,
    Deserialize
};
use core::panic;
use std::fs;
use crate::args::{
    ToSeconds,
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
    // TODO: Have this return errors OR just panic?
    pub fn save(config: &Config, config_path: &str) {
        let contents = toml::to_string(config)
            .expect("Could not parse config as string.");
        fs::write(config_path, contents.as_str())
            .expect(format!("Could not write to file {}", config_path).as_str());
    }

    // TODO: Have this handle errors gracefully or just have it panic if things aren't right?
    // To be fair, if we have errors reading or parsing we don't /want/ the program to continue.
    // Can we determine why we couldn't read from the file? If it's not there, create a default one.
    // Implement a command to create a default config?
    pub fn load(config_path: &str) -> Config {
        let contents_result = fs::read_to_string(config_path);
        match contents_result {
            Ok(contents) => {
                // TODO: If the Config.toml is formatted incorrectly it'll throw a panic here. Shall we just have it panic like this or do we wanna handle it elegantly? Propagate the error up so the function above us can handle the error!
                toml::from_str(&contents).unwrap()
            },
            Err(error) => {
                // TODO: File not present, create new one and return that. Save it too?
                panic!("{}", error.to_string());
            }
        }
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
            pomodoros_to_long_break: 4
        }
    }
}