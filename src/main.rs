use strum::IntoEnumIterator;

use crate::bus::Bus32;
use crate::main_memory::{fast_decode, fast_encode, MainMemory};
use crate::memory::{Memory512x36, Register32, Register9};
use crate::microasm::MicroAsm;
use crate::microasm::MicroAsm::Main1;
use crate::parser::parse;
use crate::processor::Mic1;

mod parser;
mod shifter;
mod asm;
mod main_memory;
mod microasm;
mod processor_elements;
mod processor;
mod bus;
mod memory;
mod decoders;
mod alu;

const PROGRAM: &str = r#"
        IADD
"#;

const PROGRAM_START: usize = 100;

fn main() {
    let mut memory = MainMemory::initialize();

    /*
    Stack start = 10
    LV = 10
    */

    // Stack
    memory.write_data(12, 10);
    memory.write_data(4, 11);

    let sp_value = 11;

    // Program
    let commands = parse(PROGRAM);
    let mut p_counter = PROGRAM_START;
    for command in &commands {
        memory.write_data(*command, p_counter);
        p_counter += 1;
    }

    let control_memory = make_control_memory();

    let mut tos = Register32::new();
    tos.update_from_bus(&Bus32::from(fast_decode(fast_encode(&memory.read(fast_decode(sp_value))))), true);

    let mut pc = Register32::new();
    pc.update_from_bus(&Bus32::from(fast_decode(99)), true);

    let mut sp = Register32::new();
    sp.update_from_bus(&Bus32::from(fast_decode(sp_value)), true);

    let mut mpc = Register9::new();
    let mut mpc_data = [false; 9];
    let decoded = fast_decode(Main1 as i32);
    for x in 0..9 {
        mpc_data[x] = decoded[x];
    }
    mpc.update(mpc_data, true);

    let mut mic1 = Mic1::init(memory, control_memory, tos, pc, sp, mpc);

    let last_command = commands.len() + 1 + PROGRAM_START;
    let mut pc_counter = 0;
    while pc_counter < last_command {
        mic1.execute_command();
        pc_counter = fast_encode(&mic1.pc.get()) as usize;
    }

    let mut res = [false; 32];
    for i in 0..32 {
        res[i] = mic1.tos.registers[i].state;
    }
    let tos_res = fast_encode(&res);
    print!("{:?}", tos_res)
}

fn make_control_memory() -> Memory512x36 {
    let mut control_memory = Memory512x36::new();

    for command in MicroAsm::iter() {
        control_memory.write_data(command.command(), command as usize)
    }
    return control_memory;
}
