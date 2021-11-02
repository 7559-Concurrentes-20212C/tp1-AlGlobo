use crate::moving_stats::MovingStats;
use crate::reservation_result::ReservationResult;
use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct StatsService {
    history: VecDeque<ReservationResult>,
}

impl StatsService {
    pub fn new(moving_avg: usize) -> StatsService {
        StatsService {
            history: VecDeque::with_capacity(moving_avg),
        }
    }

    pub fn process_result_stats(&mut self, f: ReservationResult) {
        if self.history.len() >= self.history.capacity() {
            self.history.pop_back();
        }
        self.history.push_front(f);
    }

    pub fn calculate_stats(&self) -> MovingStats {
        let mut sample_size = 0;
        let mut success_rate = 0.0;
        let mut avg_latency = 0.0;
        let mut highest_latency: f32 = 0.0;
        let mut lowest_latency: f32 = f32::MAX;
        let mut result: HashMap<String, usize> = HashMap::new();

        for val in self.history.iter() {
            if val.accepted {
                *result
                    .entry(format!(
                        "{}->{}",
                        val.reservation.origin.clone(),
                        val.reservation.destination.clone()
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
                sample_size: 0,
                success_rate: 0.0,
                avg_latency: 0.0,
                highest_latency: 0.0,
                lowest_latency: 0.0,
                top_routes: vec![],
            };
        }

        MovingStats {
            sample_size,
            success_rate: success_rate / self.history.len() as f32,
            avg_latency: avg_latency / self.history.len() as f32,
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
