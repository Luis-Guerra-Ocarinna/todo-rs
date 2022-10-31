use std::env;

use todo_rs::{Cli, Run};

fn main() {
    let mut program = choose();

    program.run();
}

fn choose() -> Box<dyn Run> {
    if env::var("TODO_API").is_ok() {
        todo!("Running web server");
        // Box::new(todo_api::new())
    } else {
        Box::new(Cli::new())
    }
}

/*
// TODO
Objetivo:
    [x] criar/listar tarefas
    [] persistir lol
    [] deletar/atualizar ^
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

 */
