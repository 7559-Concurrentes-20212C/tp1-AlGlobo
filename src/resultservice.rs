use crate::thread_pool;
use crate::reservation;

use std::sync::Mutex;
use reservation::{ReservationResult};
use thread_pool::{ThreadPool};

pub struct ResultService {
    pub thread_pool : Mutex<ThreadPool>,
}

impl ResultService {

    pub fn new(rate_limit: usize) -> ResultService {
        ResultService {
            thread_pool : Mutex::new(ThreadPool::new(rate_limit)),
        }
    }

    pub fn process_result(&self, id_str : String, accepted : bool) {

        print!("procesando el id: {}", id_str);

        self.thread_pool.lock().unwrap().execute(|| { // TODO validar unwrap
            ReservationResult::new(id_str, true);
        });

    }
}