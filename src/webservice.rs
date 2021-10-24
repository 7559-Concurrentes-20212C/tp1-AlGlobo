use crate::thread_pool;
use crate::reservation;
extern crate rand;

use std::thread;
use std::time;
use thread_pool::{ThreadPool};
use reservation::{Reservation};

enum Decision {
    Accepted,
    Rejected,
}

pub struct Webservice {
    pub thread_pool : ThreadPool,
}

impl Webservice {

    pub fn new(rate_limit: usize) -> Webservice {
        Webservice {
            thread_pool : ThreadPool::new(rate_limit),
        }
    }

    pub fn decide(&self) -> Decision{
        Decision::Accepted // TODO (usar el count del Arc del recvr del channel de la thread pool para saber si tomar o no)
    }

    pub fn process(&self, reservation: Reservation){

        const WAIT_TIME : u64 = 100;
        const ATTEMPTS : u64 = 10; 

        for _ in 0..ATTEMPTS { //ensures termination
            match self.decide() {
                Decision::Accepted => {
                    self.thread_pool.execute(move || {_process(reservation);});
                    break;
                },
                Decision::Rejected => wait(WAIT_TIME),
            }
        }
    }
}

fn _process(reservation: Reservation) {

    println!(
        "Processing flight {} to {} from {}",
        reservation.airline, reservation.destination, reservation.origin
    );
    let id_str = format!(
        "{}{}{}",
        reservation.airline,
        reservation.destination,
        reservation.origin,
    );

    random_wait();
    reservation.result_service.process_result(id_str, true);
}

fn wait(miliseconds: u64){
    thread::sleep(time::Duration::from_millis(miliseconds));
}

fn random_wait() {
    let i: i32 = rand::random();
    wait(i as u64 % 1000);
}