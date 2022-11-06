use clap::Parser;
use todo_rs::{Cli, Run};
use todo_rs_core::{Config, Context};

fn main() {
    let config = Config::load();
    let context = Context::load(config);

    let cmd = Cli::parse();

    if cmd.repl {
        println!("REPL");
    } else {
        cmd.cli.unwrap().run(context);
    }
}
