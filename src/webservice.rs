use crate::thread_pool;
use crate::reservation;
extern crate rand;

use std::thread;
use std::time;
use thread_pool::{ThreadPool};
use reservation::{Reservation};
use std::sync::Arc;
use std::time::{Duration, Instant};
use crate::reservation::{ReservationResult, ReservationProcessRequest};

enum Decision {
    Accepted,
    Rejected,
}

pub struct Webservice {
    thread_pool: ThreadPool,
    success_rate: usize
}

impl Webservice {

    pub fn new(capacity: usize, success_chance: usize) -> Webservice {
        Webservice {
            thread_pool: ThreadPool::new(capacity),
            success_rate: success_chance % 100
        }
    }

    fn decide(&self) -> Decision{
        let i: i32 = rand::random();
        if i % 100 <= self.success_rate as i32 {
            return Decision::Accepted;
        }
        return Decision::Rejected;
    }

    pub fn process(&self, req: Arc<ReservationProcessRequest>){
        let start = Instant::now();
        let decision = self.decide();
        self.thread_pool.execute(move || {
            let id = _process(req.reservation.clone());

            random_wait();
            let result = ReservationResult::from_reservation_ref(req.reservation,
                                                matches!(decision , Decision::Accepted),
                                                start.elapsed());
            req.resolve(result);
        })
    }
}

fn _process(reservation: Arc<Reservation>) -> String {

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

    return id_str;
}

fn wait(miliseconds: u64){
    thread::sleep(time::Duration::from_millis(miliseconds));
}

fn random_wait() {
    let i: i32 = rand::random();
    wait(i as u64 % 1000);
}