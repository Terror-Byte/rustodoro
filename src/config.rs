use serde::{
    Serialize,
    Deserialize
};
use std::fs;
use super::args::{
    SetWorkTimerCommand,
    SetShortBreakTimerCommand,
    SetLongBreakTimerCommand,
    SetPomodorosToLongBreakCommand,
};

// TODO: Do we want the config to be aware of its own path?
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Config {
    pub work_time: u16,
    pub short_break_time: u16,
    pub long_break_time: u16,
    pub pomodoros_till_long_break: u8,
    display_in_secs: bool,
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
    pub fn set_work_timer(self, command: SetWorkTimerCommand) -> Config {
        let mut work_timer: u16 = 0;

        if let Some(minutes) = command.minutes {
            work_timer += minutes * 60;
        }

        if let Some(seconds) = command.seconds {
            match seconds {
                0..=60 => work_timer += seconds as u16,
                _ => println!("Error!") // TODO: What do we do in this case? Should this return a Result?
            }
        }

        Config { 
            work_time: work_timer,
            ..self
        }
    }

    pub fn set_short_break_timer(self, command: SetShortBreakTimerCommand) -> Config {
        let mut short_break_timer: u16 = 0;

        if let Some(minutes) = command.minutes {
            short_break_timer += minutes * 60; 
        }

        if let Some(seconds) = command.seconds {
            match seconds {
                0..=60 => short_break_timer += seconds as u16,
                _ => println!("Error!") // TODO: What do we do in this case? Should this return a result and not a Config? Or just panic?
            }
        }

        Config {
            short_break_time: short_break_timer,
            ..self
        }
    }

    pub fn set_long_break_timer(self, command: SetLongBreakTimerCommand) -> Config {
        let mut long_break_timer: u16 = 0;

        if let Some(minutes) = command.minutes {
            long_break_timer += minutes * 60; 
        }

        if let Some(seconds) = command.seconds {
            match seconds {
                0..=60 => long_break_timer += seconds as u16,
                _ => println!("Error!") // TODO: What do we do in this case? Should this return a result and not a Config? Or just panic?
            }
        }

        Config {
            long_break_time: long_break_timer,
            ..self
        }
    }

    pub fn set_pomodoros_to_long_break(self, command: SetPomodorosToLongBreakCommand) -> Config {
        // TODO: Is there any validation we want to do on this? It should handle itself I think?
        Config {
            pomodoros_till_long_break: command.pomodoros_to_long_break,
            ..self
        }
    }
}