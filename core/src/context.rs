use crate::{
    config,
    infrastructure::storage_repos::{json_repo::BoardJsonRepo, storage, Storage},
    repositories::BoardRepo,
    Config,
};

pub struct Context {
    pub board_repo: Box<dyn BoardRepo>,
}

impl Context {
    pub fn load(config: Config) -> Self {
        let board_repo = get_board_repo(config);

        Context { board_repo }
    }
}

fn get_board_repo(config: Config) -> Box<dyn BoardRepo> {
    match config.storage_type {
        config::StorageType::Json => {
            let storage = get_storage(config);
            Box::new(BoardJsonRepo::new(storage))
        }
        config::StorageType::SQL => todo!(),
    }
}

fn get_storage(config: Config) -> Box<dyn Storage> {
    match config.storage_location {
        config::StorageLocation::Local => {
            let dir = config
                .local_storage_dir
                .expect("Local storage path is not set");

            Box::new(storage::LocalStorage::create(dir))
        }
        config::StorageLocation::Remote => todo!("Remote storage not implemented yet"),
    }
}
