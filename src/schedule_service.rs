use actix::{Actor, Context, Handler, Addr, AsyncContext};
use std::sync::{Arc};
use crate::webservice::Webservice;
use crate::reservation::{Reservation, ReservationKind, ReservationResult, ToProcessReservation};
use crate::resultservice::{ResultService};
use std::time::Duration;

pub struct ScheduleService {
    webservice: Addr<Webservice>,
    hotel_webservice: Arc<Addr<Webservice>>,
    result_service: Arc<Addr<ResultService>>,
    rate_limit : usize,
    results : Vec<ReservationResult>,
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
            rate_limit,
            results : vec!(),
        };
    }
}


impl Actor for ScheduleService{
    type Context = Context<Self>;
}

impl Handler<Reservation> for ScheduleService {
    type Result = ();

    fn handle(&mut self, msg: Reservation, _ctx: &mut Self::Context)  -> Self::Result {

        println!("schedule request for {} with {}-{}", msg.airline, msg.destination, msg.destination);

        match msg.kind {
            ReservationKind::Flight => {
                self.webservice.try_send(ToProcessReservation {reservation : msg.clone(), sender: _ctx.address().recipient()}).unwrap();
            }
            ReservationKind::Package => {
                self.webservice.try_send(ToProcessReservation {reservation : msg.clone(), sender: _ctx.address().recipient()}).unwrap();
                self.hotel_webservice.try_send(ToProcessReservation {reservation : msg.clone(), sender: _ctx.address().recipient()}).unwrap();
            }
        }
    }
}

impl Handler<ReservationResult> for ScheduleService {
    type Result = ();

    fn handle(&mut self, msg: ReservationResult, _ctx: &mut Self::Context)  -> Self::Result {

        match msg.reservation.kind {
            ReservationKind::Flight => {
                self.result_service.try_send(msg);
            }
            ReservationKind::Package => {
                if self.results.len() == 2{     //webservice y hotel

                    let r1 = msg;
                    let r2 = self.results.pop().unwrap();
        
                    let reservation_accepted_val = r1.accepted && r2.accepted;
                    
                    let result = ReservationResult::from_reservation_ref(r1.reservation,
                                                                        reservation_accepted_val,
                                                                        max_duration_between(
                                                                            r1.time_to_process,
                                                                            r2.time_to_process));

                    if reservation_accepted_val == false && result.reservation.current_attempt_num < result.reservation.max_attempts{
                        let mut next_iteration_msg = result.reservation.clone();
                        next_iteration_msg.current_attempt_num = next_iteration_msg.current_attempt_num + 1;
                        _ctx.address().try_send(next_iteration_msg).unwrap();
                    }
                    
                    self.result_service.try_send(result);
                } else {
                    self.results.push(msg);
                }
            }
        }
    }
}

fn max_duration_between(d1 : Duration, d2: Duration) -> Duration{
    Duration::from_secs_f32(d1.as_secs_f32().max(d2.as_secs_f32()))
}