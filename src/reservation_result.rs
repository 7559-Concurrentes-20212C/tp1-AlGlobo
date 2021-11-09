use crate::reservation::Reservation;
use std::fmt;
use std::time::Duration;

pub struct ReservationResult {
    pub reservation: Reservation,
    pub accepted: bool,
    pub time_to_process: Duration,
}

impl Clone for ReservationResult {
    fn clone(&self) -> ReservationResult {
        ReservationResult {
            reservation: self.reservation.clone(),
            accepted: self.accepted,
            time_to_process: self.time_to_process,
        }
    }
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

    pub fn mix(r1: ReservationResult, r2: ReservationResult) -> ReservationResult {
        let duration = Duration::from_secs_f32(
            r1.time_to_process
                .as_secs_f32()
                .max(r2.time_to_process.as_secs_f32()),
        );

        ReservationResult::from_reservation_ref(
            r1.reservation.clone(),
            r1.accepted && r2.accepted,
            duration,
        )
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
