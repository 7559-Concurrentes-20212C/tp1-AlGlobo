use crate::moving_stats::MovingStats;
use crate::reservation::ReservationResult;
use crate::thread_pool::ThreadPool;
use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, RwLock};

pub struct StatsService {
    thread_pool: Mutex<ThreadPool>,
    history: Arc<RwLock<VecDeque<ReservationResult>>>,
}

impl StatsService {
    pub fn new(rate_limit: usize, moving_avg: usize) -> StatsService {
        let thread_pool = ThreadPool::new(rate_limit);
        StatsService {
            thread_pool: Mutex::new(thread_pool),
            history: Arc::new(RwLock::new(VecDeque::with_capacity(moving_avg))),
        }
    }

    pub fn process_result_stats(&self, f: ReservationResult) {
        let history = self.history.clone();

        self.thread_pool
            .lock()
            .expect("poisoned lock")
            .execute(move || {
                let mut h = history.write().expect("could not acquire history");
                if h.len() >= h.capacity() {
                    h.pop_back();
                }
                h.push_front(f);
            });
    }

    pub fn calculate_stats(&self) -> MovingStats {
        let mut sample_size = 0;
        let mut success_rate = 0.0;
        let mut avg_latency = 0.0;
        let mut highest_latency: f32 = 0.0;
        let mut lowest_latency: f32 = f32::MAX;
        let history = self.history.read().expect("could not load history");
        let mut result: HashMap<String, usize> = HashMap::new();

        for val in history.iter() {
            if val.accepted {
                *result
                    .entry(format!(
                        "{}->{}",
                        val.origin.clone(),
                        val.destination.clone()
                    ))
                    .or_insert(0) += 1;
                success_rate += 1.0;
                sample_size += 1;
            }
            avg_latency += val.time_to_process.as_secs_f32();
            highest_latency = highest_latency.max(val.time_to_process.as_secs_f32());
            lowest_latency = lowest_latency.min(val.time_to_process.as_secs_f32())
        }
        if sample_size == 0 {
            return MovingStats {
                successful: 0,
                failed: 0,
                success_rate: 0.0,
                avg_latency: 0.0,
                highest_latency: 0.0,
                lowest_latency: 0.0,
                top_routes: vec![],
            };
        }

        MovingStats {
            successful: sample_size,
            failed: (history.len() as u32 - sample_size) as u32,
            success_rate: success_rate / history.len() as f32,
            avg_latency: avg_latency / history.len() as f32,
            highest_latency,
            lowest_latency,
            top_routes: get_ranking(result.into_iter()),
        }
    }
}

fn get_ranking(count_iter: IntoIter<String, usize>) -> Vec<(String, usize)> {
    let mut count_vec: Vec<(String, usize)> = count_iter.collect();
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    count_vec
}
