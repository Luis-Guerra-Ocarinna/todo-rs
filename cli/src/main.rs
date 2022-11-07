use clap::Parser;
use todo_rs::{Args, Run};
use todo_rs_core::{Config, Context};

fn main() {
    let config = Config::load();
    let context = Context::load(config);

    let args = Args::parse();

    if args.repl {
        // TODO: Implement REPL
        todo!("Sorry, REPL is not implemented yet :/");
    } else {
        args.cli.unwrap().run(context);
    }
}
