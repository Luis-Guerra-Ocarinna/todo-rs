use crate::Id;

use super::task::Task;

pub struct Board {
    pub id: Id,
    pub tasks: Vec<Task>,
}
