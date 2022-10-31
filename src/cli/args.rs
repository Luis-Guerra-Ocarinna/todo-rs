use clap::{Parser, Subcommand};

/// Simple TODO program to no more procrastinate :)
#[derive(Parser, Debug)]
#[command(name = "TODO :)")]
#[command(author = "Luis Guerra <luisguerra2004@gmail.com>")]
#[command(version)]
#[command(about, long_about = None)]
pub struct Commands {
    #[command(subcommand)]
    pub crud: Crud,
}

#[derive(Subcommand, Debug)]
pub enum Crud {
    #[command(name = "add")]
    Create(AddTaskCmd),

    #[command(name = "list")]
    Read,
}

#[derive(Parser, Debug)]
pub struct AddTaskCmd {
    pub title: String,

    #[arg(short, long)]
    pub description: Option<String>,
}
