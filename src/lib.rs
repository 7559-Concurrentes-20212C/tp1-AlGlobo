pub mod program;
mod schedule_service;
mod resultservice;
mod stats_service;
mod webservice;
mod messages;

use actix::{System, Actor};
use program::{Program};
use messages::{Run};

//use std::env;

pub fn run() {

    let system = System::new();
    system.block_on(async {
        const RATE_LIMIT: usize = 4;
    
        let program = Program::new(RATE_LIMIT).start();
        //program.run(env::args().collect());
        program.try_send(Run {});
    });

    system.run().unwrap();
}