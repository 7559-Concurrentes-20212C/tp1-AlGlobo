use crate::thread_pool;
use crate::reservation;

use std::sync::Mutex;
use reservation::{ReservationResult};
use thread_pool::{ThreadPool};
use crate::stats_service::{StatsService, MovingStats};

pub struct ResultService {
    pub thread_pool : Mutex<ThreadPool>,
    stats: StatsService,
}

impl ResultService {

    pub fn new(rate_limit: usize) -> ResultService {
        ResultService {
            thread_pool : Mutex::new(ThreadPool::new(rate_limit)),
            stats: StatsService::new(rate_limit, 1000)
        }
    }

    pub fn process_result(&self, id_str : String, accepted : bool) {

        print!("procesando el id: {}", id_str);

        self.thread_pool.lock().unwrap().execute(|| { // TODO validar unwrap
            ReservationResult::new(id_str, true);
        });

    }

    pub fn get_stats_history(&self) -> MovingStats {
        return self.stats.calculate_stats();
    }
}