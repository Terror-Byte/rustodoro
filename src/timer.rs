use crate::display;
use crate::error::Result;
use std::time::{Instant, SystemTime};

#[derive(Copy, Clone)]
pub enum TimerType {
    Work,
    ShortBreak,
    LongBreak,
}

pub fn run_timer(time: u16, timer_type: TimerType) -> Result<(u64, u64)> {
    let start_timestamp = get_current_unix_time()?;
    let start = Instant::now();
    display::print_time_remaining(time, time, timer_type)?;

    let mut old_printed_value: u16 = 0;
    loop {
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

    Ok((start_timestamp, end_timestamp))
}

fn get_current_unix_time() -> Result<u64> {
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs())
}
