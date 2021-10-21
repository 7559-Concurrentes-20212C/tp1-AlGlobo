mod flight;
mod thread_pool;
mod resultservice;
mod webservice;

use webservice::Webservice;
use resultservice::ResultService;
use flight::{Flight};

use std::sync::Arc;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();

    const RATE_LIMIT: usize = 4;

    // let filename = &args[0];
    let file = File::open("test.txt").expect("could not open file");
    let reader = BufReader::new(file);

    //creates service for handling incoming results
    let results_service = Arc::new(ResultService::new(RATE_LIMIT));

    //creates all web services
    let mut web_services = HashMap::new();
    web_services.insert("aerolineas argentinas".to_owned(), Arc::new(Webservice::new(RATE_LIMIT)));

    for line in reader.lines().flatten() {

        let reservation = build_reservation(line, Arc::clone(&results_service));
        let ws = web_services.get(&reservation.airline).unwrap();
        ws.process(reservation);
    }

    println!("finished!");
}

fn build_reservation(line: String, result_service: Arc<ResultService>) -> Flight {
    Flight {
        origin: line,
        destination: "BRC".to_owned(),
        airline: "aerolineas argentinas".to_owned(),
        result_service: result_service,
    }
}