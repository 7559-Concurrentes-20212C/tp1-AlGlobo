mod reservation;
mod thread_pool;
mod resultservice;
mod webservice;

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

fn main() {

    let args: Vec<String> = env::args().collect();

    const RATE_LIMIT: usize = 4;
    let reservations : String = String::from("reservations.txt");

    // let filename = &args[0];
    let file = File::open(reservations).expect("could not open file");
    let reader = BufReader::new(file);

    //creates service for handling incoming results
    let results_service = Arc::new(ResultService::new(RATE_LIMIT));

    //creates all web services container
    let mut web_services : HashMap<Arc<String>, Webservice> = HashMap::new();

    for line in reader.lines().flatten() {
        let reservation = Reservation::new(line, Arc::clone(&results_service));
        let ws = web_services.entry(Arc::clone(&reservation.airline)).or_insert(Webservice::new(RATE_LIMIT));
        ws.process(reservation);
    }

    thread::sleep(time::Duration::from_millis(15000)); //TODO si no pongo esto el main sale de scope y ordena a terminar el resto de los threads
    println!("finished!");
}