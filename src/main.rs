mod args;
mod config;
mod error;
mod timer;

use args::{RustodoroArgs, RustodoroCommand};
use clap::Parser;
use config::Config;
use directories::ProjectDirs;
use error::Result;
use timer::{run_timer, TimerType};

const RELATIVE_CONFIG_PATH: &str = "./config.toml";

fn main() -> Result<()> {
    // TODO: For the commands where we're modifying the config, what sort of user feedback do we want to let the user know the command executed successfully?
    let config_path = get_config_path();
    let config = Config::load(config_path.as_str())?;
    let args: RustodoroArgs = RustodoroArgs::parse();
    match args.command {
        RustodoroCommand::Work => run_timer(config.work_time, TimerType::Work)?,
        RustodoroCommand::ShortBreak => run_timer(config.short_break_time, TimerType::ShortBreak)?,
        RustodoroCommand::LongBreak => run_timer(config.long_break_time, TimerType::LongBreak)?,
        RustodoroCommand::SetWorkTime(command) => {
            let new_config = config.set_work_time(command)?;
            Config::save(&new_config, config_path.as_str())?
        }
        RustodoroCommand::SetShortBreakTime(command) => {
            let new_config = config.set_short_break_time(command)?;
            Config::save(&new_config, config_path.as_str())?
        }
        RustodoroCommand::SetLongBreakTime(command) => {
            let new_config = config.set_long_break_time(command)?;
            Config::save(&new_config, config_path.as_str())?
        }
        RustodoroCommand::SetPomodorosToLongBreak(command) => {
            let new_config = config.set_pomodoros_to_long_break(command)?;
            Config::save(&new_config, config_path.as_str())?
        }
    }

    Ok(())
}

fn get_config_path() -> String {
    if !cfg!(debug_assertions) {
        if let Some(proj_dirs) = ProjectDirs::from("com", "TerrorByte", "Rustodoro") {
            if let Some(directory) = proj_dirs.config_dir().to_str() {
                let mut directory_str = String::from(directory);
                directory_str.push_str("/config.toml");
                return directory_str;
            }
        }
    }
    String::from(RELATIVE_CONFIG_PATH)
}
