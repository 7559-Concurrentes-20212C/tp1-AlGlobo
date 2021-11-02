use crate::job::Job;

pub enum Message {
    NewJob(Job),
    Terminate,
}
