use crate::messages::{Finished, Stats, ToProcessReservationResult};
use crate::stats_service::{MovingStats, StatsService};
use actix::{Actor, Context, Handler};
use std::fs;

pub struct ResultService {
    stats: StatsService,
    log_file_name: String,
}

impl ResultService {
    pub fn new(log_file_name: String) -> ResultService {
        let cpy_log_file_name = String::from(&log_file_name);
        ResultService {
            stats: StatsService::new(1000, cpy_log_file_name),
            log_file_name,
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
        stats
    }
}

impl Actor for ResultService {
    type Context = Context<Self>;
}

impl Handler<ToProcessReservationResult> for ResultService {
    type Result = ();

    fn handle(
        &mut self,
        msg: ToProcessReservationResult,
        _ctx: &mut Self::Context,
    ) -> Self::Result {

        let to_log = format!(
            "RESULT SERVICE: received result <{}>({}|{}-{}|{}|{})",
            msg.result.reservation.id,
            msg.result.reservation.airline,
            msg.result.reservation.origin,
            msg.result.reservation.destination,
            msg.result.reservation.kind,
            msg.result.accepted
        );
        fs::write(String::from(&self.log_file_name), to_log).unwrap_or_else(|_| panic!("RESULT SERVICE: Couldn't write to log"));
        println!(
            "RESULT SERVICE: received result <{}>({}|{}-{}|{}|{})",
            msg.result.reservation.id,
            msg.result.reservation.airline,
            msg.result.reservation.origin,
            msg.result.reservation.destination,
            msg.result.reservation.kind,
            msg.result.accepted
        );

        let success = msg.result.accepted;
        self.stats.process_result_stats(msg.result);

        if success {
            msg.sender.try_send(Finished {}).unwrap_or_else(|_| panic!("Could send FINISH message from RESULT SERVICE"));
        }
    }
}

impl Handler<Stats> for ResultService {
    type Result = ();

    fn handle(&mut self, _msg: Stats, _ctx: &mut Self::Context) -> Self::Result {
        let _ = self.print_results();
    }
}
