use crate::reservation::Reservation;
use crate::webservice_kind::WebserviceKind;
use actix::prelude::*;
use std::fmt;
use std::time::Duration;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ReservationResult {
    pub reservation: Reservation,
    pub accepted: bool,
    pub time_to_process: Duration,
    pub creator: WebserviceKind,
}

impl ReservationResult {
    pub fn from_reservation_ref(
        reservation: Reservation,
        accepted: bool,
        delay: Duration,
        creator: WebserviceKind,
    ) -> ReservationResult {
        ReservationResult {
            reservation,
            accepted,
            time_to_process: delay,
            creator,
        }
    }
}

impl Clone for ReservationResult {
    fn clone(&self) -> ReservationResult {
        ReservationResult {
            reservation: self.reservation.clone(),
            accepted: self.accepted,
            time_to_process: self.time_to_process,
            creator: self.creator,
        }
    }
}

impl fmt::Display for ReservationResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<{}>({}|{}-{}|{}|{}|{})",
            self.reservation.id,
            self.reservation.airline,
            self.reservation.origin,
            self.reservation.destination,
            self.reservation.kind,
            self.accepted,
            self.creator,
        )
    }
}
