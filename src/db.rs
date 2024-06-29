use rusqlite::{
    Connection,
    Result,
};
use chrono::{
    Days,
    Local,
    NaiveTime,
};
use std::time::SystemTime;

const DB_PATH: &str = "rustodoro.db";

const CREATE_POMODORO_TABLE_QUERY: &str = 
    "CREATE TABLE IF NOT EXISTS pomodoros (
            start_time INTEGER PRIMARY KEY,
            completion_time INTEGER NOT NULL
    )";
const CREATE_SHORT_BREAK_TABLE_QUERY: &str = 
    "CREATE TABLE IF NOT EXISTS short_breaks (
            start_time INTEGER PRIMARY KEY,
            completion_time INTEGER NOT NULL
    )";
const CREATE_LONG_BREAK_TABLE_QUERY: &str = 
    "CREATE TABLE IF NOT EXISTS long_breaks (
            start_time INTEGER PRIMARY KEY,
            completion_time INTEGER NOT NULL
    )";

const INSERT_POMODORO_QUERY: &str = 
    "INSERT INTO pomodoros (start_time, completion_time) VALUES (?1, ?2)";
const INSERT_SHORT_BREAK_QUERY: &str = 
    "INSERT INTO short_breaks (start_time, completion_time) VALUES (?1, ?2)";
const INSERT_LONG_BREAK_QUERY: &str = 
    "INSERT INTO long_breaks (start_time, completion_time) VALUES (?1, ?2)";

pub const POMODORO_TABLE_NAME: &str = "pomodoros";
pub const SHORT_BREAK_TABLE_NAME: &str = "short_breaks";
pub const LONG_BREAK_TABLE_NAME: &str = "long_breaks";

pub fn save_pomodoro_to_db(start_time: u64, completion_time: u64) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    // Create table if it doesn't exist
    conn.execute(CREATE_POMODORO_TABLE_QUERY, ())?;
    
    // Insert new value into table.
    conn.execute(INSERT_POMODORO_QUERY, (start_time, completion_time))?;

    Ok(())
}

pub fn save_short_break_to_db(start_time: u64, completion_time: u64) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    // Create table if it doesn't exist
    conn.execute(CREATE_SHORT_BREAK_TABLE_QUERY, ())?;
    
    // Insert new value into table.
    conn.execute(INSERT_SHORT_BREAK_QUERY, (start_time, completion_time))?;

    Ok(())
}

pub fn save_long_break_to_db(start_time: u64, completion_time: u64) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    // Create table if it doesn't exist
    conn.execute(CREATE_LONG_BREAK_TABLE_QUERY, ())?;
    
    // Insert new value into table.
    conn.execute(INSERT_LONG_BREAK_QUERY, (start_time, completion_time))?;

    Ok(())
}

fn execute_statement(query: &str) {
    let conn = Connection::open(DB_PATH).expect("Failed to open rustodoro.db");

    let mut statement = conn.prepare(query)
        .expect("Failed to prepare query for table"); // TODO: USER FRIENDLY ERROR HERE

    let pomodoro_iter = statement.query_map([], |row| {
        let start_time: u64 = row.get(0).expect("foo");
        let completion_time: u64 = row.get(1).expect("bar");
        Ok((start_time, completion_time))
    }).expect("Failed to parse statement.");

    for pomodoro in pomodoro_iter {
        if let Ok((start_time, completion_time)) = pomodoro {
            println!("Start Time: {}, End Time: {}", start_time, completion_time);
        }
    }
}

// Timing Stuff
// TODO: Is there some better error handling we could do?
pub fn get_current_unix_time() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Failed to attain current unix epoch.").as_secs()
}

fn get_todays_date_midnight() -> i64 {
    // TODO: Handle the error from unwrapping this
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let local = Local::now().with_time(midnight).unwrap();
    local.timestamp()
}

fn get_tomorrows_date_midnight() -> i64 {
    // TODO: Handle the error from unwrapping this
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let local = Local::now().with_time(midnight).unwrap();
    let local = local.checked_add_days(Days::new(1)).unwrap();
    local.timestamp()
}

// Debug Print Functions
pub fn print_all_records(table: &str) {
    execute_statement(format!("SELECT start_time, completion_time FROM {}", table).as_str());
}

pub fn print_records_from_today(table: &str) {
    let midnight_today = get_todays_date_midnight();
    let midnight_tomorrow = get_tomorrows_date_midnight();
    execute_statement(format!("SELECT start_time, completion_time FROM {} WHERE start_time BETWEEN {} AND {}", table, midnight_today, midnight_tomorrow).as_str());
}

pub fn count_records_from_today(table: &str) -> u64 {
    let mut res = 0;
    let conn = Connection::open(DB_PATH).expect("Failed to open rustodoro.db");

    let midnight_today = get_todays_date_midnight();
    let midnight_tomorrow = get_tomorrows_date_midnight();

    let mut statement = conn.prepare(format!("SELECT start_time, completion_time FROM {} WHERE start_time BETWEEN {} AND {}", table, midnight_today, midnight_tomorrow).as_str())
        .expect("Failed to prepare query for pomodoro table"); // TODO: USER FRIENDLY ERROR HERE

    let pomodoro_iter = statement.query_map([], |row| {
        let start_time: u64 = row.get(0).expect("foo");
        let completion_time: u64 = row.get(1).expect("bar");
        Ok((start_time, completion_time))
    }).expect("Failed to parse statement.");

    for pomodoro in pomodoro_iter {
        if let Ok((_, _)) = pomodoro {
            res += 1;
        }
    }

    res
}