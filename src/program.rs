
use std::sync::{Arc};
use actix::{Addr, Actor};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use crate::schedule_service::ScheduleService;
use crate::resultservice::ResultService;
use crate::webservice::Webservice;
use crate::reservation::Reservation;

pub struct Program {
    results_service : Arc<Addr<ResultService>>,
    schedule_services : HashMap<String, Addr<ScheduleService>>,
    hotel : Arc<Addr<Webservice>>,
    rate : usize,
}

impl Program {
    pub fn new(rate : usize) -> Program {
        return Program{
            results_service: Arc::new(ResultService::new().start()),
            schedule_services: HashMap::new(),
            hotel:  Arc::new(Webservice::new(100).start()),
            rate : rate,
        }
    }

    pub fn run(&mut self, _args: Vec<String>){
        println!("Setting up environment...");

        let reservations: String = String::from("reservations.txt");
        let airlines: String = String::from("valid_airlines.txt");

        // let filename = &args[0];
        let f = File::open(reservations);
        let file = match f {
            Ok(file) => file,
            Err(error) => {
                println!("problem opening file: {:?}", error);
                return;
            },
        };
        let reader = BufReader::new(file);

        //creates all web services
        self.load_services(airlines);

        println!("Set up finished!");

        for line in reader.lines().flatten() {
            let reservation = Reservation::from_line(line);
            let scheduler = match self.schedule_services.get(&*reservation.airline) {
                None => {
                    println!("invalid airline reservation: {}", reservation.airline);
                    continue
                }
                Some(s) => { &*s }
            };
            scheduler.try_send(reservation);
        }
        println!("finished scheduling reservations!");
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

            let schedule_service_addr = ScheduleService::new(capacity, rate, self.hotel.clone(), self.results_service.clone()).start();

            self.schedule_services.insert(params[0].parse().unwrap(), schedule_service_addr);
        }
    }
}