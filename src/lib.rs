pub mod cli;
pub mod model;

pub use cli::Cli;

type Id = u32;

pub trait Run {
    fn run(&mut self);
}
