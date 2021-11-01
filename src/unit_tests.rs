use std::sync::{Arc, mpsc, Mutex};

#[cfg(test)]
mod tests_thread_pool {
    use super::*;
    use crate::thread_pool;
    use thread_pool::{ThreadPool};

    #[test]
    #[should_panic]
    fn test_err_if_less_than_1() {
        let _t = ThreadPool::new(0);
    }

    #[test]
    fn should_execute() {
        let (sender, recv) = mpsc::channel();
        let sd = Arc::new(Mutex::new(sender));
        let t = ThreadPool::new(1);

        t.execute(move || {
            sd.lock().expect("could not lock").send(1 + 1);
        });

        assert_eq!(recv.recv().expect("failed to receive"), 2);
    }
}

#[cfg(test)]
mod tests_webservice {
    use super::*;
    use crate::webservice::Webservice;
    use std::time::Instant;
    use crate::reservation::{Reservation, ReservationKind};

    #[test]
    fn should_execute() {
        let w = Webservice::new(1);
        let now = Arc::new(Instant::now());
        let f = Arc::new(Reservation{
            origin: "a".to_string(),
            destination: "b".to_string(),
            airline: "c".to_string(),
            kind: ReservationKind::Flight
        });

        assert_eq!( w.process(f.clone(), now).airline,  f.airline);
    }
}

#[cfg(test)]
mod test_stats_service {

    use super::*;
    use crate::webservice::Webservice;
    use crate::stats_service::StatsService;
    use std::time::Instant;
    use crate::reservation::{Reservation, ReservationKind};

    #[test]
    fn should_execute() {
        let s = StatsService::new(1, 1);
        let w = Webservice::new(1);
        let now = Arc::new(Instant::now());
        let f = Reservation{
            origin: "a".to_string(),
            destination: "b".to_string(),
            airline: "c".to_string(),
            kind: ReservationKind::Flight
        };

        s.process_result_stats( w.process(Arc::from(f), now) );
        assert_eq!(1, 1);
    }
}

#[cfg(test)]
mod test_result_service {

    use super::*;
    use crate::webservice::Webservice;
    use std::time::Instant;
    use crate::resultservice::ResultService;
    use crate::reservation::{Reservation, ReservationKind};
    use crate::stats_service::StatsService;

    #[test]
    #[should_panic]
    fn test_err_if_less_than_1() {
        let _t = ResultService::new(0);
    }

    #[test]
    fn should_execute() {
        let w = Webservice::new(1);
        let now = Arc::new(Instant::now());
        let r = ResultService::new(1);
        let f = Reservation{
            origin: "a".to_string(),
            destination: "b".to_string(),
            airline: "c".to_string(),
            kind: ReservationKind::Flight
        };

        r.process_result( w.process(Arc::from(f), now) );
        //since there are threads involved, might be wise to use a condvar?
        assert_eq!(1, 1);
    }
}
