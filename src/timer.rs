use crate::display;
use crate::error::Result;
use crossterm::{
    event::{
        poll, read, Event, KeyCode, KeyModifiers, KeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::stdout;
use std::time::{Duration, Instant, SystemTime};

#[derive(Copy, Clone)]
pub enum TimerType {
    Work,
    ShortBreak,
    LongBreak,
}

pub fn run_timer(time: u16, timer_type: TimerType) -> Result<(u64, u64)> {
    // Set up crossterm input handling, to allow users to pause/unpause the timer
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(
        stdout,
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                | KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
        )
    )?;

    let start_timestamp = get_current_unix_time()?;
    let start = Instant::now();
    display::print_time_remaining(time, time, timer_type)?;

    let mut old_printed_value: u16 = 0;

    let mut is_paused = false;
    let mut unpaused_this_frame = false;
    let mut pause_start = Instant::now();
    let mut time_spent_paused: u64 = 0;

    loop {
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Key(key) => {
                    if key.code == KeyCode::Char(' ') && key.is_press() {
                        is_paused = !is_paused;

                        if is_paused {
                            pause_start = Instant::now();
                            display::print_paused_text()?;
                        } else {
                            unpaused_this_frame = true;
                            let pause_duration = pause_start.elapsed().as_secs();
                            time_spent_paused += pause_duration;
                        }
                    } else if key.code == KeyCode::Esc
                        || (key.code == KeyCode::Char('c')
                            && key.modifiers.contains(KeyModifiers::CONTROL))
                    {
                        std::process::exit(0);
                    }
                }
                _ => {}
            }
        }

        if !is_paused {
            let elapsed_seconds = (start.elapsed().as_secs() - time_spent_paused) as u16;

            if elapsed_seconds > old_printed_value || unpaused_this_frame {
                let time_remaining = time - elapsed_seconds;
                display::print_time_remaining(time_remaining, time, timer_type)?;
                old_printed_value = elapsed_seconds;
                unpaused_this_frame = false;
            }

            if elapsed_seconds >= time {
                break;
            }
        }
    }

    let end_timestamp = get_current_unix_time()?;

    crate::display::print_timer_elapsed()?;

    execute!(stdout, PopKeyboardEnhancementFlags)?;
    disable_raw_mode()?;

    Ok((start_timestamp, end_timestamp))
}

fn get_current_unix_time() -> Result<u64> {
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs())
}
