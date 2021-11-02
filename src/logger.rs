use std::fs::OpenOptions;
use std::io::prelude::*;

pub struct Logger {
    log_file_name: String,
}

impl Logger {
    pub fn new(log_file_name: String) -> Logger {
        Logger { log_file_name }
    }

    pub fn log(&self, emiting: String, msg: String, info: String) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(String::from(&self.log_file_name))
            .expect("LOGGER: Couldn't open log file");
        writeln!(
            file,
            "{}: {} {}",
            String::from(&emiting),
            String::from(&msg),
            String::from(&info)
        )
        .expect("LOGGER: Couldn't log to file");
        println!(
            "{}: {} {}",
            String::from(&emiting),
            String::from(&msg),
            String::from(&info)
        );
    }

    pub fn log_extra_arg(&self, emiting: String, msg: String, info: String, extra: String) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(String::from(&self.log_file_name))
            .expect("LOGGER: Couldn't open log file");
        writeln!(
            file,
            "{}: {} {} --- {}",
            String::from(&emiting),
            String::from(&msg),
            String::from(&info),
            String::from(&extra)
        )
        .expect("LOGGER: Couldn't log to file");
        println!(
            "{}: {} {} --- {}",
            String::from(&emiting),
            String::from(&msg),
            String::from(&info),
            String::from(&extra)
        );
    }
}
