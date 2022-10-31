use crate::{infrastructure::storage::Storage, model::task::Task};

pub trait BoardRepo {
    fn add_task(&self, task: Task);
    fn list_tasks(&self) -> Vec<Task>;
}

pub struct BoardJsonRepo {
    storage: Box<dyn Storage>,
}

impl BoardJsonRepo {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self { storage }
    }
}

impl BoardRepo for BoardJsonRepo {
    fn add_task(&self, task: Task) {
        let mut board = self.storage.load();
        board.tasks.push(task);
        self.storage.save(board);
    }

    fn list_tasks(&self) -> Vec<Task> {
        self.storage.load().tasks
    }
}
