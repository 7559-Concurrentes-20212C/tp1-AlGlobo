use crate::resultservice;

use resultservice::{ResultService}; 
use std::sync::Arc;

pub struct Flight {
    pub origin: String,
    pub destination: String,
    pub airline: String,
    pub result_service: Arc<ResultService>,
}

pub struct FlightResult {
    pub id: String,
    pub accepted: bool,
}
