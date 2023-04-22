use std::{env, process, collections::HashMap};

use keywords::BranchingCodes;

mod executor;
mod keywords;
mod loading;
mod logger;

fn execute_instruction(instruction: u32, registers: &mut [i64; 16], flags: &mut HashMap<String, bool>) {
    let executor = executor::Executor;
    //println!("instruction: {:032b}", instruction);
    //println!("--- OPCODE ---");
    let opcode = (instruction >> 20) & 0b1111; // 0xf
    let ope1 = registers[(instruction >> 16 & 0xf) as usize];
    let ivf = (instruction >> 24) & 0b1;
    let ope2: i64 = if ivf == 1 {
        //println!("Immediate Value Present");
        (instruction & 0b11111111).try_into().unwrap()
    } else {
        registers[(instruction >> 12 & 0xf) as usize]
    };
    let dest: &mut i64 = &mut registers[(instruction >> 8 & 0xf) as usize];
    let carry_flag: &mut bool = flags.get_mut(&String::from("carry")).unwrap();


    match opcode.try_into() {
        Ok(keywords::OperationCodes::AND) => executor.and(ope1, ope2, dest),
        Ok(keywords::OperationCodes::ORR) => executor.orr(ope1, ope2, dest),
        Ok(keywords::OperationCodes::EOR) => executor.eor(ope1, ope2, dest),
        Ok(keywords::OperationCodes::ADD) => executor.add(ope1, ope2, dest, carry_flag),
        Ok(keywords::OperationCodes::ADC) => executor.adc(ope1, ope2, dest, carry_flag),
        Ok(keywords::OperationCodes::CMP) => executor.cmp(ope1, ope2, flags),
        Ok(keywords::OperationCodes::SUB) => executor.sub(ope1, ope2, dest, carry_flag),
        Ok(keywords::OperationCodes::SBC) => executor.sbc(ope1, ope2, dest, carry_flag),
        Ok(keywords::OperationCodes::MOV) => executor.mov(ope2, dest),
        Ok(keywords::OperationCodes::LSH) => executor.lsh(ope1, ope2, dest, carry_flag),
        Ok(keywords::OperationCodes::RSH) => executor.rsh(ope1, ope2, dest, carry_flag),
        _ => println!("Unknown opcode"),
    }
    //println!("r0  = {:#018x}", registers[0]);
    //println!("r1  = {:#018x}", registers[1]);
    //println!("r2  = {:#018x}", registers[2]);
    //println!("r3  = {:#018x}", registers[3]);
    //println!("r13 = {:#018x}", registers[13]);
    //println!("r14 = {:#018x}", registers[14]);
    //println!("r15 = {:#018x}", registers[15]);
    //println!("carry = {0}", flags.get(&String::from("carry")).unwrap());
    //println!("BNE = {0}", flags.get(&String::from("BNE")).unwrap());


    //println!("---------------");
}

fn fetch(instruction: u32, pc: u32, flags: &mut HashMap<String, bool>, is_verbose: bool) -> (bool, u32) {
    let bcc = (instruction >> 28) & 0b1111;
    let mut offset: i32 = 0;
    if bcc != 0 {
        let branching_opcode: BranchingCodes = bcc.try_into().unwrap();
        let string = branching_opcode.to_string();
        if bcc == 8 || *flags.get(&string).unwrap() {
            let sign_bit = instruction >> 27 & 0b1;
            offset = (-1 as i32).pow(sign_bit) * (instruction & 0x7ffffff) as i32;
        } else {
            offset += 1;
        }
    } else {
        offset += 1;
    }
    let (new_pc, _) = pc.overflowing_add_signed(offset);

    if is_verbose {
        //println!("------- fetch -----");
        //println!("\tBCC: {:02x}", bcc);
        //println!("\toffset: {offset}");
        //println!("\tinstruction: {:032b}", instruction);
        //println!("\tNew PC: {new_pc}");
    }

    return (bcc != 0, new_pc)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        //println!("Usage: cargo run <CODE> <STATE> (VERBOSE)");
        process::exit(0);
    }

    // Set registers
    let mut registers: [i64; 16] = [0; 16];
    loading::set_internal_state(args[2].to_string(), &mut registers);
    // for r in registers.iter() {
    //     ////println!("{r}");
    // }

    // Set flags
    let mut flags = loading::init_flags();


    // Load instructions
    let mut instructions: Vec<u32> = Vec::new();
    loading::load_binary_file(args[1].to_string(), &mut instructions);

    let mut pc: u32 = 0;
    let is_verbose: bool = true;
    let instructions_size = instructions.len().try_into().unwrap();
    while pc < instructions_size {
        //println!("\n-------------");
        let instruction = instructions[pc as usize];
        //println!("Current PC: {pc}");
        let (is_bcc, new_pc) = fetch(instruction, pc, &mut flags, is_verbose);
        if !is_bcc {
            // decode
            // execute
            execute_instruction(instruction, &mut registers, &mut flags);
        }
        pc = new_pc;
        //println!("-------------\n");
    }
    for (i, r) in registers.iter().enumerate() {
        println!("r{:#02} = {:#018x}", i, r);
    }

}
