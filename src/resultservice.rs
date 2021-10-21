use crate::thread_pool;

use thread_pool::{Message, ThreadPool};
use std::sync::mpsc;

pub struct ResultService {
    pub thread_pool : ThreadPool,
    pub result_send: mpsc::Sender<Message>,
}

impl ResultService {

    pub fn new(rate_limit: usize) -> ResultService {
        let thread_pool = ThreadPool::new(rate_limit);
        ResultService {
            thread_pool : thread_pool,
            result_send : thread_pool.sender,
        }
    }
}