use std::sync::Arc;
use std::time::Duration;

pub enum ReservationKind {
    Flight,
    Package,
}

pub struct Reservation {
    pub origin: String,
    pub destination: String,
    pub airline: String,
    pub kind: ReservationKind,
}

impl Reservation {
    pub fn from_line(line: String) -> Reservation {
        let params = line.split(',').collect::<Vec<&str>>();

        Reservation {
            origin: String::from(params[0]),
            destination: String::from(params[1]),
            airline: String::from(params[2]),
            kind: match params[3] {
                "flight" => ReservationKind::Flight,
                _ => ReservationKind::Package,
            },
        }
    }
}

pub struct ReservationResult {
    pub origin: String,
    pub destination: String,
    pub airline: String,
    pub kind: ReservationKind,
    pub accepted: bool,
    pub time_to_process: Duration,
}

impl ReservationResult {
    pub fn from_reservation_ref(
        reservation: Arc<Reservation>,
        accepted: bool,
        delay: Duration,
    ) -> ReservationResult {
        ReservationResult {
            origin: reservation.origin.clone(),
            destination: reservation.destination.clone(),
            airline: reservation.airline.clone(),
            accepted,
            time_to_process: delay,
            kind: if matches!(reservation.kind, ReservationKind::Flight) {
                ReservationKind::Flight
            } else {
                ReservationKind::Package
            },
        }
    }
}
