use std::time::{Duration, Instant};
use actix::prelude::*;
use actix::{Recipient};

#[derive(Clone)]
pub enum ReservationKind {
    Flight,
    Package,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Reservation {
    pub origin: String,
    pub destination: String,
    pub airline: String,
    pub kind: ReservationKind,
    pub alive_timer: Instant,
    pub max_attempts : u32,
    pub current_attempt_num : u32,
}

impl Clone for Reservation {
    fn clone(&self) -> Reservation {
        Reservation {
            origin: self.origin.clone(),
            destination: self.destination.clone(),
            airline: self.airline.clone(),
            kind: self.kind.clone(),
            alive_timer: self.alive_timer.clone(),
            max_attempts: self.max_attempts,
            current_attempt_num: self.current_attempt_num,
        }
    }
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
            alive_timer: Instant::now(),
            max_attempts: 10,
            current_attempt_num : 1,
        }
    }

}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ReservationResult {
    pub reservation: Reservation,
    pub accepted: bool,
    pub time_to_process: Duration,
}

impl ReservationResult {

    pub fn from_reservation_ref(reservation : Reservation, accepted : bool, delay : Duration) -> ReservationResult {
        ReservationResult {
            reservation,
            accepted,
            time_to_process : delay,
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ToProcessReservation {
    pub reservation: Reservation,
    pub sender: Recipient<ReservationResult>,
}
