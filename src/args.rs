use clap::{
    Args,
    Parser,
    Subcommand, 
};

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
    SetWorkTime(SetWorkTimeCommand),

    /// Configure the short break timer
    SetShortBreakTime(SetShortBreakTimeCommand),

    /// Configure the long break timer
    SetLongBreakTime(SetLongBreakTimeCommand),

    /// Configure the amount of pomodoros (work stints) to complete for a long break
    SetPomodorosToLongBreak(SetPomodorosToLongBreakCommand),
}

#[derive(Debug, Args)]
pub struct SetWorkTimeCommand {
    /// Minutes component of the work timer
    #[arg(short, long)]
    pub minutes: Option<u16>,
    
    /// Seconds component of the work timer
    #[arg(short, long)]
    pub seconds: Option<u8>, 
}

#[derive(Debug, Args)]
pub struct SetShortBreakTimeCommand {
    /// Minutes component of the short break timer
    #[arg(short, long)]
    pub minutes: Option<u16>,
    
    /// Seconds component of the long break timer
    #[arg(short, long)]
    pub seconds: Option<u8>,
}

#[derive(Debug, Args)]
pub struct SetLongBreakTimeCommand {
    /// Minutes component of the long break timer
    #[arg(short, long)]
    pub minutes: Option<u16>,
    
    /// Seconds component of the short break timer
    #[arg(short, long)]
    pub seconds: Option<u8>,
}

#[derive(Debug, Args)]
pub struct SetPomodorosToLongBreakCommand {
    pub pomodoros_to_long_break: u8,
}

impl ToSeconds for SetWorkTimeCommand {
    fn to_seconds(&self) -> u16 {
        let mut time_in_seconds: u16 = 0;

        if let Some(minutes) = self.minutes {
            time_in_seconds += minutes * 60;
        }

        if let Some(seconds) = self.seconds {
            match seconds {
                0..=60 => time_in_seconds += seconds as u16,
                _ => println!("Error!") // TODO: What do we do in this case? Should this return a Result? Or set to a default value and spit out an error for the user?
            }
        }

        time_in_seconds
    }
}

impl ToSeconds for SetShortBreakTimeCommand {
    fn to_seconds(&self) -> u16 {
        let mut time_in_seconds: u16 = 0;

        if let Some(minutes) = self.minutes {
            time_in_seconds += minutes * 60;
        }

        if let Some(seconds) = self.seconds {
            match seconds {
                0..=60 => time_in_seconds += seconds as u16,
                _ => println!("Error!") // TODO: What do we do in this case? Should this return a Result? Or set to a default value and spit out an error for the user?
            }
        }

        time_in_seconds
    }
}

impl ToSeconds for SetLongBreakTimeCommand {
    fn to_seconds(&self) -> u16 {
        let mut time_in_seconds: u16 = 0;

        if let Some(minutes) = self.minutes {
            time_in_seconds += minutes * 60;
        }

        if let Some(seconds) = self.seconds {
            match seconds {
                0..=60 => time_in_seconds += seconds as u16,
                _ => println!("Error!") // TODO: What do we do in this case? Should this return a Result? Or set to a default value and spit out an error for the user?
            }
        }

        time_in_seconds
    }
}