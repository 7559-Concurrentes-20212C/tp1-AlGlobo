use crate::messages::{ReservationResult, ToProcessReservation};
use actix::{Actor, ActorFutureExt, Context, Handler, ResponseActFuture, WrapFuture};
extern crate rand;
use actix::clock::sleep;
use std::fmt;
use std::time::Duration;
use std::fs;


enum Decision {
    Accepted,
    Rejected,
}

impl fmt::Display for Decision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Decision::Accepted => write!(f, "Accepted"),
            Decision::Rejected => write!(f, "Rejected"),
        }
    }
}

pub struct Webservice {
    success_rate: usize,
    id: usize,
    log_file_name: String,
}

impl Webservice {
    pub fn new(success_chance: usize, id: usize, log_file_name: String) -> Webservice {
        Webservice {
            success_rate: success_chance.min(100),
            id,
            log_file_name,
        }
    }

    fn decide(&self) -> Decision {
        let i: i32 = rand::random();
        if self.success_rate > 0 && (i % 100000) <= (self.success_rate * 1000) as i32 {
            return Decision::Accepted;
        }
        Decision::Rejected
    }
}

impl Actor for Webservice {
    type Context = Context<Self>;
}

impl Handler<ToProcessReservation> for Webservice {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: ToProcessReservation, _ctx: &mut Self::Context) -> Self::Result {
        let decision = self.decide();

        let to_log = format!(
            "WEBSERVICE <{}>: received reservation <{}>({}|{}-{}|{}|{})",
            self.id,
            msg.reservation.id,
            msg.reservation.airline,
            msg.reservation.origin,
            msg.reservation.destination,
            msg.reservation.kind,
            decision
        );
        fs::write(String::from(&self.log_file_name), to_log).unwrap_or_else(|_| panic!("WEBSERVICE <{}>: Couldn't write to log", self.id));
        println!(
            "WEBSERVICE <{}>: received reservation <{}>({}|{}-{}|{}|{})",
            self.id,
            msg.reservation.id,
            msg.reservation.airline,
            msg.reservation.origin,
            msg.reservation.destination,
            msg.reservation.kind,
            decision
        );

        let i: i32 = rand::random();
        let error_msg = format!("Could send message from WEBSERVICE <{}>", self.id);

        Box::pin(
            sleep(Duration::from_millis(i as u64 % 1000))
                .into_actor(self)
                .map(move |_result, _me, _ctx| {
                    let reservation = msg.reservation;
                    let elapsed = reservation.alive_timer.elapsed();
                    let result = ReservationResult::from_reservation_ref(
                        reservation,
                        matches!(decision, Decision::Accepted),
                        elapsed,
                    );
                    msg.sender.try_send(result).expect(&error_msg);
                }),
        )
    }
}
