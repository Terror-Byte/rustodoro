use chrono::{Datelike, Local, NaiveDate, NaiveTime, TimeZone, Weekday};
use directories::ProjectDirs;
use rusqlite::Connection;
use std::time::SystemTime;

use crate::args::DisplayCommand;
use crate::error::{Error, Result};
use crate::timer::TimerType;

const DATABASE_NAME: &str = "rustodoro.db";
const RELATIVE_DB_PATH: &str = "./rustodoro.db";

// Table creation queries
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

// Table insertion queries
const INSERT_POMODORO_QUERY: &str =
    "INSERT INTO pomodoros (start_time, completion_time) VALUES (?1, ?2)";
const INSERT_SHORT_BREAK_QUERY: &str =
    "INSERT INTO short_breaks (start_time, completion_time) VALUES (?1, ?2)";
const INSERT_LONG_BREAK_QUERY: &str =
    "INSERT INTO long_breaks (start_time, completion_time) VALUES (?1, ?2)";

const POMODORO_TABLE_NAME: &str = "pomodoros";
const SHORT_BREAK_TABLE_NAME: &str = "short_breaks";
const LONG_BREAK_TABLE_NAME: &str = "long_breaks";

pub type SessionVector = Vec<(u64, u64)>;

fn get_database_path() -> String {
    if !cfg!(debug_assertions) {
        if let Some(proj_dirs) = ProjectDirs::from("com", "TerrorByte", "Rustodoro") {
            if let Some(directory) = proj_dirs.data_dir().to_str() {
                let mut dbpath = String::from(directory);
                dbpath.push_str("/");
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

    Ok(())
}

// TODO: Do we want to move the DisplayCommand type elsewhere and rename it?
pub fn get_sessions(
    session_type: TimerType,
    timespan_opt: &Option<DisplayCommand>,
) -> Result<SessionVector> {
    match timespan_opt {
        Some(timespan) => match timespan {
            DisplayCommand::Day => get_todays_sessions(session_type),
            DisplayCommand::Week => get_weeks_sessions(session_type),
            DisplayCommand::Month => get_months_sessions(session_type),
        },
        None => get_todays_sessions(session_type),
    }
}

fn get_sessions_internal(
    session_type: TimerType,
    start_timestamp: i64,
    end_timestamp: i64,
) -> Result<SessionVector> {
    let conn = Connection::open(get_database_path())?;

    let table_name = match session_type {
        TimerType::Work => POMODORO_TABLE_NAME,
        TimerType::ShortBreak => SHORT_BREAK_TABLE_NAME,
        TimerType::LongBreak => LONG_BREAK_TABLE_NAME,
    };
    let query = format!(
        "SELECT start_time, completion_time FROM {} WHERE start_time BETWEEN {} AND {}",
        table_name, start_timestamp, end_timestamp
    );

    let mut statement = conn.prepare(query.as_str())?;

    let session_iter = statement.query_map([], |row| {
        let start_time: u64 = row.get(0)?;
        let completion_time: u64 = row.get(1)?;
        Ok((start_time, completion_time))
    })?;

    let mut session_vector: SessionVector = Vec::new();

    for session in session_iter {
        if let Ok((start_time, end_time)) = session {
            session_vector.push((start_time, end_time))
        }
    }

    Ok(session_vector)
}

fn get_todays_sessions(session_type: TimerType) -> Result<SessionVector> {
    let start_of_day = get_start_of_day_timestamp()?;
    let end_of_day = get_end_of_day_timestamp()?;
    let session_vector = get_sessions_internal(session_type, start_of_day, end_of_day)?;

    Ok(session_vector)
}

fn get_weeks_sessions(session_type: TimerType) -> Result<SessionVector> {
    let start_of_week = get_start_of_week_timestamp()?;
    let end_of_week = get_end_of_week_timestamp()?;
    let session_vector = get_sessions_internal(session_type, start_of_week, end_of_week)?;

    Ok(session_vector)
}

fn get_months_sessions(session_type: TimerType) -> Result<SessionVector> {
    let start_of_month = get_start_of_month_timestamp()?;
    let end_of_month = get_end_of_month_timestamp()?;
    let session_vector = get_sessions_internal(session_type, start_of_month, end_of_month)?;

    Ok(session_vector)
}

// Timing Stuff
fn get_start_of_day_timestamp() -> Result<i64> {
    let start_of_day_timestamp = NaiveTime::from_hms_opt(0, 0, 0).ok_or(Error::NaiveTimeError(
        String::from("[db::get_start_of_day_timestamp()] Failed to create NaiveTime timestamp"),
    ))?;
    let local = Local::now()
        .with_time(start_of_day_timestamp)
        .single()
        .ok_or(Error::DateTimeError(String::from(
            "[db::get_start_of_day_timestamp()] Failed to parse start_of_day_timestamp as DateTime<Local>",
        )))?;
    Ok(local.timestamp())
}

fn get_end_of_day_timestamp() -> Result<i64> {
    // TODO: Replace unwrap with error propagation!
    let end_of_day_timestamp = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
    let local = Local::now().with_time(end_of_day_timestamp).unwrap();
    Ok(local.timestamp())
}

fn get_start_of_week_timestamp() -> Result<i64> {
    // TODO: Replace unwrap with error propagation!

    // Get midnight
    let start_of_day_timestamp = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

    // Get current datetime
    let now = Local::now();

    // Get the start of the week
    let current_year = now.year();
    let current_week = now.iso_week().week();
    let start_of_week = NaiveDate::from_isoywd_opt(current_year, current_week, Weekday::Mon)
        .unwrap()
        .and_time(start_of_day_timestamp);

    let result = Local.from_local_datetime(&start_of_week).unwrap();

    Ok(result.timestamp())
}

fn get_end_of_week_timestamp() -> Result<i64> {
    // TODO: Replace unwrap with error propagation!

    // Get midnight
    let end_of_day_timestamp = NaiveTime::from_hms_opt(23, 59, 59).unwrap();

    // Get current datetime
    let now = Local::now();

    // Get the end of the week
    let current_year = now.year();
    let current_week = now.iso_week().week();
    let end_of_week = NaiveDate::from_isoywd_opt(current_year, current_week, Weekday::Sun)
        .unwrap()
        .and_time(end_of_day_timestamp);

    let result = Local.from_local_datetime(&end_of_week).unwrap();

    Ok(result.timestamp())
}

fn get_start_of_month_timestamp() -> Result<i64> {
    // TODO: Replace unwrap with error propagation!

    // Get midnight
    let start_of_day_timestamp = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

    // Get current datetime
    let now = Local::now();

    // Get the start of the month
    let current_year = now.year();
    let current_month = now.month();
    let start_of_month = NaiveDate::from_ymd_opt(current_year, current_month, 1)
        .unwrap()
        .and_time(start_of_day_timestamp);

    let result = Local.from_local_datetime(&start_of_month).unwrap();

    Ok(result.timestamp())
}

fn get_end_of_month_timestamp() -> Result<i64> {
    // TODO: Replace unwrap with error propagation!

    // Get midnight
    let end_of_day_timestamp = NaiveTime::from_hms_opt(23, 59, 59).unwrap();

    // Get current datetime
    let now = Local::now();

    // Get the end of the month
    let current_year = now.year();
    let current_month = now.month();

    // Wonky way to get the last day of the month (can this be improved?)
    let end_of_month = if current_month < 12 {
        NaiveDate::from_ymd_opt(current_year, current_month + 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
            .and_time(end_of_day_timestamp)
    } else {
        NaiveDate::from_ymd_opt(current_year + 1, 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
            .and_time(end_of_day_timestamp)
    };

    let result = Local.from_local_datetime(&end_of_month).unwrap();

    Ok(result.timestamp())
}

pub fn get_most_recent_session(session_type: TimerType) -> Result<Option<(u64, u64)>> {
    let conn = Connection::open(get_database_path())?;

    let start_timestamp = get_start_of_day_timestamp()?;
    let end_timestamp = get_end_of_day_timestamp()?;

    let table_name = match session_type {
        TimerType::Work => POMODORO_TABLE_NAME,
        TimerType::ShortBreak => SHORT_BREAK_TABLE_NAME,
        TimerType::LongBreak => LONG_BREAK_TABLE_NAME,
    };
    let query = format!(
        "SELECT start_time, completion_time FROM {} WHERE start_time BETWEEN {} AND {} ORDER BY start_time DESC LIMIT 1",
        table_name, start_timestamp, end_timestamp
    );

    let mut statement = conn.prepare(query.as_str())?;

    let session_iter = statement.query_map([], |row| {
        let start_time: u64 = row.get(0)?;
        let completion_time: u64 = row.get(1)?;
        Ok((start_time, completion_time))
    })?;

    let mut session_vector: SessionVector = Vec::new();

    for session in session_iter {
        if let Ok((start_time, end_time)) = session {
            session_vector.push((start_time, end_time))
        }
    }

    // TODO: Is there a more elegant way to get just one result? Do we need the result vector?
    if session_vector.len() > 0 {
        let result = session_vector[0];
        return Ok(Some(result));
    } else {
        return Ok(None);
    }
}

pub fn get_sessions_since(session_type: TimerType, start_timestamp: i64) -> Result<SessionVector> {
    let end_timestamp = get_current_unix_time()? as i64;
    let session_vector = get_sessions_internal(session_type, start_timestamp, end_timestamp)?;
    Ok(session_vector)
}

// TODO: This is a duplicate of the function in timer.rs, do we want to make that one public or
// move it to a utility module?
fn get_current_unix_time() -> Result<u64> {
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs())
}
