use crate::decision::Decision;
use crate::reservation_result::ReservationResult;
use crate::to_process_reservation::ToProcessReservation;
use crate::webservice_kind::WebserviceKind;
use actix::{Actor, ActorFutureExt, Context, Handler, ResponseActFuture, WrapFuture};
extern crate rand;
use crate::logger::Logger;
use crate::webservice::rand::Rng;
use actix::clock::sleep;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

pub struct Webservice {
    kind: WebserviceKind,
    success_rate: usize,
    id: usize,
    logger: Arc<Logger>,
}

impl Webservice {
    pub fn new(
        kind: WebserviceKind,
        success_chance: usize,
        id: usize,
        logger: Arc<Logger>,
    ) -> Webservice {
        Webservice {
            kind,
            success_rate: success_chance.min(100),
            id,
            logger,
        }
    }

    fn decide(&self) -> Decision {
        let num = rand::thread_rng().gen_range(0..100);

        if num <= self.success_rate {
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

        let rand_time: u64 = rand::random();
        let process_time: u64 = (rand_time % 1000) as u64;

        self.logger.log_extra_arg(
            format!("{}", self),
            "received reservation".to_string(),
            format!("{}", msg.reservation),
            format!("{}", decision),
        );

        let error_msg = format!("Could send message from WEBSERVICE <{}>", self.id);

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
                        _me.kind,
                    );
                    msg.sender.try_send(result).expect(&error_msg);
                }),
        )
    }
}

impl fmt::Display for Webservice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WEBSERVICE <{}>", self.id)
    }
}
