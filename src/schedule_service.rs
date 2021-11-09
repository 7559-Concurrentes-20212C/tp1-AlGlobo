use crate::cooldown_service::CooldownService;
use crate::logger::Logger;
use crate::reservation::Reservation;
use crate::reservation_kind::ReservationKind;
use crate::reservation_result::ReservationResult;
use crate::resultservice::ResultService;
use crate::thread_pool::ThreadPool;
use crate::webservice::Webservice;
use std::fmt;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::Instant;

pub struct ScheduleService {
    id: usize,
    thread_pool: Mutex<ThreadPool>,
    webservice: Arc<Webservice>,
    hotel_webservice: Arc<Webservice>,
    result_service: Arc<ResultService>,
    logger: Arc<Mutex<Logger>>,
    cooldown_service: Arc<CooldownService>,
}

impl ScheduleService {
    pub fn new(
        id: usize,
        rate_limit: u32,
        retry_wait: u64,
        webservice: Arc<Webservice>,
        hotel_webservice: Arc<Webservice>,
        result_service: Arc<ResultService>,
        logger: Arc<Mutex<Logger>>,
    ) -> ScheduleService {
        ScheduleService {
            id,
            thread_pool: Mutex::new(ThreadPool::new(rate_limit as usize)),
            cooldown_service: Arc::new(CooldownService::new(rate_limit as usize, retry_wait)),
            webservice,
            hotel_webservice,
            result_service,
            logger,
        }
    }

    pub fn schedule_to_process(
        &self,
        arc_self: Arc<ScheduleService>,
        reservation: Arc<Reservation>,
        finished_response: Arc<Mutex<Sender<bool>>>,
    ) {
        self._schedule_to_process(arc_self, reservation, finished_response, None)
    }

    pub fn _schedule_to_process(
        &self,
        arc_self: Arc<ScheduleService>,
        reservation: Arc<Reservation>,
        finished_response: Arc<Mutex<Sender<bool>>>,
        hotel_result: Option<ReservationResult>,
    ) {
        let webservice = self.webservice.clone();
        let hotel_webservice = self.hotel_webservice.clone();
        let result_service = self.result_service.clone();
        let now = Arc::new(Instant::now());
        let id = self.id;
        let logger = self.logger.clone();
        let cooldown_service = self.cooldown_service.clone();
        let scheduler = arc_self;

        self.logger.lock().expect("poisoned lock").log(
            format!("{}", self),
            "received reservation".to_string(),
            format!("{}", reservation),
        );
        self.thread_pool
            .lock()
            .expect("lock is poisoned")
            .execute(move || {
                logger.lock().expect("poisoned lock").log(
                    format!("SCHDULER <{}>", id),
                    "processing reservation result".to_string(),
                    format!("{}", reservation),
                );

                match reservation.kind {
                    ReservationKind::Flight => {
                        let result = webservice.process(reservation.clone(), now.clone());

                        logger.lock().expect("poisoned lock").log(
                            format!("SCHDULER <{}>", id),
                            "received result".to_string(),
                            format!(
                                "{}",
                                ReservationResult::from_reservation_ref(
                                    (*reservation).clone(),
                                    result.accepted,
                                    Duration::from_secs_f32(result.time_to_process.as_secs_f32())
                                ),
                            ),
                        );

                        result_service.process_result(result.clone());

                        if result.accepted {
                            finished_response
                                .lock()
                                .expect("poisoned!")
                                .send(true)
                                .expect("could not send!");
                        } else {
                            cooldown_service.cooldown(
                                scheduler,
                                reservation,
                                finished_response.clone(),
                                None,
                            );
                        }
                    }
                    ReservationKind::Package => {
                        let airline_res = reservation.clone();
                        let hotel_res = reservation.clone();
                        let hotel_now = now.clone();
                        let ws = webservice.clone();

                        let r1 = ws.process(airline_res, now.clone());

                        let r2: ReservationResult = match hotel_result {
                            Some(r2) => r2,
                            None => thread::spawn(move || {
                                hotel_webservice.process(hotel_res, hotel_now)
                            })
                            .join()
                            .expect("SCHEDULER: failed to join thread"),
                        };

                        let result = ReservationResult::mix(r1, r2.clone());

                        logger.lock().expect("poisoned lock").log(
                            format!("SCHDULER <{}>", id),
                            "received result".to_string(),
                            format!("{}", result,),
                        );

                        result_service.process_result(result.clone());

                        if result.accepted {
                            finished_response
                                .lock()
                                .expect("poisoned!")
                                .send(true)
                                .expect("could not send!");
                        } else {
                            logger.lock().expect("poisoned lock").log(
                                format!("SCHDULER <{}>", id),
                                "reservation processing failed for".to_string(),
                                format!("{}", reservation),
                            );
                            cooldown_service.cooldown(
                                scheduler,
                                reservation,
                                finished_response.clone(),
                                Some(r2),
                            );
                        }
                    }
                }
            })
    }
}

impl fmt::Display for ScheduleService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SCHEUDLER <{}>", self.id)
    }
}
