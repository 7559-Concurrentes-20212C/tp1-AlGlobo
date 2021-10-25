use crate::webservice::Webservice;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc};
use crate::reservation::{Reservation, ReservationKind, ReservationResult, ReservationProcessRequest};
use crate::thread_pool::ThreadPool;
use crate::resultservice::ResultService;
use std::time::Duration;
use std::thread::Thread;

pub struct ScheduleService {
    thread_pool : ThreadPool,
    webservice: Arc<Webservice>,
    hotel_webservice: Arc<Webservice>,
    result_service: Arc<ResultService>
}

impl ScheduleService {
    pub fn new( rate_limit: usize,
                webservice : Arc<Webservice>,
                hotel_webservice: Arc<Webservice>,
                result_service: Arc<ResultService>) -> ScheduleService {

        return ScheduleService{
            thread_pool: ThreadPool::new(rate_limit),
            webservice,
            hotel_webservice,
            result_service};
    }

    pub fn schedule_to_process(&self, reservation : Arc<Reservation>){
        let webservice = self.webservice.clone();
        let hotel_webservice = self.hotel_webservice.clone();
        let result_service = self.result_service.clone();

        self.thread_pool.execute(move || {
            match reservation.kind {
                ReservationKind::Flight => {
                    let result = webservice.process(reservation);
                    result_service.process_result(result);

                    return;
                }
                ReservationKind::Package => {
                    let hotel_res = reservation.clone();
                    let r1 = Thread::spawn(move ||{
                        return webservice.process(hotel_res.clone())
                    } );
                    let r2 = hotel_webservice.process(reservation.clone());
                    let r1 = r1.join().unwrap();

                    let duration = Duration::from_secs_f32(
                        r1.time_to_process.as_secs_f32().max(r2.time_to_process.as_secs_f32()));

                    let result = ReservationResult::from_reservation_ref(reservation,
                                                                         r1.accepted && r2.accepted,
                                                                         duration);
                    result_service.process_result(result);
                    return;
                }
            }
        })
    }
}