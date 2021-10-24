use std::sync::{Mutex, Arc};
use crate::thread_pool::ThreadPool;
use crate::resultservice::ResultService;
use crate::flight::FlightResult;
use std::collections::VecDeque;

pub struct StatsService {
    pub thread_pool : Mutex<ThreadPool>,
    history: Arc<Mutex<VecDeque<FlightResult>>>,
}

pub struct MovingStats {
    pub sample_size: usize,
    pub success_rate: f32,
    pub avg_latency: f32,
    pub highest_latency: f32,
    pub lowest_latency: f32,
}

impl StatsService {

    pub fn new(rate_limit: usize, moving_avg: usize) -> StatsService {
        let thread_pool = Mutex::new(ThreadPool::new(rate_limit));
        StatsService {
            thread_pool,
            history :  Arc::new((Mutex::new(VecDeque::with_capacity(moving_avg)))),
        }
    }

    pub fn process_result_stats(&self, f :FlightResult) {
        let history = self.history.clone();

        self.thread_pool.lock().expect("could not get thread lock! stats_service").execute(move || {
            let mut h = history.lock().expect("could not acquire history");
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
        let mut highest_latency = 0.0;
        let mut lowest_latency = 0.0;

        for val in self.history.lock().expect("could not load history").iter() {
            sample_size += 1;
            if val.accepted {
                success_rate += 1.0;
            }
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
            success_rate: success_rate/sample_size as f32,
            avg_latency: avg_latency/sample_size as f32,
            highest_latency,
            lowest_latency};
    }
}