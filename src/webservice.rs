use crate::messages::{Finished, ReservationResult, ToProcessReservation};
use actix::{Actor, ActorFutureExt, AsyncContext, Context, Handler, ResponseActFuture, WrapFuture};
extern crate rand;
use crate::logger::Logger;
use actix::clock::sleep;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

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
    capacity: usize,
    wait_time: usize,
    success_rate: usize,
    id: usize,
    logger: Arc<Logger>,
    amount_processing: usize,
}

impl Webservice {
    pub fn new(
        capacity: usize,
        wait_time: usize,
        success_chance: usize,
        id: usize,
        logger: Arc<Logger>,
    ) -> Webservice {
        Webservice {
            capacity,
            wait_time,
            success_rate: success_chance.min(100),
            id,
            logger,
            amount_processing: 0,
        }
    }

    fn decide(&self) -> Decision {
        let i: i32 = rand::random();
        if self.success_rate > 0
            && (i % 100000) <= (self.success_rate * 1000) as i32
            && self.amount_processing <= self.capacity
        {
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
        self.amount_processing += 1;
        let decision = self.decide();

        let decision_waiting_time = match decision {
            Decision::Accepted => 0,
            Decision::Rejected => self.wait_time,
        };

        let rand_time: u64 = rand::random();
        let process_time: u64 = (rand_time % 1000) + decision_waiting_time as u64;

        self.logger.log_extra_arg(
            format!("{}", self),
            "received reservation".to_string(),
            format!("{}", msg.reservation),
            format!("{}", decision),
        );

        let error_msg1 = format!("Could send message from WEBSERVICE <{}>", self.id);
        let error_msg2 = format!("Could send message from WEBSERVICE <{}> to itself", self.id);

        Box::pin(
            sleep(Duration::from_millis(process_time))
                .into_actor(self)
                .map(move |_result, _me, _ctx| {
                    let reservation = msg.reservation;
                    let elapsed = reservation.alive_timer.elapsed();
                    let result = ReservationResult::from_reservation_ref(
                        reservation,
                        matches!(decision, Decision::Accepted),
                        elapsed,
                    );
                    msg.sender.try_send(result).expect(&error_msg1);
                    _ctx.address().try_send(Finished {}).expect(&error_msg2);
                }),
        )
    }
}

impl Handler<Finished> for Webservice {
    type Result = ();

    fn handle(&mut self, _msg: Finished, _ctx: &mut Self::Context) -> Self::Result {
        self.amount_processing -= 1;
    }
}

impl fmt::Display for Webservice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "WEBSERVICE <{}><processing: {}>",
            self.id, self.amount_processing
        )
    }
}
