use crate::{
    infrastructure::storage::Storage,
    model::{board::Board, task::Task},
    Id,
};

pub trait BoardRepo {
    fn add_task(&mut self, task: Task);
    fn list_tasks(&self) -> Vec<Task>;
    fn get_task(&self, id: &Id) -> Option<Task>;
    fn update_task(&mut self, id: &Id, title: &String, description: Option<String>);
    fn del_task(&mut self, id: &Id);
    fn clear(&mut self);
}

pub struct BoardJsonRepo {
    storage: Box<dyn Storage>,
    board: Board,
}

impl BoardJsonRepo {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        let board = storage.load();
        Self { storage, board }
    }
}

impl BoardRepo for BoardJsonRepo {
    fn add_task(&mut self, task: Task) {
        self.board.tasks.push(task);

        self.storage.save(&self.board);
    }

    fn list_tasks(&self) -> Vec<Task> {
        self.storage.load().tasks
    }

    fn get_task(&self, id: &Id) -> Option<Task> {
        let board = self.storage.load();
        board.tasks.into_iter().find(|task| task.id() == *id)
    }

    fn update_task(&mut self, id: &Id, title: &String, description: Option<String>) {
        let mut task = self
            .board
            .tasks
            .iter_mut()
            .find(|task| task.id() == *id)
            .expect("not found");

        task.title = title.to_string();
        task.description = description;

        self.storage.save(&self.board);
    }

    fn del_task(&mut self, id: &Id) {
        self.board.tasks.retain(|task| task.id() != *id);

        self.storage.save(&self.board);
    }

    fn clear(&mut self) {
        self.board.tasks = vec![];

        self.storage.save(&self.board);
    }
}
