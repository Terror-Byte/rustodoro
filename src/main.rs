mod args;
mod config;
mod db;
mod error;
mod timer;

use args::{DisplayPomodorosCommand, RustodoroArgs, RustodoroCommand};
use chrono::{DateTime, Local, TimeZone};
use clap::Parser;
use config::Config;
use error::{Error, Result};
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
        RustodoroCommand::DisplayPomodoros(command) => match command.command {
            DisplayPomodorosCommand::Day => {
                let sessions = db::get_todays_sessions(TimerType::Work)?;
                print_days_sessions(sessions, TimerType::Work)?;
            }
            DisplayPomodorosCommand::Week => {
                let sessions = db::get_weeks_sessions(TimerType::Work)?;
                print_weeks_sessions(sessions, TimerType::Work)?;
            }
            DisplayPomodorosCommand::Month => println!("Month"),
        },
    }

    Ok(())
}

fn print_days_sessions(sessions: Vec<(u64, u64)>, session_type: TimerType) -> Result<()> {
    let session_name = match session_type {
        TimerType::Work => "pomodoros",
        TimerType::ShortBreak => "short breaks",
        TimerType::LongBreak => "long breaks",
    };

    println!(
        "You have completed {} {} today.\n",
        sessions.len(),
        session_name
    );

    // TODO: Find a library to print this as a nice table?
    println!(
        "| {0: <10} | {1: <10} | {2: <10} |",
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
        let start_time: DateTime<Local> =
            Local
                .timestamp_opt(session.0 as i64, 0)
                .single()
                .ok_or(Error::DateTimeError(String::from(
                    "Failed to parse timestamp as a valid datetime!",
                )))?;
        let end_time: DateTime<Local> =
            Local
                .timestamp_opt(session.1 as i64, 0)
                .single()
                .ok_or(Error::DateTimeError(String::from(
                    "Failed to parse timestamp as a valid datetime!",
                )))?;
        println!(
            "| {0: <10} | {1: <10} | {2: <10} |",
            i,
            start_time.format("%H:%M:%S"),
            end_time.format("%H:%M:%S")
        );
        i += 1;
    }

    Ok(())
}

fn print_weeks_sessions(sessions: Vec<(u64, u64)>, session_type: TimerType) -> Result<()> {
    let session_name = match session_type {
        TimerType::Work => "pomodoros",
        TimerType::ShortBreak => "short breaks",
        TimerType::LongBreak => "long breaks",
    };

    println!(
        "You have completed {} {} this week.\n",
        sessions.len(),
        session_name
    );

    // TODO: Find a library to print this as a nice table?
    println!(
        "| {0: <10} | {1: <10} | {2: <10} | {3: <10} |",
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
        let start_time: DateTime<Local> =
            Local
                .timestamp_opt(session.0 as i64, 0)
                .single()
                .ok_or(Error::DateTimeError(String::from(
                    "Failed to parse timestamp as a valid datetime!",
                )))?;
        let end_time: DateTime<Local> =
            Local
                .timestamp_opt(session.1 as i64, 0)
                .single()
                .ok_or(Error::DateTimeError(String::from(
                    "Failed to parse timestamp as a valid datetime!",
                )))?;
        println!(
            "| {0: <10} | {1: <10} | {2: <10} | {3: <10} |",
            i,
            start_time.format("%Y-%m-%d"),
            start_time.format("%H:%M:%S"),
            end_time.format("%H:%M:%S")
        );
        i += 1;
    }

    Ok(())
}
