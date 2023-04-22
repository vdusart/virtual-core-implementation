pub struct Logger;

impl Logger {
    pub fn info(msg: String) {
        println!("\x1b[0;34m[+] {msg}\x1b[0m");
    }

    pub fn warning(msg: String) {
        println!("\x1b[0;33m[-] {msg}\x1b[0m");
    }

    pub fn error(msg: String) {
        eprintln!("\x1b[0;31m[X] {msg}\x1b[0m");
        std::process::exit(-1);
    }
}