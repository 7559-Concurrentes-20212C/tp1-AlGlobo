use crate::reservation_kind::ReservationKind;
use std::fmt;

pub struct Reservation {
    pub id: usize,
    pub origin: String,
    pub destination: String,
    pub airline: String,
    pub kind: ReservationKind,
}

impl Reservation {
    pub fn from_line(line: String, id: usize) -> Reservation {
        let params = line.split(',').collect::<Vec<&str>>();

        Reservation {
            id,
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

impl Clone for Reservation {
    fn clone(&self) -> Reservation {
        Reservation {
            id: self.id, //its important for it to have the same id
            origin: self.origin.clone(),
            destination: self.destination.clone(),
            airline: self.airline.clone(),
            kind: self.kind.clone(),
        }
    }
}

impl fmt::Display for Reservation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<{}>({}|{}-{}|{})",
            self.id, self.airline, self.origin, self.destination, self.kind
        )
    }
}
