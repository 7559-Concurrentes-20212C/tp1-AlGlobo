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
    let reservations : String = String::from("reservations.txt");

    // let filename = &args[0];
    let file = File::open(reservations).expect("could not open file");
    let reader = BufReader::new(file);

    //creates service for handling incoming results
    let results_service = Arc::new(ResultService::new(RATE_LIMIT));

    //creates all web services container
    let mut web_services : HashMap<Arc<String>, Webservice> = HashMap::new();

    for line in reader.lines().flatten() {

        let reservation = build_reservation(line, Arc::clone(&results_service));

        let ws = web_services.entry(Arc::clone(&reservation.airline)).or_insert(Webservice::new(RATE_LIMIT));
    
        ws.process(reservation);
    }

    println!("finished!");
}


fn build_reservation(line: String, result_service: Arc<ResultService>) -> Flight {

    let params = line.split(',').collect::<Vec<&str>>();

    Flight {
        origin: String::from(params[0]),
        destination: String::from(params[1]),
        airline: Arc::new(String::from(params[2])),
        reservation_type: String::from(params[3]),
        result_service: result_service,
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc};
    use crate::{build_reservation, resultservice::ResultService};

    #[test]
    fn it_works() {


        let web_services : HashMap<String, u16> = HashMap::new();
        web_services.get("a");

        const RATE_LIMIT: usize = 4;
        let results_service = Arc::new(ResultService::new(RATE_LIMIT));
        let line  = "A,B,ar1,flight".to_owned();
        let output = build_reservation(line, Arc::clone(&results_service));
        println!("{}, {}, {}, {}", output.origin, output.destination, output.airline, output.reservation_type,)
    }
}