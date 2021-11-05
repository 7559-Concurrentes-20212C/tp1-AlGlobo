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
use std::time::Duration;
use std::time::Instant;
use std::{thread};
use crate::cooldown_service::CooldownService;

pub struct ScheduleService {
    id: usize,
    thread_pool: Mutex<ThreadPool>,
    webservice: Arc<Webservice>,
    hotel_webservice: Arc<Webservice>,
    result_service: Arc<ResultService>,
    logger: Arc<Logger>,
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
        logger: Arc<Logger>,
    ) -> ScheduleService {
        ScheduleService {
            id,
            thread_pool: Mutex::new(ThreadPool::new(rate_limit as usize)),
            cooldown_service: Arc::new(CooldownService::new(rate_limit as usize, retry_wait)),
            webservice,
            hotel_webservice,
            result_service,
            logger
        }
    }

    pub fn schedule_to_process(
        &self,
        arc_self: Arc<ScheduleService>,
        reservation: Arc<Reservation>,
        finished_response: Arc<Mutex<Sender<bool>>>,
    ) {
        let webservice = self.webservice.clone();
        let hotel_webservice = self.hotel_webservice.clone();
        let result_service = self.result_service.clone();
        let now = Arc::new(Instant::now());
        let id = self.id;
        let logger = self.logger.clone();
        let cooldown_service = self.cooldown_service.clone();
        let scheduler = arc_self;

        self.logger.log(
            format!("{}", self),
            "received reservation".to_string(),
            format!("{}", reservation),
        );
        self.thread_pool
            .lock()
            .expect("lock is poisoned")
            .execute(move || {
                logger.log(
                    format!("SCHDULER <{}>", id),
                    "processing reservation result".to_string(),
                    format!("{}", reservation),
                );

                match reservation.kind {
                    ReservationKind::Flight => {
                        let result = webservice.process(reservation.clone(), now.clone());
                        let s = result.accepted;

                        logger.log(
                            format!("SCHDULER <{}>", id),
                            "received result".to_string(),
                            format!(
                                "{}",
                                ReservationResult::from_reservation_ref(
                                    (*reservation).clone(),
                                    s,
                                    Duration::from_secs_f32(
                                        result.time_to_process.as_secs_f32()
                                    )
                                ),
                            ),
                        );

                        result_service.process_result(result);

                        if !s {
                            cooldown_service.cooldown(scheduler, reservation, finished_response.clone());
                        }
                    }
                    ReservationKind::Package => {
                        let hotel_res = reservation.clone();
                        let hotel_now = now.clone();
                        let ws = webservice.clone();

                        let r1 = thread::spawn(move || ws.process(hotel_res, hotel_now));

                        let r2 = hotel_webservice.process(reservation.clone(), now.clone());
                        let r1 = r1.join().expect("SCHEDULER: failed to join thread");

                        let duration = Duration::from_secs_f32(
                            r1.time_to_process
                                .as_secs_f32()
                                .max(r2.time_to_process.as_secs_f32()),
                        );

                        let result = ReservationResult::from_reservation_ref(
                            (*reservation).clone(),
                            r1.accepted && r2.accepted,
                            duration,
                        );

                        logger.log(
                            format!("SCHDULER <{}>", id),
                            "received result".to_string(),
                            format!("{}", result,),
                        );

                        let s = result.accepted;
                        result_service.process_result(result);
                        if !s {
                            logger.log(
                                format!("SCHDULER <{}>", id),
                                "reservation processing failed for".to_string(),
                                format!("{}", reservation),
                            );
                            cooldown_service.cooldown(scheduler, reservation, finished_response.clone());
                        }
                    }
                }
                finished_response
                    .lock()
                    .expect("poisoned!")
                    .send(true)
                    .expect("could not send!");
            })
    }
}

impl fmt::Display for ScheduleService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SCHEUDLER <{}>", self.id)
    }
}
