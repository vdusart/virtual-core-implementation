use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();
}

// Sets the internal state
pub fn set_internal_state(filename: String, registers: &mut Vec<i64>) {
    let lines = read_lines(filename);
    for line_or_error in lines {
        let line = line_or_error.unwrap();
        let splitted_line: Vec<&str> = line.split("0x").collect();
        let str_value = splitted_line.get(1).unwrap();
        let value = i64::from_str_radix(&str_value, 16).unwrap();
        registers.push(value);
    }
}

pub fn load_binary_file(filename: String, instructions: &mut Vec<u32>) {
    let file_buffer = BufReader::new(File::open(filename).unwrap());
    let bytes = file_buffer.bytes();
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