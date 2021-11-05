use crate::logger::Logger;
use crate::reservation::Reservation;
use crate::resultservice::ResultService;
use crate::schedule_service::ScheduleService;
use crate::webservice::Webservice;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::{mpsc, Arc, Mutex};

pub struct Program {
    results_service: Arc<ResultService>,
    web_services: HashMap<String, Arc<ScheduleService>>,
    hotel: Arc<Webservice>,
    logger: Arc<Logger>,
}

impl Program {
    pub fn new(log_file_name: String) -> Program {
        let logger = Arc::new(Logger::new(log_file_name));
        Program {
            results_service: Arc::new(ResultService::new(100, logger.clone())),
            web_services: HashMap::new(),
            hotel: Arc::new(Webservice::new(0, 100, logger.clone())),
            logger,
        }
    }

    pub fn run(&mut self, _args: Vec<String>) {
        println!("Setting up environment...");

        let reservations: String = String::from("reservations.txt");
        let airlines: String = String::from("valid_airlines.txt");
        let (sender, receiver) = mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));

        // let filename = &args[0];
        let f = File::open(reservations);
        let file = match f {
            Ok(file) => file,
            Err(error) => {
                println!("problem opening file: {:?}", error);
                return;
            }
        };
        let reader = BufReader::new(file);

        //creates all web services
        self.load_services(airlines);

        println!("Set up finished!");
        let mut reqs = 0;
        for line in reader.lines().flatten() {
            let reservation = Arc::new(Reservation::from_line(line, reqs));

            self.logger.log(
                format!("{}", self),
                "parsed reservation".to_string(),
                format!("{}", reservation),
            );

            let scheduler = match self.web_services.get(&*reservation.airline) {
                None => {
                    println!("invalid airline reservation: {}", reservation.airline);
                    continue;
                }
                Some(s) => s,
            };

            reqs += 1;
            scheduler.schedule_to_process( scheduler.clone(), reservation, sender.clone());
        }
        println!("finished scheduling reservations!");
        while reqs > 0 {
            receiver.recv().expect("could not read from channel");
            reqs -= 1;
        }
    }

    pub fn print_results(&self) {
        self.results_service.log_results()
    }

    pub fn load_services(&mut self, file_name: String) {
        let f = File::open(file_name);
        let file = match f {
            Ok(file) => file,
            Err(error) => {
                println!("problem opening file: {:?}", error);
                return;
            }
        };
        let reader = BufReader::new(file);

        let mut id = 1;
        for line in reader.lines().flatten() {
            let params = line.split(',').collect::<Vec<&str>>();
            let capacity = params[1]
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("PROGRAM: INTERNAL ERROR WHILE PARSING FILE"));
            let acceptance_rate = params[2]
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("PROGRAM: INTERNAL ERROR WHILE PARSING FILE"));
            let retry_wait = params[3]
                .parse::<u64>()
                .unwrap_or_else(|_| panic!("PROGRAM: INTERNAL ERROR WHILE PARSING FILE"));
            let webservice = Arc::new(Webservice::new(id, acceptance_rate, self.logger.clone()));
            self.web_services.insert(
                params[0]
                    .parse()
                    .unwrap_or_else(|_| panic!("PROGRAM: INTERNAL ERROR WHILE PARSING FILE")),
                Arc::new(ScheduleService::new(
                    id,
                    capacity,
                    retry_wait,
                    webservice,
                    self.hotel.clone(),
                    self.results_service.clone(),
                    self.logger.clone(),
                )),
            );
            id += 1;
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PROGRAM")
    }
}
