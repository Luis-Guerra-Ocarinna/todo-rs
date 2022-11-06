use crate::{gen_id, Id};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: Id,
    pub title: String,
    pub description: Option<String>,
    pub time: Option<Timer>,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(title: String, description: Option<String>) -> Self {
        Self {
            id: gen_id(),
            title,
            description,
            time: None,
            status: TaskStatus::Todo,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Paused,
}

type InitialTime = String;
type ElapsedTime = String;

#[derive(Debug, Serialize, Deserialize)]
pub enum Timer {
    Started(InitialTime),
    Ended(ElapsedTime),
}
