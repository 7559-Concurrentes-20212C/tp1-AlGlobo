use crate::resultservice;

use resultservice::{ResultService}; 
use std::sync::Arc;
use std::time::{Duration};

pub enum ReservationKind {
    Flight,
    Package,
}

pub struct Reservation {
    pub origin: String,
    pub destination: String,
    pub airline: String,
    pub kind: ReservationKind,
    pub result_service: Arc<ResultService>,
}

impl Reservation {
  
    pub fn from_line(line: String, result_service: Arc<ResultService>) -> Reservation {

        let params = line.split(',').collect::<Vec<&str>>();
    
        Reservation {
            origin: String::from(params[0]),
            destination: String::from(params[1]),
            airline: String::from(params[2]),
            kind: match params[3] {
                "flight" => ReservationKind::Flight,
                _ => ReservationKind::Package,
            },
            result_service,
        }
    }

}

pub struct ReservationResult {
    pub id: String,
    pub accepted: bool,
    pub time_to_process: Duration,
}

impl ReservationResult {
    pub fn new(id_str : String, accepted : bool, delay : Duration) -> ReservationResult {
        ReservationResult {
            id: id_str,
            accepted,
            time_to_process : delay
        }
    }
}
