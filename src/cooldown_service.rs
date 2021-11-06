use std::{thread, time};
use actix::{Actor, Context, Handler};
use crate::reservation_cooldown::CooldownReservation;


pub struct CooldownService {
    retry_wait: u64,

}

impl Actor for CooldownService {
    type Context = Context<Self>;
}

impl CooldownService {
    pub fn new(retry_wait: u64) -> CooldownService {
        return CooldownService { retry_wait }
    }
}
impl Handler<CooldownReservation> for CooldownService{
    type Result = ();

    fn handle(&mut self, msg: CooldownReservation, _ctx: &mut Self::Context)  -> Self::Result {
        let start = msg.requested;
        let retry_wait = self.retry_wait;

        let elapsed = start.elapsed();
        let mut wait = 0;
        if retry_wait as u128 > elapsed.as_millis(){
            wait = retry_wait - elapsed.as_millis() as u64;
        };
        thread::sleep(time::Duration::from_millis(wait.min( retry_wait )));
        msg.caller.try_send(msg.reservation).unwrap_or_else(|_| {
            panic!("COOLDOWN SERVICE: Couldn't send RESERVATION message to SCHEDULER")
        });
    }
}