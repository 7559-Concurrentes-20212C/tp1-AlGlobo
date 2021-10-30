use actix::{Actor, Context, Handler, Addr};
use std::sync::{Arc};
use crate::webservice::Webservice;
use crate::reservation::{Reservation, ReservationKind, ReservationResult};
use crate::resultservice::{ResultService};
use std::time::Duration;

pub struct ScheduleService {
    webservice: Addr<Webservice>,
    hotel_webservice: Arc<Addr<Webservice>>,
    result_service: Arc<Addr<ResultService>>,
    rate_limit : usize
}

impl ScheduleService {
    pub fn new( rate_limit: usize,
                success_chance: usize,
                hotel_webservice: Arc<Addr<Webservice>>,
                result_service: Arc<Addr<ResultService>>) -> ScheduleService {

        return ScheduleService{
            webservice : Webservice::new(success_chance).start(),
            hotel_webservice,
            result_service,
            rate_limit
        };
    }
}


impl Actor for ScheduleService{
    type Context = Context<Self>;
}

impl Handler<Reservation> for ScheduleService {

    type Result = ();

    fn handle(&mut self, msg: Reservation, _ctx: &mut Self::Context)  -> Self::Result {
        let webservice = self.webservice.clone();
        let hotel_webservice = self.hotel_webservice.clone();
        let result_service = self.result_service.clone();
        let rate_limit = self.rate_limit;

        println!("schedule request for {} with {}-{}", msg.airline, msg.destination, msg.destination);
        for _ in 0..rate_limit {

            match msg.kind {
                ReservationKind::Flight => {
                    let reservation_accepted = async move {
                        let result = webservice.try_send(msg).unwrap();
                        let reservation_accepted = result.accepted;
                        result_service.try_send(result).unwrap();
                        return reservation_accepted;
                    };

                    if reservation_accepted {break}
                }
                ReservationKind::Package => {

                    let reservation_accepted = async move {
                        let result1 = webservice.try_send(msg).unwrap();
                        let result2 = hotel_webservice.try_send(msg).unwrap();
    
                        let result = ReservationResult::from_reservation_ref(msg,
                                                                            result1.accepted && result2.accepted,
                                                                            max_duration_between(result1.liveness_cronometer, result2.liveness_cronometer));

                        let reservation_accepted = result.accepted;
                        result_service.try_send(result);
                        return reservation_accepted;
                    };

                    if reservation_accepted {break} else {
                        println!("reservation processing failed for {} with {}-{}",
                                    msg.airline, msg.destination, msg.destination);
                    }
                }
            }
        }
    }    
}

fn max_duration_between(d1 : Duration, d2: Duration) -> Duration{
    Duration::from_secs_f32(d1.as_secs_f32().max(d2.as_secs_f32()))
}