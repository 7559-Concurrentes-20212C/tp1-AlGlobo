use crate::thread_pool;
use crate::flight;

use std::sync::Mutex;
use flight::{FlightResult};
use thread_pool::{ThreadPool};

pub struct ResultService {
    pub thread_pool : Mutex<ThreadPool>,
}

impl ResultService {

    pub fn new(rate_limit: usize) -> ResultService {
        let thread_pool = Mutex::new(ThreadPool::new(rate_limit));
        ResultService {
            thread_pool : thread_pool,
        }
    }

    pub fn process_result(&self, id_str : String, accepted : bool) {

        print!("procesando el id: {}", id_str);

        self.thread_pool.lock().unwrap().execute(|| {
            build_result(id_str, true);
        });

    }
}

fn build_result(id_str : String, accepted : bool) -> FlightResult {
    FlightResult {
        id: id_str,
        accepted: accepted,
    }
}





