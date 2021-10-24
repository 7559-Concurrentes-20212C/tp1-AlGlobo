use crate::thread_pool;
use crate::flight;

use std::sync::Mutex;
use flight::{FlightResult};
use thread_pool::{ThreadPool};
use crate::stats_service::{StatsService, MovingStats};

pub struct ResultService {
    pub thread_pool : Mutex<ThreadPool>,
    stats: StatsService,
}

impl ResultService {

    pub fn new(rate_limit: usize) -> ResultService {
        let thread_pool = Mutex::new(ThreadPool::new(rate_limit));
        ResultService {
            thread_pool,
            stats: StatsService::new(rate_limit, 1000)
        }
    }

    pub fn process_result(&self, id_str : String, accepted : bool) {

        self.thread_pool.lock().unwrap().execute(|| {
            print!("procesando el id: {}", id_str);

            build_result(id_str, true);
        });

    }

    pub fn get_stats_history(&self) -> MovingStats {
        return self.stats.calculate_stats();
    }
}

fn build_result(id_str : String, accepted : bool) -> FlightResult {
    FlightResult {
        id: id_str,
        accepted: accepted,
    }
}





