pub mod program;
mod webservice_actor;
mod reservation;

use program::{Program};

use std::env;

pub async fn run() {

    println!("Setting up environment...");

    const RATE_LIMIT: usize = 4;

    let mut program = Program::new(RATE_LIMIT);
    let task = program.run(env::args().collect()).await;
    println!("asdasdasdasdasd {} ", task.unwrap());

    //program.print_results();
}

