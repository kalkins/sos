use log::{set_logger, set_max_level, LevelFilter};

use crate::println;

pub struct KernelLogger;

const LOGGER: KernelLogger = KernelLogger {};

impl KernelLogger {
    pub fn init() {
        set_logger(&LOGGER).unwrap();
        KernelLogger::set_max_level(LevelFilter::Info);
    }

    pub fn set_max_level(level: LevelFilter) {
        set_max_level(level);
    }
}

impl log::Log for KernelLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("[{: >5}]: {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
