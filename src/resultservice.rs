use crate::finished::Finished;
use crate::logger::Logger;
use crate::ranked_route_entry::RankedRouteEntry;
use crate::stats::Stats;
use crate::stats_service::StatsService;
use crate::to_process_reservation_result::ToProcessReservationResult;
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
            stats: StatsService::new(1000),
            logger,
        }
    }

    pub fn log_results(&self) {
        let stats = self.stats.calculate_stats();

        self.logger
            .log("".to_string(), "".to_string(), "".to_string());
        self.logger
            .log("".to_string(), "--- STATS ---".to_string(), "".to_string());
        self.logger.log(
            "".to_string(),
            "sample size".to_string(),
            format!("{}", stats.sample_size),
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
                .do_send(Finished {})
                .unwrap_or_else(|_| panic!("Could send FINISH message from RESULT SERVICE"));
        }
    }
}

impl Handler<Stats> for ResultService {
    type Result = ();

    fn handle(&mut self, _msg: Stats, _ctx: &mut Self::Context) -> Self::Result {
        self.log_results()
    }
}

impl fmt::Display for ResultService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RESULT SERVICE")
    }
}
