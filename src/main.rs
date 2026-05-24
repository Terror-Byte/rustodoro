mod args;
mod config;
mod db;
mod display;
mod error;
mod timer;

use args::{RustodoroArgs, RustodoroCommand};
use clap::Parser;
use config::Config;
use error::{Error, Result};
use notify_rust::{Hint, Notification, Urgency};
use timer::TimerType;

use crate::args::{HasValue, TimeSpan, ToSeconds};

fn main() -> Result<()> {
    // TODO: For the commands where we're modifying the config, what sort of user feedback do we want to let the user know the command executed successfully?
    let config_path = config::get_config_path()
        .ok_or(Error::PathError("Failed to get config path".to_string()))?;
    let config = Config::load(config_path.as_str())?;
    let args: RustodoroArgs = RustodoroArgs::parse();
    match args.command {
        RustodoroCommand::Work(args) => {
            let work_time = if args.has_value() {
                args.to_seconds()
            } else {
                config.work_time
            };

            let (start_time, end_time) = timer::run_timer(work_time, TimerType::Work)?;
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
                            let timespan = TimeSpan::Day;
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
            if config.desktop_notifications {
                Notification::new()
                    .summary("Rustodoro")
                    .body("Your work session is complete")
                    .icon("chronometer")
                    .appname("Rustodoro")
                    .hint(Hint::Urgency(Urgency::Normal))
                    .show()?;
            }
        }
        RustodoroCommand::ShortBreak(args) => {
            let short_break_time = if args.has_value() {
                args.to_seconds()
            } else {
                config.short_break_time
            };

            let (start_time, end_time) = timer::run_timer(short_break_time, TimerType::ShortBreak)?;
            if config.log_to_db {
                db::save_session_to_db(start_time, end_time, TimerType::ShortBreak)?;
            }
            if config.desktop_notifications {
                Notification::new()
                    .summary("Rustodoro")
                    .body("Your short break is complete")
                    .icon("chronometer")
                    .appname("Rustodoro")
                    .hint(Hint::Urgency(Urgency::Normal))
                    .show()?;
            }
        }
        RustodoroCommand::LongBreak(args) => {
            let long_break_time = if args.has_value() {
                args.to_seconds()
            } else {
                config.long_break_time
            };

            let (start_time, end_time) = timer::run_timer(long_break_time, TimerType::LongBreak)?;
            if config.log_to_db {
                db::save_session_to_db(start_time, end_time, TimerType::LongBreak)?;
            }
            if config.desktop_notifications {
                Notification::new()
                    .summary("Rustodoro")
                    .body("Your long break is complete")
                    .icon("chronometer")
                    .appname("Rustodoro")
                    .hint(Hint::Urgency(Urgency::Normal))
                    .show()?;
            }
        }
        RustodoroCommand::SetWorkTime(args) => {
            let new_config = config.set_work_time(args)?;
            Config::save(&new_config, config_path.as_str())?;
        }
        RustodoroCommand::SetShortBreakTime(args) => {
            let new_config = config.set_short_break_time(args)?;
            Config::save(&new_config, config_path.as_str())?;
        }
        RustodoroCommand::SetLongBreakTime(args) => {
            let new_config = config.set_long_break_time(args)?;
            Config::save(&new_config, config_path.as_str())?;
        }
        RustodoroCommand::SetPomodorosToLongBreak(args) => {
            let new_config = config.set_pomodoros_to_long_break(args);
            Config::save(&new_config, config_path.as_str())?;
        }
        RustodoroCommand::SetLogToDB(args) => {
            let new_config = config.set_log_to_db(args);
            Config::save(&new_config, config_path.as_str())?;
        }
        RustodoroCommand::SetDesktopNotifications(args) => {
            let new_config = config.set_desktop_notifications(args);
            Config::save(&new_config, config_path.as_str())?;
        }
        RustodoroCommand::DisplayPomodoros(command) => {
            if !config.log_to_db {
                println!("WARNING: Session logging is currently disabled, only displaying sessions completed whilst session logging enabled.\n");
            }
            let timer_type = TimerType::Work;
            let timespan = command.subcommand;
            let sessions = db::get_sessions(timer_type, &timespan).unwrap_or(vec![]);
            display::print_sessions(sessions, timer_type, timespan)?;
        }
        RustodoroCommand::DisplayShortBreaks(command) => {
            if !config.log_to_db {
                println!("WARNING: Session logging is currently disabled, only displaying sessions completed whilst session logging enabled.\n");
            }
            let timer_type = TimerType::ShortBreak;
            let timespan = command.subcommand;
            let sessions = db::get_sessions(timer_type, &timespan).unwrap_or(vec![]);
            display::print_sessions(sessions, timer_type, timespan)?;
        }
        RustodoroCommand::DisplayLongBreaks(command) => {
            if !config.log_to_db {
                println!("WARNING: Session logging is currently disabled, only displaying sessions completed whilst session logging enabled.\n");
            }
            let timer_type = TimerType::LongBreak;
            let timespan = command.subcommand;
            let sessions = db::get_sessions(timer_type, &timespan).unwrap_or(vec![]);
            display::print_sessions(sessions, timer_type, timespan)?;
        }
    }

    Ok(())
}
