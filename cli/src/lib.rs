mod args;
mod commands;

pub use self::{args::*, commands::*};
use commands::{
    crud::{AddTaskCmd, DeleteTaskCmd, GetTaskCmd, UpdateTaskCmd},
    task::{TaskCommands, TaskSubCommand, TimerCommands, TimerSubCommand},
};
use todo_rs_core::{model::task::Task, Context, Id};

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

            Commands::Task(task_sub_cmd) => task_sub_cmd.run(context),
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

impl Run for TaskSubCommand {
    fn run(&mut self, context: Context) {
        match &mut self.task {
            TaskCommands::PatchName { name } => {
                patch_task_name(context, &self.id, name);
            }
            TaskCommands::PatchDescription { description } => {
                patch_task_description(context, &self.id, description);
            }
            TaskCommands::Timer(time_sub_cmd) => time_sub_cmd.run(&self.id, context),
            TaskCommands::Done => task_done(context, &self.id),
        }
    }
}

fn task_done(mut context: Context, id: &Id) {
    context.board_repo.task_done(id);
}

fn patch_task_description(mut context: Context, id: &Id, description: &Option<String>) {
    context.board_repo.patch_task_description(id, description);
}

fn patch_task_name(mut context: Context, id: &Id, name: &String) {
    context.board_repo.patch_task_name(id, name);
}

// TODO: improve how task id is passed to subcommands
impl TimerSubCommand {
    fn run(&mut self, id: &Id, context: Context) {
        match &self.timer {
            TimerCommands::Start => start_timer(context, id),
            TimerCommands::Pause => pause_timer(context, id),
            TimerCommands::Resume => resume_timer(context, id),
            TimerCommands::Stop => stop_timer(context, id),
        }
    }
}

fn stop_timer(mut context: Context, id: &Id) {
    context.board_repo.stop_timer(id);
}

fn resume_timer(mut context: Context, id: &Id) {
    context.board_repo.resume_timer(id);
}

fn pause_timer(mut context: Context, id: &Id) {
    context.board_repo.pause_timer(id);
}

fn start_timer(mut context: Context, id: &Id) {
    context.board_repo.start_timer(id);
}
