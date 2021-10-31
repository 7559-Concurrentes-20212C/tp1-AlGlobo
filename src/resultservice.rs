use crate::reservation;
use actix::{Actor, Context, Handler};
use reservation::{ReservationResult};
use crate::stats_service::{StatsService, MovingStats};

pub struct ResultService {
    stats: StatsService,
}

impl ResultService {

    pub fn new() -> ResultService {
        ResultService {
            stats: StatsService::new(1000),
        }
    }

    pub fn print_results(&self) -> MovingStats {
        let stats = self.stats.calculate_stats();
        println!("--- STATS ---");
        println!("sample size {}", stats.sample_size);
        println!("avg latency {}", stats.avg_latency);
        println!("success rate {}", stats.success_rate);
        println!("lowest latency {}", stats.lowest_latency);
        println!("highest latency {}", stats.highest_latency);
        println!("--- STATS ---");
        return stats;
    }
}


impl Actor for ResultService{
    type Context = Context<Self>;
}

impl Handler<ReservationResult> for ResultService {

    type Result = ();

    fn handle(&mut self, msg: ReservationResult, _ctx: &mut Self::Context) -> Self::Result {
        self.stats.process_result_stats(msg);
    }
}