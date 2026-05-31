use crate::args::TimeSpan;
use crate::db::SessionVector;
use crate::error::{Error, Result};
use crate::timer::TimerType;
use chrono::{DateTime, Local, TimeZone};
use crossterm::{
    cursor, queue, style,
    style::{Color, Stylize},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};

const WORK_TIMER_TITLE: &str = "Work Timer";
const SHORT_BREAK_TIMER_TITLE: &str = "Short Break Timer";
const LONG_BREAK_TIMER_TITLE: &str = "Long Break Timer";

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
        TimerType::Work => String::from(WORK_TIMER_TITLE),
        TimerType::ShortBreak => String::from(SHORT_BREAK_TIMER_TITLE),
        TimerType::LongBreak => String::from(LONG_BREAK_TIMER_TITLE),
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

pub fn print_paused_text(timer_type: TimerType) -> Result<()> {
    let title_length = match timer_type {
        TimerType::Work => WORK_TIMER_TITLE.len(),
        TimerType::ShortBreak => SHORT_BREAK_TIMER_TITLE.len(),
        TimerType::LongBreak => LONG_BREAK_TIMER_TITLE.len(),
    } as u16;

    let mut stdout = stdout();
    queue!(
        stdout,
        cursor::MoveTo(title_length, 0),
        style::Print(" - Paused!")
    )?;
    stdout.flush()?;

    Ok(())
}

pub fn print_hotkeys() -> Result<()> {
    let mut stdout = stdout();
    queue!(
        stdout,
        cursor::MoveTo(0, 4),
        Clear(ClearType::FromCursorDown),
        cursor::Hide,
        style::PrintStyledContent(
            "Space - Pause/Unpause, Esc/Ctrl + C - Quit".with(Color::DarkGrey)
        )
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
    stdout.flush()?;

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
    sessions: SessionVector,
    timer_type: TimerType,
    timespan: Option<TimeSpan>,
) -> Result<()> {
    match timespan {
        Some(TimeSpan::Day) => print_sessions_without_date(sessions, timer_type, TimeSpan::Day)?,
        Some(TimeSpan::Week) => print_sessions_with_date(sessions, timer_type, TimeSpan::Week)?,
        Some(TimeSpan::Month) => print_sessions_with_date(sessions, timer_type, TimeSpan::Month)?,
        None => print_sessions_without_date(sessions, timer_type, TimeSpan::Day)?,
    }

    Ok(())
}

fn print_summary_string(session_count: usize, session_type: TimerType, timespan: TimeSpan) {
    let session_name = match session_type {
        TimerType::Work => "pomodoro(s)",
        TimerType::ShortBreak => "short break(s)",
        TimerType::LongBreak => "long break(s)",
    };

    let timespan_string = match timespan {
        TimeSpan::Day => "today",
        TimeSpan::Week => "this week",
        TimeSpan::Month => "this month",
    };

    println!(
        "You completed {} {} {}.\n",
        session_count, session_name, timespan_string,
    );
}

fn print_sessions_without_date(
    sessions: SessionVector,
    session_type: TimerType,
    timespan: TimeSpan,
) -> Result<()> {
    print_summary_string(sessions.len(), session_type, timespan);

    // TODO: Find a library to print this as a nice table? Can I use crossterm?
    println!(
        "| {:^10} | {:^10} | {:^10} | {:^20} |",
        "session", "start time", "end time", "comment"
    );
    println!(
        "| {} | {} | {} | {} |",
        "-".repeat(10),
        "-".repeat(10),
        "-".repeat(10),
        "-".repeat(20)
    );

    // TODO: Can we do an initial loop over the result and check what the longest comment is
    // we can then use that to dynamically resize the table
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
            "| {:^10} | {:^10} | {:^10} | {:^20} |",
            i,
            start_time.format("%H:%M:%S"),
            end_time.format("%H:%M:%S"),
            session.2,
        );
        i += 1;
    }

    Ok(())
}

fn print_sessions_with_date(
    sessions: SessionVector,
    session_type: TimerType,
    timespan: TimeSpan,
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
