use crate::thread_pool;
use crate::reservation;

use std::sync::{Mutex, Arc};
use reservation::{ReservationResult};
use thread_pool::{ThreadPool};
use crate::stats_service::{StatsService, MovingStats};
use std::fs::File;
use std::io::{Write};

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
            stats.process_result_stats( reservation );
        });

    }

    pub fn print_results_to_screen(&self) -> MovingStats {
        let stats = self.stats.calculate_stats();
        println!("--- STATS ---");
        println!("successful requests {}", stats.sample_size);
        println!("avg latency {}", stats.avg_latency);
        println!("success rate {}", stats.success_rate);
        println!("lowest latency {}", stats.lowest_latency);
        println!("highest latency {}", stats.highest_latency);
        println!(" ");
        println!("--- TOP RANKED AIRLINES ---");
        for i in 0..stats.top_airlines.len().min(10){
            let stats = stats.top_airlines.get(i);
            match stats {
                None => {break;}
                Some(s) => {println!("{}. {} with {} requests", i, s.0.clone(), s.1)}
            }

        }

        println!("--- STATS ---");
        stats
    }

    pub fn print_results_to_file(&self) -> MovingStats {
        let stats = self.stats.calculate_stats();

        let mut file = File::create("stats_results.txt");
        match file {
            Ok(mut file) => {
                file.write(b"--- STATS ---\n");
                file.write(format!("successful requests {}\n", stats.sample_size).as_ref());
                file.write(format!("avg latency {}\n", stats.avg_latency).as_ref());
                file.write(format!("success rate {}\n", stats.success_rate).as_ref());
                file.write(format!("lowest latency {}\n", stats.lowest_latency).as_ref());
                file.write(format!("highest latency {}\n", stats.highest_latency).as_ref());

                file.write(format!("--- TOP RANKED AIRLINES ---\n").as_ref());
                for i in 0..stats.top_airlines.len().min(10){
                    let stats = stats.top_airlines.get(i);
                    match stats {
                        None => {break;}
                        Some(s) => {
                            file.write(format!("{}. {} with {} requests\n", i+1, s.0.clone(), s.1).as_ref());
                        }
                    }

                }

                file.write(b"--- STATS ---\n");

            }
            Err(_) => {return stats}
        }
        stats
    }
}