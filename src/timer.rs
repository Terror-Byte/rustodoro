use crate::display;
use crate::error::Result;
use crossterm::{
    cursor,
    event::{
        poll, read, Event, KeyCode, KeyModifiers, KeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    execute, queue, style,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{stdout, Write};
use std::time::{Duration, Instant, SystemTime};

#[derive(Copy, Clone)]
pub enum TimerType {
    Work,
    ShortBreak,
    LongBreak,
}

pub fn run_timer(time: u16, timer_type: TimerType) -> Result<(u64, u64)> {
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

    let mut is_paused = false;
    let mut pause_start = Instant::now(); // TODO: Explain why we're getting the current instant
    let mut overall_paused_time: u64 = 0;

    let mut old_printed_value: u16 = 0;
    loop {
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Key(key) => {
                    // TODO: How do we debounce this input?
                    if key.code == KeyCode::Char(' ') {
                        is_paused = !is_paused;

                        if is_paused {
                            pause_start = Instant::now();
                            // TODO: Move this over to the display module?
                            queue!(stdout, cursor::MoveToNextLine(1), style::Print("Paused!"))?;
                            stdout.flush()?;
                        } else {
                            let pause_duration = pause_start.elapsed().as_secs();
                            overall_paused_time += pause_duration;
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
            let elapsed_seconds = (start.elapsed().as_secs() - overall_paused_time) as u16;

            if elapsed_seconds > old_printed_value {
                let time_remaining = time - elapsed_seconds;
                display::print_time_remaining(time_remaining, time, timer_type)?;
                old_printed_value = elapsed_seconds;
            }

            if elapsed_seconds >= time {
                break;
            }
        }
    }
    let end_timestamp = get_current_unix_time()?;

    crate::display::print_timer_elapsed()?;

    disable_raw_mode()?;
    execute!(stdout, PopKeyboardEnhancementFlags)?;

    Ok((start_timestamp, end_timestamp))
}

fn get_current_unix_time() -> Result<u64> {
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs())
}
