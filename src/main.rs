use crate::main_memory::{MainMemory, fast_decode};
use crate::memory::{Memory512x36, Register32};
use crate::microasm::commands;
use crate::bus::Bus32;
use crate::processor::Mic1;

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

    // Stack
    memory.write_data(1, 10);
    memory.write_data(2, 11);

    // Program
    memory.write_data(0, 100);

    let commands = commands();
    let mut control_memory = Memory512x36::new();

    control_memory.write_data(commands.get("iadd1").unwrap().clone(), 0);
    control_memory.write_data(commands.get("iadd2").unwrap().clone(), 1);
    control_memory.write_data(commands.get("iadd3").unwrap().clone(), 2);

    let mut tos = Register32::new();
    tos.update_from_bus(&Bus32::from(fast_decode(2)), true);

    let mut pc = Register32::new();
    pc.update_from_bus(&Bus32::from(fast_decode(100)), true);

    let mut sp = Register32::new();
    sp.update_from_bus(&Bus32::from(fast_decode(11)), true);

    let mut mic1 = Mic1::init(memory, control_memory, tos, pc, sp);

    mic1.execute_command();
    mic1.execute_command();
}
