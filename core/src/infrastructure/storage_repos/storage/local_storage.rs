use std::{fs, io};

use super::Storage;

pub struct LocalStorage {
    dir: String,
}

impl LocalStorage {
    pub fn create(dir: String) -> Self {
        if !std::path::Path::new(&dir).exists() {
            fs::create_dir_all(&dir).expect("Failed to create directory");

            add_schemas(&dir);
        }

        Self { dir }
    }
}

// TODO: move and refact
fn add_schemas(dir: &str) {
    let schemas = vec![
        (
            "board.json",
            include_str!("../../../model/.schema/board.json"),
        ),
        (
            "task.json",
            include_str!("../../../model/.schema/task.json"),
        ),
        ("id.json", include_str!("../../../model/.schema/id.json")),
    ];

    for (file, schema) in schemas {
        let dir = format!("{dir}/schema");
        fs::create_dir_all(&dir).expect("Failed to create schema directory");
        let path = format!("{dir}/{file}");
        fs::write(path, schema).expect("Failed to write schema");
    }
}

impl Storage for LocalStorage {
    fn load(&self, file: &str) -> String {
        let path = format!("{}/{file}", self.dir);
        match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    String::new()
                } else {
                    panic!("Failed to read file: {e}")
                }
            }
        }
    }

    fn save(&self, file: &str, data: &String) {
        let path = format!("{}/{file}", self.dir);
        fs::write(&path, data).expect(format!("Failed to save file [{path}]").as_str());
    }
}
