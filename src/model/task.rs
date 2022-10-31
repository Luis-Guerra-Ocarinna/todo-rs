#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub description: Option<String>,
    pub time: Option<Timer>,
    pub status: TaskStatus,
}

#[derive(Debug)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Paused,
}

#[derive(Debug)]
pub enum Timer {
    Started,
    Ended,
    Total,
}
