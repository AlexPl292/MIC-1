use std::collections::HashMap;
use std::str::FromStr;

use tree_sitter::{Language, Parser};
use linked_hash_map::LinkedHashMap;

extern "C" { fn tree_sitter_jas() -> Language; }

pub struct ProcessorInfo {
    constants: Vec<i32>
}

pub fn compile(source: &str) -> ProcessorInfo {
    let language = unsafe { tree_sitter_jas() };
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();

    let tree = parser.parse(source, None).unwrap();
    let pointer = tree.root_node();

    let current = pointer.child(0).unwrap();

    let mut constants = LinkedHashMap::new();
    if current.kind() == "constants" {
        for i in 1..(current.child_count() - 1) {
            let expression = current.child(i).unwrap();
            let const_name = expression.child(0).unwrap().utf8_text(source.as_ref()).unwrap();
            let const_value = expression.child(1).unwrap().utf8_text(source.as_ref()).unwrap();
            constants.insert(const_name, i32::from_str(const_value).unwrap());
        }
    }

    return ProcessorInfo {
        constants: constants.values().cloned().collect()
    };
}

#[cfg(test)]
mod tests {
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

    fn assert_constants(expected: Vec<i32>, info: &ProcessorInfo) {
        assert_eq!(expected, info.constants);
    }
}
