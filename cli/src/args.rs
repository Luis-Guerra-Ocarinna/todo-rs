use crate::Commands;
use clap::Parser;

/// Simple TODO program for no longer procrastinating :)
#[derive(Parser, Debug)]
#[command(version, about)]
#[command(arg_required_else_help(true))]
pub struct Args {
    #[command(subcommand)]
    pub cli: Option<Commands>,

    #[arg(short, long)]
    pub repl: bool,
}
