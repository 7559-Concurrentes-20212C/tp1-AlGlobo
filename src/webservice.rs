use crate::flight;

pub struct Webservice {
    pub thread_pool : ThreadPool,
}

impl Webservice {

    pub fn new(rate_limit: u32) -> Webservice {
        Webservice {
            thread_pool : ThreadPool::new(rate_limit),
        }
    }

    pub process(reservation: Flight){
        thread_pool.execute(|| {
            _process(reservation);
        })
    }

    fn _process(reservation: Flight) {
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
}