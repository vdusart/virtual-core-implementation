use std::{env, process};

mod executor;
mod keywords;
mod loading;
mod logger;
mod pipeline;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        logger::Logger::init(log::LevelFilter::Error);
        log::error!("Usage: cargo run [BINARY FILE] [INITIAL STATE FILE] (--verbose)");
        process::exit(0);
    }

    if args.len() > 3 && args[3] == "--verbose" {
        logger::Logger::init(log::LevelFilter::Info);
    } else {
        logger::Logger::init(log::LevelFilter::Error);
    }

    // Set registers
    let mut registers: [i64; 16] = [0; 16];
    loading::set_internal_state(args[2].to_string(), &mut registers);
    log::info!("{}", logger::current_register_states(&registers));

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
                execute_infos.push_str(&logger::current_register_states(&registers));
                execute_infos.push_str("\n\n");
                execute_infos.push_str(&flags.current_flag_states());
                log::info!("{}", execute_infos);
            }
        }
        pc = new_pc;
    }
    log::info!("Final register states");
    println!("{}", logger::current_register_states(&registers));
}
