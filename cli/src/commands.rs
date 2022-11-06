pub mod crud;

use clap::{command, Parser, Subcommand};

/// Simple TODO program for no longer procrastinating :)
#[derive(Parser, Debug)]
#[command(version, about)]
#[command(arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub cli: Option<Commands>,

    #[arg(short, long)]
    pub repl: bool,
}

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
}
