use crate::webservice::Webservice;
use std::sync::{Arc, Mutex};
use crate::reservation::{Reservation, ReservationKind, ReservationResult};
use crate::thread_pool::ThreadPool;
use crate::resultservice::ResultService;
use std::time::Duration;
use std::thread;
use std::time::{Instant};

pub struct ScheduleService {
    thread_pool : Mutex<ThreadPool>,
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
            thread_pool: Mutex::new(ThreadPool::new(rate_limit)),
            webservice,
            hotel_webservice,
            result_service};
    }


    pub fn schedule_to_process(&self, reservation : Arc<Reservation>){
        let webservice = self.webservice.clone();
        let hotel_webservice = self.hotel_webservice.clone();
        let result_service = self.result_service.clone();
        let now = Arc::new(Instant::now());
        const TRIES: u32 = 10;

        self.thread_pool.lock().expect("lock is poisoned").execute(move || {
            for _i in 0..TRIES {

                match reservation.kind {
                    ReservationKind::Flight => {
                        let result = webservice.process(reservation.clone(), now.clone());
                        let s = result.accepted;
                        result_service.process_result(result);

                        if s {break}
                    }
                    ReservationKind::Package => {
                        let hotel_res = reservation.clone();
                        let hotel_now = now.clone();
                        let ws = webservice.clone();

                        let r1 = thread::spawn(move ||{
                            return ws.process(hotel_res, hotel_now)
                        } );

                        let r2 = hotel_webservice.process(reservation.clone(), now.clone());
                        let r1 = r1.join().unwrap();

                        let duration = Duration::from_secs_f32(
                            r1.time_to_process.as_secs_f32().max(r2.time_to_process.as_secs_f32()));

                        let result = ReservationResult::from_reservation_ref(reservation.clone(),
                                                                             r1.accepted && r2.accepted,
                                                                             duration);
                        let s = result.accepted;
                        result_service.process_result(result);
                        if s {break}
                    }
                }

            }
        })
    }
}