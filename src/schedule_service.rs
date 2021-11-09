use crate::finished::Finished;
use crate::logger::Logger;
use crate::program::Program;
use crate::reservation::Reservation;
use crate::reservation_kind::ReservationKind;
use crate::reservation_result::ReservationResult;
use crate::resultservice::ResultService;
use crate::to_process_reservation::ToProcessReservation;
use crate::to_process_reservation_result::ToProcessReservationResult;
use crate::webservice::Webservice;
use crate::webservice_kind::WebserviceKind;
use actix::{Actor, Addr, AsyncContext, Context, Handler};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

use crate::cooldown_service::CooldownService;
use crate::reservation_cooldown::CooldownReservation;
use actix::clock::Instant;
use std::fmt;

pub struct ScheduleService {
    webservice: Addr<Webservice>,
    hotel_webservice: Arc<Addr<Webservice>>,
    result_service: Arc<Addr<ResultService>>,
    cooldown_service: Addr<CooldownService>,
    logger: Arc<Logger>,
    results: HashMap<usize, ReservationResult>,
    queued_reservations: VecDeque<Reservation>,
    caller: Arc<Addr<Program>>,
    amount_processing: usize,
    capacity: usize,
    pub id: usize,
}

impl ScheduleService {
    pub fn new(
        params: (usize, usize),
        webservice: Addr<Webservice>,
        hotel_webservice: Arc<Addr<Webservice>>,
        result_service: Arc<Addr<ResultService>>,
        logger: Arc<Logger>,
        caller: Arc<Addr<Program>>,
        id: usize,
    ) -> ScheduleService {
        ScheduleService {
            webservice,
            hotel_webservice,
            result_service,
            cooldown_service: CooldownService::new(params.1 as u64).start(),
            logger,
            results: HashMap::new(),
            queued_reservations: VecDeque::new(),
            caller,
            amount_processing: 0,
            capacity: params.0,
            id,
        }
    }
}

impl Actor for ScheduleService {
    type Context = Context<Self>;
}

impl Handler<Reservation> for ScheduleService {
    type Result = ();

    fn handle(&mut self, msg: Reservation, _ctx: &mut Self::Context) -> Self::Result {
        self.logger.log(
            format!("{}", self),
            "received reservation".to_string(),
            format!("{}", msg),
        );

        if self.amount_processing < self.capacity {
            self.webservice
                .do_send(ToProcessReservation {
                    reservation: msg.clone(),
                    sender: _ctx.address().recipient(),
                });
            self.amount_processing += 1;
        } else {
            self.queued_reservations.push_front(msg.clone());
            
        }

        if let ReservationKind::Package = msg.kind {
            if msg.fresh {
                self.hotel_webservice
                    .do_send(ToProcessReservation {
                        reservation: msg,
                        sender: _ctx.address().recipient(),
                    });
            }
        }
    }
}

impl Handler<ReservationResult> for ScheduleService {
    type Result = ();

    fn handle(&mut self, mut msg: ReservationResult, _ctx: &mut Self::Context) -> Self::Result {
        self.logger.log(
            format!("{}", self),
            "received result".to_string(),
            format!("{}", msg),
        );

        if let WebserviceKind::Airline = msg.creator {
            self.amount_processing -= 1;
        }

        if !msg.accepted {
            let mut reservation = msg.reservation;
            self.cooldown_service
                .do_send(CooldownReservation {
                    caller: _ctx.address().recipient(),
                    reservation,
                    requested: Instant::now(),
                });
        } else {
            let mut ready_to_process_result = true;
            if let ReservationKind::Package = msg.reservation.kind {
                if self.results.contains_key(&msg.reservation.id) {
                    let id = msg.reservation.id;
                    let r1 = msg;
                    let r2 = self
                        .results
                        .get(&id)
                        .unwrap_or_else(|| panic!("SCHEDULER <{}>: INTERNAL ERROR", self.id));

                    let reservation_accepted_val = r1.accepted && r2.accepted;

                    msg = ReservationResult::from_reservation_ref(
                        r1.reservation,
                        reservation_accepted_val,
                        max_duration_between(r1.time_to_process, r2.time_to_process),
                        WebserviceKind::Merge,
                    );
                } else {
                    ready_to_process_result = false;
                    self.results
                        .entry(msg.reservation.id)
                        .or_insert_with(|| msg.clone());
                }
            }
            if ready_to_process_result {
                self.result_service
                    .do_send(ToProcessReservationResult {
                        result: msg,
                        sender: _ctx.address().recipient(),
                    });
            }
        }

        if !self.queued_reservations.is_empty() {
            let queued_msg = self
                .queued_reservations
                .pop_back()
                .expect("INTERNAL ERROR: Coudnt' pop queued reservation");
            _ctx.address().do_send(queued_msg);
        }
    }
}

impl Handler<Finished> for ScheduleService {
    type Result = ();
    fn handle(&mut self, _msg: Finished, _ctx: &mut Self::Context) -> Self::Result {
        if self.queued_reservations.is_empty() && self.amount_processing == 0 {
            self.caller.do_send(Finished {});
        }
    }
}

impl fmt::Display for ScheduleService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SCHEUDLER <{}>(procesing <{}>)",
            self.id, self.amount_processing
        )
    }
}

fn max_duration_between(d1: Duration, d2: Duration) -> Duration {
    Duration::from_secs_f32(d1.as_secs_f32().max(d2.as_secs_f32()))
}
