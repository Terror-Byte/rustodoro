use chrono::{Days, Local, NaiveTime};
use directories::ProjectDirs;
use rusqlite::{Connection, Result};

use crate::timer::TimerType;

const DATABASE_NAME: &str = "rustodoro.db";
const RELATIVE_DB_PATH: &str = "./rustodoro.db";

// Queries
const CREATE_POMODORO_TABLE_QUERY: &str = "CREATE TABLE IF NOT EXISTS pomodoros (
            start_time INTEGER PRIMARY KEY,
            completion_time INTEGER NOT NULL
    )";
const CREATE_SHORT_BREAK_TABLE_QUERY: &str = "CREATE TABLE IF NOT EXISTS short_breaks (
            start_time INTEGER PRIMARY KEY,
            completion_time INTEGER NOT NULL
    )";
const CREATE_LONG_BREAK_TABLE_QUERY: &str = "CREATE TABLE IF NOT EXISTS long_breaks (
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

fn get_database_path() -> String {
    if !cfg!(debug_assertions) {
        if let Some(proj_dirs) = ProjectDirs::from("com", "TerrorByte", "Rustodoro") {
            if let Some(directory) = proj_dirs.data_dir().to_str() {
                let mut dbpath = String::from(directory);
                dbpath.push_str(DATABASE_NAME);
                return dbpath;
            }
        }
    }
    String::from(RELATIVE_DB_PATH)
}

pub fn save_session_to_db(start_time: u64, end_time: u64, session_type: TimerType) -> Result<()> {
    let (create_table_query, insert_query) = match session_type {
        TimerType::Work => (CREATE_POMODORO_TABLE_QUERY, INSERT_POMODORO_QUERY),
        TimerType::ShortBreak => (CREATE_SHORT_BREAK_TABLE_QUERY, INSERT_SHORT_BREAK_QUERY),
        TimerType::LongBreak => (CREATE_LONG_BREAK_TABLE_QUERY, INSERT_LONG_BREAK_QUERY),
    };

    let conn = Connection::open(get_database_path())?;

    // Create table if it doesn't exist
    conn.execute(create_table_query, ())?;

    // Insert new value into table
    conn.execute(insert_query, (start_time, end_time))?;

    if cfg!(debug_assertions) {
        debug_print_records(session_type);
    }

    Ok(())
}

pub fn get_todays_sessions(session_type: TimerType) -> Result<Vec<(u64, u64)>> {
    let conn = Connection::open(get_database_path())?;
    let midnight_today = get_todays_date_midnight()?;
    let midnight_tomorrow = get_tomorrows_date_midnight()?;
    let table_name = match session_type {
        TimerType::Work => POMODORO_TABLE_NAME,
        TimerType::ShortBreak => SHORT_BREAK_TABLE_NAME,
        TimerType::LongBreak => LONG_BREAK_TABLE_NAME,
    };
    let query = format!(
        "SELECT start_time, completion_time FROM {} WHERE start_time BETWEEN {} AND {}",
        table_name, midnight_today, midnight_tomorrow
    );

    // TODO: Replace expect() with ?
    let mut statement = conn
        .prepare(query.as_str())
        .expect(format!("Failed to prepare query for table {}", table_name).as_str());

    // TODO: Replace expect() with ?
    let session_iter = statement
        .query_map([], |row| {
            let start_time: u64 = row.get(0).expect("foo");
            let completion_time: u64 = row.get(1).expect("bar");
            Ok((start_time, completion_time))
        })
        .expect("Failed to parse statement");

    let mut session_vector: Vec<(u64, u64)> = Vec::new();

    for session in session_iter {
        if let Ok((start_time, end_time)) = session {
            session_vector.push((start_time, end_time))
        }
    }

    // TODO: Return this iterator?
    // TODO: Do we parse the timestamps in human-readable time?

    Ok(session_vector)
}

// Timing Stuff
fn get_todays_date_midnight() -> Result<i64> {
    // TODO: Replace unwrap() with ?
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let local = Local::now().with_time(midnight).unwrap();
    Ok(local.timestamp())
}

fn get_tomorrows_date_midnight() -> Result<i64> {
    // TODO: Replace unwrap() with ?
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let local = Local::now().with_time(midnight).unwrap();
    let local = local.checked_add_days(Days::new(1)).unwrap();
    Ok(local.timestamp())
}

// Debug Print Functions (move these to their own file?)
#[cfg(debug_assertions)]
pub fn debug_print_records_from_today(table: &str) {
    let midnight_today = get_todays_date_midnight().unwrap();
    let midnight_tomorrow = get_tomorrows_date_midnight().unwrap();
    execute_statement(
        format!(
            "SELECT start_time, completion_time FROM {} WHERE start_time BETWEEN {} AND {}",
            table, midnight_today, midnight_tomorrow
        )
        .as_str(),
    );
}

#[cfg(debug_assertions)]
pub fn debug_count_records_from_today(table: &str) -> u64 {
    let mut res = 0;
    let conn = Connection::open(RELATIVE_DB_PATH).unwrap();

    let midnight_today = get_todays_date_midnight().unwrap();
    let midnight_tomorrow = get_tomorrows_date_midnight().unwrap();

    let mut statement = conn
        .prepare(
            format!(
                "SELECT start_time, completion_time FROM {} WHERE start_time BETWEEN {} AND {}",
                table, midnight_today, midnight_tomorrow
            )
            .as_str(),
        )
        .expect("Failed to prepare query for pomodoro table"); // TODO: USER FRIENDLY ERROR HERE

    let pomodoro_iter = statement
        .query_map([], |row| {
            let start_time: u64 = row.get(0).expect("foo");
            let completion_time: u64 = row.get(1).expect("bar");
            Ok((start_time, completion_time))
        })
        .expect("Failed to parse statement.");

    for pomodoro in pomodoro_iter {
        if let Ok((_, _)) = pomodoro {
            res += 1;
        }
    }

    res
}

#[cfg(debug_assertions)]
fn execute_statement(query: &str) {
    let conn = Connection::open(RELATIVE_DB_PATH).expect("Failed to open rustodoro.db");

    let mut statement = conn
        .prepare(query)
        .expect("Failed to prepare query for table"); // TODO: USER FRIENDLY ERROR HERE

    let pomodoro_iter = statement
        .query_map([], |row| {
            let start_time: u64 = row.get(0).expect("foo");
            let completion_time: u64 = row.get(1).expect("bar");
            Ok((start_time, completion_time))
        })
        .expect("Failed to parse statement.");

    for pomodoro in pomodoro_iter {
        if let Ok((start_time, completion_time)) = pomodoro {
            println!("Start Time: {}, End Time: {}", start_time, completion_time);
        }
    }
}

#[cfg(debug_assertions)]
fn debug_print_records(timer_type: TimerType) {
    let (table_name, debug_name) = match timer_type {
        TimerType::Work => (POMODORO_TABLE_NAME, "Pomodoros"),
        TimerType::ShortBreak => (SHORT_BREAK_TABLE_NAME, "Short breaks"),
        TimerType::LongBreak => (LONG_BREAK_TABLE_NAME, "Long breaks"),
    };

    debug_print_records_from_today(table_name);
    println!(
        "{} today: {}",
        debug_name,
        debug_count_records_from_today(table_name)
    );
}
