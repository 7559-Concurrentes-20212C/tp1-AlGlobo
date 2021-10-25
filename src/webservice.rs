use crate::reservation;
extern crate rand;

use std::thread;
use std::time;
use reservation::{Reservation};
use std::sync::Arc;
use std::time::{Instant};
use crate::reservation::{ReservationResult};

enum Decision {
    Accepted,
    Rejected,
}

pub struct Webservice {
    success_rate: usize,
}

impl Webservice {

    pub fn new(success_chance: usize) -> Webservice {
        Webservice {
            success_rate: success_chance.min(100),
        }
    }

    fn decide(&self) -> Decision{
        let i: i32 = rand::random();
        if self.success_rate > 0 && (i % 100000) <= (self.success_rate * 1000)  as i32 {
            return Decision::Accepted;
        }
        return Decision::Rejected;
    }

    pub fn process(&self, req: Arc<Reservation>, time_requested : Arc<Instant>) -> ReservationResult {
        let decision = self.decide();

        random_wait();
        let result = ReservationResult::from_reservation_ref(req,
                                            matches!(decision , Decision::Accepted),
                                            time_requested.elapsed());
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