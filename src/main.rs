mod program;
mod reservation;
mod thread_pool;
mod resultservice;
mod webservice;
mod stats_service;
mod schedule_service;
mod unit_tests;

use program::{Program};

use std::sync::{Arc, Condvar, Mutex};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {

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

