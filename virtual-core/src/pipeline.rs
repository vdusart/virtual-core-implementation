use crate::executor;
use crate::keywords::{BranchingCodes, Flags, OperationCodes};

pub struct DecodedInstruction<'a> {
    opcode: u32,
    ope1: i64,
    ope2: i64,
    dest: &'a mut i64,
}

pub fn fetch(instruction: u32, pc: u32, flags: &Flags) -> (bool, u32) {
    let bcc = (instruction >> 28) & 0b1111;
    let mut offset: i32 = 0;
    let mut branching_name = String::from("N/A");
    if bcc != 0 {
        let branching_opcode: BranchingCodes = bcc.try_into().unwrap();
        branching_name = branching_opcode.to_string();
        if bcc == 8 || flags.get_from_bcc(branching_opcode) {
            let sign_bit = instruction >> 27 & 0b1;
            offset = (-1 as i32).pow(sign_bit) * (instruction & 0x7ffffff) as i32;
        } else {
            offset += 1;
        }
    } else {
        offset += 1;
    }
    let (new_pc, _) = pc.overflowing_add_signed(offset);

    if log::max_level() == log::LevelFilter::Info {
        let mut fetch_infos = String::from("-> FETCH\n");
        fetch_infos.push_str(&format!("\tBCC: {} ({})\n", bcc, branching_name));
        fetch_infos.push_str(&format!("\tPC: {}\n", pc));
        fetch_infos.push_str(&format!("\tOffset: {}\n", offset));
        fetch_infos.push_str(&format!("\tNew PC: {}", new_pc));
        log::info!("{}", fetch_infos);
    }

    return (bcc != 0, new_pc);
}

pub fn decode(instruction: u32, registers: &mut [i64; 16]) -> DecodedInstruction {
    let opcode = (instruction >> 20) & 0b1111; // 0xf
    let ope1 = registers[(instruction >> 16 & 0xf) as usize];
    let ivf = (instruction >> 24) & 0b1;
    let ope2: i64 = if ivf == 1 {
        (instruction & 0b11111111).try_into().unwrap()
    } else {
        registers[(instruction >> 12 & 0xf) as usize]
    };
    let dest: &mut i64 = &mut registers[(instruction >> 8 & 0xf) as usize];

    if log::max_level() == log::LevelFilter::Info {
        let opcode_entity: OperationCodes = opcode.try_into().unwrap();
        let mut decode_infos = String::from("-> DECODE\n");
        decode_infos.push_str(&format!(
            "\tOPCODE: {:02x} ({})\n",
            opcode,
            opcode_entity.to_string()
        ));
        decode_infos.push_str(&format!("\tope1: {:#018x}\n", ope1));
        decode_infos.push_str(&format!("\tIVF: {}\n", ivf));
        decode_infos.push_str(&format!("\tope2: {:#018x}", ope2));
        log::info!("{}", decode_infos);
    }

    DecodedInstruction {
        opcode,
        ope1,
        ope2,
        dest,
    }
}

pub fn execute(instruction: DecodedInstruction, flags: &mut Flags) {
    let DecodedInstruction {
        opcode,
        ope1,
        ope2,
        dest,
    } = instruction;
    let executor = executor::Executor;

    let carry_flag: &mut bool = &mut flags.carry;

    match opcode.try_into() {
        Ok(OperationCodes::AND) => executor.and(ope1, ope2, dest),
        Ok(OperationCodes::ORR) => executor.orr(ope1, ope2, dest),
        Ok(OperationCodes::EOR) => executor.eor(ope1, ope2, dest),
        Ok(OperationCodes::ADD) => executor.add(ope1, ope2, dest, carry_flag),
        Ok(OperationCodes::ADC) => executor.adc(ope1, ope2, dest, carry_flag),
        Ok(OperationCodes::CMP) => executor.cmp(ope1, ope2, flags),
        Ok(OperationCodes::SUB) => executor.sub(ope1, ope2, dest, carry_flag),
        Ok(OperationCodes::SBC) => executor.sbc(ope1, ope2, dest, carry_flag),
        Ok(OperationCodes::MOV) => executor.mov(ope2, dest),
        Ok(OperationCodes::LSH) => executor.lsh(ope1, ope2, dest, carry_flag),
        Ok(OperationCodes::RSH) => executor.rsh(ope1, ope2, dest, carry_flag),
        _ => println!("Unknown opcode"),
    }
}
