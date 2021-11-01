pub mod program;
pub mod reservation;
pub mod thread_pool;
pub mod resultservice;
pub mod webservice;
pub mod stats_service;
pub mod schedule_service;
pub mod unit_tests;

use program::{Program};

use std::env;

pub fn run() {

    println!("Setting up environment...");

    const RATE_LIMIT: usize = 4;
    let mut program = Program::new(RATE_LIMIT);
    program.run(env::args().collect());
    program.print_results();
}

