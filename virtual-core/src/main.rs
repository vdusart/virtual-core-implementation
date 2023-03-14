use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::{env, i64, process};

mod executor;

enum OperationCodes  {
    AND = 0x0,
    ORR = 0x1,
    EOR = 0x2,
    ADD = 0x3,
    ADC = 0x4,
    CMP = 0x5,
    SUB = 0x6,
    SBC = 0x7,
    MOV = 0x8,
    LSH = 0x9,
    RSH = 0xa
}

impl TryFrom<u32> for OperationCodes {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == OperationCodes::ADD as u32 => Ok(OperationCodes::ADD),
            _ => Err(()),
        }
    }
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

// Sets the internal state
fn set_internal_state(filename: String, registers: &mut Vec<i64>) {
    let lines = read_lines(filename);
    for line in lines {
        let line = line.unwrap();
        let splitted_line: Vec<&str> = line.split("0x").collect();
        let str_value = splitted_line.get(1).unwrap();
        let value = i64::from_str_radix(&str_value, 16).unwrap();
        registers.push(value);
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

fn execute_instruction(instruction: u32, registers: &mut Vec<i64>) {
    let executor = executor::Executor;
    println!("instruction: {:032b}", instruction);
    let bcc = (instruction >> 28) & 0b1111;
    // println!("BCC: {:04b}", bcc);
    if bcc == 0 {
        println!("--- OPCODE ---");
        let opcode = (instruction >> 20) & 0b1111; // 0xf
        let ope1 = registers[(instruction >> 16 & 0xf) as usize];
        let ivf = (instruction >> 24) & 0b1;
        let ope2: i64 = if ivf == 1 {
            println!("Immediate Value Present");
            (instruction & 0b11111111).try_into().unwrap()
        } else {
            registers[(instruction >> 12 & 0xf) as usize]
        };
        let dest: &mut i64 = &mut registers[(instruction >> 8 & 0xf) as usize];



        match opcode.try_into() {
            Ok(OperationCodes::ADD) => executor.add(ope1, ope2, dest),
            _ => println!("Unknown opcode"),
        }
        println!("r2 = {0}", registers[2]);
        println!("r3 = {0}", registers[3]);


    } else {
        println!("--- BCC ---");
    }
    println!("---------------");
    // xxd -b -c 4 -g 0 file.bin | cut -d' ' -f2
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run <CODE> <STATE> (VERBOSE)");
        process::exit(0);
    }

    // Set the registre
    let mut registers: Vec<i64> = Vec::new();
    set_internal_state(args[2].to_string(), &mut registers);
    // for r in registers.iter() {
    //     println!("{r}");
    // }

    // Load instructions
    let mut instructions: Vec<u32> = Vec::new();
    load_binary_file(args[1].to_string(), &mut instructions);
    for instruction in instructions.iter() {
        // println!("{:032b}", instruction);
        execute_instruction(*instruction, &mut registers);
    }
    for r in registers.iter() {
        println!("{r}");
    }

}
