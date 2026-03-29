mod args;
mod config;
mod db;
mod error;
mod timer;

use args::{DisplayPomodorosCommand, RustodoroArgs, RustodoroCommand};
// use chrono::{DateTime, Local, Utc};
use chrono::{Local, TimeZone};
use clap::Parser;
use config::Config;
use directories::ProjectDirs;
use error::Result;
// use std::time::SystemTime;
use timer::TimerType;

const RELATIVE_CONFIG_PATH: &str = "./config.toml";

fn main() -> Result<()> {
    // TODO: For the commands where we're modifying the config, what sort of user feedback do we want to let the user know the command executed successfully?
    let config_path = get_config_path();
    let config = Config::load(config_path.as_str())?;
    let args: RustodoroArgs = RustodoroArgs::parse();
    match args.command {
        RustodoroCommand::Work => {
            let (start_time, end_time) = timer::run_timer(config.work_time, TimerType::Work)?;
            if config.log_to_db {
                db::save_session_to_db(start_time, end_time, TimerType::Work)?;
            }
        }
        RustodoroCommand::ShortBreak => {
            let (start_time, end_time) = timer::run_timer(config.work_time, TimerType::ShortBreak)?;
            if config.log_to_db {
                db::save_session_to_db(start_time, end_time, TimerType::ShortBreak)?;
            }
        }
        RustodoroCommand::LongBreak => {
            let (start_time, end_time) = timer::run_timer(config.work_time, TimerType::LongBreak)?;
            if config.log_to_db {
                db::save_session_to_db(start_time, end_time, TimerType::LongBreak)?;
            }
        }
        RustodoroCommand::SetWorkTime(command) => {
            let new_config = config.set_work_time(command)?;
            Config::save(&new_config, config_path.as_str())?;
        }
        RustodoroCommand::SetShortBreakTime(command) => {
            let new_config = config.set_short_break_time(command)?;
            Config::save(&new_config, config_path.as_str())?;
        }
        RustodoroCommand::SetLongBreakTime(command) => {
            let new_config = config.set_long_break_time(command)?;
            Config::save(&new_config, config_path.as_str())?;
        }
        RustodoroCommand::SetPomodorosToLongBreak(command) => {
            let new_config = config.set_pomodoros_to_long_break(command)?;
            Config::save(&new_config, config_path.as_str())?;
        }
        RustodoroCommand::SetLogToDB(command) => {
            let new_config = config.set_log_to_db(command);
            Config::save(&new_config, config_path.as_str())?;
        }
        RustodoroCommand::DisplayPomodoros(command) => match command.command {
            DisplayPomodorosCommand::Day => {
                let todays_pomodoros = db::get_todays_sessions(TimerType::Work)?;
                println!(
                    "You have completed {} pomodoros today.\n",
                    todays_pomodoros.len()
                );

                // TODO: Print this as a table?
                let mut i = 1;
                for pomodoro in todays_pomodoros {
                    // TODO: Replace .unwrap() with ?
                    let start_time = Local.timestamp_opt(pomodoro.0 as i64, 0).unwrap();
                    let end_time = Local.timestamp_opt(pomodoro.1 as i64, 0).unwrap();
                    println!(
                        "Pomodoro {} - Start Time: {}, End Time: {}",
                        i,
                        start_time.format("%H:%M:%S"),
                        end_time.format("%H:%M:%S")
                    );
                    i += 1;
                }
            }
            DisplayPomodorosCommand::Week => println!("Week"),
            DisplayPomodorosCommand::Month => println!("Month"),
        },
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
