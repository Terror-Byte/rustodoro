use clap::{ArgAction, Args, Parser, Subcommand};

pub trait ToSeconds {
    fn to_seconds(&self) -> u16;
}

pub trait HasValue {
    fn has_value(&self) -> bool;
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct RustodoroArgs {
    #[clap(subcommand)]
    pub command: RustodoroCommand,
}

#[derive(Debug, Subcommand)]
pub enum RustodoroCommand {
    /// Start a work timer - If no arguments are passed, the work_time property from
    /// the config will be used
    Work(RunTimerArgs),

    /// Start a short break timer - If no arguments are passed, the short_break_time
    /// property from the config will be used
    ShortBreak(RunTimerArgs),

    /// Start a long break timer - If no arguments are passed, the long_break_time
    /// property from the config will be used
    LongBreak(RunTimerArgs),

    /// Configure the work timer
    SetWorkTime(ConfigureTimerArgs),

    /// Configure the short break timer
    SetShortBreakTime(ConfigureTimerArgs),

    /// Configure the long break timer
    SetLongBreakTime(ConfigureTimerArgs),

    /// Configure the amount of pomodoros (work stints) to complete for a long break
    SetPomodorosToLongBreak(SetPomodorosToLongBreakArgs),

    /// Configure whether to log all pomodoros, short breaks and long breaks to a local SQLite database
    SetLogToDB(SetLogToDBArgs),

    /// Configure whether to trigger desktop notifications when a session is completed
    SetDesktopNotifications(SetDesktopNotificationsArgs),

    /// Display the pomodoros from today, this week or this month
    DisplayPomodoros(DisplayPomodorosArgs),

    /// Display the short breaks from today, this week or this month
    DisplayShortBreaks(DisplayShortBreaksArgs),

    /// Display the long breaks from today, this week or this month
    DisplayLongBreaks(DisplayLongBreaksArgs),
}

#[derive(Debug, Args)]
pub struct RunTimerArgs {
    /// Minutes component of the timer
    #[arg(short, long)]
    pub minutes: Option<u16>,

    /// Seconds component of the timer
    #[arg(short, long)]
    pub seconds: Option<u16>,

    #[arg(short, long)]
    pub comment: Option<String>,
}

#[derive(Debug, Args)]
pub struct ConfigureTimerArgs {
    /// Minutes component of the timer
    #[arg(short, long)]
    pub minutes: Option<u16>,

    /// Seconds component of the timer
    #[arg(short, long)]
    pub seconds: Option<u16>,
}

#[derive(Debug, Args)]
pub struct SetPomodorosToLongBreakArgs {
    pub pomodoros_to_long_break: u8,
}

#[derive(Debug, Args)]
pub struct SetLogToDBArgs {
    #[arg(action = ArgAction::Set)]
    pub log_to_db: bool,
}

#[derive(Debug, Subcommand)]
pub enum TimeSpan {
    Day,
    Week,
    Month,
}

#[derive(Debug, Args)]
pub struct DisplayPomodorosArgs {
    #[command(subcommand)]
    pub subcommand: Option<TimeSpan>,
}

#[derive(Debug, Args)]
pub struct DisplayShortBreaksArgs {
    #[command(subcommand)]
    pub subcommand: Option<TimeSpan>,
}

#[derive(Debug, Args)]
pub struct DisplayLongBreaksArgs {
    #[command(subcommand)]
    pub subcommand: Option<TimeSpan>,
}

#[derive(Debug, Args)]
pub struct SetDesktopNotificationsArgs {
    #[arg(action = ArgAction::Set)]
    pub desktop_notifications: bool,
}

impl ToSeconds for RunTimerArgs {
    fn to_seconds(&self) -> u16 {
        let mut time_in_seconds: u16 = 0;

        if let Some(minutes) = self.minutes {
            time_in_seconds += minutes * 60;
        }

        if let Some(seconds) = self.seconds {
            time_in_seconds += seconds;
        }

        time_in_seconds
    }
}

impl ToSeconds for ConfigureTimerArgs {
    fn to_seconds(&self) -> u16 {
        let mut time_in_seconds: u16 = 0;

        if let Some(minutes) = self.minutes {
            time_in_seconds += minutes * 60;
        }

        if let Some(seconds) = self.seconds {
            time_in_seconds += seconds;
        }

        time_in_seconds
    }
}

impl HasValue for RunTimerArgs {
    fn has_value(&self) -> bool {
        self.minutes.is_some() || self.seconds.is_some()
    }
}

impl HasValue for ConfigureTimerArgs {
    fn has_value(&self) -> bool {
        self.minutes.is_some() || self.seconds.is_some()
    }
}
