use crate::reservation::Reservation;
use actix::prelude::*;
use std::fmt;
use std::time::Duration;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ReservationResult {
    pub reservation: Reservation,
    pub accepted: bool,
    pub time_to_process: Duration,
}

impl ReservationResult {
    pub fn from_reservation_ref(
        reservation: Reservation,
        accepted: bool,
        delay: Duration,
    ) -> ReservationResult {
        ReservationResult {
            reservation,
            accepted,
            time_to_process: delay,
        }
    }
}

impl fmt::Display for ReservationResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<{}>({}|{}-{}|{}|{})",
            self.reservation.id,
            self.reservation.airline,
            self.reservation.origin,
            self.reservation.destination,
            self.reservation.kind,
            self.accepted
        )
    }
}
