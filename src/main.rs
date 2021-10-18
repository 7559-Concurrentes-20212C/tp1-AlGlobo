use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::{thread, time};
use std_semaphore::Semaphore;

/* (archivo con de configuracion con rate limit y numero de aerolineas)
/ 1. inicializar web services de aerolineas y hotel (struct)
/ 2. leer pedidos del archivo y spawnear 1 thread (thread cliente) por pedido
/ 3. cada thread cliente invoca el metodo "procesar" del struct aerolinea y se queda esperando a que este retorne
/ 4. el metodo procesar, spawnea internamente un thread que simulara el trabajo a realizar
        controlar rate limit
/ 5. el cliente recibe el valor de retorno del metodo procesar y finaliza
*/

pub struct Flight {
    pub origin: String,
    pub destination: String,
    pub airline: String,
    pub result_gateway: Sender<FlightResult>,
}

pub struct FlightResult {
    pub id: String,
    pub accepted: bool,
}

pub struct Webservice {
    recv: Arc<Mutex<Receiver<Flight>>>,
    send: Arc<Mutex<Sender<Flight>>>,
}

fn process_flight(reservation: Flight) {
    thread::sleep(time::Duration::from_millis(100));
    println!(
        "Processing flight {} to {} from {}",
        reservation.airline, reservation.destination, reservation.origin
    );
    let id_str = format!(
        "{}{}{}",
        reservation.airline.to_owned(),
        reservation.destination,
        reservation.origin
    );
    reservation.result_gateway.send(FlightResult {
        id: id_str,
        accepted: true,
    });
}

impl Webservice {
    pub fn run_webservice(&self) {
        let recver = self.recv.lock().unwrap();
        loop {
            let flight = recver.recv();
            if flight.is_ok() {
                thread::spawn(move || process_flight(flight.unwrap()));
            } else {
                println!("error!");
                break;
            }
        }
    }

    pub fn close_webservice(&self){
        println!("closing webservice");
        //drop(self.send);
    }
}

fn create_webservice(rate_limit: u32) -> Webservice {
    let (wbs_send, wbs_recv) = mpsc::channel();
    Webservice {
        recv: Arc::new(Mutex::new(wbs_recv)),
        send: Arc::new(Mutex::new(wbs_send)),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    const RATE_LIMIT: u32 = 1;

    let filename = &args[0];
    let file = File::open("test.txt").expect("could not open file");
    let reader = BufReader::new(file);

    let mut threads = vec![];
    let flight_ws = Arc::new(create_webservice(RATE_LIMIT));

    let ws_copy = flight_ws.clone();
    threads.push(thread::spawn(move || ws_copy.run_webservice()));

    for line in reader.lines().flatten() {
        let (result_send, result_recv) = mpsc::channel();
        let wbs_message_sender = flight_ws.send.clone();

        let reservation = Flight {
            origin: line,
            destination: "BRC".to_owned(),
            airline: "aerolineas argentinas".to_owned(),
            result_gateway: result_send,
        };
        threads.push(thread::spawn(move || {
            // acquire lock
            let sender = wbs_message_sender.lock().unwrap();
            sender.send(reservation);
            drop(sender);
            //release lock

            let result = result_recv.recv().unwrap();
            println!(
                "Flight {} has been proccessed with result {}",
                result.id, result.accepted
            );
            drop(wbs_message_sender);
        }));
    }

    flight_ws.close_webservice();
    for thread in threads {
        thread
            .join()
            .expect("Couldn't join on the associated thread");
    }
    println!("finished!")
}
