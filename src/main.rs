use std::io::stdout;
use std::io::Write;
use std::time::Instant;
use args::SetWorkTimerCommand;
use crossterm::queue;
use crossterm::style;
use serde::Serialize;
use serde::Deserialize;
use std::fs;
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
    work_time: u16,
    short_break_time: u16,
    long_break_time: u16,
    pomodoros_till_long_break: u16,
    display_in_secs: bool,
}

impl Config {
    fn set_work_timer(&mut self, command: SetWorkTimerCommand) {
        // TODO: Do we wanna return an error from this if we don't parse the numbers correctly, or don't write to the file correctly?
        // Do we want a specific function for updating the config too?
        // NOTE: Need to constrain seconds from 0-60!
        
        // This is in seconds!
        let mut work_timer: u16 = 0;

        if let Some(minutes) = command.minutes {
            work_timer += minutes * 60;
        }

        if let Some(seconds) = command.minutes {
            match seconds {
                0..=60 => work_timer += seconds,
                _ => println!("Error!")
            }
        }

        // Save it to config here!
        // Does this want to be a method of Config? Yep!
        self.work_time = work_timer;
        // Return not error here!
    }
}

#[derive(Copy, Clone)]
enum TimerType {
    Work,
    ShortBreak,
    LongBreak,
}

const CONFIG_PATH: &str = "./Config.toml";

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(CONFIG_PATH)
        .expect(format!("Could not read file {}", CONFIG_PATH).as_str());
    let config: Config = toml::from_str(&contents).unwrap();

    let args: RustodoroArgs = RustodoroArgs::parse();
    match args.command {
        RustodoroCommand::Work => run_timer(config, TimerType::Work)?,
        RustodoroCommand::ShortBreak => run_timer(config, TimerType::ShortBreak)?,
        RustodoroCommand::LongBreak => run_timer(config, TimerType::LongBreak)?,
        RustodoroCommand::SetWorkTimer(foo) => println!("{:?}", foo), // TODO: Set up a function to pass in the info here and configure
        RustodoroCommand::SetShortBreakTimer(foo) => println!("{:?}", foo),
        RustodoroCommand::SetLongBreakTimer(foo) => println!("{:?}", foo),
        RustodoroCommand::SetPomodorosToLongBreak(foo) => println!("{:?}", foo),
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

fn print_time_remaining(time_remaining: u16, total_time: u16, timer_type: TimerType) -> Result<(), std::io::Error> {
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