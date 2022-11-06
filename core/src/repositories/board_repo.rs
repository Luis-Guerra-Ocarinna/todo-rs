use crate::{model::task::Task, Id};

pub trait BoardRepo {
    fn add_task(&mut self, task: Task);
    fn list_tasks(&mut self) -> &Vec<Task>;
    fn get_task(&mut self, id: &Id) -> Option<&mut Task>;
    fn update_task(&mut self, id: &Id, title: &String, description: Option<String>);
    fn del_task(&mut self, id: &Id);
    fn clear(&mut self);
}
