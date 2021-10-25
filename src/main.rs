mod reservation;
mod thread_pool;
mod resultservice;
mod webservice;
mod stats_service;
mod schedule_service;

use webservice::Webservice;
use resultservice::ResultService;
use reservation::{Reservation};

use std::sync::{Arc, Condvar, Mutex};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use crate::schedule_service::ScheduleService;

fn main() {

    println!("Setting up environment...");
    let _args: Vec<String> = env::args().collect();

    const RATE_LIMIT: usize = 4;
    let reservations : String = String::from("reservations.txt");
    let airlines : String = String::from("valid_airlines.txt");
    let exit_cv = Arc::new((Mutex::new(false), Condvar::new()));
    let exit_cv2 = Arc::clone(&exit_cv);

    ctrlc::set_handler(move || {
        let (lock, cvar) = &*exit_cv2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    }).expect("Error setting Ctrl-C handler");

    // let filename = &args[0];
    let f = File::open(reservations);
    let file = match f {
        Ok(file) => file,
        Err(error) => {
            println!("problem opening file: {:?}", error);
            return;},
    };
    let reader = BufReader::new(file);

    //creates service for handling incoming results
    let results_service = Arc::new(ResultService::new(RATE_LIMIT));
    //creates hotel
    let hotel = Arc::new(Webservice::new(100));
    //creates all web services
    let web_services = load_services(airlines,&results_service, hotel).unwrap();

    println!("Set up finished!");

    for line in reader.lines().flatten() {
        let reservation = Arc::new(Reservation::from_line(line));
        let scheduler = match web_services.get(&*reservation.airline) {
            None => {println!("invalid airline reservation: {}", reservation.airline); continue}
            Some(s) => {s}
        };
        scheduler.schedule_to_process(reservation);
    }
    println!("finished scheduling reservations!");

    let (lock, cvar) = &*exit_cv;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    results_service.print_results();
}

fn load_services(file_name: String,
                 resultservice: &Arc<ResultService>,
                 hotel : Arc<Webservice> ) -> Option<HashMap<String, ScheduleService>>{

    let f = File::open(file_name);
    let file = match f {
        Ok(file) => file,
        Err(error) => {
            println!("problem opening file: {:?}", error);
            return None;},
    };
    let reader = BufReader::new(file);

    let mut web_services : HashMap<String, ScheduleService> = HashMap::new();

    for line in reader.lines().flatten() {
        let params = line.split(',').collect::<Vec<&str>>();
        let capacity = params[1].parse::<usize>().unwrap();
        let rate = params[2].parse::<usize>().unwrap();
        let webservice = Arc::new(Webservice::new(rate));
        web_services.insert(params[0].parse().unwrap(), ScheduleService::new(capacity,
                                                                             webservice,
                                                                             hotel.clone(),
                                                                             resultservice.clone()));
    }
    return Some(web_services)
}