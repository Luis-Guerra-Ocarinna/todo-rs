pub mod crud;
pub mod task;

use clap::{command, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(name = "add")]
    Create(crud::AddTaskCmd),

    #[command(name = "list")]
    Read,

    #[command(name = "info")]
    Get(crud::GetTaskCmd),

    #[command(name = "update")]
    Update(crud::UpdateTaskCmd),

    #[command(name = "delete", alias = "del")]
    Delete(crud::DeleteTaskCmd),

    Clear,

    Task(task::TaskSubCommand),
}
