mod messages;
pub mod program;
mod resultservice;
mod schedule_service;
mod stats_service;
mod webservice;

use actix::{Actor, System};
use messages::Run;
use program::Program;

pub fn run() {
    let system = System::new();
    system.block_on(async {
        const RATE_LIMIT: usize = 4;

        let log_file_name: String = String::from("stats_results.txt");
    
        let program = Program::new(RATE_LIMIT, log_file_name).start();
        program.try_send(Run {}).unwrap_or_else( |_| { panic!("{}", "LIB: Couldn't send RUN message to PROGRAM".to_owned())});
    });

    system.run().expect("LIB: Couldn't run the PROGRAM");
}
