use crate::thread_pool;

use crate::logger::Logger;
use crate::ranked_route_entry::RankedRouteEntry;
use crate::reservation_result::ReservationResult;
use crate::stats_service::StatsService;
use std::fmt;
use std::sync::{Arc, Mutex};
use thread_pool::ThreadPool;

pub struct ResultService {
    thread_pool: Mutex<ThreadPool>,
    stats: Arc<StatsService>,
    logger: Arc<Logger>,
}

impl ResultService {
    pub fn new(rate_limit: u32, logger: Arc<Logger>) -> ResultService {
        ResultService {
            thread_pool: Mutex::new(ThreadPool::new(rate_limit as usize)),
            stats: Arc::new(StatsService::new(rate_limit as usize, 1000)),
            logger,
        }
    }

    pub fn process_result(&self, result: ReservationResult) {
        let stats = self.stats.clone();

        self.logger.log(
            format!("{}", self),
            "received result".to_string(),
            format!("{}", result),
        );

        self.thread_pool
            .lock()
            .expect("could not acquire thread")
            .execute(move || {
                stats.process_result_stats(result);
            });
    }

    pub fn log_results(&self) {
        let stats = self.stats.calculate_stats();

        self.logger
            .log("".to_string(), "".to_string(), "".to_string());
        self.logger
            .log("".to_string(), "--- STATS ---".to_string(), "".to_string());
        self.logger.log(
            "".to_string(),
            "succesful".to_string(),
            format!("{}", stats.successful),
        );
        self.logger.log(
            "".to_string(),
            "failed".to_string(),
            format!("{}", stats.failed),
        );
        self.logger.log(
            "".to_string(),
            "avg latency".to_string(),
            format!("{}", stats.avg_latency),
        );
        self.logger.log(
            "".to_string(),
            "success rate".to_string(),
            format!("{}", stats.success_rate),
        );
        self.logger.log(
            "".to_string(),
            "lowest latency".to_string(),
            format!("{}", stats.lowest_latency),
        );
        self.logger.log(
            "".to_string(),
            "highest latency".to_string(),
            format!("{}", stats.highest_latency),
        );
        self.logger
            .log("".to_string(), "--- STATS ---".to_string(), "".to_string());
        self.logger
            .log("".to_string(), "".to_string(), "".to_string());
        self.logger.log(
            "".to_string(),
            "--- TOP RANKED ROUTES ---".to_string(),
            "".to_string(),
        );
        for i in 0..stats.top_routes.len().min(10) {
            let stats = stats.top_routes.get(i);
            match stats {
                None => {
                    break;
                }

                Some(s) => {
                    self.logger.log(
                        "".to_string(),
                        "".to_string(),
                        format!(
                            "{}",
                            RankedRouteEntry {
                                rank: i,
                                route: s.0.clone(),
                                count: s.1
                            }
                        ),
                    );
                }
            }
        }
    }
}

impl fmt::Display for ResultService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RESULT SERVICE")
    }
}
