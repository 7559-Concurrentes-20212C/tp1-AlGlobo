
use std::sync::{Arc};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use crate::reservation::{Reservation, ReservationKind};
use crate::webservice_actor::Webservice;
use actix::Actor;
use actix::dev::Request;

pub struct Program {


}

impl Program {
    pub fn new(rate_limit: usize) -> Program {
        return Program{};
    }

    pub fn run(&self, _args: Vec<String>) -> Request<Webservice, Reservation> {
        println!("setting system");
        let ws = Webservice::new(100);
        let addr = ws.start();
        let r = Reservation{
            airline: "AAA".to_string(),
            origin: "BBB".to_string(),
            destination: "CCC".to_string(),
            kind: ReservationKind::Flight
        };
        return addr.send(r);
    }

    pub fn load_services(&mut self, file_name: String) {
        let f = File::open(file_name);
        let file = match f {
            Ok(file) => file,
            Err(error) => {
                println!("problem opening file: {:?}", error);
                return;
            },
        };
        let reader = BufReader::new(file);

        for line in reader.lines().flatten() {
            let params = line.split(',').collect::<Vec<&str>>();
            let capacity = params[1].parse::<usize>().unwrap();
            let rate = params[2].parse::<usize>().unwrap();

        }
    }
}