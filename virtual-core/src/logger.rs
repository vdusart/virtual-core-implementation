pub struct Logger;

impl Logger {
    const ERROR: &str = "\x1b[0;31m";
    const INFO: &str = "\x1b[0;34m";
    const RESET: &str = "\x1b[0m";

    pub fn info(msg: &str) {
        println!("{}[INFO]{} {}", Logger::INFO, Logger::RESET, msg);
    }

    pub fn error(msg: &str) {
        eprintln!("{}[ERROR]{} {}", Logger::ERROR, Logger::RESET, msg);
        std::process::exit(-1);
    }
}