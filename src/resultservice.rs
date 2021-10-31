use actix::{Actor, Context, Handler};
use crate::messages::{Stats, ToProcessReservationResult, Finished};
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

impl Handler<ToProcessReservationResult> for ResultService {

    type Result = ();

    fn handle(&mut self, msg: ToProcessReservationResult, _ctx: &mut Self::Context) -> Self::Result {

        println!("RESULT SERVICE: received result <{}>({}|{}-{}|{}|{})", msg.result.reservation.id,
                                                                msg.result.reservation.airline, msg.result.reservation.origin,msg.result.reservation.destination,
                                                                msg.result.reservation.kind, msg.result.accepted);

        let success = msg.result.accepted.clone();
        self.stats.process_result_stats(msg.result);

        if success {
            msg.sender.try_send(Finished {});
        }
    }
}

impl Handler<Stats> for ResultService {

    type Result = ();

    fn handle(&mut self, msg: Stats, _ctx: &mut Self::Context) -> Self::Result {

        let _ = self.print_results();
    }
}