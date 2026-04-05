mod args;
mod config;
mod db;
mod display;
mod error;
mod timer;

use args::{RustodoroArgs, RustodoroCommand};
use clap::Parser;
use config::Config;
use error::Result;
use timer::TimerType;

use crate::args::DisplayCommand;

fn main() -> Result<()> {
    // TODO: For the commands where we're modifying the config, what sort of user feedback do we want to let the user know the command executed successfully?
    let config_path = config::get_config_path();
    let config = Config::load(config_path.as_str())?;
    let args: RustodoroArgs = RustodoroArgs::parse();
    match args.command {
        RustodoroCommand::Work => {
            let (start_time, end_time) = timer::run_timer(config.work_time, TimerType::Work)?;
            if config.log_to_db {
                db::save_session_to_db(start_time, end_time, TimerType::Work)?;

                // If the amount of pomodoros completed since the last long break (or since the
                // start of the day, if no long breaks have been taken yet) is equal to or greater
                // than pomodoros_to_long_break, inform the user that they're due a long break
                if config.pomodoros_to_long_break > 0 {
                    // If there's no long break table, we don't want to print the SQL error to the user
                    let latest_long_break =
                        db::get_most_recent_session(TimerType::LongBreak).unwrap_or(None);
                    let sessions = match latest_long_break {
                        Some(session) => {
                            let sessions =
                                db::get_sessions_since(TimerType::Work, session.1 as i64)?;
                            sessions
                        }
                        None => {
                            let timespan = DisplayCommand::Day;
                            let sessions = db::get_sessions(TimerType::Work, &Some(timespan))?;
                            sessions
                        }
                    };

                    if sessions.len() >= config.pomodoros_to_long_break as usize {
                        print!("You're due a long break!");
                    } else {
                        let delta = config.pomodoros_to_long_break - sessions.len() as u8;
                        print!(
                            "You've got {} more pomodoros to complete before you're due a long break!",
                            delta
                        );
                    }
                }
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
            let new_config = config.set_pomodoros_to_long_break(command);
            Config::save(&new_config, config_path.as_str())?;
        }
        RustodoroCommand::SetLogToDB(command) => {
            let new_config = config.set_log_to_db(command);
            Config::save(&new_config, config_path.as_str())?;
        }
        RustodoroCommand::DisplayPomodoros(command) => {
            if !config.log_to_db {
                println!("WARNING: Session logging is currently disabled, only displaying sessions completed whilst session logging enabled.\n");
            }
            let timer_type = TimerType::Work;
            let timespan = command.subcommand;
            let sessions = db::get_sessions(timer_type, &timespan)?;
            display::print_sessions(sessions, timer_type, timespan)?;
        }
        RustodoroCommand::DisplayShortBreaks(command) => {
            if !config.log_to_db {
                println!("WARNING: Session logging is currently disabled, only displaying sessions completed whilst session logging enabled.\n");
            }
            let timer_type = TimerType::ShortBreak;
            let timespan = command.subcommand;
            let sessions = db::get_sessions(timer_type, &timespan)?;
            display::print_sessions(sessions, timer_type, timespan)?;
        }
        RustodoroCommand::DisplayLongBreaks(command) => {
            if !config.log_to_db {
                println!("WARNING: Session logging is currently disabled, only displaying sessions completed whilst session logging enabled.\n");
            }
            let timer_type = TimerType::LongBreak;
            let timespan = command.subcommand;
            let sessions = db::get_sessions(timer_type, &timespan)?;
            display::print_sessions(sessions, timer_type, timespan)?;
        }
    }

    Ok(())
}
