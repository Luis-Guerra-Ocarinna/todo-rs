use crate::{model::task::Task, Id};

pub trait BoardRepo {
    fn add_task(&mut self, task: Task);
    fn list_tasks(&mut self) -> &Vec<Task>;
    fn get_task(&mut self, id: &Id) -> Option<&mut Task>;
    fn update_task(&mut self, id: &Id, title: &String, description: Option<String>);
    fn del_task(&mut self, id: &Id);
    fn clear(&mut self);

    // TODO: proper repo
    fn patch_task_name(&mut self, id: &Id, title: &String);
    fn patch_task_description(&mut self, id: &Id, description: &Option<String>);
    fn task_done(&mut self, id: &Id);

    fn start_timer(&mut self, id: &Id);
    fn pause_timer(&mut self, id: &Id);
    fn resume_timer(&mut self, id: &Id);
    fn stop_timer(&mut self, id: &Id);
}
