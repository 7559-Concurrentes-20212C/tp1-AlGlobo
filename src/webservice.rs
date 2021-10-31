use actix::{Actor, ActorFutureExt, Context, Handler, ResponseActFuture, WrapFuture};
use crate::reservation::{ReservationResult, ToProcessReservation};
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

impl Handler<ToProcessReservation> for Webservice {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: ToProcessReservation, _ctx: &mut Self::Context) -> Self::Result {
        let decision = self.decide();

        let i: i32 = rand::random();
        Box::pin(sleep(Duration::from_millis(i as u64 % 1000))
        .into_actor(self)
        .map(move |_result, _me, _ctx| {

            let reservation = msg.reservation;
            let elapsed = reservation.alive_timer.elapsed();
            let result = ReservationResult::from_reservation_ref(reservation, matches!(decision , Decision::Accepted), elapsed);
            msg.sender.try_send(result);
        }))
    }
}