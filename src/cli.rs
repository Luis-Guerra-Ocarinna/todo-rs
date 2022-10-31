mod args;

use crate::{
    model::{
        board::Board,
        task::{Task, TaskStatus},
    },
    Run,
};
use clap::Parser;

use self::args::{AddTaskCmd, Crud};

pub struct Cli {
    board: Board,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            board: Board {
                id: 1,
                tasks: vec![Task {
                    title: "Test".to_string(),
                    description: None,
                    time: None,
                    status: TaskStatus::Todo,
                }],
            },
        }
    }

    fn list(&self) {
        println!("Tasks: {:#?}", self.board.tasks);
    }

    fn add(&mut self, task: &AddTaskCmd) {
        let task = Task {
            title: task.title.clone(),
            description: task.description.clone(),
            time: None,
            status: TaskStatus::Todo,
        };

        self.board.tasks.push(task);

        println!("Task added! {:#?}", self.board.tasks);
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
