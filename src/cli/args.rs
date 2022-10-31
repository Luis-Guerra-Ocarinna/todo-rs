use crate::Id;
use clap::{Parser, Subcommand};

/// Simple TODO program to no more procrastinate :)
#[derive(Parser, Debug)]
#[command(name = "TODO :)")]
#[command(author = "Luis Guerra <luisguerra2004@gmail.com>")]
#[command(version)]
#[command(about, long_about = None)]
pub struct Commands {
    #[command(subcommand)]
    pub crud: Option<Crud>,

    #[arg(short, long)]
    pub clear: bool,
}

#[derive(Subcommand, Debug)]
pub enum Crud {
    #[command(name = "add")]
    Create(AddTaskCmd),

    #[command(name = "list")]
    Read,

    #[command(name = "update")]
    Update(UpdateTaskCmd),

    #[command(name = "delete", alias = "del")]
    Delete(DeleteTaskCmd),
}

#[derive(Parser, Debug)]
pub struct AddTaskCmd {
    pub title: String,

    #[arg(short, long)]
    pub description: Option<String>,
}

#[derive(Parser, Debug)]
pub struct UpdateTaskCmd {
    pub id: Id,

    pub title: String,

    #[arg(short, long)]
    pub description: Option<String>,
}

#[derive(Parser, Debug)]
pub struct DeleteTaskCmd {
    pub id: Id,
}
