use crate::{gen_id, Id};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: Id,
    pub title: String,
    pub description: Option<String>,
    pub timer: Option<Timer>,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(title: String, description: Option<String>) -> Self {
        Self {
            id: gen_id(),
            title,
            description,
            timer: None,
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

type StartTimes = Vec<String>;
type PauseTimes = Vec<String>;
type ElapsedTime = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Timer {
    pub started: Option<StartTimes>,
    pub paused: Option<PauseTimes>,
    pub ended: Option<ElapsedTime>,
}
