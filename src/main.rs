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

const STACK_START: i32 = 10;

fn main() {
    let commands = parse(PROGRAM);
    let mut mic1 = create_processor(&commands, vec![2, 3]);

    mic1.run(commands.len() + 1, PROGRAM_START);

    let tos_res = fast_encode(&mic1.tos.read(true));
    print!("{:?}", tos_res)
}

fn create_processor(commands: &Vec<i32>, initial_stack: Vec<i32>) -> Mic1 {
    let mut memory = MainMemory::initialize();

    // Stack
    let mut stack_pointer = STACK_START;
    let mut top_of_stack = 0;
    for stack in initial_stack {
        memory.write_data(stack, stack_pointer as usize);
        stack_pointer += 1;
        top_of_stack = stack;
    }
    stack_pointer -= 1;

    // Program
    let mut p_counter = PROGRAM_START;
    for command in commands {
        memory.write_data(*command, p_counter);
        p_counter += 1;
    }

    let control_memory = make_control_memory();

    let mut tos = Register32::new();
    tos.update_from_bus(&Bus32::from(fast_decode(top_of_stack)), true);

    let mut pc = Register32::new();
    pc.update_from_bus(&Bus32::from(fast_decode(99)), true);

    let mut lv = Register32::new();
    lv.update_from_bus(&Bus32::from(fast_decode(STACK_START)), true);

    let mut sp = Register32::new();
    sp.update_from_bus(&Bus32::from(fast_decode(stack_pointer)), true);

    let mut mpc = Register9::new();
    let mut mpc_data = [false; 9];
    let decoded = fast_decode(Main1 as i32);
    for x in 0..9 {
        mpc_data[x] = decoded[x];
    }
    mpc.update(mpc_data, true);

    Mic1::init(memory, control_memory, tos, pc, sp, lv, mpc)
}

fn make_control_memory() -> Memory512x36 {
    let mut control_memory = Memory512x36::new();

    for command in MicroAsm::iter() {
        control_memory.write_data(command.command(), command as usize)
    }
    return control_memory;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let commands = parse("IADD");
        let mut mic1 = create_processor(&commands, vec![1, 2]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        let tos_res = fast_encode(&mic1.tos.read(true));
        assert_eq!(3, tos_res)
    }

    #[test]
    fn add2() {
        let commands = parse("IADD");
        let mut mic1 = create_processor(&commands, vec![10, 20]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        let tos_res = fast_encode(&mic1.tos.read(true));
        assert_eq!(30, tos_res)
    }

    #[test]
    fn iload() {
        let commands = parse("ILOAD 0x00");
        let mut mic1 = create_processor(&commands, vec![10, 20]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        let tos_res = fast_encode(&mic1.tos.read(true));
        assert_eq!(10, tos_res)
    }

    #[test]
    fn iload2() {
        let commands = parse("ILOAD 0x01");
        let mut mic1 = create_processor(&commands, vec![10, 20, 30]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        let tos_res = fast_encode(&mic1.tos.read(true));
        assert_eq!(20, tos_res)
    }

    #[test]
    fn sub() {
        let commands = parse("ISUB");
        let mut mic1 = create_processor(&commands, vec![2, 1]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        let tos_res = fast_encode(&mic1.tos.read(true));
        assert_eq!(1, tos_res)
    }

    #[test]
    fn sub2() {
        let commands = parse("ISUB");
        let mut mic1 = create_processor(&commands, vec![20, 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        let tos_res = fast_encode(&mic1.tos.read(true));
        assert_eq!(10, tos_res)
    }
}
