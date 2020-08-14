use std::collections::HashMap;
use std::process::Command;
use std::str::FromStr;

use linked_hash_map::LinkedHashMap;
use tree_sitter::{Language, Parser};

use crate::asm::IjvmCommand;
use crate::main;

extern "C" { fn tree_sitter_jas() -> Language; }

pub struct ProcessorInfo {
    constants: Vec<i32>,
    main_program: Vec<i32>,
}

pub fn compile(source: &str) -> ProcessorInfo {
    let language = unsafe { tree_sitter_jas() };
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();

    let tree = parser.parse(source, None).unwrap();
    let pointer = tree.root_node();

    let mut constants = LinkedHashMap::new();
    let mut main_program = Vec::new();
    for i in 0..pointer.child_count() {
        let current_node = pointer.child(i).unwrap();

        if current_node.kind() == "constants" {
            assert_eq!(0, i);
            for i in 1..(current_node.child_count() - 1) {
                let expression = current_node.child(i).unwrap();
                let const_name = expression.child(0).unwrap().utf8_text(source.as_ref()).unwrap();
                let const_value = expression.child(1).unwrap().utf8_text(source.as_ref()).unwrap();
                constants.insert(const_name, i32::from_str(const_value).unwrap());
            }
        }

        if current_node.kind() == "main_program" {
            assert!(i == 0 || i == 1);
            for x in 1..current_node.child_count() - 1 {
                let command = current_node.child(x).unwrap();
                main_program.push(match command.kind() {
                    "command" => IjvmCommand::parse(command.utf8_text(source.as_ref()).unwrap()).unwrap() as i32,
                    "dec_number" => i32::from_str(command.utf8_text(source.as_ref()).unwrap()).unwrap(),
                    "oct_number" => i32::from_str_radix(command.utf8_text(source.as_ref()).map(|x| x.trim_start_matches("0")).unwrap(), 8).unwrap(),
                    "hex_number" => i32::from_str_radix(command.utf8_text(source.as_ref()).map(|x| x.trim_start_matches("0x")).unwrap(), 16).unwrap(),
                    "bin_number" => i32::from_str_radix(command.utf8_text(source.as_ref()).map(|x| x.trim_start_matches("0b")).unwrap(), 2).unwrap(),
                    "identifier" => {
                        *constants.get(command.utf8_text(source.as_ref()).unwrap()).unwrap()
                    },
                    _ => panic!("Unexpected type")
                })
            }
        }
    }

    return ProcessorInfo {
        constants: constants.values().cloned().collect(),
        main_program,
    };
}

#[cfg(test)]
mod tests {
    use crate::asm::IjvmCommand::{BIPUSH, DUP, IADD};

    use super::*;

    #[test]
    fn empty_constant() {
        let program = r#"
                       .constant
                       .end-constant
                       .main
                       .end-main
"#;
        let info = compile(program);

        assert_constants(vec![], &info);
    }

    #[test]
    fn one_constant() {
        let program = r#"
                       .constant
                       my_var 1
                       .end-constant
                       .main
                       .end-main
"#;
        let info = compile(program);

        assert_constants(vec![1], &info);
    }

    #[test]
    fn multiple_constants() {
        let program = r#"
                       .constant
                       my_var 1
                       my_var_x 1
                       my_var_y 2
                       .end-constant
                       .main
                       .end-main
"#;
        let info = compile(program);

        assert_constants(vec![1, 1, 2], &info);
    }

    #[test]
    fn simple_program() {
        let program = r#"
                       .main
                       DUP
                       IADD
                       .end-main
"#;
        let info = compile(program);

        assert_constants(vec![], &info);
        assert_main(vec![DUP as i32, IADD as i32], &info);
    }

    #[test]
    fn program_with_constant() {
        let program = r#"
                       .main
                       BIPUSH 1
                       .end-main
"#;
        let info = compile(program);

        assert_constants(vec![], &info);
        assert_main(vec![BIPUSH as i32, 1], &info);
    }

    #[test]
    fn program_with_hex_constant() {
        let program = r#"
                       .main
                       BIPUSH 0x15
                       .end-main
"#;
        let info = compile(program);

        assert_constants(vec![], &info);
        assert_main(vec![BIPUSH as i32, 0x15], &info);
    }

    #[test]
    fn program_with_constants() {
        let program = r#"
                        .constant
                        my_var 2
                        .end-constant

                        .main
                        BIPUSH my_var
                        .end-main
"#;
        let info = compile(program);

        assert_constants(vec![2], &info);
        assert_main(vec![BIPUSH as i32, 2], &info);
    }

    fn assert_constants(expected: Vec<i32>, info: &ProcessorInfo) {
        assert_eq!(expected, info.constants);
    }

    fn assert_main(expected: Vec<i32>, info: &ProcessorInfo) {
        assert_eq!(expected, info.main_program);
    }
}
