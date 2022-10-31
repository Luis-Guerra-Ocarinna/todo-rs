use crate::model::board::Board;

pub mod local_storage;

pub trait Storage {
    fn load(&self) -> Board;
    fn save(&self, data: Board);
}
