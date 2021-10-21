use crate::thread_pool;
use thread_pool::{Message}; 

use std::sync::mpsc::{Sender};


pub struct Flight {
    pub origin: String,
    pub destination: String,
    pub airline: String,
    pub result_gateway: Sender<Message>,
}

pub struct FlightResult {
    pub id: String,
    pub accepted: bool,
}
