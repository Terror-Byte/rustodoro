use clap::{ArgAction, Args, Parser, Subcommand};

pub trait ToSeconds {
    fn to_seconds(&self) -> u16; // TODO: Make this generic instead of a solid u16 type?
}

#[derive(Debug, Parser)]
pub struct RustodoroArgs {
    #[clap(subcommand)]
    pub command: RustodoroCommand,
}

#[derive(Debug, Subcommand)]
pub enum RustodoroCommand {
    /// Start a work timer
    Work,

    /// Start a short break timer
    ShortBreak,

    /// Start a long break timer
    LongBreak,

    /// Configure the work timer
    SetWorkTime(SetWorkTimeArgs),

    /// Configure the short break timer
    SetShortBreakTime(SetShortBreakTimeArgs),

    /// Configure the long break timer
    SetLongBreakTime(SetLongBreakTimeArgs),

    /// Configure the amount of pomodoros (work stints) to complete for a long break
    SetPomodorosToLongBreak(SetPomodorosToLongBreakArgs),

    /// Configure whether to log all pomodoros, short breaks and long breaks to a local SQLite database
    SetLogToDB(SetLogToDBArgs),

    /// Display the pomodoros from today, this week or this month
    DisplayPomodoros(DisplayPomodorosArgs),

    /// Display the short breaks from today, this week or this month
    DisplayShortBreaks(DisplayShortBreaksArgs),

    /// Display the long breaks from today, this week or this month
    DisplayLongBreaks(DisplayLongBreaksArgs),
}

#[derive(Debug, Args)]
pub struct SetWorkTimeArgs {
    /// Minutes component of the work timer
    #[arg(short, long)]
    pub minutes: Option<u16>,

    /// Seconds component of the work timer
    #[arg(short, long)]
    pub seconds: Option<u16>,
}

#[derive(Debug, Args)]
pub struct SetShortBreakTimeArgs {
    /// Minutes component of the short break timer
    #[arg(short, long)]
    pub minutes: Option<u16>,

    /// Seconds component of the short break timer
    #[arg(short, long)]
    pub seconds: Option<u16>,
}

#[derive(Debug, Args)]
pub struct SetLongBreakTimeArgs {
    /// Minutes component of the long break timer
    #[arg(short, long)]
    pub minutes: Option<u16>,

    /// Seconds component of the long break timer
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

impl ToSeconds for SetWorkTimeArgs {
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

impl ToSeconds for SetShortBreakTimeArgs {
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

impl ToSeconds for SetLongBreakTimeArgs {
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
