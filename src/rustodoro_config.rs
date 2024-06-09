//#[path = "args.rs"] mod args;
// use std::cmp;
use serde::Serialize;
use serde::Deserialize;
use std::fs;

// TODO: Do we want the config to be aware of its own path?
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct RustodoroConfig {
    pub work_time: u16,
    pub short_break_time: u16,
    pub long_break_time: u16,
    pub pomodoros_till_long_break: u16,
    display_in_secs: bool,
}

impl RustodoroConfig {
    // TODO: Should saving and loading be a method or a static function?
    // TODO: Have this return errors OR just panic?
    //fn save(&self, config_path: &str) {
    pub fn save(config: &RustodoroConfig, config_path: &str) {
        //let contents = toml::to_string(config);
        let contents = toml::to_string(config)
            .expect("Could not parse config as string.");
        fs::write(config_path, contents.as_str())
            .expect(format!("Could not write to file {}", config_path).as_str());
        // if let Ok(foo) = contents {
        //     // TODO: Probs not the best way to handle an error, but what else do we want to do?
        //     fs::write(config_path, foo.as_str())
        //         .expect(format!("Could not write to file {}", config_path).as_str());
        // }
    }

    // TODO: Have this handle errors gracefully or just have it panic if things aren't right?
    // To be fair, if we have errors reading or parsing we don't /want/ the program to continue.
    pub fn load(config_path: &str) -> RustodoroConfig {
        let contents = fs::read_to_string(config_path)
            .expect(format!("Could not read file {}", config_path).as_str());
        let config: RustodoroConfig = toml::from_str(&contents)
            .unwrap();
        config
    }

    // Make new config!
    pub fn set_work_timer(self, command: args::SetWorkTimerCommand) -> RustodoroConfig {
        // This is in seconds!
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

        RustodoroConfig { 
            work_time: work_timer,
            ..self
        }
    }
}