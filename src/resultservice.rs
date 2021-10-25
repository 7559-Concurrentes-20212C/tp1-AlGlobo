use crate::thread_pool;
use crate::reservation;

use std::sync::{Mutex, Arc};
use reservation::{ReservationResult};
use thread_pool::{ThreadPool};
use crate::stats_service::{StatsService, MovingStats};

pub struct ResultService {
    thread_pool : Mutex<ThreadPool>,
    stats: Arc<StatsService>,
}

impl ResultService {

    pub fn new(rate_limit: usize) -> ResultService {
        ResultService {
            thread_pool : Mutex::new(ThreadPool::new(rate_limit)),
            stats: Arc::new(StatsService::new(rate_limit, 1000))
        }
    }

    pub fn process_result(&self, reservation: ReservationResult) {
        let stats = self.stats.clone();
        self.thread_pool.lock().expect("could not acquire thread").execute(move || { // TODO estaria bueno que escriba en un archivo tmb
            print!("processing result with id: {}", reservation.airline);
            stats.process_result_stats( reservation );
        });

    }

    pub fn print_results(&self) {
        let stats = self.stats.calculate_stats();
        println!("sample size {}", stats.sample_size);
        println!("avg latency {}", stats.avg_latency);
        println!("success rate {}", stats.success_rate);
    }
}