mod args;
mod rustodoro_config;

// use std::cmp;
use std::io::stdout;
use std::io::Write;
use std::time::Instant;
// use args::SetWorkTimerCommand;
use crossterm::queue;
use crossterm::style;
// use serde::Serialize;
// use serde::Deserialize;
// use std::fs;
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
use args::RustodoroArgs;
use args::RustodoroCommand;
use clap::Parser;

//mod config;
use rustodoro_config::RustodoroConfig;

// // TODO: Do we want the config to be aware of its own path?
// #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
// struct Config {
//     work_time: u16,
//     short_break_time: u16,
//     long_break_time: u16,
//     pomodoros_till_long_break: u16,
//     display_in_secs: bool,
// }

// impl Config {
//     // TODO: Should saving and loading be a method or a static function?
//     // TODO: Have this return errors OR just panic?
//     //fn save(&self, config_path: &str) {
//     fn save(config: &Config, config_path: &str) {
//         //let contents = toml::to_string(config);
//         let contents = toml::to_string(config)
//             .expect("Could not parse config as string.");
//         fs::write(config_path, contents.as_str())
//             .expect(format!("Could not write to file {}", config_path).as_str());
//         // if let Ok(foo) = contents {
//         //     // TODO: Probs not the best way to handle an error, but what else do we want to do?
//         //     fs::write(config_path, foo.as_str())
//         //         .expect(format!("Could not write to file {}", config_path).as_str());
//         // }
//     }

//     // TODO: Have this handle errors gracefully or just have it panic if things aren't right?
//     // To be fair, if we have errors reading or parsing we don't /want/ the program to continue.
//     fn load(config_path: &str) -> Config {
//         let contents = fs::read_to_string(config_path)
//             .expect(format!("Could not read file {}", config_path).as_str());
//         let config: Config = toml::from_str(&contents)
//             .unwrap();
//         config
//     }

//     // Make new config!
//     fn set_work_timer(self, command: SetWorkTimerCommand) -> Config {
//         // This is in seconds!
//         let mut work_timer: u16 = 0;

//         if let Some(minutes) = command.minutes {
//             work_timer += minutes * 60;// TODO: Do we want the config to be aware of its own path?
// #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
// struct Config {
//     work_time: u16,
//     short_break_time: u16,
//     long_break_time: u16,
//     pomodoros_till_long_break: u16,
//     display_in_secs: bool,
// }

// impl Config {
//     // TODO: Should saving and loading be a method or a static function?
//     // TODO: Have this return errors OR just panic?
//     //fn save(&self, config_path: &str) {
//     fn save(config: &Config, config_path: &str) {
//         //let contents = toml::to_string(config);
//         let contents = toml::to_string(config)
//             .expect("Could not parse config as string.");
//         fs::write(config_path, contents.as_str())
//             .expect(format!("Could not write to file {}", config_path).as_str());
//         // if let Ok(foo) = contents {
//         //     // TODO: Probs not the best way to handle an error, but what else do we want to do?
//         //     fs::write(config_path, foo.as_str())
//         //         .expect(format!("Could not write to file {}", config_path).as_str());
//         // }
//     }

//     // TODO: Have this handle errors gracefully or just have it panic if things aren't right?
//     // To be fair, if we have errors reading or parsing we don't /want/ the program to continue.
//     fn load(config_path: &str) -> Config {use args::SetWorkTimerCommand;
//             .unwrap();
//         config
//     }

//     // Make new config!
//     fn set_work_timer(self, command: SetWorkTimerCommand) -> Config {
//         // This is in seconds!
//         let mut work_timer: u16 = 0;

//         if let Some(minutes) = command.minutes {
//             work_timer += minutes * 60;
//         }

//         if let Some(seconds) = command.seconds {
//             match seconds {
//                 0..=60 => work_timer += seconds as u16,
//                 _ => println!("Error!") // TODO: What do we do in this case? Should this return a Result?
//             }
//         }

//         Config { 
//             work_time: work_timer,
//             ..self
//         }
//     }
// }
//         }

//         Config { 
//             work_time: work_timer,
//             ..self
//         }
//     }
// }

#[derive(Copy, Clone)]
enum TimerType {
    Work,
    ShortBreak,
    LongBreak,
}

const CONFIG_PATH: &str = "./Config.toml";

fn main() -> Result<(), std::io::Error> {
    // let contents = fs::read_to_string(CONFIG_PATH)
    //     .expect(format!("Could not read file {}", CONFIG_PATH).as_str());
    // let config: Config = toml::from_str(&contents).unwrap();
    let config = RustodoroConfig::load(CONFIG_PATH);

    // TODO: What errors do we want these functions all to throw? Do we want them all to be propagatable updwards (if that's even a phrase)?
    let args: RustodoroArgs = RustodoroArgs::parse();
    match args.command {
        RustodoroCommand::Work => run_timer(config, TimerType::Work)?,
        RustodoroCommand::ShortBreak => run_timer(config, TimerType::ShortBreak)?,
        RustodoroCommand::LongBreak => run_timer(config, TimerType::LongBreak)?,
        RustodoroCommand::SetWorkTimer(command) => {
            let new_config = config.set_work_timer(command);
            RustodoroConfig::save(&new_config, CONFIG_PATH);
            //new_config.save(CONFIG_PATH); // We want this as its own function and not as part of set_work_timer, right?
        },
        RustodoroCommand::SetShortBreakTimer(command) => {
            println!("{:?}", command)
        },
        RustodoroCommand::SetLongBreakTimer(command) => {
            println!("{:?}", command)
        },
        RustodoroCommand::SetPomodorosToLongBreak(command) => {
            println!("{:?}", command)
        },
    }

    Ok(())
}

// TODO: Can we make the config global? How do we tell it which timer to run? Does it need to know which one? Do we want to print out which timer is running?
fn run_timer(config: RustodoroConfig, timer_type: TimerType) -> Result<(), std::io::Error> {
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