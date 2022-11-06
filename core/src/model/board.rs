use super::task::Task;
use crate::Id;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub id: Id,

    pub tasks: Vec<Task>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            tasks: Vec::new(),
        }
    }
}
