use lib;
use std::sync::{Arc, Mutex, Condvar};

fn main() {
    /*
    let exit_cv = Arc::new((Mutex::new(false), Condvar::new()));
    let exit_cv2 = Arc::clone(&exit_cv);

    ctrlc::set_handler(move || {
        let (lock, cvar) = &*exit_cv2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    }).expect("Error setting Ctrl-C handler");*/

    lib::run();
    /*
    let (lock, cvar) = &*exit_cv;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }*/
}