use std::fs;

use crate::model::board::Board;

use super::Storage;

static FILE: &str = "./data/board.json";
static SCHEMA: &str = "\n  \"$schema\": \"./schema/board.json\",";

pub struct LocalStorage;

impl Storage for LocalStorage {
    fn load(&self) -> Board {
        fs::read_to_string(FILE)
            .map(|s| serde_json::from_str(&s).unwrap())
            .unwrap_or_default()
    }

    fn save(&self, data: Board) {
        let mut json = serde_json::to_string_pretty(&data).unwrap();
        json.insert_str(1, SCHEMA);

        fs::write(FILE, json).unwrap();
    }
}
