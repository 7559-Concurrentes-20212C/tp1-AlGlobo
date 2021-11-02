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
use std::{thread, time};

pub struct ScheduleService {
    id: usize,
    thread_pool: Mutex<ThreadPool>,
    webservice: Arc<Webservice>,
    hotel_webservice: Arc<Webservice>,
    result_service: Arc<ResultService>,
    logger: Arc<Logger>,
    rate_limit: u32,
    retry_wait: u64,
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
            retry_wait,
            webservice,
            hotel_webservice,
            result_service,
            logger,
            rate_limit,
        }
    }

    pub fn schedule_to_process(
        &self,
        reservation: Arc<Reservation>,
        finished_response: Arc<Mutex<Sender<bool>>>,
    ) {
        let webservice = self.webservice.clone();
        let hotel_webservice = self.hotel_webservice.clone();
        let result_service = self.result_service.clone();
        let mut now = Arc::new(Instant::now());
        let rate_limit = self.rate_limit;
        let retry_wait = self.retry_wait;
        let id = self.id;
        let logger = self.logger.clone();

        self.logger.log(
            format!("{}", self),
            "received reservation".to_string(),
            format!("{}", reservation),
        );
        self.thread_pool
            .lock()
            .expect("lock is poisoned")
            .execute(move || {
                for _ in 0..rate_limit {
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

                            if s {
                                break;
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
                            if s {
                                break;
                            } else {
                                now = Arc::new(Instant::now());
                                logger.log(
                                    format!("SCHDULER <{}>", id),
                                    "reservation processing failed for".to_string(),
                                    format!("{}", reservation),
                                );
                            }
                        }
                    }
                }
                thread::sleep(time::Duration::from_millis(retry_wait));
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
