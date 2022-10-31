use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{serializable_uuid, Id};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    #[serde(with = "serializable_uuid")]
    id: Id,
    pub title: String,
    pub description: Option<String>,
    pub time: Option<Timer>,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(
        title: String,
        description: Option<String>,
        time: Option<Timer>,
        status: TaskStatus,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            time,
            status,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
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
