mod args;
mod config;
mod db;
mod error;
mod timer;

use args::{RustodoroArgs, RustodoroCommand};
use clap::Parser;
use config::Config;
use error::Result;
use timer::TimerType;

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
        RustodoroCommand::DisplayPomodoros(command) => {
            let timer_type = TimerType::Work;
            let timespan = command.subcommand;
            let sessions = db::get_sessions(timer_type, &timespan)?;
            print::print_sessions(sessions, timer_type, timespan)?;
        }
        RustodoroCommand::DisplayShortBreaks(command) => {
            let timer_type = TimerType::ShortBreak;
            let timespan = command.subcommand;
            let sessions = db::get_sessions(timer_type, &timespan)?;
            print::print_sessions(sessions, timer_type, timespan)?;
        }
        RustodoroCommand::DisplayLongBreaks(command) => {
            let timer_type = TimerType::LongBreak;
            let timespan = command.subcommand;
            let sessions = db::get_sessions(timer_type, &timespan)?;
            print::print_sessions(sessions, timer_type, timespan)?;
        }
    }

    Ok(())
}

// TODO: Rename this module and move it to its own file?
// TODO: Move the timer printing code into this module too, so our code for displaying to console
// is packaged together?
mod print {
    use crate::args::DisplayCommand;
    use crate::error::{Error, Result};
    use crate::timer::TimerType;
    use chrono::{DateTime, Local, TimeZone};

    pub fn print_sessions(
        sessions: Vec<(u64, u64)>,
        timer_type: TimerType,
        timespan: Option<DisplayCommand>,
    ) -> Result<()> {
        match timespan {
            Some(DisplayCommand::Day) => {
                print_sessions_without_date(sessions, timer_type, DisplayCommand::Day)?
            }
            Some(DisplayCommand::Week) => {
                print_sessions_with_date(sessions, timer_type, DisplayCommand::Week)?
            }
            Some(DisplayCommand::Month) => {
                print_sessions_with_date(sessions, timer_type, DisplayCommand::Month)?
            }
            None => print_sessions_without_date(sessions, timer_type, DisplayCommand::Day)?,
        }

        Ok(())
    }

    fn print_summary_string(
        session_count: usize,
        session_type: TimerType,
        timespan: DisplayCommand,
    ) {
        let session_name = match session_type {
            TimerType::Work => "pomodoro(s)",
            TimerType::ShortBreak => "short break(s)",
            TimerType::LongBreak => "long break(s)",
        };

        let timespan_string = match timespan {
            DisplayCommand::Day => "today",
            DisplayCommand::Week => "this week",
            DisplayCommand::Month => "this month",
        };

        println!(
            "You completed {} {} {}.\n",
            session_count, session_name, timespan_string,
        );
    }

    fn print_sessions_without_date(
        sessions: Vec<(u64, u64)>,
        session_type: TimerType,
        timespan: DisplayCommand,
    ) -> Result<()> {
        print_summary_string(sessions.len(), session_type, timespan);

        // TODO: Find a library to print this as a nice table? Can I use crossterm?
        println!(
            "| {:^10} | {:^10} | {:^10} |",
            "session", "start time", "end time"
        );
        println!(
            "| {} | {} | {} |",
            "-".repeat(10),
            "-".repeat(10),
            "-".repeat(10)
        );

        let mut i = 1;
        for session in sessions {
            let start_time: DateTime<Local> = Local
                .timestamp_opt(session.0 as i64, 0)
                .single()
                .ok_or(Error::DateTimeError(String::from(
                    "Failed to parse timestamp as a valid datetime!",
                )))?;
            let end_time: DateTime<Local> = Local
                .timestamp_opt(session.1 as i64, 0)
                .single()
                .ok_or(Error::DateTimeError(String::from(
                    "Failed to parse timestamp as a valid datetime!",
                )))?;
            println!(
                "| {:^10} | {:^10} | {:^10} |",
                i,
                start_time.format("%H:%M:%S"),
                end_time.format("%H:%M:%S")
            );
            i += 1;
        }

        Ok(())
    }

    fn print_sessions_with_date(
        sessions: Vec<(u64, u64)>,
        session_type: TimerType,
        timespan: DisplayCommand,
    ) -> Result<()> {
        print_summary_string(sessions.len(), session_type, timespan);

        // TODO: Find a library to print this as a nice table? Can I use crossterm?
        println!(
            "| {:^10} | {:^10} | {:^10} | {:^10} |",
            "session", "date", "start time", "end time"
        );
        println!(
            "| {} | {} | {} | {} |",
            "-".repeat(10),
            "-".repeat(10),
            "-".repeat(10),
            "-".repeat(10),
        );

        // TODO: Do we number them by-day, or by week overall?
        let mut i = 1;
        for session in sessions {
            let start_time: DateTime<Local> = Local
                .timestamp_opt(session.0 as i64, 0)
                .single()
                .ok_or(Error::DateTimeError(String::from(
                    "Failed to parse timestamp as a valid datetime!",
                )))?;
            let end_time: DateTime<Local> = Local
                .timestamp_opt(session.1 as i64, 0)
                .single()
                .ok_or(Error::DateTimeError(String::from(
                    "Failed to parse timestamp as a valid datetime!",
                )))?;
            println!(
                "| {:^10} | {:^10} | {:^10} | {:^10} |",
                i,
                start_time.format("%Y-%m-%d"),
                start_time.format("%H:%M:%S"),
                end_time.format("%H:%M:%S")
            );
            i += 1;
        }

        Ok(())
    }
}
