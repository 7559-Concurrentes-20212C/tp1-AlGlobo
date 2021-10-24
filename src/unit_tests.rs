use std::sync::{Arc, mpsc, Mutex};

#[cfg(test)]
mod tests_thread_pool {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::thread_pool;
    use thread_pool::{ThreadPool};

    #[test]
    #[should_panic]
    fn test_err_if_less_than_1() {
        let t = ThreadPool::new(0);
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
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::resultservice::ResultService;

    #[test]
    #[should_panic]
    fn test_err_if_less_than_1() {
        let t = ResultService::new(0);
    }
}

#[cfg(test)]
mod tests_webservice {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::webservice::Webservice;
    use crate::resultservice::ResultService;
    use crate::flight::Flight;

    #[test]
    #[should_panic]
    fn test_err_if_less_than_1() {
        let t = Webservice::new(0);
    }

    #[test]
    fn should_execute() {
        let w = Webservice::new(1);
        let f = Flight{
            origin: "a".to_string(),
            destination: "b".to_string(),
            airline: "c".to_string(),
            result_service: Arc::new(ResultService::new(1))};

        w.process(f);

        // unfinished until resultservice does something interesting with
        assert_eq!(0, 2);
    }
}
