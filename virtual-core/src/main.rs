use colog;
use std::{env, process};

mod executor;
mod keywords;
mod loading;
mod pipeline;

// Returns a string containing the current state of the registers
// (Only for the verbose mode)
pub fn current_register_states(registers: &[i64; 16]) -> String {
    let mut states = String::from("Current register states.");
    for (i, r) in registers.iter().enumerate() {
        states = format!("{}\nr{:#02} = {:#018x}", states, i, r);
    }
    states
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut clog = colog::builder();
    clog.filter(None, log::LevelFilter::Error);

    if args.len() < 3 {
        clog.init();
        log::error!("Usage: cargo run [BINARY FILE] [INITIAL STATE FILE] (--verbose)");
        process::exit(0);
    }

    // Setting the log level to Info, if the user chooses the verbose mode
    if args.len() > 3 && args[3] == "--verbose" {
        clog.filter(None, log::LevelFilter::Info);
    }
    clog.init();

    // Set registers
    let mut registers: [i64; 16] = [0; 16];
    loading::set_internal_state(args[2].to_string(), &mut registers);
    log::info!("{}", current_register_states(&registers));

    // Set flags
    let mut flags = loading::init_flags();

    // Load instructions
    let mut instructions: Vec<u32> = Vec::new();
    loading::load_binary_file(args[1].to_string(), &mut instructions);

    let mut pc: u32 = 0;

    let instructions_size = instructions.len().try_into().unwrap();
    while pc < instructions_size {
        let instruction = instructions[pc as usize];
        log::info!("");
        log::info!("");
        log::info!("");
        log::info!("+--------------------------------------------------------+");
        log::info!("| Current instruction [{:032b}] |", instruction);
        log::info!("+--------------------------------------------------------+");
        log::info!("");
        let (is_branch, new_pc) = pipeline::fetch(instruction, pc, &flags);
        if !is_branch {
            let decoded_instruction = pipeline::decode(instruction, &mut registers);
            pipeline::execute(decoded_instruction, &mut flags);

            if log::max_level() == log::LevelFilter::Info {
                let mut execute_infos = String::from("-> EXECUTE\n");
                execute_infos.push_str(&current_register_states(&registers));
                execute_infos.push_str("\n\n");
                execute_infos.push_str(&flags.current_flag_states());
                log::info!("{}", execute_infos);
            }
        }
        pc = new_pc;
    }
    log::info!("Final register states");
    println!("{}", current_register_states(&registers));
}
