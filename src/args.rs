use clap::{ 
    Args, 
    Parser, 
    Subcommand 
};

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
// use crossterm::terminal::{ 
//     Clear, 
//     ClearType 
// };
// use crossterm::style::{ 
//     Color, 
//     Stylize 
// };
    /// Configure the work timer
    SetWorkTimer(SetWorkTimerCommand),

    /// Configure the short break timer
    SetShortBreakTimer(SetShortBreakTimerCommand),

    /// Configure the long break timer
    SetLongBreakTimer(SetLongBreakTimerCommand),

    /// Configure the amount of pomodoros (work stints) to complete for a long break
    SetPomodorosToLongBreak(SetPomodorosToLongBreakCommand),
}

#[derive(Debug, Args)]
pub struct SetWorkTimerCommand {
    /// Minutes component of the work timer
    #[arg(short, long)]
    pub minutes: Option<u16>,
    
    /// Seconds component of the work timer
    #[arg(short, long)]
    pub seconds: Option<u8>, 
}

#[derive(Debug, Args)]
pub struct SetShortBreakTimerCommand {
    /// Minutes component of the short break timer
    #[arg(short, long)]
    pub minutes: Option<u16>,
    
    /// Seconds component of the long break timer
    #[arg(short, long)]
    pub seconds: Option<u8>,
}

#[derive(Debug, Args)]
pub struct SetLongBreakTimerCommand {
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