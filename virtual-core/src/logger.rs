pub struct Logger;

impl Logger {
    pub fn info(msg: &str) {
        println!("\x1b[0;34mInfo: {msg}\x1b[0m");
    }

    pub fn error(msg: &str) {
        eprintln!("\x1b[0;31mError: {msg}\x1b[0m");
        std::process::exit(-1);
    }
}