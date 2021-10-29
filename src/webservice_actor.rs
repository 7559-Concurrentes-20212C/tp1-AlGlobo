use actix::{Actor, Context, Handler};
use crate::reservation::{Reservation};
use std::{thread, time};


pub struct Webservice{
    success_rate: usize,
}

impl Webservice{
    pub fn new(success_chance: usize) -> Webservice {
        Webservice {
            success_rate: success_chance.min(100),
        }
    }

    fn decide(&self) -> bool{
        let i: i32 = rand::random();
        if self.success_rate > 0 && (i % 100000) <= (self.success_rate * 1000)  as i32 {
            return true;
        }
        return false;
    }

}

impl Actor for Webservice{
    type Context = Context<Self>;
}

impl Handler<Reservation> for Webservice {
    type Result = bool;

    fn handle(&mut self, msg: Reservation, _ctx: &mut Self::Context) -> Self::Result {
        random_wait();
        return self.decide();
    }
}

fn wait(miliseconds: u64){
    thread::sleep(time::Duration::from_millis(miliseconds));
}

fn random_wait() {
    let i: i32 = rand::random();
    wait(i as u64 % 1000);
}