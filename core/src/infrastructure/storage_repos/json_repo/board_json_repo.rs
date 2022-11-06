use crate::{
    infrastructure::storage_repos::Storage,
    model::{board::Board, task::Task},
    repositories::board_repo::BoardRepo,
    Id,
};

const FILE: &str = "board.json";

pub struct BoardJsonRepo {
    storage: Box<dyn Storage>,
    board: Board,
}

impl BoardJsonRepo {
    fn load_board(storage: &Box<dyn Storage>) -> Board {
        let content = storage.load(FILE);

        serde_json::from_str(&content).unwrap_or_default()
    }

    pub fn new(storage: Box<dyn Storage>) -> Self {
        let board = Self::load_board(&storage);
        Self { storage, board }
    }

    fn load(&mut self) -> &mut Self {
        self.board = Self::load_board(&self.storage);

        self
    }

    fn save(&self) {
        let mut json = serde_json::to_string_pretty(&self.board).unwrap();
        insert_schema(&mut json);
        self.storage.save(FILE, &json);
    }
}

fn insert_schema(json: &mut String) {
    let schema = format!("\n  \"$schema\": \"./schema/{FILE}\",");
    json.insert_str(1, &schema);
}

impl BoardRepo for BoardJsonRepo {
    fn add_task(&mut self, task: Task) {
        self.board.tasks.push(task);

        self.save();
    }

    fn list_tasks(&mut self) -> &Vec<Task> {
        &&self.load().board.tasks
    }

    fn get_task(&mut self, id: &Id) -> Option<&mut Task> {
        self.load()
            .board
            .tasks
            .iter_mut()
            .find(|task| task.id() == id)
    }

    fn update_task(&mut self, id: &Id, title: &String, description: Option<String>) {
        let task = self.get_task(id).expect("Task not found");

        task.title = title.to_string();
        task.description = description;

        self.save();
    }

    fn del_task(&mut self, id: &Id) {
        self.board.tasks.retain(|task| task.id() != id);

        self.save();
    }

    fn clear(&mut self) {
        self.board.tasks.clear();

        self.save();
    }
}
