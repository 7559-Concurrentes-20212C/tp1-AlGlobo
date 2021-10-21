use crate::flight;

pub struct ResultService {
    pub thread_pool : ThreadPool,
    pub result_send: mpsc::Sender<Message>,
}

impl ResultService {

    pub fn new(rate_limit: u32) -> ResultService {
        ResultService {
            thread_pool : ThreadPool::new(rate_limit),
            result_send : thread_pool.sender,
        }
    }
}