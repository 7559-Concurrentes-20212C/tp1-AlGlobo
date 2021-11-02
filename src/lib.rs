pub mod program;
pub mod reservation;
pub mod resultservice;
pub mod schedule_service;
pub mod stats_service;
pub mod thread_pool;
pub mod unit_tests;
pub mod webservice;

use program::Program;

use std::env;

pub fn run() {
    println!("Setting up environment...");

    const RATE_LIMIT: u32 = 4;
    let mut program = Program::new(RATE_LIMIT);
    program.run(env::args().collect());
    program.print_results();
}
