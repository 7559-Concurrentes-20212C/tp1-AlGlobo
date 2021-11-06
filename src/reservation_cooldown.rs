use actix::clock::Instant;
use crate::reservation::Reservation;
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct CooldownReservation{
    pub caller:  Recipient<Reservation>,
    pub reservation: Reservation,
    pub requested: Instant
}

impl Clone for CooldownReservation {
    fn clone(&self) -> CooldownReservation {
        CooldownReservation {
            caller: self.caller.clone(),
            reservation: self.reservation.clone(),
            requested: self.requested.clone()
        }
    }
}