use crate::flight;

use flight::{Flight, FlightResult};
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::{thread, time};

pub struct Webservice {
    pub(crate) recv: Arc<Mutex<Receiver<Flight>>>,
    pub(crate) send: Arc<Mutex<Sender<Flight>>>,
}

fn process_flight(reservation: Flight) {
    thread::sleep(time::Duration::from_millis(100));
    println!(
        "Processing flight {} to {} from {}",
        reservation.airline, reservation.destination, reservation.origin
    );
    let id_str = format!(
        "{}{}{}",
        reservation.airline.to_owned(),
        reservation.destination,
        reservation.origin
    );
    reservation.result_gateway.send(FlightResult {
        id: id_str,
        accepted: true,
    });
}

impl Webservice {
    pub fn run_webservice(&self) {
        let recver = self.recv.lock().unwrap();
        loop {
            let flight = recver.recv();
            if flight.is_ok() {
                thread::spawn(move || process_flight(flight.unwrap()));
            } else {
                println!("error!");
                break;
            }
        }
    }

    pub fn close_webservice(&self){
        println!("closing webservice");
        //drop(self.send);
    }
}