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

    /// Configure the work timer
    SetWorkTimer(SetWorkTimerCommand)
}

#[derive(Debug, Args)]
pub struct SetWorkTimerCommand {
    /// Minutes component of the work timer
    pub minutes: i32,
    
    /// Seconds component of the work timer
    pub seconds: i32, 
}