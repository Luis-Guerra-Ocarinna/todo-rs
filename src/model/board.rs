use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{serializable_uuid, Id};

use super::task::Task;

#[derive(Serialize, Deserialize)]
pub struct Board {
    #[serde(with = "serializable_uuid")]
    pub id: Id,

    pub tasks: Vec<Task>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            tasks: Vec::new(),
        }
    }
}
