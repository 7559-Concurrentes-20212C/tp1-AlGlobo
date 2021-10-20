use std::sync::mpsc::{Receiver, Sender};

pub struct Flight {
    pub origin: String,
    pub destination: String,
    pub airline: String,
    pub result_gateway: Sender<FlightResult>,
}

pub struct FlightResult {
    pub id: String,
    pub accepted: bool,
}
