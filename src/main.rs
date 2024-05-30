use std::time::Instant;
use colored::Colorize;
use serde::Serialize;
use serde::Deserialize;
use std::fs;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    work_time: u64,
    short_break_time: u64,
    long_break_time: u64,
    pomodoros_till_long_break: u64,
    display_in_secs: bool,
}

const CONFIG_PATH: &str = "./Config.toml";
const WORK_FLAG: &str = "--work";
const SHORT_BREAK_FLAG: &str = "--short-break";
const LONG_BREAK_FLAG: &str = "--long-break";
const SECONDS_FLAG_LONG: &str = "--seconds";
const SECONDS_FLAG_SHORT: &str = "-s";
const MINUTES_FLAG_LONG: &str = "--minutes";
const MINUTES_FLAG_SHORT: &str = "-m";

fn main() {
    let contents = fs::read_to_string(CONFIG_PATH)
        .expect(format!("Could not read file {}", CONFIG_PATH).as_str());
    let config: Config = toml::from_str(&contents).unwrap();

    // TODO: Parse commandline args here!
    // Is the user starting a work timer, a short/long break timer, modifying their config?     

    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => (),
        3 => (),
        4 => (),
        _ => {
            println!("Too few or too many arguments supplied! Pass --help argument to see possible options.");
            return;
        },
    }

    // Work Timer (TODO: Put this in its own function! Can be reused for break timer too.)
    let start = Instant::now();
    print_time_remaining(config.work_time, config.work_time);

    let mut old_printed_value: u64 = 0;
    loop {
        let elapsed_seconds = start.elapsed().as_secs();

        if elapsed_seconds > old_printed_value {
            let time_remaining = config.work_time - elapsed_seconds;
            print_time_remaining(time_remaining, config.work_time);
            old_printed_value = elapsed_seconds;
        }

        if elapsed_seconds >= config.work_time {
            break;
        }
    }
    
    println!("Timer elapsed!");
    // TODO: Save to file that we've done another work/break stint. Do we want to save logs per day? That might be best!
    // Do we have a max size/amount of logs? Might be worth looking into later but don't worry for now.
}

fn run_timer(time_in_secs: u64) {

}

fn print_time_remaining(time_remaining: u64, total_time: u64) {
    print!("{esc}c", esc = 27 as char); // Hacky way of clearing the console! https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed 
    println!("{} seconds to go.", time_remaining); // Do this in mins? Allow use to config!

    let percentage: u64 = (100.0 - ((time_remaining as f64/total_time as f64) * 100.0)) as u64;
    let mut progress_bar: String = String::new();
    let progress_amount = percentage/10;
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

    println!("[{}] {}%", progress_bar.green(), percentage);
}