use chrono::{DateTime, FixedOffset, Local};

use crate::{
    infrastructure::storage_repos::Storage,
    model::{
        board::Board,
        task::{self, Task},
    },
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

    fn patch_task_name(&mut self, id: &Id, title: &String) {
        self.get_task(id).expect("Task not found").title = title.to_string();

        self.save();
    }

    fn patch_task_description(&mut self, id: &Id, description: &Option<String>) {
        self.get_task(id).expect("Task not found").description = description.clone();

        self.save();
    }

    fn task_done(&mut self, id: &Id) {
        self.get_task(id).expect("Task not found").status = task::TaskStatus::Done;

        self.save();
    }

    fn start_timer(&mut self, id: &Id) {
        let task = self.get_task(id).expect("Task not found");

        match task.status {
            task::TaskStatus::Done => panic!("Task is already done"),
            task::TaskStatus::Paused => panic!("Timer is paused"),
            task::TaskStatus::InProgress => panic!("Timer is already running"),
            task::TaskStatus::Todo => {
                task.status = task::TaskStatus::InProgress;

                let now = current_time();

                if let Some(task::Timer { started, .. }) = &task.timer {
                    if let Some(starts) = started {
                        let mut starts = starts.clone();

                        starts.push(now);

                        task.timer.as_mut().unwrap().started = Some(starts);
                    } else {
                        task.timer.as_mut().unwrap().started = Some(vec![now]);
                    }
                } else {
                    task.timer = Some(task::Timer {
                        started: Some(vec![now]),
                        paused: None,
                        ended: None,
                    });
                }

                self.save();
            }
        }
    }

    fn pause_timer(&mut self, id: &Id) {
        let task = self.get_task(id).expect("Task not found");

        match task.status {
            task::TaskStatus::Done => panic!("Task is already done"),
            task::TaskStatus::Todo => panic!("Task is not timing"),
            task::TaskStatus::Paused => panic!("Timer is already paused"),
            task::TaskStatus::InProgress => {
                task.status = task::TaskStatus::Paused;

                let now = current_time();

                if let Some(pauses) = &task
                    .timer
                    .as_ref()
                    .expect("Something went wrong, timer is not initialized")
                    .paused
                {
                    let mut pauses = pauses.clone();

                    pauses.push(now);

                    task.timer.as_mut().unwrap().paused = Some(pauses);
                } else {
                    task.timer.as_mut().unwrap().paused = Some(vec![now]);
                }

                self.save();
            }
        }
    }

    fn resume_timer(&mut self, id: &Id) {
        let task = self.get_task(id).expect("Task not found");

        match task.status {
            task::TaskStatus::Done => panic!("Task is already done"),
            task::TaskStatus::Todo => panic!("Task is not timing"),
            task::TaskStatus::InProgress => panic!("Timer is already running"),
            task::TaskStatus::Paused => {
                task.status = task::TaskStatus::InProgress;

                let now = current_time();

                task.timer
                    .as_mut()
                    .expect("Something went wrong, timer is not initialized")
                    .started
                    .as_mut()
                    .expect("Something went wrong, timer was never started")
                    .push(now);

                self.save();
            }
        }
    }

    fn stop_timer(&mut self, id: &Id) {
        let task = self.get_task(id).expect("Task not found");

        match task.status {
            task::TaskStatus::Done => panic!("Task is already done"),
            task::TaskStatus::Todo => panic!("Task is not timing"),
            task::TaskStatus::InProgress => panic!("Please, pause the timer first"),
            task::TaskStatus::Paused => {
                task.status = task::TaskStatus::Done;

                let starts = task
                    .timer
                    .as_ref()
                    .expect("Something went wrong, timer is not initialized")
                    .started
                    .as_ref()
                    .expect("Something went wrong, timer was never started");

                let pauses = task
                    .timer
                    .as_ref()
                    .expect("Something went wrong, timer is not initialized")
                    .paused
                    .as_ref()
                    .expect("Something went wrong, timer was never paused");

                let mut elapsed = chrono::Duration::zero();

                for (start, pause) in starts.iter().zip(pauses.iter()) {
                    let start = DateTime::parse_from_rfc3339(start).unwrap();
                    let pause = DateTime::parse_from_rfc3339(pause).unwrap();

                    let duration = pause - start;

                    elapsed = elapsed + duration;
                }

                task.timer
                    .as_mut()
                    .expect("Something went wrong, timer is not initialized")
                    .ended = Some(format!("{} min", elapsed.num_minutes()));
            }
        }

        self.save();
    }
}

// TODO: move to a better place
fn current_time() -> String {
    Local::now()
        .with_timezone(&FixedOffset::east(0))
        .to_rfc3339()
}
