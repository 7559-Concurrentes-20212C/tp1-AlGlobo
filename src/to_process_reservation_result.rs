use crate::finished::Finished;
use crate::reservation_result::ReservationResult;
use actix::prelude::*;
use actix::Recipient;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ToProcessReservationResult {
    pub result: ReservationResult,
    pub sender: Recipient<Finished>,
}
