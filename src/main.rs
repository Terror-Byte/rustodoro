use std::time::Instant;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("No arguments supplied.");
        return;
    } else if args.len() > 2 {
        println!("Too many arguments supplied.");
        return;
    }

    // Attempt to parse the first argument as an unsigned 64-bit int.
    // If success, run timer.
    // If failure, return error.
    match &args[1].parse::<u64>() {
        Ok(n) => run_timer(n),
        Err(e) => println!("Failed to parse argument as integer! Error: {}", e),
    }
}

fn run_timer(time_in_secs: &u64) {
    let start = Instant::now();
    print_time_remaining(*time_in_secs);

    let mut old_printed_value: u64 = 0;
    loop {
        let elapsed_seconds = start.elapsed().as_secs();

        if elapsed_seconds > old_printed_value {
            let time_remaining = time_in_secs - elapsed_seconds;
            print_time_remaining(time_remaining);
            old_printed_value = elapsed_seconds;
        }

        if elapsed_seconds >= *time_in_secs {
            break;
        }
    }
    println!("Timer elapsed!");
}

fn print_time_remaining(time_remaining: u64) {
    print!("{esc}c", esc = 27 as char); // Hacky way of clearing the console! https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
    println!("{} seconds to go.", time_remaining);
}

//use std::fs;
// fn temp_read_file_and_print(filename: &str) {
//     let contents = fs::read_to_string(filename).expect("Unable to read file");
//     println!("Contents of file {}", filename);
//     println!("{}", contents);
// }