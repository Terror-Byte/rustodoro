mod args;
mod config;
mod timer;

use args::RustodoroArgs;
use args::RustodoroCommand;
use clap::Parser;
use config::Config;
use timer::{
    run_timer,
    TimerType,
};

const CONFIG_PATH: &str = "./Config.toml";

fn main() -> Result<(), std::io::Error> {
    let config = Config::load(CONFIG_PATH);

    // TODO: What errors do we want these functions all to throw? Do we want them all to be propagatable updwards (if that's even a phrase)?
    let args: RustodoroArgs = RustodoroArgs::parse();
    match args.command {
        RustodoroCommand::Work => run_timer(config, TimerType::Work)?,
        RustodoroCommand::ShortBreak => run_timer(config, TimerType::ShortBreak)?,
        RustodoroCommand::LongBreak => run_timer(config, TimerType::LongBreak)?,
        RustodoroCommand::SetWorkTimer(command) => {
            let new_config = config.set_work_timer(command);
            Config::save(&new_config, CONFIG_PATH);
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