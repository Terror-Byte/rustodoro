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
use rusqlite::Connection;
use rusqlite::params;

const CONFIG_PATH: &str = "./Config.toml";

fn main() -> Result<(), std::io::Error> {
    let config = Config::load(CONFIG_PATH);

    // TODO: What errors do we want these functions all to throw? Do we want them all to be propagatable updwards (if that's even a phrase)?
    // TODO: For the commands where we're modifying the config, what sort of user feedback do we want to let the user know the command executed successfully?
    let args: RustodoroArgs = RustodoroArgs::parse();
    match args.command {
        RustodoroCommand::Work => {
            run_timer(config.work_time, TimerType::Work)?
        },
        RustodoroCommand::ShortBreak => {
            run_timer(config.short_break_time, TimerType::ShortBreak)?
        },
        RustodoroCommand::LongBreak => {
            run_timer(config.long_break_time, TimerType::LongBreak)?
        }
        RustodoroCommand::SetWorkTime(command) => {
            let new_config = config.set_work_time(command);
            Config::save(&new_config, CONFIG_PATH);
        },
        RustodoroCommand::SetShortBreakTime(command) => {
            let new_config = config.set_short_break_time(command);
            Config::save(&new_config, CONFIG_PATH);
        },
        RustodoroCommand::SetLongBreakTime(command) => {
            let new_config = config.set_long_break_time(command);
            Config::save(&new_config, CONFIG_PATH);
        },
        RustodoroCommand::SetPomodorosToLongBreak(command) => {
            let new_config = config.set_pomodoros_to_long_break(command);
            Config::save(&new_config, CONFIG_PATH);
        },
    }

    // TEST: Connect to rustodoro database and add a table
    let conn = Connection::open("rustodoro.db");
    if let Ok(connection) = conn {
        println!("Connection to table rustodoro.db established successfully");
        let query_result = connection.execute(
        "create table if not exists test_table (
            id integer primary key,
            name text not null unique
        )
        ", ());

        if let Ok(_) = query_result {
            println!("Table created successfully!");
        }
    }

    Ok(())
}