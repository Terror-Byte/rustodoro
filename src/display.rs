use crate::args::DisplayCommand;
use crate::error::{Error, Result};
use crate::timer::TimerType;
use chrono::{DateTime, Local, TimeZone};

// TODO: Move the timer printing code into this module too, so our code for displaying to console
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

fn print_summary_string(session_count: usize, session_type: TimerType, timespan: DisplayCommand) {
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
