use std::io::stdout;
use std::io::Write;
use std::time::Instant;
use clap::Command;
use crossterm::queue;
use crossterm::style;
use serde::Serialize;
use serde::Deserialize;
use std::fs;
use std::env;
use crossterm::terminal::{ 
    Clear, 
    ClearType 
};
use crossterm::cursor;
use crossterm::style::{ 
    Color, 
    Stylize 
};

// My modules
mod args;
use args::RustodoroArgs;
use args::RustodoroCommand;
use clap::Parser;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    work_time: u64,
    short_break_time: u64,
    long_break_time: u64,
    pomodoros_till_long_break: u64,
    display_in_secs: bool,
}

#[derive(Copy, Clone)]
enum TimerType {
    Work,
    ShortBreak,
    LongBreak,
}

const CONFIG_PATH: &str = "./Config.toml";
const WORK_FLAG: &str = "--work";
const SHORT_BREAK_FLAG: &str = "--short-break";
const LONG_BREAK_FLAG: &str = "--long-break";
const SECONDS_FLAG_LONG: &str = "--seconds";
const SECONDS_FLAG_SHORT: &str = "-s";
const MINUTES_FLAG_LONG: &str = "--minutes";
const MINUTES_FLAG_SHORT: &str = "-m";

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(CONFIG_PATH)
        .expect(format!("Could not read file {}", CONFIG_PATH).as_str());
    let config: Config = toml::from_str(&contents).unwrap();

    // TODO: Parse commandline args here!
    // Is the user starting a work timer, a short/long break timer, modifying their config?     

    // OLD CMD ARG PARSING
    // let args: Vec<String> = env::args().collect();
    // // match args.len() {
    // //     2 => (),
    // //     3 => (),
    // //     4 => (),
    // //     _ => {
    // //         println!("Too few or too many arguments supplied! Pass --help argument to see possible options.");
    // //         return Ok(());
    // //     },
    // // }
    // if args.len() != 2 {
    //     println!("Incorrect number of arguments supplied! Pass --help argument to see possible options.");
    //     return Ok(());
    // }

    // let timer_type = match args[1].as_str() {
    //     WORK_FLAG => TimerType::Work,
    //     SHORT_BREAK_FLAG => TimerType::ShortBreak,
    //     LONG_BREAK_FLAG => TimerType::LongBreak,
    //     _ => {
    //         println!("Unrecognised argument supplied! Pass --help argument to see possible options.");
    //         return Ok(());
    //     }
    // };

    // run_timer(config, timer_type)?; // TODO: Enum for this? Work time, short break, long break?

    let args: RustodoroArgs = RustodoroArgs::parse();
    //println!("{:?}", args);

    match args.command {
        RustodoroCommand::Work => run_timer(config, TimerType::Work)?,
        RustodoroCommand::ShortBreak => run_timer(config, TimerType::ShortBreak)?,
        RustodoroCommand::LongBreak => run_timer(config, TimerType::LongBreak)?,
        RustodoroCommand::SetWorkTimer(_foo) => (),
    }

    Ok(())
}

// TODO: Can we make the config global? How do we tell it which timer to run? Does it need to know which one? Do we want to print out which timer is running?
fn run_timer(config: Config, timer_type: TimerType) -> Result<(), std::io::Error> {
    let time = match timer_type {
        TimerType::Work => config.work_time,
        TimerType::ShortBreak => config.short_break_time,
        TimerType::LongBreak => config.long_break_time
    };

    let start = Instant::now();
    print_time_remaining(time, time, timer_type)?;

    let mut old_printed_value: u64 = 0;
    loop {
        let elapsed_seconds = start.elapsed().as_secs();

        if elapsed_seconds > old_printed_value {
            let time_remaining = time - elapsed_seconds;
            print_time_remaining(time_remaining, time, timer_type)?;
            old_printed_value = elapsed_seconds;
        }

        if elapsed_seconds >= time {
            break;
        }
    }
    
    // TODO: Save to file that we've done another work/break stint. Do we want to save logs per day? That might be best!
    // Do we have a max size/amount of logs? Might be worth looking into later but don't worry for now.
    let mut stdout = stdout();
    queue!(
        stdout,
        cursor::MoveToNextLine(1),
        style::Print("Timer elapsed!"),
        cursor::Show
    )?;
    Ok(())
}

fn print_time_remaining(time_remaining: u64, total_time: u64, timer_type: TimerType) -> Result<(), std::io::Error> {
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

    let header = match timer_type {
        TimerType::Work => String::from("Work Timer"),
        TimerType::ShortBreak => String::from("Short Break Timer"),
        TimerType::LongBreak => String::from("Long Break Timer")
    };

    let mut stdout = stdout();
    queue!(
        stdout,
        cursor::MoveTo(0, 0),
        Clear(ClearType::FromCursorDown),
        cursor::Hide,
        style::Print(header),
        cursor::MoveToNextLine(1),
        style::Print(format!("{} seconds to go.", time_remaining)),
        cursor::MoveToNextLine(1),
        style::Print("["),
        style::PrintStyledContent(progress_bar.with(Color::Green)),
        style::Print(format!("] {}%", percentage))
    )?;
    stdout.flush()?;
    Ok(())
}