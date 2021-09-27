use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::thread;
use std_semaphore::Semaphore;
use std::collections::VecDeque;

/* (archivo con de configuracion con rate limit y numero de aerolineas)
/ 1. inicializar web services de aerolineas y hotel (struct)
/ 2. leer pedidos del archivo y spawnear 1 thread (thread cliente) por pedido
/ 3. cada thread cliente invoca el metodo "procesar" del struct aerolinea y se queda esperando a que este retorne
/ 4. el metodo procesar, spawnea internamente un thread que simulara el trabajo a realizar
        controlar rate limit
/ 5. el cliente recibe el valor de retorno del metodo procesar y finaliza
*/

pub struct Webservice {
    notEmpty: Semaphore,
    notFull: Semaphore,
    requests: Queue<String>
}

pub struct Flight {
    pub origin: String,
    pub destination: String,
    pub airline: String
}

pub struct FlightResult {
    pub flight: Flight,
    pub accepted: bool
}

impl Webservice {
    pub const fn new(rate_limit: u32) -> Webservice {
        Webservice {
            notEmpty: Semaphore(0),
            notFull: Semaphore(rate_limit),
            requests: VecDeque<Flight>::new()
        }
    }

    pub fn run_webservice(&self) {
        loop {
            self.notEmpty.acquire();
            let request = self.requests.pop_front();
            self.consume(request);
            self.notFull.release();
        }
    }

    pub fn consume(&self, request: Flight) -> FlightResult{
        // hacer cosas importantes y utiles
        FlightResult {
            flight: request,
            accepted: true,
        }
    }

    pub fn process(&self, reservation: Flight) -> FlightResult {
        self.notFull.acquire();
        self.requests.append(reservation);
        self.notEmpty.signal();
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();

    const RATE_LIMIT: u32 = 1;

    let flight_ws: Webservice = Webservice::new(RATE_LIMIT);


    let filename = &args[1];
    let file = File::open(filename).expect("could not open file");
    let reader = BufReader::new(file);

    let mut threads = vec![];
    threads.push(thread::spawn(|| { flight_ws.run_webservice() }));


    for line in reader.lines().flatten() {
        let reservation = Flight {
            origin: "APQ",
            destination: "BRC",
            airline: "aerolineas argentinas"
        }
        threads.push(thread::spawn(move |reservation| flight_ws.process(reservation) ));
    }

    for thread in threads {
        thread
            .join()
            .expect("Couldn't join on the associated thread");
    }
}

fn parse(line: String) -> Flight{

}

fn process(text: String) {
    println!("{}", text);
}
