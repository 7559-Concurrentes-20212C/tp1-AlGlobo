pub mod program;
pub mod reservation;
pub mod thread_pool;
pub mod resultservice;
pub mod webservice;
pub mod stats_service;
pub mod schedule_service;
pub mod unit_tests;

use program::{Program};

use std::sync::{Arc, Condvar, Mutex};
use std::env;

pub fn run() {

    println!("Setting up environment...");

    const RATE_LIMIT: usize = 4;
    let exit_cv = Arc::new((Mutex::new(false), Condvar::new()));
    let exit_cv2 = Arc::clone(&exit_cv);

    ctrlc::set_handler(move || {
        let (lock, cvar) = &*exit_cv2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    }).expect("Error setting Ctrl-C handler");

    let mut program = Program::new(RATE_LIMIT);
    program.run(env::args().collect());

    let (lock, cvar) = &*exit_cv;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    program.print_results();
}

