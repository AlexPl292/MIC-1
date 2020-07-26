use crate::bus::Bus32;
use crate::decoders::{decoder_4x9, decoder_9x512};
use crate::main_memory::{fast_decode, MainMemory, fast_encode};
use crate::memory::{Memory512x36, Register32, Register9};
use crate::microasm::MicroAsm::{iadd1, iadd2, iadd3, Main1};
use crate::processor::Mic1;
use crate::microasm::MicroAsm;
use strum::IntoEnumIterator;
use crate::asm::IjvmCommand::IADD;

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
mod elements;

fn main() {
    let mut memory = MainMemory::initialize();

    /*
    Stack start = 10
    LV = 10

    program start = 100
    */

    // Stack
    memory.write_data(12, 10);
    memory.write_data(4, 11);

    // Program
    memory.write_data(IADD as i32, 100);

    let mut control_memory = make_control_memory();

    let mut tos = Register32::new();
    tos.update_from_bus(&Bus32::from(fast_decode(4)), true);

    let mut pc = Register32::new();
    pc.update_from_bus(&Bus32::from(fast_decode(99)), true);

    let mut sp = Register32::new();
    sp.update_from_bus(&Bus32::from(fast_decode(11)), true);

    let mut mpc = Register9::new();
    let mut mpc_data = [false; 9];
    let decoded = fast_decode(Main1 as i32);
    for x in 0..9 {
        mpc_data[x] = decoded[x];
    }
    mpc.update(mpc_data, true);

    let mut mic1 = Mic1::init(memory, control_memory, tos, pc, sp, mpc);

    mic1.execute_command();
    mic1.execute_command();
    mic1.execute_command();
    mic1.execute_command();
    mic1.execute_command();
    mic1.execute_command();

    let mut res = [false; 32];
    for i in 0..32 {
        res[i] = mic1.tos.registers[i].state;
    }
    let tos_res = fast_encode(res);
    print!("{:?}", tos_res)
}

fn make_control_memory() -> Memory512x36 {
    let mut control_memory = Memory512x36::new();

    for command in MicroAsm::iter() {
        control_memory.write_data(command.command(), command as usize)
    }
    return control_memory;
}
