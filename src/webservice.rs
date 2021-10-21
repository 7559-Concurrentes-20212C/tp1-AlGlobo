use crate::thread_pool;
use crate::flight;

use std::thread;
use std::time;
use thread_pool::{Message, ThreadPool};
use flight::{FlightResult, Flight};

pub struct Webservice {
    pub thread_pool : ThreadPool,
}

impl Webservice {

    pub fn new(rate_limit: usize) -> Webservice {
        Webservice {
            thread_pool : ThreadPool::new(rate_limit),
        }
    }

    pub fn process(&self, reservation: Flight){
        self.thread_pool.execute(|| {
            _process(reservation);
        })
    }
}

fn _process(reservation: Flight) {
    thread::sleep(time::Duration::from_millis(100));
    println!(
        "Processing flight {} to {} from {}",
        reservation.airline, reservation.destination, reservation.origin
    );
    let id_str = format!(
        "{}{}{}",
        reservation.airline.to_owned(),
        reservation.destination,
        reservation.origin,
    );

    let job = Box::new(|| {
        build_result(id_str, true);
    });
    reservation.result_gateway.send(Message::NewJob(job));
}

fn build_result(id_str : String, accepted : bool) -> FlightResult {
    FlightResult {
        id: id_str,
        accepted: true,
    }
}