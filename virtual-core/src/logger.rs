use env_logger::fmt::Formatter;
use env_logger::Builder;
use log::Level;
use log::LevelFilter;
use log::Record;
use std::io::{Error, Write};

pub fn current_register_states(registers: &[i64; 16]) -> String {
    let mut states = String::from("Current register states.");
    for (i, r) in registers.iter().enumerate() {
        states = format!("{}\nr{:#02} = {:#018x}", states, i, r);
    }
    states
}

pub struct Logger;

impl Logger {
    const ERROR: &str = "\x1b[1;91m";
    const INFO: &str = "\x1b[1;94m";
    const RESET: &str = "\x1b[0;0m";

    fn level_name(level: &Level) -> String {
        match *level {
            Level::Error => format!("{}[ERROR]{}", Logger::ERROR, Logger::RESET),
            Level::Info => format!("{}[+]{}", Logger::INFO, Logger::RESET),
            _ => format!("{}[{}]{}", Logger::RESET, level, Logger::RESET),
        }
    }

    fn format_log(buf: &mut Formatter, record: &Record<'_>) -> Result<(), Error> {
        let sep = format!("\n |  ");
        writeln!(
            buf,
            "{} {}",
            Logger::level_name(&record.level()),
            format!("{}", record.args()).replace("\n", &sep),
        )
    }

    pub fn init(level_fitler: LevelFilter) {
        Builder::new()
            .format(Logger::format_log)
            .filter(None, level_fitler)
            .init();
    }
}
