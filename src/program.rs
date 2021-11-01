use crate::messages::{Finished, Reservation, Run, Stats};
use crate::resultservice::ResultService;
use crate::schedule_service::ScheduleService;
use crate::webservice::Webservice;
use actix::{Actor, Addr, AsyncContext, Context, Handler};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::Arc;
use std::fs;

pub struct Program {
    results_service: Arc<Addr<ResultService>>,
    schedule_services: HashMap<String, Addr<ScheduleService>>,
    hotel: Arc<Addr<Webservice>>,
    rate: usize,
    amount_to_process: usize,
    processed: usize,
    log_file_name: String,
}

impl Program {
    pub fn new(rate: usize, log_file_name: String) -> Program {
        let cpy1_log_file_name = String::from(&log_file_name);
        let cpy2_log_file_name = String::from(&log_file_name);
        Program {
            results_service: Arc::new(ResultService::new(cpy1_log_file_name).start()),
            schedule_services: HashMap::new(),
            hotel: Arc::new(Webservice::new(100, 0, cpy2_log_file_name).start()),
            rate,
            amount_to_process: 0,
            processed: 0,
            log_file_name,
        }
    }

    pub fn load_services(&mut self, file_name: String, caller_addr: Arc<Addr<Program>>) {
        let f = File::open(file_name);
        let file = match f {
            Ok(file) => file,
            Err(error) => {
                println!("problem opening file: {:?}", error);
                return;
            }
        };
        let reader = BufReader::new(file);

        let mut i = 1;
        for line in reader.lines().flatten() {
            let params = line.split(',').collect::<Vec<&str>>();
            let capacity = params[1].parse::<usize>().unwrap_or_else(|_| panic!("PROGRAM: INTERNAL ERROR"));
            let rate = params[2].parse::<usize>().unwrap_or_else(|_| panic!("PROGRAM: INTERNAL ERROR"));

            let schedule_service_addr = ScheduleService::new(
                capacity,
                rate,
                self.hotel.clone(),
                self.results_service.clone(),
                caller_addr.clone(),
                i,
                String::from(&self.log_file_name),
            )
            .start();

            self.schedule_services
                .insert(params[0].parse().unwrap_or_else(|_| panic!("PROGRAM: INTERNAL ERROR")),
                schedule_service_addr);

            i += 1;
        }
    }

    pub fn finish(&self) {
        println!(
            "PROGRAM: finished processing all {} reservations",
            self.amount_to_process
        );
        self.results_service.try_send(Stats {}).unwrap_or_else(|_| panic!("PROGRAM: Couldn't send STATS message to RESULT SERVICE"));
    }
}

impl Actor for Program {
    type Context = Context<Self>;
}

impl Handler<Run> for Program {
    type Result = ();

    fn handle(&mut self, _msg: Run, _ctx: &mut Self::Context) -> Self::Result {
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
            }
        };

        let mut lines = vec![];
        for line in BufReader::new(file).lines().flatten() {
            self.amount_to_process += 1;
            lines.push(line);
        }

        //creates all web services
        self.load_services(airlines, Arc::new(_ctx.address()));

        println!("Set up finished!");

        let mut i = 1;
        for line in lines {
            let reservation = Reservation::from_line(line, i);

            let to_log = format!("PROGRAM: parsed reservation <{}>({}|{}-{}|{})",
            i,
            reservation.airline,
            reservation.origin,
            reservation.destination,
            reservation.kind
            );
            fs::write(String::from(&self.log_file_name), to_log).unwrap_or_else(|_| panic!("PROGRAM: Couldn't write to log"));
            println!("PROGRAM: parsed reservation <{}>({}|{}-{}|{})",
            i,
            reservation.airline,
            reservation.origin,
            reservation.destination,
            reservation.kind
            );

            let scheduler = match self.schedule_services.get(&*reservation.airline) {
                None => {
                    println!("invalid airline reservation: {}", reservation.airline);
                    continue;
                }
                Some(s) => &*s,
            };
            scheduler.try_send(reservation).unwrap_or_else(|_| panic!("PROGRAM: Couldn't send RESERVATION message to SCHEDULER"));
            i += 1;
        }
    }
}

impl Handler<Finished> for Program {
    type Result = ();

    fn handle(&mut self, _msg: Finished, _ctx: &mut Self::Context) -> Self::Result {
        self.processed += 1;

        if self.processed == self.amount_to_process {
            self.finish();
        }
    }
}
