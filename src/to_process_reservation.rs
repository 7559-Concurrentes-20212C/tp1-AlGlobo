use crate::reservation::Reservation;
use crate::reservation_result::ReservationResult;
use actix::prelude::*;
use actix::Recipient;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ToProcessReservation {
    pub reservation: Reservation,
    pub sender: Recipient<ReservationResult>,
}
