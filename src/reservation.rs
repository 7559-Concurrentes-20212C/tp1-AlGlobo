use crate::resultservice;

use resultservice::{ResultService}; 
use std::sync::Arc;

pub struct Reservation {
    pub origin: String,
    pub destination: String,
    pub airline: Arc<String>,
    pub reservation_type: String,
    pub result_service: Arc<ResultService>,
}

impl Reservation {
  
    pub fn new(line: String, result_service: Arc<ResultService>) -> Reservation {

        let params = line.split(',').collect::<Vec<&str>>();
    
        Reservation {
            origin: String::from(params[0]),
            destination: String::from(params[1]),
            airline: Arc::new(String::from(params[2])),
            reservation_type: String::from(params[3]),
            result_service: result_service,
        }
    }

}

pub struct ReservationResult {
    pub id: String,
    pub accepted: bool,
}

impl ReservationResult {
    pub fn new(id_str : String, accepted : bool) -> ReservationResult {
        ReservationResult {
            id: id_str,
            accepted: accepted,
        }
    }
}
