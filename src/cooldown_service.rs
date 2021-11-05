use crate::schedule_service::ScheduleService;
use crate::reservation::Reservation;
use crate::thread_pool::ThreadPool;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::time::Instant;
use std::{thread, time};


pub struct CooldownService {
    thread_pool: Mutex<ThreadPool>,
    retry_wait: u64,

}

impl CooldownService{
    pub fn new(rate_limit: usize, retry_wait: u64) -> CooldownService {
        return CooldownService{ thread_pool: Mutex::new(ThreadPool::new(rate_limit)), retry_wait }
    }

    pub fn cooldown(&self, scheduler: Arc<ScheduleService>, reservation: Arc<Reservation>,
                    finished_response: Arc<Mutex<Sender<bool>>>){
        let start = Instant::now();
        let retry_wait = self.retry_wait;

        self.thread_pool.lock().expect("lock is poisoned").execute(move || {
            let elapsed = start.elapsed();
            let wait = 0.max(retry_wait - elapsed.as_millis() as u64);
            thread::sleep(time::Duration::from_millis(wait.min( retry_wait )));
            scheduler.schedule_to_process(scheduler.clone(), reservation, finished_response);
        });
    }
}