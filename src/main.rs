mod args;
mod config;
mod timer;

use args::{
    RustodoroArgs,
    RustodoroCommand,
};
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
    // TODO: For the commands where we're modifying the config, what sort of user feedback do we want to let the user know the command executed successfully?
    let args: RustodoroArgs = RustodoroArgs::parse();
    match args.command {
        RustodoroCommand::Work => run_timer(config.work_time, TimerType::Work)?,
        RustodoroCommand::ShortBreak => run_timer(config.short_break_time, TimerType::ShortBreak)?,
        RustodoroCommand::LongBreak => run_timer(config.long_break_time, TimerType::LongBreak)?,
        RustodoroCommand::SetWorkTimer(command) => {
            let new_config = config.set_work_timer(command);
            Config::save(&new_config, CONFIG_PATH);
        },
        RustodoroCommand::SetShortBreakTimer(command) => {
            let new_config = config.set_short_break_timer(command);
            Config::save(&new_config, CONFIG_PATH);
        },
        RustodoroCommand::SetLongBreakTimer(command) => {
            let new_config = config.set_long_break_timer(command);
            Config::save(&new_config, CONFIG_PATH);
        },
        RustodoroCommand::SetPomodorosToLongBreak(command) => {
            let new_config = config.set_pomodoros_to_long_break(command);
            Config::save(&new_config, CONFIG_PATH);
        },
    }

    Ok(())
}