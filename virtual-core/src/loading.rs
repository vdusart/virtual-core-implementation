use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::collections::HashMap;

use crate::keywords::BranchingCodes;
use crate::logger::Logger;

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();
}

fn from_str_radix_u64_to_i64(raw_str: &str, radix: u32) -> i64 {
    let mut value = 0;
    match u64::from_str_radix(raw_str, radix) {
        Ok(z) => value = z,
        Err(e) => {
            let prefix = if radix == 10 { "" } else { "0x" };
            let error_msg = &format!("Invalid initial register state:\n{}: '{}{}'", &e.to_string(), prefix, raw_str);
            Logger::error(&error_msg)
        }
    };
    value as i64
}

// Sets the internal state
pub fn set_internal_state(filename: String, registers: &mut [i64; 16]) {
    let lines = read_lines(filename);
    for line_or_error in lines {
        let line = line_or_error.unwrap();
        let splitted_line: Vec<&str> = line.split("=0x").collect();
        if splitted_line.len() != 2 {
            let error_msg = format!("Internal state file is malformed.\nline: '{}'", line);
            Logger::error(&error_msg);
        }

        let str_index = splitted_line.get(0).unwrap();
        if str_index.len() < 2 || !str_index.starts_with('r') {
            let error_msg = format!("Internal state file is malformed.\nline: '{}'.", line);
            Logger::error(&error_msg);
        }
        let index = from_str_radix_u64_to_i64(&str_index[1..str_index.len()], 10);
        if index < 0 || index > 15 {
            let error_msg = format!("Internal state file is malformed.\nline: '{}'\nri with i ∈ [0;15].", line);
            Logger::error(&error_msg);
        }

        let str_value = splitted_line.get(1).unwrap();
        let value = from_str_radix_u64_to_i64(str_value, 16);
        registers[index as usize] = value;
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

pub fn init_flags() -> HashMap<String, bool> {
    let mut flags: HashMap<String, bool> = HashMap::new();
    flags.insert(BranchingCodes::BEQ.to_string(), false);
    flags.insert(BranchingCodes::BNE.to_string(), false);
    flags.insert(BranchingCodes::BLE.to_string(), false);
    flags.insert(BranchingCodes::BGE.to_string(), false);
    flags.insert(BranchingCodes::BG.to_string(), false);
    flags.insert(BranchingCodes::BL.to_string(), false);
    flags.insert(String::from("carry"), false);
    flags
}
