pub mod cli;
pub mod infrastructure;
pub mod model;

pub use cli::Cli;
use infrastructure::repository::board_repo::BoardRepo;

type Id = uuid::Uuid;

mod serializable_uuid {
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
    use std::str::FromStr;
    use uuid::Uuid;

    pub fn serialize<S>(val: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        val.to_string().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: &str = Deserialize::deserialize(deserializer)?;
        Uuid::from_str(val).map_err(D::Error::custom)
    }
}

pub struct Context {
    pub board_repo: Box<dyn BoardRepo>,
}

pub trait Run {
    fn run(&mut self);
}

pub trait Repo<T> {
    fn create(&self, data: T) -> T;

    fn read(&self) -> Vec<T>;
}
