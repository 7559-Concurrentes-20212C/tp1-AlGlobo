use crate::logger::Logger;
use crate::messages::{
    Finished, Reservation, ReservationKind, ReservationResult, ToProcessReservation,
    ToProcessReservationResult,
};
use crate::program::Program;
use crate::resultservice::ResultService;
use crate::webservice::Webservice;
use actix::{Actor, Addr, AsyncContext, Context, Handler};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use std::fmt;

pub struct ScheduleService {
    webservice: Addr<Webservice>,
    hotel_webservice: Arc<Addr<Webservice>>,
    result_service: Arc<Addr<ResultService>>,
    logger: Arc<Logger>,
    rate_limit: usize,
    results: HashMap<usize, ReservationResult>,
    caller: Arc<Addr<Program>>,
    pub id: usize,
}

impl ScheduleService {
    pub fn new(
        rate_limit: usize,
        success_chance: usize,
        hotel_webservice: Arc<Addr<Webservice>>,
        result_service: Arc<Addr<ResultService>>,
        logger: Arc<Logger>,
        caller: Arc<Addr<Program>>,
        id: usize,
    ) -> ScheduleService {
        ScheduleService {
            webservice: Webservice::new(success_chance, id, logger.clone()).start(),
            hotel_webservice,
            result_service,
            logger,
            rate_limit,
            results: HashMap::new(),
            caller,
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

        match msg.kind {
            ReservationKind::Flight => {
                self.webservice
                    .try_send(ToProcessReservation {
                        reservation: msg,
                        sender: _ctx.address().recipient(),
                    })
                    .unwrap_or_else(|_| {
                        panic!(
                            "SCHEDULER <{}>: Couldn't send RESULT message to WEBSERVICE",
                            self.id
                        )
                    });
            }
            ReservationKind::Package => {
                self.webservice
                    .try_send(ToProcessReservation {
                        reservation: msg.clone(),
                        sender: _ctx.address().recipient(),
                    })
                    .unwrap_or_else(|_| {
                        panic!(
                            "SCHEDULER <{}>: Couldn't send RESERVATION message to WEBSERVICE",
                            self.id
                        )
                    });
                self.hotel_webservice
                    .try_send(ToProcessReservation {
                        reservation: msg,
                        sender: _ctx.address().recipient(),
                    })
                    .unwrap_or_else(|_| {
                        panic!(
                            "SCHEDULER <{}>: Couldn't send RESULT message to HOTEL",
                            self.id
                        )
                    });
            }
        }
    }
}

impl Handler<ReservationResult> for ScheduleService {
    type Result = ();

    fn handle(&mut self, msg: ReservationResult, _ctx: &mut Self::Context) -> Self::Result {
        self.logger.log(
            format!("{}", self),
            "received result".to_string(),
            format!("{}", msg),
        );

        match msg.reservation.kind {
            ReservationKind::Flight => {
                if !msg.accepted
                    && msg.reservation.current_attempt_num < msg.reservation.max_attempts
                {
                    let mut next_iteration_msg = msg.reservation.clone();
                    next_iteration_msg.current_attempt_num += 1;
                    _ctx.address()
                        .try_send(next_iteration_msg)
                        .unwrap_or_else(|_| {
                            panic!(
                                "SCHEDULER <{}>: Couldn't send RESERVATION to itself for retry",
                                self.id
                            )
                        });
                }

                self.result_service
                    .try_send(ToProcessReservationResult {
                        result: msg,
                        sender: _ctx.address().recipient(),
                    })
                    .unwrap_or_else(|_| {
                        panic!(
                            "SCHEDULER <{}>: Couldn't send RESULT message to RESULT SERVICE",
                            self.id
                        )
                    });
            }
            ReservationKind::Package => {
                if self.results.contains_key(&msg.reservation.id) {
                    let id = msg.reservation.id;
                    let r1 = msg;
                    let r2 = self
                        .results
                        .get(&id)
                        .unwrap_or_else(|| panic!("SCHEDULER <{}>: INTERNAL ERROR", self.id));

                    let reservation_accepted_val = r1.accepted && r2.accepted;

                    let result = ReservationResult::from_reservation_ref(
                        r1.reservation,
                        reservation_accepted_val,
                        max_duration_between(r1.time_to_process, r2.time_to_process),
                    );

                    if !reservation_accepted_val
                        && result.reservation.current_attempt_num < result.reservation.max_attempts
                    {
                        let mut next_iteration_msg = result.reservation.clone();
                        next_iteration_msg.current_attempt_num += 1;
                        _ctx.address()
                            .try_send(next_iteration_msg)
                            .unwrap_or_else(|_| {
                                panic!(
                                    "SCHEDULER <{}>: Couldn't send RESERVATION to itself for retry",
                                    self.id
                                )
                            });
                    }

                    self.result_service
                        .try_send(ToProcessReservationResult {
                            result,
                            sender: _ctx.address().recipient(),
                        })
                        .unwrap_or_else(|_| {
                            panic!(
                                "SCHEDULER <{}>: Couldn't send RESULT message to RESULT SERVICE",
                                self.id
                            )
                        });

                    self.results.remove_entry(&id);
                } else {
                    self.results.entry(msg.reservation.id).or_insert(msg);
                }
            }
        }
    }
}

impl Handler<Finished> for ScheduleService {
    type Result = ();
    fn handle(&mut self, _msg: Finished, _ctx: &mut Self::Context) -> Self::Result {
        self.caller.try_send(Finished {}).unwrap_or_else(|_| {
            panic!(
                "SCHEDULER <{}>: Couldn't
        send FINISH message to PROGRAM",
                self.id
            )
        });
    }
}

fn max_duration_between(d1: Duration, d2: Duration) -> Duration {
    Duration::from_secs_f32(d1.as_secs_f32().max(d2.as_secs_f32()))
}

impl fmt::Display for ScheduleService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SCHEUDLER <{}>", self.id)
    }
}
