mod args;

use crate::{
    model::task::{Task, TaskStatus},
    Context, Run,
};
use clap::Parser;

use self::args::{AddTaskCmd, Crud};

pub struct Cli {
    context: Context,
}

impl Cli {
    pub fn new(context: Context) -> Self {
        Self { context }
    }

    fn list(&self) {
        println!("{:?}", self.context.board_repo.list_tasks());
    }

    fn add(&mut self, task: &AddTaskCmd) {
        let task = Task {
            title: task.title.clone(),
            description: task.description.clone(),
            time: None,
            status: TaskStatus::Todo,
        };

        self.context.board_repo.add_task(task);
    }
}

impl Run for Cli {
    fn run(&mut self) {
        let cmd = args::Commands::parse();

        match &cmd.crud {
            Crud::Create(task) => self.add(&task),
            Crud::Read => self.list(),
        }
    }
}
