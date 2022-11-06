use clap::Parser;
use todo_rs_core::Id;

#[derive(Parser, Debug)]
pub struct AddTaskCmd {
    pub title: String,

    pub description: Option<String>,

    #[arg(short, long)]
    pub list: bool,
}

#[derive(Parser, Debug)]
pub struct GetTaskCmd {
    pub id: Id,
}

#[derive(Parser, Debug)]
pub struct UpdateTaskCmd {
    pub id: Id,

    pub title: String,

    pub description: Option<String>,

    #[arg(short, long)]
    pub list: bool,
}

#[derive(Parser, Debug)]
pub struct DeleteTaskCmd {
    pub id: Id,

    #[arg(short, long)]
    pub list: bool,
}
