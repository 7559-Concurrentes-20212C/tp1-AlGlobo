
use std::sync::{Arc};
use actix::{Addr, Actor, Context, Handler, AsyncContext};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use crate::schedule_service::ScheduleService;
use crate::resultservice::{ResultService};
use crate::webservice::Webservice;
use crate::messages::{Stats, Reservation, Finished, Run};

pub struct Program {
    results_service : Arc<Addr<ResultService>>,
    schedule_services : HashMap<String, Addr<ScheduleService>>,
    hotel : Arc<Addr<Webservice>>,
    rate : usize,
    amount_to_process : usize,
    processed : usize,
}

impl Program {
    pub fn new(rate : usize) -> Program {
        return Program{
            results_service: Arc::new(ResultService::new().start()),
            schedule_services: HashMap::new(),
            hotel:  Arc::new(Webservice::new(100, 0).start()),
            rate : rate,
            amount_to_process: 0,
            processed: 0,
        }
    }

    pub fn load_services(&mut self, file_name: String, caller_addr: Arc<Addr<Program>>) {
        let f = File::open(file_name);
        let file = match f {
            Ok(file) => file,
            Err(error) => {
                println!("problem opening file: {:?}", error);
                return;
            },
        };
        let reader = BufReader::new(file);

        let mut i = 1;
        for line in reader.lines().flatten() {
            let params = line.split(',').collect::<Vec<&str>>();
            let capacity = params[1].parse::<usize>().unwrap();
            let rate = params[2].parse::<usize>().unwrap();

            let schedule_service_addr = ScheduleService::new(capacity, rate, 
                                                            self.hotel.clone(), self.results_service.clone(),
                                                            caller_addr.clone(), i).start();

            self.schedule_services.insert(params[0].parse().unwrap(), schedule_service_addr);

            i = i + 1;
        }
    }

    pub fn finish(&self) {
        println!("PROGRAM: finished processing all {} reservations", self.amount_to_process);
        self.results_service.try_send(Stats {});
    }
}

impl Actor for Program{
    type Context = Context<Self>;
}

impl Handler<Run> for Program {
    type Result = ();

    fn handle(&mut self, msg: Run, _ctx: &mut Self::Context)  -> Self::Result {
        
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
        

        let mut lines = vec!();
        for line in BufReader::new(file).lines().flatten() {
            self.amount_to_process = self.amount_to_process + 1;
            lines.push(line);
        }
        
        //creates all web services
        self.load_services(airlines, Arc::new(_ctx.address()));
    
        println!("Set up finished!");
    
        let mut i = 1;
        for line in lines {
            let reservation = Reservation::from_line(line, i);
            println!("PROGRAM: parsed reservation <{}>({}|{}-{}|{})", i, reservation.airline, reservation.origin, reservation.destination, reservation.kind);
    
            let scheduler = match self.schedule_services.get(&*reservation.airline) {
                None => {
                    println!("invalid airline reservation: {}", reservation.airline);
                    continue
                }
                Some(s) => { &*s }
            };
            scheduler.try_send(reservation);
            i = i + 1;
        }
    }
}

impl Handler<Finished> for Program {
    type Result = ();

    fn handle(&mut self, msg: Finished, _ctx: &mut Self::Context)  -> Self::Result {

        self.processed = self.processed + 1;

        if self.processed == self.amount_to_process {
            self.finish();
        }
    }
}