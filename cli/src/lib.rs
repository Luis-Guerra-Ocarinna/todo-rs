mod commands;

use commands::crud::{AddTaskCmd, DeleteTaskCmd, GetTaskCmd, UpdateTaskCmd};
pub use commands::*;
use todo_rs_core::{model::task::Task, Context};

pub trait Run {
    fn run(&mut self, context: Context);
}

impl Run for Commands {
    fn run(&mut self, mut context: Context) {
        match self {
            Commands::Create(cmd) => add_task(context, cmd),
            Commands::Read => list_tasks(context),
            Commands::Get(cmd) => get_task(context, cmd),
            Commands::Update(cmd) => update_task(context, cmd),
            Commands::Delete(cmd) => delete_task(context, cmd),
            Commands::Clear => context.board_repo.clear(),
        }
    }
}

fn list_tasks(mut context: Context) {
    println!("{:#?}", context.board_repo.list_tasks());
}

fn get_task(mut context: Context, cmd: &GetTaskCmd) {
    match context.board_repo.get_task(&cmd.id) {
        Some(task) => println!("{:#?}", task),
        None => panic!("Task not found"),
    }
}

fn add_task(mut context: Context, cmd: &AddTaskCmd) {
    let task = Task::new(cmd.title.clone(), cmd.description.clone());

    context.board_repo.add_task(task);

    if cmd.list {
        list_tasks(context);
    }
}

fn update_task(mut context: Context, cmd: &UpdateTaskCmd) {
    context
        .board_repo
        .update_task(&cmd.id, &cmd.title, cmd.description.to_owned());

    if cmd.list {
        list_tasks(context);
    }
}

fn delete_task(mut context: Context, cmd: &DeleteTaskCmd) {
    context.board_repo.del_task(&cmd.id);

    if cmd.list {
        list_tasks(context);
    }
}
