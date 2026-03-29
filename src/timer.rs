use crate::error::Result;
use crossterm::{
    cursor, queue, style,
    style::{Color, Stylize},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};
use std::time::{Instant, SystemTime};

#[derive(Copy, Clone)]
pub enum TimerType {
    Work,
    ShortBreak,
    LongBreak,
}

pub fn run_timer(time: u16, timer_type: TimerType) -> Result<(u64, u64)> {
    // TODO: Feels weird having a SystemTime *and* an Instant, is there a way we can combine them?
    let start_timestamp = get_current_unix_time()?;
    let start = Instant::now();
    print_time_remaining(time, time, timer_type)?;

    let mut old_printed_value: u16 = 0;
    loop {
        let elapsed_seconds = start.elapsed().as_secs() as u16;

        if elapsed_seconds > old_printed_value {
            let time_remaining = time - elapsed_seconds;
            print_time_remaining(time_remaining, time, timer_type)?;
            old_printed_value = elapsed_seconds;
        }

        if elapsed_seconds >= time {
            break;
        }
    }
    let end_timestamp = get_current_unix_time()?;

    let mut stdout = stdout();
    queue!(
        stdout,
        cursor::MoveToNextLine(1),
        style::Print("Timer elapsed!"),
        cursor::MoveToNextLine(1),
        cursor::Show
    )?;

    Ok((start_timestamp, end_timestamp))
}

fn print_time_remaining(time_remaining: u16, total_time: u16, timer_type: TimerType) -> Result<()> {
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

fn format_time(minutes: u16, seconds: u16) -> String {
    match seconds {
        0..=10 => format!("{}:{:0>2} Remaining", minutes, seconds),
        _ => format!("{}:{} Remaining", minutes, seconds),
    }
}

fn get_current_unix_time() -> Result<u64> {
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs())
}
