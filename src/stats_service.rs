use std::collections::VecDeque;
use crate::reservation::ReservationResult;

pub struct StatsService {
    history: VecDeque<ReservationResult>,
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

    pub fn new(moving_avg: usize) -> StatsService {
        StatsService {
            history :  VecDeque::with_capacity(moving_avg),
        }
    }

    pub fn process_result_stats(&mut self, f :ReservationResult) {

        if self.history.len() >= self.history.capacity(){
            self.history.pop_back();
        }
        self.history.push_front(f);
    }

    pub fn calculate_stats(&self) -> MovingStats{
        let mut sample_size = 0;
        let mut success_rate = 0.0;
        let mut avg_latency = 0.0;
        let mut highest_latency :f32 = 0.0;
        let mut lowest_latency :f32 = f32::MAX;

        for val in self.history.iter() {
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
            success_rate: success_rate/self.history.len() as f32,
            avg_latency: avg_latency/self.history.len() as f32,
            highest_latency,
            lowest_latency};
    }
}