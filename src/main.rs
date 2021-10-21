mod webservice;
mod flight;
mod thread_pool;

use webservice::Webservice;
use flight::{Flight};

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();

    const RATE_LIMIT: u32 = 1;

    let filename = &args[0];
    let file = File::open("test.txt").expect("could not open file");
    let reader = BufReader::new(file);

    //creates service for handling incoming results
    let results_service = Arc::new(ResultService::new(RATE_LIMIT));
    let result_sender = Arc::new(Mutex::new(results_service.result_sender));

    //creates all web services
    let mut web_services = HashMap::new();
    web_services.insert("aerolineas argentinas".to_owned(), Arc::new(Webservice::new(RATE_LIMIT)));

    for line in reader.lines().flatten() {
        
        let reservation = build_reservation(line, Arc::clone(&result_send)));
        let ws = web_services.get(reservation.airline);
        ws.process(reservation);
    }

    println!("finished!");
}

fn build_reservation(line: String, result_send: Sender<FlightResult>) -> Flight {
    Flight {
        origin: line,
        destination: "BRC".to_owned(),
        airline: "aerolineas argentinas".to_owned(),
        result_gateway: result_send,
    };
}