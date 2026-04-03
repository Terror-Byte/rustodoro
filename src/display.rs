use crate::args::DisplayCommand;
use crate::error::{Error, Result};
use crate::timer::TimerType;
use chrono::{DateTime, Local, TimeZone};
use crossterm::{
    cursor, queue, style,
    style::{Color, Stylize},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};

// TODO: Put this in a submodule called timer?
pub fn print_time_remaining(
    time_remaining: u16,
    total_time: u16,
    timer_type: TimerType,
) -> Result<()> {
    let percentage: u64 = (100.0 - ((time_remaining as f64 / total_time as f64) * 100.0)) as u64;
    let mut progress_bar: String = String::new();
    let progress_amount = percentage / 10;
    let space_amount = 10 - progress_amount;

    if progress_amount > 0 {
        for _i in 0..progress_amount {
            progress_bar += "=";
        }
    }

    if space_amount > 0 {
        for _i in 0..space_amount {
            progress_bar += " ";
        }
    }

    let header = match timer_type {
        TimerType::Work => String::from("Work Timer"),
        TimerType::ShortBreak => String::from("Short Break Timer"),
        TimerType::LongBreak => String::from("Long Break Timer"),
    };

    let minutes_component = time_remaining / 60;
    let seconds_component = time_remaining % 60;

    let mut stdout = stdout();
    queue!(
        stdout,
        cursor::MoveTo(0, 0),
        Clear(ClearType::FromCursorDown),
        cursor::Hide,
        style::Print(header),
        cursor::MoveToNextLine(1),
        style::Print(format_time(minutes_component, seconds_component)),
        cursor::MoveToNextLine(1),
        style::Print("["),
        style::PrintStyledContent(progress_bar.with(Color::Green)),
        style::Print(format!("] {}%", percentage))
    )?;
    stdout.flush()?;

    Ok(())
}

pub fn print_timer_elapsed() -> Result<()> {
    let mut stdout = stdout();
    queue!(
        stdout,
        cursor::MoveToNextLine(1),
        style::Print("Timer elapsed!"),
        cursor::MoveToNextLine(1),
        cursor::Show
    )?;

    Ok(())
}

fn format_time(minutes: u16, seconds: u16) -> String {
    match seconds {
        0..=10 => format!("{}:{:0>2} Remaining", minutes, seconds),
        _ => format!("{}:{} Remaining", minutes, seconds),
    }
}

// TODO: Put this in a submodule called session?
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
