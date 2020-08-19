use std::collections::HashMap;
use std::env::var;
use std::process::Command;
use std::str::FromStr;

use linked_hash_map::LinkedHashMap;
use tree_sitter::{Language, Node, Parser};

use crate::asm::IjvmCommand;
use crate::asm::IjvmCommand::{BIPUSH, GOTO, IFEQ, IFLT, IINC, ILOAD, INVOKEVIRTUAL, ISTORE, LDC_W};
use crate::compiler::IdentifierRole::{CONSTANT, LABEL, METHOD, VARIABLE};
use crate::main;
use tree_sitter::LogType::Parse;

extern "C" { fn tree_sitter_jas() -> Language; }

pub struct ProcessorInfo {
    constants: Vec<i32>,
    main_program: Vec<i32>,
}

const PLACEHOLDER: i32 = 0x00;

pub fn compile(source: &str, program_start_offset: u32) -> ProcessorInfo {
    let language = unsafe { tree_sitter_jas() };
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();

    let tree = parser.parse(source, None).unwrap();
    let pointer = tree.root_node();

    let mut constants = LinkedHashMap::new();
    let mut methods = LinkedHashMap::new();
    let mut method_placeholders = LinkedHashMap::new();
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
            let mut process_from = 1;
            let mut vars = Vec::new();
            if current_node.child(1).unwrap().kind() == "variables" {
                vars = process_variables(&current_node.child(1).unwrap(), source);
                process_from = 2;
            }
            parse_method_body(source, &mut constants, &mut methods, &mut method_placeholders, &Vec::new(), &vars, &mut main_program, current_node, program_start_offset, process_from)
        }

        if current_node.kind() == "method" {
            method_parsing::process_method(source, program_start_offset, &mut constants, &mut methods, &mut method_placeholders, &mut main_program, current_node)
        }
    }

    // Add methods to constants
    let mut method_constants = HashMap::new();
    for (key, value) in methods {
        method_constants.insert(key, constants.len());
        constants.insert(key, value + program_start_offset as i32);
    }

    // Replace method placeholders
    for (key, value) in method_placeholders {
        let method_value = method_constants.get(value).unwrap();
        main_program[key] = *method_value as i32;
    }

    return ProcessorInfo {
        constants: constants.values().cloned().collect(),
        main_program,
    };
}

mod method_parsing {
    use linked_hash_map::LinkedHashMap;
    use tree_sitter::Node;

    use crate::compiler::{parse_method_body, process_variables};

    pub fn process_method<'a>(
        source: &'a str,
        program_start_offset: u32,
        mut constants: &mut LinkedHashMap<&str, i32>,
        mut methods: &mut LinkedHashMap<&'a str, i32>,
        mut method_placeholders: &mut LinkedHashMap<usize, &'a str>,
        mut main_program: &mut Vec<i32>,
        current_node: Node,
    ) {
        let name = current_node.child(1).unwrap().utf8_text(source.as_ref()).unwrap();
        methods.insert(name, main_program.len() as i32);

        let parameters = process_parameters(source, &current_node.child(2).unwrap());

        let mut process_from = 3;
        let mut vars = Vec::new();
        if current_node.child(3).unwrap().kind() == "variables" {
            vars = process_variables(&current_node.child(3).unwrap(), source);
            process_from = 4;
        }

        // TODO fix it
        main_program.push(((parameters.len() / 0x100) % 0x100) as i32);
        main_program.push((parameters.len() % 0x100) as i32);
        main_program.push(0x00);
        main_program.push(0x00);

        parse_method_body(source, &mut constants, &mut methods, &mut method_placeholders, &parameters, &vars, &mut main_program, current_node, program_start_offset, process_from)
    }

    fn process_parameters<'a>(
        source: &'a str,
        current_node: &Node,
    ) -> Vec<&'a str> {
        (0..current_node.named_child_count())
            .map(|x| current_node.named_child(x).unwrap().utf8_text(source.as_ref()).unwrap())
            .collect()
    }
}

fn parse_method_body<'a>(
    source: &'a str,
    constants: &mut LinkedHashMap<&str, i32>,
    methods: &mut LinkedHashMap<&str, i32>,
    mut method_placeholders: &mut LinkedHashMap<usize, &'a str>,
    parameters: &Vec<&str>,
    variables: &Vec<&str>,
    mut main_program: &mut Vec<i32>,
    current_node: Node,
    program_start_offset: u32,
    inspect_from: usize
) {
    let mut label_positions = HashMap::new();
    let mut labels = HashMap::new();
    for x in inspect_from..current_node.child_count() - 1 {
        let command = current_node.child(x).unwrap();
        main_program.push(match command.kind() {
            "command" => IjvmCommand::parse(command.utf8_text(source.as_ref()).unwrap()).unwrap() as i32,
            "dec_number" => i32::from_str(command.utf8_text(source.as_ref()).unwrap()).unwrap(),
            "oct_number" => i32::from_str_radix(command.utf8_text(source.as_ref()).map(|x| x.trim_start_matches("0")).unwrap(), 8).unwrap(),
            "hex_number" => i32::from_str_radix(command.utf8_text(source.as_ref()).map(|x| x.trim_start_matches("0x")).unwrap(), 16).unwrap(),
            "bin_number" => i32::from_str_radix(command.utf8_text(source.as_ref()).map(|x| x.trim_start_matches("0b")).unwrap(), 2).unwrap(),
            "identifier" => {
                let role = identifier_role(main_program.last().unwrap());
                match role {
                    CONSTANT => *constants.get(command.utf8_text(source.as_ref()).unwrap()).unwrap(),
                    LABEL => {
                        label_positions.insert(main_program.len(), command.utf8_text(source.as_ref()).unwrap());
                        PLACEHOLDER
                    },
                    VARIABLE => {
                        let var_name = command.utf8_text(source.as_ref()).unwrap();
                        let parameter_position = parameters.iter().position(|&x| x == var_name);
                        match parameter_position {
                            None => (variables.iter().position(|&x| x == var_name).unwrap() + parameters.len()) as i32,
                            Some(t) => t as i32
                        }
                    }
                    METHOD => {
                        method_placeholders.insert(main_program.len(), command.utf8_text(source.as_ref()).unwrap());
                        PLACEHOLDER
                    }
                }
            }
            "label" => {
                labels.insert(command.child(0).unwrap().utf8_text(source.as_ref()).unwrap(), main_program.len());
                continue
            }
            _ => panic!("Unexpected type: {}", command.kind())
        })
    }

    // Replace labels placeholders
    for (key, value) in label_positions {
        let label_value = labels.get(value).unwrap();
        main_program[key] = *label_value as i32 + program_start_offset as i32;
    }
}

fn process_variables<'a>(node: &Node, source: &'a str) -> Vec<&'a str> {
    let mut vars = Vec::new();
    for x in 1..node.child_count() - 1 {
        vars.push(node.child(x).unwrap().utf8_text(source.as_ref()).unwrap());
    }
    return vars
}

fn identifier_role(previous_command: &i32) -> IdentifierRole {
    let label_expected = [GOTO as i32, IFEQ as i32, IFLT as i32];
    let var_expected = [IINC as i32, ILOAD as i32, ISTORE as i32];
    let const_expected = [LDC_W as i32, BIPUSH as i32];
    let method_expected = [INVOKEVIRTUAL as i32];

    if label_expected.contains(&previous_command) {
        return LABEL;
    } else if var_expected.contains(&previous_command) {
        return VARIABLE;
    } else if const_expected.contains(&previous_command) {
        return CONSTANT;
    } else if method_expected.contains(&previous_command) {
        return METHOD;
    } else { panic!("Unexpected") }
}

enum IdentifierRole {
    CONSTANT,
    LABEL,
    VARIABLE,
    METHOD
}

#[cfg(test)]
mod tests {
    use crate::asm::IjvmCommand::*;

    use super::*;

    #[test]
    fn empty_constant() {
        let program = r#"
                       .constant
                       .end-constant
                       .main
                       .end-main
"#;
        let info = compile(program, 0);

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
        let info = compile(program, 0);

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
        let info = compile(program, 0);

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
        let info = compile(program, 0);

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
        let info = compile(program, 0);

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
        let info = compile(program, 0);

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
        let info = compile(program, 0);

        assert_constants(vec![2], &info);
        assert_main(vec![BIPUSH as i32, 2], &info);
    }

    #[test]
    fn program_with_labels() {
        let program = r#"
                       .main
                       label: DUP
                       GOTO label
                       .end-main
"#;
        let info = compile(program, 10);

        assert_constants(vec![], &info);
        assert_main(vec![DUP as i32, GOTO as i32, 10], &info);
    }

    #[test]
    fn program_with_label_in_future() {
        let program = r#"
                       .main
                       GOTO label
                       label: DUP
                       .end-main
"#;
        let info = compile(program, 10);

        assert_constants(vec![], &info);
        assert_main(vec![GOTO as i32, 12, DUP as i32], &info);
    }

    #[test]
    fn program_with_variables() {
        let program = r#"
                       .main
                       .var
                       my_var_x
                       my_var_y
                       .end-var
                       ILOAD my_var_x
                       .end-main
"#;
        let info = compile(program, 10);

        assert_constants(vec![], &info);
        assert_main(vec![ILOAD as i32, 0x00], &info);
    }

    #[test]
    fn program_with_method() {
        let program = r#"
                       .main
                       .end-main
                       .method my()
                       DUP
                       .end-method
"#;
        let info = compile(program, 10);

        assert_constants(vec![10], &info);
        assert_main(vec![0x00, 0x00, 0x00, 0x00, DUP as i32], &info);
    }

    #[test]
    fn program_with_method_and_parameters() {
        let program = r#"
                       .main
                       .end-main
                       .method my(first_var, second_var, third_var)
                       DUP
                       .end-method
"#;
        let info = compile(program, 10);

        assert_constants(vec![10], &info);
        assert_main(vec![0x00, 0x03, 0x00, 0x00, DUP as i32], &info);
    }

    #[test]
    fn program_with_method_with_variables() {
        let program = r#"
                       .main
                       .end-main
                       .method my()
                       .var
                       first_var
                       second_var
                       .end-var
                       ILOAD second_var
                       .end-method
"#;
        let info = compile(program, 10);

        assert_constants(vec![10], &info);
        assert_main(vec![0x00, 0x00, 0x00, 0x00, ILOAD as i32, 0x01], &info);
    }

    #[test]
    fn program_with_method_and_parameters_and_variable() {
        let program = r#"
                       .main
                       .end-main
                       .method my(first_par, second_par, third_par)
                       .var
                       first_var
                       second_var
                       .end-var
                       ILOAD first_par
                       ILOAD second_var
                       .end-method
"#;
        let info = compile(program, 10);

        assert_constants(vec![10], &info);
        assert_main(vec![0x00, 0x03, 0x00, 0x00, ILOAD as i32, 0x00, ILOAD as i32, 0x04], &info);
    }

    #[test]
    fn program_with_method_and_invoking() {
        let program = r#"
                       .main
                       INVOKEVIRTUAL my
                       .end-main
                       .method my()
                       DUP
                       .end-method
"#;
        let info = compile(program, 10);

        assert_constants(vec![12], &info);
        assert_main(vec![INVOKEVIRTUAL as i32, 0, 0x00, 0x00, 0x00, 0x00, DUP as i32], &info);
    }

    fn assert_constants(expected: Vec<i32>, info: &ProcessorInfo) {
        assert_eq!(expected, info.constants);
    }

    fn assert_main(expected: Vec<i32>, info: &ProcessorInfo) {
        assert_eq!(expected, info.main_program);
    }
}
