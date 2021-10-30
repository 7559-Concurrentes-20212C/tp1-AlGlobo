pub mod program;
mod schedule_service;
mod resultservice;
mod stats_service;
mod webservice;
mod reservation;

use program::{Program};

use std::env;

pub async fn run() {

    println!("Setting up environment...");

    const RATE_LIMIT: usize = 4;

    let mut program = Program::new(RATE_LIMIT);
    program.run(env::args().collect());

    //program.print_results();
}