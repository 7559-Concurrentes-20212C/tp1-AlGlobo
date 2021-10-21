use crate::flight;

pub struct ResultService {
    pub thread_pool : ThreadPool,
}

impl ResultService {

    pub fn new(rate_limit: u32) -> ResultService {
        ResultService {
            thread_pool : ThreadPool::new(rate_limit),
        }
    }
}