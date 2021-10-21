use crate::thread_pool;
use crate::flight;
extern crate rand;

use std::thread;
use std::time;
use thread_pool::{ThreadPool};
use flight::{Flight};

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
        self.thread_pool.execute(move || {
            _process(reservation);
        })
    }
}

fn _process(reservation: Flight) {

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

    let i: i32 = rand::random();
    thread::sleep(time::Duration::from_millis(i as u64 % 1000));

    //reservation.result_gateway.lock().expect("error result gateway").send(Message::NewJob(job));

    reservation.result_service.process_result(id_str, true);
}

