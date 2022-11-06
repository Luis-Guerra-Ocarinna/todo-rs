pub mod local_storage;

pub use local_storage::LocalStorage;

pub trait Storage {
    fn load(&self, file: &str) -> String;
    fn save(&self, file: &str, data: &String);
}
