use crate::reservation;
extern crate rand;

use crate::reservation::ReservationResult;
use reservation::Reservation;
use std::sync::Arc;
use std::thread;
use std::time;
use std::time::Instant;

enum Decision {
    Accepted,
    Rejected,
}

pub struct Webservice {
    success_rate: u32,
}

impl Webservice {
    pub fn new(success_chance: u32) -> Webservice {
        Webservice {
            success_rate: success_chance.min(100),
        }
    }

    fn decide(&self) -> Decision {
        let i: i32 = rand::random();
        if self.success_rate > 0 && (i % 100000) <= (self.success_rate * 1000) as i32 {
            return Decision::Accepted;
        }
        Decision::Rejected
    }

    pub fn process(
        &self,
        req: Arc<Reservation>,
        time_requested: Arc<Instant>,
    ) -> ReservationResult {
        let decision = self.decide();

        random_wait();
        ReservationResult::from_reservation_ref(
            req,
            matches!(decision, Decision::Accepted),
            time_requested.elapsed(),
        )
    }
}

fn wait(milliseconds: u64) {
    thread::sleep(time::Duration::from_millis(milliseconds));
}

fn random_wait() {
    let i: i32 = rand::random();
    wait(i as u64 % 1000);
}
