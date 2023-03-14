use std::{env, process};

mod executor;
mod keywords;
mod loading;

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
            Ok(keywords::OperationCodes::ADD) => executor.add(ope1, ope2, dest),
            Ok(keywords::OperationCodes::SUB) => executor.sub(ope1, ope2, dest),
            Ok(keywords::OperationCodes::MOV) => executor.mov(ope2, dest),
            _ => println!("Unknown opcode"),
        }
        println!("r1 = {0}", registers[1]);
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
    loading::set_internal_state(args[2].to_string(), &mut registers);
    // for r in registers.iter() {
    //     println!("{r}");
    // }

    // Load instructions
    let mut instructions: Vec<u32> = Vec::new();
    loading::load_binary_file(args[1].to_string(), &mut instructions);
    for instruction in instructions.iter() {
        // println!("{:032b}", instruction);
        execute_instruction(*instruction, &mut registers);
    }
    for (i, r) in registers.iter().enumerate() {
        println!("r{i} = {r}");
    }

}
