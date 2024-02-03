use std::time::Instant;
use std::env;
/*
What args do we want?
--seconds
--minutes
Whatever comes after --seconds is parsed in seconds.
Whatever comes after --minutes is converted to seconds and added to the total time.

./rustodoro 60
./rustodoro --seconds 60
./rustodoro --minutes 1 --seconds 30

Minus the first arg that's always there, we have either:
2 args,
3 args,
5 args.

Anything else, ignore!
*/
// These should be case insensitive!
const SECONDS_FLAG_LONG: &str = "--seconds";
const SECONDS_FLAG_SHORT: &str = "-s";
const MINUTES_FLAG_LONG: &str = "--minutes";
const MINUTES_FLAG_SHORT: &str = "-m";

#[derive(PartialEq)]
enum TimeOption {
    Seconds,
    Minutes,
}

struct ArgumentPair {
    option: TimeOption,
    value: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 | 1 => println!("Too few arguments supplied."),
        2 => { // expect seconds as int
            match &args[1].parse::<u64>() {
                Ok(n) => run_timer(*n),
                Err(e) => println!("Failed to parse argument as integer! Error: {}", e),
            }
        }, 
        3 => { // expect --seconds flag and int
            let parsed_args = parse_args(&args[1], &args[2]);
            match parsed_args {
                Ok(a) => {
                    match a.option {
                        TimeOption::Seconds => run_timer(a.value),
                        TimeOption::Minutes => run_timer(a.value*60),
                    }
                },
                Err(e) => println!("{}", e),
            }
        },
        5 => { // expect --seconds flag followed by int and --minutes flag followed by int. Doesn't matter if --seconds or --minutes comes first
            // Replace this with unwrap, maybe?
            let parsed_args_first_result = parse_args(&args[1], &args[2]);
            let parsed_args_first = match parsed_args_first_result {
                Ok(a) => a,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };

            let parsed_args_second_result= parse_args(&args[3], &args[4]);
            let parsed_args_second = match parsed_args_second_result {
                Ok(a) => a,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };

            // Check the args aren't duped
            if parsed_args_first.option == parsed_args_second.option {
                println!("Cannot use same option twice.");
                return;
            }

            let mut time_in_secs: u64 = 0;
            match parsed_args_first.option {
                TimeOption::Seconds => time_in_secs += parsed_args_first.value,
                TimeOption::Minutes => time_in_secs += parsed_args_first.value * 60,
            }

            match parsed_args_second.option {
                TimeOption::Seconds => time_in_secs += parsed_args_second.value,
                TimeOption::Minutes => time_in_secs += parsed_args_second.value * 60,
            }

            run_timer(time_in_secs);
        },
        _ => println!("Too many arguments supplied."),
    }
}

fn parse_args(option: &String, value: &String) -> Result<ArgumentPair, &'static str> {
    let parsed_option = match option.to_lowercase().as_str() {
        SECONDS_FLAG_LONG | SECONDS_FLAG_SHORT => TimeOption::Seconds,
        MINUTES_FLAG_LONG | MINUTES_FLAG_SHORT => TimeOption::Minutes,
        _ => return Err("Unrecognised flag."),
    };

    let parsed_value = match value.parse::<u64>() {
        Ok(n) => n,
        Err(_) => return Err("Couldn't parse value as integer."),
    };

    Ok(ArgumentPair{ option: parsed_option, value: parsed_value })
}

fn run_timer(time_in_secs: u64) {
    if time_in_secs == 0 {
        println!("Supply a time in seconds greater than 0.");
        return;
    }

    let start = Instant::now();
    print_time_remaining(time_in_secs);

    let mut old_printed_value: u64 = 0;
    loop {
        let elapsed_seconds = start.elapsed().as_secs();

        if elapsed_seconds > old_printed_value {
            let time_remaining = time_in_secs - elapsed_seconds;
            print_time_remaining(time_remaining);
            old_printed_value = elapsed_seconds;
        }

        if elapsed_seconds >= time_in_secs {
            break;
        }
    }
    println!("Timer elapsed!");
}

fn print_time_remaining(time_remaining: u64) {
    // Modify this to display minutes AND seconds!
    print!("{esc}c", esc = 27 as char); // Hacky way of clearing the console! https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
    println!("{} seconds to go.", time_remaining);
}

//use std::fs;
// fn temp_read_file_and_print(filename: &str) {
//     let contents = fs::read_to_string(filename).expect("Unable to read file");
//     println!("Contents of file {}", filename);
//     println!("{}", contents);
// }