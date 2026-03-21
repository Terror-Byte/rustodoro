mod args;
mod config;
mod timer;

use args::{RustodoroArgs, RustodoroCommand};
use clap::Parser;
use config::Config;
use directories::ProjectDirs;
use timer::{run_timer, TimerType};

const CONFIG_PATH: &str = "./config.toml";

fn main() -> Result<(), std::io::Error> {
    let config_path = if cfg!(debug_assertions) {
        String::from(CONFIG_PATH)
    } else {
        match ProjectDirs::from("com", "TerrorByte", "Rustodoro") {
            Some(proj_dirs) => match proj_dirs.config_dir().to_str() {
                Some(directory) => {
                    // TODO: Is there a cleaner way to do this?
                    let mut directory_str = String::from(directory);
                    directory_str.push_str("/config.toml");
                    directory_str
                }
                None => String::from(CONFIG_PATH),
            },
            None => String::from(CONFIG_PATH),
        }
    };
    let config = Config::load(config_path.as_str());

    // TODO: What errors do we want these functions all to throw? Do we want them all to be propagatable updwards (if that's even a phrase)?
    // TODO: For the commands where we're modifying the config, what sort of user feedback do we want to let the user know the command executed successfully?
    let args: RustodoroArgs = RustodoroArgs::parse();
    match args.command {
        RustodoroCommand::Work => run_timer(config.work_time, TimerType::Work)?,
        RustodoroCommand::ShortBreak => run_timer(config.short_break_time, TimerType::ShortBreak)?,
        RustodoroCommand::LongBreak => run_timer(config.long_break_time, TimerType::LongBreak)?,
        RustodoroCommand::SetWorkTime(command) => {
            let new_config = config.set_work_time(command);
            Config::save(&new_config, CONFIG_PATH);
        }
        RustodoroCommand::SetShortBreakTime(command) => {
            let new_config = config.set_short_break_time(command);
            Config::save(&new_config, CONFIG_PATH);
        }
        RustodoroCommand::SetLongBreakTime(command) => {
            let new_config = config.set_long_break_time(command);
            Config::save(&new_config, CONFIG_PATH);
        }
        RustodoroCommand::SetPomodorosToLongBreak(command) => {
            let new_config = config.set_pomodoros_to_long_break(command);
            Config::save(&new_config, CONFIG_PATH);
        }
    }

    Ok(())
}
