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
    success_rate: usize
}

impl Webservice {

    pub fn new(success_chance: usize) -> Webservice {
        Webservice {
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

    pub fn process(&self, req: Arc<Reservation>) -> ReservationResult {
        let start = Instant::now();
        let decision = self.decide();

        random_wait();
        let result = ReservationResult::from_reservation_ref(req,
                                            matches!(decision , Decision::Accepted),
                                            start.elapsed());
        return result;
    }
}


fn wait(miliseconds: u64){
    thread::sleep(time::Duration::from_millis(miliseconds));
}

fn random_wait() {
    let i: i32 = rand::random();
    wait(i as u64 % 1000);
}