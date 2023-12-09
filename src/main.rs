use std::time::Instant;

// Useful examples:
// let seconds = Duration::from_secs(10); (Duration lives in std::time::Duration).
// while start.elapsed().as_secs() < 5 { } <- do stuff whilst the timer hasn't elapsed
// Had use core::time and use std::time::Duration initially but didn't end up using them.

const TIME_IN_SECS: u64 = 60;
fn main() {
    let start = Instant::now();
    print_time_remaining(TIME_IN_SECS);

    let mut old_printed_value: u64 = 0;
    loop {
        let elapsed_seconds = start.elapsed().as_secs();

        if elapsed_seconds > old_printed_value {
            let time_remaining = TIME_IN_SECS - elapsed_seconds;
            print_time_remaining(time_remaining);
            old_printed_value = elapsed_seconds;
        }

        if elapsed_seconds >= TIME_IN_SECS {
            break;
        }
    }

    println!("Timer elapsed!");
}

fn print_time_remaining(time_remaining: u64) {
    print!("{esc}c", esc = 27 as char); // Hacky way of clearing the console! https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
    println!("{} seconds to go.", time_remaining);
}