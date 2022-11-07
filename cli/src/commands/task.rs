use clap::{Parser, Subcommand};
use todo_rs_core::Id;

#[derive(Parser, Debug)]
pub struct TaskSubCommand {
    pub id: Id,

    #[command(subcommand)]
    pub task: TaskCommands,
}

#[derive(Subcommand, Debug)]
pub enum TaskCommands {
    #[command(name = "name")]
    PatchName {
        name: String,
    },

    #[command(name = "description", alias = "desc")]
    PatchDescription {
        description: Option<String>,
    },

    Timer(TimerSubCommand),

    Done,
}

#[derive(Parser, Debug)]
pub struct TimerSubCommand {
    #[command(subcommand)]
    pub timer: TimerCommands,
}

#[derive(Subcommand, Debug)]
pub enum TimerCommands {
    Start,
    Pause,
    Resume,
    Stop,
}
