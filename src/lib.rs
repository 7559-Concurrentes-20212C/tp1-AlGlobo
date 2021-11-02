pub mod logger;
pub mod program;
pub mod reservation;
pub mod resultservice;
pub mod schedule_service;
pub mod stats_service;
pub mod thread_pool;
pub mod unit_tests;
pub mod webservice;

pub mod decision;
pub mod job;
pub mod message;
pub mod moving_stats;
pub mod worker;

pub mod ranked_route_entry;
pub mod reservation_kind;
pub mod reservation_result;

use program::Program;

use std::env;

pub fn run() {
    println!("Setting up environment...");
    let log_file_name: String = String::from("stats_results.txt");
    let mut program = Program::new(log_file_name);
    program.run(env::args().collect());
    program.print_results();
}
