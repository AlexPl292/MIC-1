#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use strum::IntoEnumIterator;
use tree_sitter::{Language, Parser};

use crate::bus::Bus32;
use crate::main_memory::{fast_decode, fast_encode, MainMemory};
use crate::memory::{Memory512x36, Register32, Register9};
use crate::microasm::MicroAsm;
use crate::microasm::MicroAsm::Main1;
use crate::parser::parse;
use crate::processor::Mic1;
use crate::compiler::{ProcessorInfo, compile};

mod compiler;
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

extern "C" { fn tree_sitter_jas() -> Language; }

const PROGRAM: &str = r#"
        .constant
            OBJREF 0
        .end-constant
        .main
            LDC_W OBJREF
            BIPUSH 1
            BIPUSH 2
            INVOKEVIRTUAL sum
        .end-main
        .method sum(first, second)
            ILOAD first
            ILOAD second
            IADD
            IRETURN
        .end-method
"#;

const PROGRAM_START: usize = 100;

const STACK_START: i32 = 10;

fn main() {
    let info = compile(PROGRAM, PROGRAM_START as u32, Some(0xFF));
    let mut mic1 = create_processor_from_info(&info);

    mic1.run_until_stop(0xFF);

    let tos_res = fast_encode(&mic1.tos.read(true));
    print!("Result: {:?}", tos_res)
}

fn create_processor_from_info(info: &ProcessorInfo) -> Mic1 {
    let mut constants = [0; 10];
    for x in 0..10 {
        if x >= info.constants.len() {
            break
        }
        constants[x] = *info.constants.get(x).unwrap();
    }
    create_processor(&info.main_program, Vec::new(), constants)
}

fn create_processor(commands: &Vec<i32>, initial_stack: Vec<i32>, constants: [i32; 10]) -> Mic1 {
    let mut memory = MainMemory::initialize();

    // Constants
    for x in 0..10 {
        memory.write_data(constants[x], x)
    }

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
    use crate::compiler::compile;

    #[test]
    fn add() {
        let commands = parse("IADD");
        let mut mic1 = create_processor(&commands, vec![1, 2], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        let tos_res = fast_encode(&mic1.tos.read(true));
        assert_eq!(3, tos_res)
    }

    #[test]
    fn add2() {
        let commands = parse("IADD");
        let mut mic1 = create_processor(&commands, vec![10, 20], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        let tos_res = fast_encode(&mic1.tos.read(true));
        assert_eq!(30, tos_res)
    }

    #[test]
    fn iload() {
        let commands = parse("ILOAD 0x00");
        let mut mic1 = create_processor(&commands, vec![10, 20], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        let tos_res = fast_encode(&mic1.tos.read(true));
        assert_eq!(10, tos_res)
    }

    #[test]
    fn iload2() {
        let commands = parse("ILOAD 0x01");
        let mut mic1 = create_processor(&commands, vec![10, 20, 30], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        let tos_res = fast_encode(&mic1.tos.read(true));
        assert_eq!(20, tos_res)
    }

    #[test]
    fn sub() {
        let commands = parse("ISUB");
        let mut mic1 = create_processor(&commands, vec![2, 1], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        let tos_res = fast_encode(&mic1.tos.read(true));
        assert_eq!(1, tos_res)
    }

    #[test]
    fn sub2() {
        let commands = parse("ISUB");
        let mut mic1 = create_processor(&commands, vec![20, 10], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        let tos_res = fast_encode(&mic1.tos.read(true));
        assert_eq!(10, tos_res)
    }

    #[test]
    fn bipush() {
        let commands = parse("BIPUSH 0x01");
        let mut mic1 = create_processor(&commands, vec![], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        let tos_res = fast_encode(&mic1.tos.read(true));
        assert_eq!(0x01, tos_res)
    }

    #[test]
    fn bipush2() {
        let commands = parse("BIPUSH 0x0B");
        let mut mic1 = create_processor(&commands, vec![], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        let tos_res = fast_encode(&mic1.tos.read(true));
        assert_eq!(11, tos_res)
    }

    #[test]
    fn swap() {
        let commands = parse("SWAP");
        let mut mic1 = create_processor(&commands, vec![1, 2], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![2, 1], &mic1)
    }

    #[test]
    fn swap1() {
        let commands = parse("SWAP");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4, 5], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 2, 3, 5, 4], &mic1)
    }

    #[test]
    fn dup() {
        let commands = parse("DUP");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4, 5], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 2, 3, 4, 5, 5], &mic1)
    }

    #[test]
    fn dup2() {
        let commands = parse("DUP");
        let mut mic1 = create_processor(&commands, vec![1], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 1], &mic1)
    }

    #[test]
    fn pop() {
        let commands = parse("POP");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4, 5], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 2, 3, 4], &mic1)
    }

    #[test]
    fn pop2() {
        let commands = parse("POP");
        let mut mic1 = create_processor(&commands, vec![1, 2], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1], &mic1)
    }

    #[test]
    fn istore() {
        let commands = parse("ISTORE 0x1");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 4, 3], &mic1)
    }

    #[test]
    fn istore1() {
        let commands = parse("ISTORE 0x0");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4, 5], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![5, 2, 3, 4], &mic1)
    }

    #[test]
    fn wide_iload() {
        let commands = parse("WIDE\nILOAD 0x0 0x0");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4, 5], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 2, 3, 4, 5, 1], &mic1)
    }

    #[test]
    fn wide_iload2() {
        let commands = parse("WIDE\nILOAD 0x0 0x1");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4, 5], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 2, 3, 4, 5, 2], &mic1)
    }

    #[test]
    fn wide_istore() {
        let commands = parse("WIDE\nISTORE 0x0 0x0");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4, 5], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![5, 2, 3, 4], &mic1)
    }

    #[test]
    fn wide_istore2() {
        let commands = parse("WIDE\nISTORE 0x0 0x1");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4, 5], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 5, 3, 4], &mic1)
    }

    #[test]
    fn ldc_w() {
        let commands = parse("LDC_W 0x00 0x00");
        let constants = [1, 2, 3, 4, 0, 0, 0, 0, 0, 0];
        let mut mic1 = create_processor(&commands, vec![1, 2], constants);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 2, 1], &mic1)
    }

    #[test]
    fn ldc_w2() {
        let commands = parse("LDC_W 0x00 0x02");
        let constants = [1, 2, 3, 4, 0, 0, 0, 0, 0, 0];
        let mut mic1 = create_processor(&commands, vec![1, 2], constants);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 2, 3], &mic1)
    }

    #[test]
    fn ldc_w3() {
        let commands = parse("LDC_W 0x00 0x05");
        let constants = [1, 2, 3, 4, 5, 6, 0, 0, 0, 0];
        let mut mic1 = create_processor(&commands, vec![1, 2], constants);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 2, 6], &mic1)
    }

    #[test]
    fn iinc() {
        let commands = parse("IINC 0x00 0x05");
        let mut mic1 = create_processor(&commands, vec![1, 2], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![6, 2], &mic1)
    }

    #[test]
    fn iinc2() {
        let commands = parse("IINC 0x02 0x05");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 2, 8, 4], &mic1)
    }

    #[test]
    fn goto() {
        let commands = parse("GOTO 0x00 0x03\nIINC 0x00 0x01\nIADD");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![2, 2, 7], &mic1)
    }

    #[test]
    fn goto2() {
        let commands = parse("GOTO 0x00 0x05\nIINC 0x00 0x01\nIADD");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 2, 7], &mic1)
    }

    #[test]
    fn iflt() {
        let commands = parse("IFLT 0x00 0x05\nIINC 0x00 0x01\nIADD");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![2, 5], &mic1)
    }

    #[test]
    fn iflt1() {
        let commands = parse("IFLT 0x00 0x05\nIINC 0x00 0x01\nIADD");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, -4], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 5], &mic1)
    }

    #[test]
    fn ifeq() {
        let commands = parse("IFEQ 0x00 0x05\nIINC 0x00 0x01\nIADD");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![2, 5], &mic1)
    }

    #[test]
    fn ifeq1() {
        let commands = parse("IFEQ 0x00 0x05\nIINC 0x00 0x01\nIADD");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 0], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 5], &mic1)
    }

    #[test]
    fn if_icmpeq() {
        let commands = parse("IF_ICMPEQ 0x00 0x05\nIINC 0x00 0x01\nIADD");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4, 5], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![2, 5], &mic1)
    }

    #[test]
    fn if_icmpeq1() {
        let commands = parse("IF_ICMPEQ 0x00 0x05\nIINC 0x00 0x01\nIADD");
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4, 4], [0; 10]);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 5], &mic1)
    }

    #[test]
    fn invokevirtual() {
        let program = r#"
                BIPUSH 0x03
                INVOKEVIRTUAL 0x00 0x02
                IADD
                0x00  0x01
                0x00  0x00
                BIPUSH 0x03
"#;
        let commands = parse(program);
        let constants = [0xCA, 0x11, PROGRAM_START as i32 + 0x06, 4, 4, 5, 6, 0, 0, 0];
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4], constants);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 2, 3, 4, 15, 105, 10, 0x03], &mic1);

        let lv = fast_encode(&mic1.lv.get());
        assert_eq!(STACK_START + 4, lv);
    }

    #[test]
    fn invokevirtual1() {
        let program = r#"
                BIPUSH 0x03
                BIPUSH 0x05
                BIPUSH 0x06
                BIPUSH 0x06
                INVOKEVIRTUAL 0x00 0x02
                IADD
                0x00  0x04
                0x00  0x05
                BIPUSH 0x03
"#;
        let commands = parse(program);
        let constants = [0xCA, 0x11, PROGRAM_START as i32 + 0x0c, 4, 4, 5, 6, 0, 0, 0];
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4], constants);
        mic1.run(commands.len() + 1, PROGRAM_START);

        assert_stack(vec![1, 2, 3, 4, 23, 5, 6, 6, 0, 0, 0, 0, 0, 111, 10, 3], &mic1);

        let lv = fast_encode(&mic1.lv.get());
        assert_eq!(STACK_START + 4, lv);
    }

    #[test]
    fn invokevirtual2() {
        let program = r#"
                BIPUSH 0x02
                INVOKEVIRTUAL 0x00 0x02
                IADD
                0x00  0x01
                0x00  0x00
                BIPUSH 0x1c
                IRETURN
"#;
        let commands = parse(program);
        let constants = [0xCA, 0x11, PROGRAM_START as i32 + 0x06, 4, 4, 5, 6, 0, 0, 0];
        let mut mic1 = create_processor(&commands, vec![1, 2, 3, 4], constants);
        mic1.run_n_times(43);

        assert_stack(vec![1, 2, 3, 4, 0x1c], &mic1);

        let lv = fast_encode(&mic1.lv.get());
        assert_eq!(STACK_START, lv);
    }

    #[test]
    fn test_parser() {
        let language = unsafe { tree_sitter_jas() };
        let mut parser = Parser::new();
        parser.set_language(language).unwrap();

        let source_code = ".main\n.end-main";
        let tree = parser.parse(source_code, None).unwrap();

        assert_eq!(tree.root_node().to_sexp(), "(source_file (main_program))");
    }

    #[test]
    fn program_from_asm() {
        let source = r#"
           .main
               BIPUSH 0x01
               BIPUSH 0x02
               IADD
           .end-main
        "#;
        let compiled = compile(source, PROGRAM_START as u32, Some(0xFF));
        let mut mic1 = create_processor_from_info(&compiled);
        mic1.run_until_stop(0xFF);

        assert_stack(vec![3], &mic1);
    }

    #[test]
    fn program_from_asm_with_function() {
        let source = r#"
           .main
               BIPUSH 0x01
               BIPUSH 0x01
               BIPUSH 0x02
               INVOKEVIRTUAL sum
           .end-main
           .method sum(first, second)
               ILOAD first
               ILOAD second
               IADD
               IRETURN
           .end-method
        "#;
        let compiled = compile(source, PROGRAM_START as u32, Some(0xFF));
        let mut mic1 = create_processor_from_info(&compiled);
        mic1.run_until_stop(0xFF);

        assert_stack(vec![3], &mic1);
    }

    #[test]
    fn program_from_asm_with_function_and_variable_loading() {
        let source = r#"
           .main
               BIPUSH 0x02
               BIPUSH 0x03
               INVOKEVIRTUAL sum
           .end-main
           .method sum(first)
               ILOAD first
           .end-method
        "#;
        let compiled = compile(source, PROGRAM_START as u32, None);
        let mut mic1 = create_processor_from_info(&compiled);
        mic1.run_n_times(40);

        assert_stack(vec![12, 3, 107, 10, 3], &mic1);
    }

    fn assert_stack(expected_stack: Vec<i32>, mic1: &Mic1) {
        let stack_ptr = fast_encode(&mic1.sp.get());
        let stack_size = stack_ptr - STACK_START + 1;
        let mut real_stack = Vec::new();
        for x in 0..stack_size {
            real_stack.push(fast_encode(&mic1.main_memory.read(fast_decode(x + STACK_START))));
        }

        assert_eq!(expected_stack, real_stack);
    }
}
