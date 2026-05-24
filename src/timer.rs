use crate::display;
use crate::error::Result;
use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::time::{Duration, Instant, SystemTime};

#[derive(Copy, Clone)]
pub enum TimerType {
    Work,
    ShortBreak,
    LongBreak,
}

pub fn run_timer(time: u16, timer_type: TimerType) -> Result<(u64, u64)> {
    enable_raw_mode()?;

    let start_timestamp = get_current_unix_time()?;
    let start = Instant::now();
    display::print_time_remaining(time, time, timer_type)?;

    let mut old_printed_value: u16 = 0;
    loop {
        // TODO: Make sure the spacebar press doesn't keep triggering if it's held down
        // How do we factor in pausing, as right now we're just using the time since we started?
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Key(key) => {
                    if key.code == KeyCode::Char(' ') {
                        // TODO: Pause/unpause the timer
                        // TODO: Maybe take a timestamp of when we paused and unpaused, to figure
                        // out how much time to remove from the elapsed seconds?
                        // Print old_printed_value when we're paused?
                        // What if we have multiple pauses in our session?
                        println!("Space bar!");
                    }
                }
                _ => {}
            }
        }

        let elapsed_seconds = start.elapsed().as_secs() as u16;

        if elapsed_seconds > old_printed_value {
            let time_remaining = time - elapsed_seconds;
            display::print_time_remaining(time_remaining, time, timer_type)?;
            old_printed_value = elapsed_seconds;
        }

        if elapsed_seconds >= time {
            break;
        }
    }
    let end_timestamp = get_current_unix_time()?;

    crate::display::print_timer_elapsed()?;

    disable_raw_mode()?;

    Ok((start_timestamp, end_timestamp))
}

fn get_current_unix_time() -> Result<u64> {
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs())
}
