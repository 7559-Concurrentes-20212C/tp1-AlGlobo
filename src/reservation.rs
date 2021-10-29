use crate::webservice_actor::Webservice;
use actix::prelude::*;

pub enum ReservationKind {
    Flight,
    Package,
}

#[derive(Message)]
#[rtype(result = "bool")]
pub struct Reservation {
    pub airline: String,
    pub origin: String,
    pub destination: String,
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
