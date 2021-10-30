use actix::{Actor, ActorFutureExt, Context, Handler, ResponseActFuture, WrapFuture};
use crate::reservation::{Reservation, ReservationResult};
extern crate rand;
use std::time::Duration;
use actix::clock::sleep;

enum Decision {
    Accepted,
    Rejected,
}

pub struct Webservice{
    success_rate: usize,
}

impl Webservice{
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

}

impl Actor for Webservice{
    type Context = Context<Self>;
}

impl Handler<Reservation> for Webservice {
    type Result = ResponseActFuture<Self, ReservationResult>;

    fn handle(&mut self, msg: Reservation, _ctx: &mut Self::Context) -> Self::Result {
        let decision = self.decide();



        let i: i32 = rand::random();
        Box::pin(sleep(Duration::from_millis(i as u64 % 1000))
        .into_actor(self)
        .map(move |_result, me, _ctx| {

            return ReservationResult::from_reservation_ref(msg,
                matches!(decision , Decision::Accepted),
                msg.liveness_cronometer.elapsed());
        }))
    }
}