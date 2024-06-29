mod args;
mod config;
mod db;
mod timer;
use args::{
    RustodoroArgs,
    RustodoroCommand,
};
use clap::Parser;
use config::Config;
use timer::TimerType;

const CONFIG_PATH: &str = "./Config.toml";

fn main() {
    let config = Config::load(CONFIG_PATH);

    // TODO: What errors do we want these functions all to throw? Do we want them all to be propagatable updwards (if that's even a phrase)?
    // TODO: For the commands where we're modifying the config, what sort of user feedback do we want to let the user know the command executed successfully?
    let args: RustodoroArgs = RustodoroArgs::parse();
    match args.command {
        RustodoroCommand::Work => work_timer(config),
        RustodoroCommand::ShortBreak => short_break(config),
        RustodoroCommand::LongBreak => long_break(config),
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
}

fn work_timer(config: Config) {
    let start_time = db::get_current_unix_time();
    let result = timer::run_timer(config.work_time, TimerType::Work);
    let completion_time = db::get_current_unix_time();
    
    // TODO: Handle the possible database error more elegantly!
    match result {
        Ok(_) => {
            db::save_pomodoro_to_db(start_time, completion_time)
                .expect("Failed to save pomodoro to database.");
            // TODO: If we've hit the pomodoro threshold for long break, notify the user.
            // If todays_pomodoros % pomodoros_to_long_break, increment the long break counter in the DB.

            // Debug Print Stuff
            db::print_records_from_today(db::POMODORO_TABLE_NAME);
            println!("Pomodoros Today: {}", db::count_records_from_today(db::POMODORO_TABLE_NAME));
        },
        Err(e) => println!("{}", e),
    }
}

fn short_break(config: Config) {
    let start_time = db::get_current_unix_time();
    let result = timer::run_timer(config.short_break_time, TimerType::ShortBreak);
    let completion_time = db::get_current_unix_time();

    // TODO: Handle the possible database error more elegantly!
    match result {
        Ok(_) => {
            db::save_short_break_to_db(start_time, completion_time)
                .expect("Failed to save short break to database.");

            // Debug Print Stuff
            db::print_records_from_today(db::SHORT_BREAK_TABLE_NAME);
            println!("Short breaks Today: {}", db::count_records_from_today(db::SHORT_BREAK_TABLE_NAME));
        },
        Err(e) => println!("{}", e),
    }
}

fn long_break(config: Config) {
    let start_time = db::get_current_unix_time();
    let result = timer::run_timer(config.long_break_time, TimerType::LongBreak);
    let completion_time = db::get_current_unix_time();

    // TODO: Handle the possible database error more elegantly!
    match result {
        Ok(_) => {
            db::save_long_break_to_db(start_time, completion_time)
                .expect("Failed to save long break to database.");

            // Debug Print Stuff
            db::print_records_from_today(db::LONG_BREAK_TABLE_NAME);
            println!("Long breaks Today: {}", db::count_records_from_today(db::LONG_BREAK_TABLE_NAME));
        },
        Err(e) => println!("{}", e),
    }
    
    // TODO: Decrement the long break counter in the DB.
}