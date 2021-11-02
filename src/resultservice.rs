use crate::logger::Logger;
use crate::messages::{Finished, Stats, ToProcessReservationResult};
use crate::stats_service::{MovingStats, StatsService};
use actix::{Actor, Context, Handler};
use std::fmt;
use std::sync::Arc;

pub struct ResultService {
    stats: StatsService,
    logger: Arc<Logger>,
}

impl ResultService {
    pub fn new(logger: Arc<Logger>) -> ResultService {
        ResultService {
            stats: StatsService::new(1000, logger.clone()),
            logger,
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
        self.logger.log(
            format!("{}", self),
            "received result".to_string(),
            format!("{}", msg.result),
        );

        let success = msg.result.accepted;
        self.stats.process_result_stats(msg.result);

        if success {
            msg.sender
                .try_send(Finished {})
                .unwrap_or_else(|_| panic!("Could send FINISH message from RESULT SERVICE"));
        }
    }
}

impl Handler<Stats> for ResultService {
    type Result = ();

    fn handle(&mut self, _msg: Stats, _ctx: &mut Self::Context) -> Self::Result {
        let _ = self.print_results();
    }
}

impl fmt::Display for ResultService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RESULT SERVICE")
    }
}
