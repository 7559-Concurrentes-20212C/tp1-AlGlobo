mod reservation;
mod thread_pool;
mod resultservice;
mod webservice;
mod stats_service;
mod schedule_service;

use webservice::Webservice;
use resultservice::ResultService;
use reservation::{Reservation};

use core::time;
use std::sync::Arc;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::thread;
use crate::schedule_service::ScheduleService;

fn main() {

    let _args: Vec<String> = env::args().collect();

    const RATE_LIMIT: usize = 4;
    let reservations : String = String::from("reservations.txt");
    let airlines : String = String::from("valid_airlines.txt");


    // let filename = &args[0];
    let f = File::open(reservations);
    let file = match f {
        Ok(file) => file,
        Err(error) => {
            println!("problem opening file: {:?}", error);
            return;},
    };
    println!("loaded");

    let reader = BufReader::new(file);

    //creates service for handling incoming results
    let results_service = Arc::new(ResultService::new(RATE_LIMIT));

    //creates hotel
    let hotel = Arc::new(Webservice::new(100));

    //creates all web services
    let web_services = load_services(airlines,&results_service, hotel).unwrap();
    println!("loaded");
    for line in reader.lines().flatten() {
        let reservation = Arc::new(Reservation::from_line(line));
        web_services.get(&*reservation.airline).unwrap().schedule_to_process(reservation); //todo handle unwrap => nonexistent airline
    }

    //si aca habria que hacer esperar al main, por ahi que lea de consola un x o algo
    thread::sleep(time::Duration::from_millis(15000)); //TODO si no pongo esto el main sale de scope y ordena a terminar el resto de los threads
    println!("finished!");
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