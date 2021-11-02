use lib::reservation::Reservation;
use lib::resultservice::ResultService;
use lib::schedule_service::ScheduleService;
use lib::webservice::Webservice;
use std::collections::HashMap;
use std::sync::Arc;
use std::{thread, time};

#[test]
fn test_run_program_with_3mixed_reqs_2webservices_100success_rate() {
    const RATE_LIMIT: usize = 4;
    let test_args = vec![
        "aa,bb,american,package".to_string(),
        "aa,bb,italian,flight".to_string(),
        "aa,bb,american,flight".to_string(),
    ];

    //creates service for handling incoming results
    let results_service = Arc::new(ResultService::new(RATE_LIMIT));
    //creates hotel
    let hotel = Arc::new(Webservice::new(100));
    //creates all web services
    let web_services = load_services(
        &results_service,
        hotel,
        vec!["american,10,100".to_string(), "italian,10,100".to_string()],
    );

    for line in test_args {
        let reservation = Arc::new(Reservation::from_line(line));
        let scheduler = match web_services.get(&*reservation.airline) {
            None => {
                println!("invalid airline reservation: {}", reservation.airline);
                continue;
            }
            Some(s) => s,
        };
        scheduler.schedule_to_process(reservation);
    }
    thread::sleep(time::Duration::from_secs(3));
    assert_eq!(results_service.print_results().sample_size, 3);
}

#[test]
fn test_run_program_test_webservice_always_fails() {
    const RATE_LIMIT: usize = 4;
    let test_args = vec!["aa,bb,american,package".to_string()];

    //creates service for handling incoming results
    let results_service = Arc::new(ResultService::new(1));
    //creates hotel
    let hotel = Arc::new(Webservice::new(100));
    //creates all web services
    let web_services = load_services(
        &results_service,
        hotel,
        vec!["american,4,0".to_string(), "italian,1,0".to_string()],
    );

    for line in test_args {
        let reservation = Arc::new(Reservation::from_line(line));
        let scheduler = match web_services.get(&*reservation.airline) {
            None => {
                println!("invalid airline reservation: {}", reservation.airline);
                continue;
            }
            Some(s) => s,
        };
        scheduler.schedule_to_process(reservation);
    }
    thread::sleep(time::Duration::from_secs(2));
    assert_eq!(results_service.print_results().sample_size, 0);
}

#[test]
fn test_run_program_for_bottleneck() {
    const RATE_LIMIT: usize = 4;
    let test_args = vec![
        "aa,bb,american,package".to_string(),
        "aa,bb,american,package".to_string(),
        "aa,bb,american,package".to_string(),
        "aa,bb,american,package".to_string(),
        "aa,bb,american,package".to_string(),
        "aa,bb,american,package".to_string(),
    ];

    //creates service for handling incoming results
    let results_service = Arc::new(ResultService::new(RATE_LIMIT));
    //creates hotel
    let hotel = Arc::new(Webservice::new(100));
    //creates all web services
    let web_services = load_services(&results_service, hotel, vec!["american,1,1".to_string()]);

    for line in test_args {
        let reservation = Arc::new(Reservation::from_line(line));
        let scheduler = match web_services.get(&*reservation.airline) {
            None => {
                println!("invalid airline reservation: {}", reservation.airline);
                continue;
            }
            Some(s) => s,
        };
        scheduler.schedule_to_process(reservation);
    }
    thread::sleep(time::Duration::from_millis(3000));
    assert_ne!(results_service.print_results().sample_size, 6);
}

#[test]
fn test_run_program_incorrect_airlines() {
    const RATE_LIMIT: usize = 4;
    let test_args = vec!["aa,bb,italian,package".to_string()];

    //creates service for handling incoming results
    let results_service = Arc::new(ResultService::new(RATE_LIMIT));
    //creates hotel
    let hotel = Arc::new(Webservice::new(100));
    //creates all web services
    let web_services = load_services(&results_service, hotel, vec!["american,1,100".to_string()]);

    for line in test_args {
        let reservation = Arc::new(Reservation::from_line(line));
        let scheduler = match web_services.get(&*reservation.airline) {
            None => {
                println!("invalid airline reservation: {}", reservation.airline);
                continue;
            }
            Some(s) => s,
        };
        scheduler.schedule_to_process(reservation);
    }
    thread::sleep(time::Duration::from_secs(2));
    assert_ne!(results_service.print_results().sample_size, 1);
}

fn load_services(
    resultservice: &Arc<ResultService>,
    hotel: Arc<Webservice>,
    test_args: Vec<String>,
) -> Option<HashMap<String, ScheduleService>> {
    let mut web_services: HashMap<String, ScheduleService> = HashMap::new();

    for line in test_args {
        let params = line.split(',').collect::<Vec<&str>>();
        let capacity = params[1].parse::<usize>();
        let rate = params[2].parse::<usize>();
        let webservice = Arc::new(Webservice::new(rate));
        web_services.insert(
            params[0].parse(),
            ScheduleService::new(capacity, webservice, hotel.clone(), resultservice.clone()),
        );
    }
    return Some(web_services);
}
