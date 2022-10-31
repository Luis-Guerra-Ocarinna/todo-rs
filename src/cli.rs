mod args;

use crate::{
    model::task::{Task, TaskStatus},
    Context, Id, Run,
};
use clap::Parser;

use self::args::{AddTaskCmd, Crud, UpdateTaskCmd};

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
        let task = Task::new(
            task.title.clone(),
            task.description.clone(),
            None,
            TaskStatus::Todo,
        );

        self.context.board_repo.add_task(task);
    }

    fn update(&mut self, task: &UpdateTaskCmd) {
        self.context
            .board_repo
            .update_task(&task.id, &task.title, task.description.to_owned());
    }

    fn del(&mut self, id: &Id) {
        self.context.board_repo.del_task(id);
    }
}

impl Run for Cli {
    fn run(&mut self) {
        let cmd = args::Commands::parse();

        if cmd.clear {
            self.context.board_repo.clear();
        }

        if let Some(crud) = cmd.crud {
            match crud {
                Crud::Create(task_cmd) => self.add(&task_cmd),
                Crud::Read => self.list(),
                Crud::Update(update_cmd) => self.update(&update_cmd),
                Crud::Delete(del_cmd) => self.del(&del_cmd.id),
            }
        }
    }
}
