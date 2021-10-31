pub mod program;
mod schedule_service;
mod resultservice;
mod stats_service;
mod webservice;
mod reservation;

use actix::{System};
use program::{Program};

use std::env;

pub fn run() {

    let system = System::new();
    system.block_on(async {
        println!("Setting up environment...");

        const RATE_LIMIT: usize = 4;
    
        let mut program = Program::new(RATE_LIMIT);
        program.run(env::args().collect());
    
        //program.print_results();
    });

    system.run().unwrap();
}