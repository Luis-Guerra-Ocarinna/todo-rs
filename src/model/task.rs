use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub description: Option<String>,
    pub time: Option<Timer>,
    pub status: TaskStatus,
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
