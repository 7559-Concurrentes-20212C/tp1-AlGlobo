use crate::reservation;
extern crate rand;

use crate::decision::Decision;
use crate::logger::Logger;
use crate::reservation_result::ReservationResult;
use reservation::Reservation;
use std::fmt;
use std::sync::Arc;
use std::thread;
use std::time;
use std::time::Instant;

pub struct Webservice {
    id: usize,
    success_rate: u32,
    logger: Arc<Logger>,
}

impl Webservice {
    pub fn new(id: usize, success_chance: u32, logger: Arc<Logger>) -> Webservice {
        Webservice {
            id,
            success_rate: success_chance.min(100),
            logger,
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

        self.logger.log_extra_arg(
            format!("{}", self),
            "received reservation".to_string(),
            format!("{}", (*req).clone()),
            format!("{}", decision),
        );

        random_wait();

        ReservationResult::from_reservation_ref(
            (*req).clone(),
            matches!(decision, Decision::Accepted),
            time_requested.elapsed(),
        )
    }
}

impl fmt::Display for Webservice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WEBSERVICE <{}>", self.id)
    }
}

fn wait(milliseconds: u64) {
    thread::sleep(time::Duration::from_millis(milliseconds));
}

fn random_wait() {
    let i: i32 = rand::random();
    wait(i as u64 % 1000);
}
