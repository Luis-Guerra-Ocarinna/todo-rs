pub mod cli;
pub mod infrastructure;
pub mod model;

pub use cli::Cli;
use infrastructure::repository::board_repo::BoardRepo;

type Id = u32;

pub struct Context {
    pub board_repo: Box<dyn BoardRepo>,
}

pub trait Run {
    fn run(&mut self);
}

pub trait Repo<T> {
    fn create(&self, data: T) -> T;

    fn read(&self) -> Vec<T>;
}
