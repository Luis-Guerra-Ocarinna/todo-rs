use serde::{Serialize, Deserialize};

use crate::Id;

use super::task::Task;

#[derive(Serialize, Deserialize, Default)]
pub struct Board {
    pub id: Id,
    pub tasks: Vec<Task>,
}
