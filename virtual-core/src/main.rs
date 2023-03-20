use std::{env, process, collections::HashMap};

mod executor;
mod keywords;
mod loading;

fn execute_instruction(instruction: u32, registers: &mut Vec<u64>, flags: &mut HashMap<String, bool>) {
    let executor = executor::Executor;
    println!("instruction: {:032b}", instruction);
    let bcc = (instruction >> 28) & 0b1111;
    // println!("BCC: {:04b}", bcc);
    if bcc == 0 {
        println!("--- OPCODE ---");
        let opcode = (instruction >> 20) & 0b1111; // 0xf
        let ope1 = registers[(instruction >> 16 & 0xf) as usize];
        let ivf = (instruction >> 24) & 0b1;
        let ope2: u64 = if ivf == 1 {
            println!("Immediate Value Present");
            (instruction & 0b11111111).try_into().unwrap()
        } else {
            registers[(instruction >> 12 & 0xf) as usize]
        };
        let dest: &mut u64 = &mut registers[(instruction >> 8 & 0xf) as usize];
        let carry_flag: &mut bool = flags.get_mut(&String::from("carry")).unwrap();


        match opcode.try_into() {
            Ok(keywords::OperationCodes::AND) => executor.and(ope1, ope2, dest),
            Ok(keywords::OperationCodes::ORR) => executor.orr(ope1, ope2, dest),
            Ok(keywords::OperationCodes::EOR) => executor.eor(ope1, ope2, dest),
            Ok(keywords::OperationCodes::ADD) => executor.add(ope1, ope2, dest, carry_flag),
            Ok(keywords::OperationCodes::ADC) => executor.adc(ope1, ope2, dest, carry_flag),
            Ok(keywords::OperationCodes::CMP) => executor.cmp(ope1, ope2, dest),
            Ok(keywords::OperationCodes::SUB) => executor.sub(ope1, ope2, dest, carry_flag),
            Ok(keywords::OperationCodes::SBC) => executor.sbc(ope1, ope2, dest, carry_flag),
            Ok(keywords::OperationCodes::MOV) => executor.mov(ope2, dest),
            Ok(keywords::OperationCodes::LSH) => executor.lsh(ope1, ope2, dest, carry_flag),
            Ok(keywords::OperationCodes::RSH) => executor.rsh(ope1, ope2, dest, carry_flag),
            _ => println!("Unknown opcode"),
        }
        println!("r1 = {0}", registers[1]);
        println!("r2 = {0}", registers[2]);
        println!("r3 = {0}", registers[3]);
        println!("carry = {0}", flags.get(&String::from("carry")).unwrap());


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

    // Set registers
    let mut registers: Vec<u64> = Vec::new();
    loading::set_internal_state(args[2].to_string(), &mut registers);
    // for r in registers.iter() {
    //     println!("{r}");
    // }

    // Set flags
    let mut flags = loading::init_flags();


    // Load instructions
    let mut instructions: Vec<u32> = Vec::new();
    loading::load_binary_file(args[1].to_string(), &mut instructions);
    for instruction in instructions.iter() {
        // println!("{:032b}", instruction);
        // fetch
        // decode
        execute_instruction(*instruction, &mut registers, &mut flags);
    }
    for (i, r) in registers.iter().enumerate() {
        println!("r{:#02} = {:#018x}", i, r);
    }

}
