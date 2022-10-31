use std::{env, fs::create_dir};

use todo_rs::{
    infrastructure::{repository::board_repo::BoardJsonRepo, storage::local_storage::LocalStorage},
    Cli, Context, Run,
};

static mut CONFIG: Config = Config {
    web: false,
    remote: false,
    data_dir: "./data",
};

fn main() {
    unsafe { setup() };

    create_dir(unsafe { CONFIG.data_dir }).unwrap_or_default();

    let context = Context {
        board_repo: Box::new(BoardJsonRepo::new(if unsafe { CONFIG.remote } {
            todo!()
        } else {
            Box::new(LocalStorage)
        })),
    };

    let mut program: Box<dyn Run> = if unsafe { CONFIG.web } {
        todo!()
    } else {
        Box::new(Cli::new(context))
    };

    program.run();
}

struct Config {
    web: bool,
    remote: bool,
    data_dir: &'static str,
}

unsafe fn setup() {
    CONFIG.web = env::var("TODO_API").is_ok();
    CONFIG.remote = env::var("TODO_REMOTE").is_ok();
}

/*
// TODO
Objetivo:
    [x] criar/listar tarefas
    [x] persistir lol
    [x] deletar/atualizar ^
    [] implementar cronometro
    [] versão webapi
/* Models
    Board {
        id;
        tasks: Task[]
    }

    Task {
        title;
        descripion?
        time: Time?
    }

    [] Time {
        started;
        ended;
        total;
    }

    [] User? {
        name;
        pass;
        board: Board;
    }
*/

/* Dir
    model/
    web/

    lib.rs
    main.rs
 */

 /* `Repo`
    trait Repo {
        fn create(&self, task: Task) -> Task;
        fn read(&self) -> Vec<Task>;
        fn get(&self, id: Id) -> Task; ?
        fn update(&self, task: Task) -> Task;
        fn delete(&self, task: Task) -> Task;
    }
  */

  /* `Storage`
    trait Storage {
        type Data;
        fn save(&self, data: Data) -> Result<,>; ?
        fn load(&self) -> Data;
    }
   */
 */
