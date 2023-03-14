use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::{env, i64, process};

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

fn load_binary_file(filename: String, instructions: &mut Vec<u32>) {
    let my_buf = BufReader::new(File::open(filename).unwrap());
    let bytes = my_buf.bytes();
    let mut tmp = 0;
    let mut i: u32 = 0;
    for byte_or_error in bytes {
        i += 1;
        let byte = byte_or_error.unwrap();
        tmp += (byte as u32) << (8 * (4 - i));
        if i == 4 {
            instructions.push(tmp);
            i = 0;
            tmp = 0;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run <CODE> <STATE> (VERBOSE)");
        process::exit(0);
    }

    // Set the registre
    let mut registres: Vec<i64> = Vec::new();
    set_internal_state(args[2].to_string(), &mut registres);
    // for r in registres.iter() {
    //     println!("{r}");
    // }

    // Load instructions
    let mut instructions: Vec<u32> = Vec::new();
    load_binary_file(args[1].to_string(), &mut instructions);
    for instruction in instructions.iter() {
        println!("{:032b}", instruction);
    }
}
