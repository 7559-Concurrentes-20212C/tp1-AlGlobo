mod logger;
pub mod program;
mod resultservice;
mod schedule_service;
mod stats_service;
mod webservice;

mod decision;
mod finished;
mod moving_stats;
mod ranked_route_entry;
mod reservation;
mod reservation_kind;
mod reservation_result;
mod run;
mod stats;
mod to_process_reservation;
mod to_process_reservation_result;
mod cooldown_service;
mod reservation_cooldown;

use actix::{Actor, System};
use program::Program;
use run::Run;

pub fn run() {
    let system = System::new();
    system.block_on(async {
        let log_file_name: String = String::from("stats_results.txt");

        let program = Program::new(log_file_name).start();
        program.try_send(Run {}).unwrap_or_else(|_| {
            panic!("{}", "LIB: Couldn't send RUN message to PROGRAM".to_owned())
        });
    });

    system.run().expect("LIB: Couldn't run the PROGRAM");
}
