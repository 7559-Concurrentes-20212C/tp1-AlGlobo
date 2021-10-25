use std::sync::{Mutex, Arc, RwLock};
use crate::thread_pool::ThreadPool;
use std::collections::VecDeque;
use crate::reservation::ReservationResult;

pub struct StatsService {
    thread_pool : Mutex<ThreadPool>,
    history: Arc<RwLock<VecDeque<ReservationResult>>>,
}

//its called moving stats because it return stats for a moving window of max size history.capacity
pub struct MovingStats {
    pub sample_size: usize,
    pub success_rate: f32,
    pub avg_latency: f32,
    pub highest_latency: f32,
    pub lowest_latency: f32,
}

impl StatsService {

    pub fn new(rate_limit: usize, moving_avg: usize) -> StatsService {
        let thread_pool = ThreadPool::new(rate_limit);
        StatsService {
            thread_pool : Mutex::new(thread_pool),
            history :  Arc::new(RwLock::new(VecDeque::with_capacity(moving_avg))),
        }
    }

    pub fn process_result_stats(&self, f :ReservationResult) {
        let history = self.history.clone();

        self.thread_pool.lock().expect("poisoned lock").execute(move || {
            let mut h = history.write().expect("could not acquire history");
            if h.len() >= h.capacity(){
                h.pop_back();
            }
            h.push_front(f);
        });
    }

    pub fn calculate_stats(&self) -> MovingStats{
        let mut sample_size = 0;
        let mut success_rate = 0.0;
        let mut avg_latency = 0.0;
        let mut highest_latency :f32 = 0.0;
        let mut lowest_latency :f32 = f32::MAX;
        let history = self.history.read().expect("could not load history");

        for val in history.iter() {
            if val.accepted {
                success_rate += 1.0;
                sample_size += 1;
            }
            avg_latency += val.time_to_process.as_secs_f32();
            highest_latency = highest_latency.max(val.time_to_process.as_secs_f32());
            lowest_latency = lowest_latency.min(val.time_to_process.as_secs_f32())
        }
        if sample_size == 0{
            return MovingStats{
                sample_size: 0,
                success_rate: 0.0,
                avg_latency: 0.0,
                highest_latency: 0.0,
                lowest_latency: 0.0,};
        }

        return MovingStats{
            sample_size,
            success_rate: success_rate/history.len() as f32,
            avg_latency: avg_latency/history.len() as f32,
            highest_latency,
            lowest_latency};
    }
}