use crate::resultservice;


use resultservice::{ResultService}; 
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Sender;
use std::time::{Duration};
use std::sync::{mpsc};
use std::time::{Instant};

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


pub struct ReservationProcessRequest {
    pub reservation: Arc<Reservation>,
    requested: Instant,
}

impl ReservationProcessRequest{
    pub fn new(reservation: Arc<Reservation>, requested: Instant) -> ReservationProcessRequest {
        return ReservationProcessRequest{reservation, requested};
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
    pub fn from_reservation(reservation : Reservation, accepted : bool, delay : Duration) -> ReservationResult {
        ReservationResult {
            origin: reservation.origin,
            destination: reservation.destination,
            airline: reservation.airline,
            accepted,
            time_to_process : delay,
            kind: reservation.kind
        }
    }

    pub fn from_reservation_ref(reservation : Arc<Reservation>, accepted : bool, delay : Duration) -> ReservationResult {
        ReservationResult {
            origin: reservation.origin.clone(),
            destination: reservation.destination.clone(),
            airline: reservation.airline.clone(),
            accepted,
            time_to_process : delay,
            kind: if matches!( reservation.kind, ReservationKind::Flight)
                    {ReservationKind::Flight}
                else
                    {ReservationKind::Package}
        }
    }
}

