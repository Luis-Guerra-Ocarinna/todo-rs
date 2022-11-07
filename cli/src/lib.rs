mod args;
mod commands;

pub use self::{args::*, commands::*};
use ascii_table::AsciiTable;
use cli_table::{format::Justify, print_stdout, Color, Table, WithTitle};
use commands::{
    crud::{AddTaskCmd, DeleteTaskCmd, GetTaskCmd, UpdateTaskCmd},
    task::{TaskCommands, TaskSubCommand, TimerCommands, TimerSubCommand},
};
use todo_rs_core::{
    model::task::{self, Task},
    Context, Id,
};

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
    #[derive(Table)]
    struct PrintTask {
        #[table(title = "#", color = "Color::Cyan")]
        id: Id,

        #[table(title = "Title")]
        title: String,

        #[table(title = "Status")]
        status: String,

        #[table(title = "Timer ⌚", justify = "Justify::Center")]
        timer: String,
    }

    const NONE: &str = "X";

    let tasks = context.board_repo.list_tasks();

    let tasks = tasks
        .iter()
        .map(|task| {
            let timer = if let task::TaskStatus::Done = task.status {
                if let Some(task::Timer { ended, .. }) = task.timer.as_ref() {
                    ended.as_ref().unwrap().clone()
                } else {
                    NONE.to_string()
                }
            } else {
                match task.timer_to_string() {
                    Some(timer) => timer,
                    None => NONE.to_string(),
                }
            };

            let status = match task.status {
                task::TaskStatus::Todo => String::from("Todo"),
                task::TaskStatus::InProgress => String::from("In Progress"),
                task::TaskStatus::Done => String::from("Done"),
                task::TaskStatus::Paused => String::from("Paused"),
            };

            PrintTask {
                id: task.id().to_string(),
                title: task.title.clone(),
                status,
                timer,
            }
        })
        .collect::<Vec<_>>();

    print_stdout(tasks.with_title()).unwrap();
}

fn get_task(mut context: Context, cmd: &GetTaskCmd) {
    const NONE: &str = "X";

    match context.board_repo.get_task(&cmd.id) {
        None => panic!("Task not found"),
        Some(task) => {
            let title = &task.title;
            let description = &task.description.clone().unwrap_or_default();
            let status = &task.status.to_string();
            let timer = &if let task::TaskStatus::Done = task.status {
                if let Some(task::Timer { ended, .. }) = task.timer.as_ref() {
                    ended.as_ref().unwrap().clone()
                } else {
                    NONE.to_string()
                }
            } else {
                match task.timer_to_string() {
                    Some(timer) => timer,
                    None => NONE.to_string(),
                }
            };

            let data = vec![
                ["Title", title],
                ["", ""],
                ["Description", description],
                ["Status", status],
                ["Timer", timer],
            ];

            let ascii_table = AsciiTable::default();

            ascii_table.print(data);
        }
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
