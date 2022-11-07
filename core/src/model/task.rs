use crate::{gen_id, Id};
use chrono::{DateTime, FixedOffset, Local};
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

    pub fn timer_to_string(&self) -> Option<String> {
        if let TaskStatus::Todo = self.status {
            return None;
        }

        let starts = self
            .timer
            .as_ref()
            .expect("Something went wrong, timer is not initialized")
            .started
            .as_ref()
            .expect("Something went wrong, timer was never started");

        // TODO: refact
        let empty_vec = Vec::<String>::new();

        let pauses = self
            .timer
            .as_ref()
            .expect("Something went wrong, timer is not initialized")
            .paused
            .as_ref()
            .unwrap_or_else(|| &empty_vec);

        let mut pauses = pauses.clone();

        if let TaskStatus::InProgress = self.status {
            let now = Local::now()
                .with_timezone(&FixedOffset::east(0))
                .to_rfc3339();

            pauses.push(now);
        }

        let mut elapsed = chrono::Duration::zero();

        for (start, pause) in starts.iter().zip(pauses.iter()) {
            let start = DateTime::parse_from_rfc3339(start).unwrap();
            let pause = DateTime::parse_from_rfc3339(pause).unwrap();

            let duration = pause - start;

            elapsed = elapsed + duration;
        }

        let secs = elapsed.num_seconds() % 60;
        let mins = (elapsed.num_seconds() / 60) % 60;
        let hours = (elapsed.num_seconds() / 60) / 60;

        if hours > 0 {
            Some(format!("{:02}h {:02}m {:02}s", hours, mins, secs))
        } else if mins > 0 {
            Some(format!("{:02}m {:02}s", mins, secs))
        } else {
            Some(format!("{:02}s", secs))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Paused,
}

impl ToString for TaskStatus {
    fn to_string(&self) -> String {
        match self {
            TaskStatus::Todo => String::from("Todo"),
            TaskStatus::InProgress => String::from("In Progress"),
            TaskStatus::Done => String::from("Done"),
            TaskStatus::Paused => String::from("Paused"),
        }
    }
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
