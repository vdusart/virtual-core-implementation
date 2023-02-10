use std::fs::File;
use std::{env, process, i64};
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap(); 
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines(); 
}

// Sets the internal state
fn set_internal_state(filename: String, registres: &mut Vec<i64>) {
    let lines = read_lines(filename);
    for line in lines {
        let line = line.unwrap();
        let splitted_line: Vec<&str> = line.split("0x").collect();
        let str_value = splitted_line.get(1).unwrap();
        let value = i64::from_str_radix(&str_value, 16).unwrap();
        registres.push(value);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run <CODE> <STATE> (VERBOSE)");
        process::exit(0);
    }
    let mut registres: Vec<i64> = Vec::new();
    set_internal_state(args[2].to_string(), &mut registres);
    for r in registres.iter() {
        println!("{r}");
    }
}
