use crate::reservation::Reservation;
use crate::reservation_result::ReservationResult;
use crate::schedule_service::ScheduleService;
use crate::thread_pool::ThreadPool;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{thread, time};

pub struct CooldownService {
    thread_pool: Mutex<ThreadPool>,
    retry_wait: u64,
}

impl CooldownService {
    pub fn new(rate_limit: usize, retry_wait: u64) -> CooldownService {
        CooldownService {
            thread_pool: Mutex::new(ThreadPool::new(rate_limit)),
            retry_wait,
        }
    }

    pub fn cooldown(
        &self,
        scheduler: Arc<ScheduleService>,
        reservation: Arc<Reservation>,
        finished_response: Arc<Mutex<Sender<bool>>>,
        hotel_result: Option<ReservationResult>,
    ) {
        let start = Instant::now();
        let retry_wait = self.retry_wait;

        self.thread_pool
            .lock()
            .expect("lock is poisoned")
            .execute(move || {
                let elapsed = start.elapsed();
                let mut wait = 0;
                if retry_wait as u128 > elapsed.as_millis() {
                    wait = retry_wait - elapsed.as_millis() as u64;
                };
                thread::sleep(time::Duration::from_millis(wait.min(retry_wait)));
                scheduler._schedule_to_process(
                    scheduler.clone(),
                    reservation,
                    finished_response,
                    hotel_result,
                );
            });
    }
}
