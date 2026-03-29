mod args;
mod config;
mod db;
mod error;
mod timer;

use args::{DisplayPomodorosCommand, RustodoroArgs, RustodoroCommand};
use clap::Parser;
use config::Config;
use directories::ProjectDirs;
use error::Result;
use timer::TimerType;

const RELATIVE_CONFIG_PATH: &str = "./config.toml";

fn main() -> Result<()> {
    // TODO: For the commands where we're modifying the config, what sort of user feedback do we want to let the user know the command executed successfully?
    let config_path = get_config_path();
    let config = Config::load(config_path.as_str())?;
    let args: RustodoroArgs = RustodoroArgs::parse();
    match args.command {
        RustodoroCommand::Work => {
            if config.log_to_db {
                work_timer_with_logging(config);
            } else {
                timer::run_timer(config.work_time, TimerType::Work)?;
            }
        }
        RustodoroCommand::ShortBreak => {
            if config.log_to_db {
                short_break_with_logging(config);
            } else {
                timer::run_timer(config.short_break_time, TimerType::ShortBreak)?;
            }
        }
        RustodoroCommand::LongBreak => {
            if config.log_to_db {
                long_break_with_logging(config);
            } else {
                timer::run_timer(config.long_break_time, TimerType::LongBreak)?;
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
            DisplayPomodorosCommand::Day => println!("Day"),
            DisplayPomodorosCommand::Week => println!("Week"),
            DisplayPomodorosCommand::Month => println!("Month"),
        },
    }

    Ok(())
}

fn work_timer_with_logging(config: Config) {
    // Do we want these functions in the timer module instead?

    let start_time = db::get_current_unix_time();
    let result = timer::run_timer(config.work_time, TimerType::Work);
    let completion_time = db::get_current_unix_time();

    // TODO: Handle the possible database error more elegantly!
    match result {
        Ok(_) => {
            db::save_pomodoro_to_db(start_time, completion_time)
                .expect("Failed to save pomodoro to database.");

            // TODO: If we've hit the pomodoro threshold for long break, notify the user.
            // If todays_pomodoros % pomodoros_to_long_break, increment the long break counter in the DB.

            // Debug Print Stuff
            if cfg!(debug_assertions) {
                db::debug_print_records_from_today(db::POMODORO_TABLE_NAME);
                println!(
                    "Pomodoros Today: {}",
                    db::debug_count_records_from_today(db::POMODORO_TABLE_NAME)
                );
            }
        }
        Err(e) => println!("{}", e),
    }
}

fn short_break_with_logging(config: Config) {
    let start_time = db::get_current_unix_time();
    let result = timer::run_timer(config.short_break_time, TimerType::ShortBreak);
    let completion_time = db::get_current_unix_time();

    // TODO: Handle the possible database error more elegantly!
    match result {
        Ok(_) => {
            db::save_short_break_to_db(start_time, completion_time)
                .expect("Failed to save short break to database.");

            if cfg!(debug_assertions) {
                db::debug_print_records_from_today(db::SHORT_BREAK_TABLE_NAME);
                println!(
                    "Short breaks Today: {}",
                    db::debug_count_records_from_today(db::SHORT_BREAK_TABLE_NAME)
                );
            }
        }
        Err(e) => println!("{}", e),
    }
}

fn long_break_with_logging(config: Config) {
    let start_time = db::get_current_unix_time();
    let result = timer::run_timer(config.long_break_time, TimerType::LongBreak);
    let completion_time = db::get_current_unix_time();

    // TODO: Handle the possible database error more elegantly!
    match result {
        Ok(_) => {
            db::save_long_break_to_db(start_time, completion_time)
                .expect("Failed to save long break to database.");

            // Debug Print Stuff
            if cfg!(debug_assertions) {
                db::debug_print_records_from_today(db::LONG_BREAK_TABLE_NAME);
                println!(
                    "Long breaks Today: {}",
                    db::debug_count_records_from_today(db::LONG_BREAK_TABLE_NAME)
                );
            }
        }
        Err(e) => println!("{}", e),
    }

    // TODO: Decrement the long break counter in the DB.
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
