mod reservation;
mod thread_pool;
mod resultservice;
mod webservice;
mod stats_service;

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
    const SUCCESS_CHANCE : usize = 70;
    let reservations : String = String::from("reservations.txt");

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

    //creates all web services container
    let mut web_services : HashMap<String, Webservice> = HashMap::new();

    //creates hotel
    //let hotel = Arc::new(Hotel::new());

    for line in reader.lines().flatten() {
        let reservation = Arc::new(Reservation::from_line(line, Arc::clone(&results_service)));
        let ws = web_services.entry(reservation.airline.clone())
                                            .or_insert(Webservice::new(RATE_LIMIT, SUCCESS_CHANCE));

        ws.process(reservation.clone());
        //todo, esto esta mal, se tiene que evaluar todo junto el paquete (no podemos aceptar hotel si y vuelo no)
        //todo no se me ocurre como hacerlo con este modelo, voy a pensar
        //if matches!(reservation.kind, ReservationKind::Package){hotel.process(reservation.clone());}
    }

    //si aca habria que hacer esperar al main, por ahi que lea de consola un x o algo
    thread::sleep(time::Duration::from_millis(15000)); //TODO si no pongo esto el main sale de scope y ordena a terminar el resto de los threads
    println!("finished!");
    results_service.print_results();
}