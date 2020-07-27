use crate::asm::IjvmCommand;
use crate::main;

pub fn parse(program: &str) -> Vec<i32> {
    let mut res = Vec::new();

    let program_lines = program.split("\n");
    let program_lines = program_lines.filter(|x| !x.is_empty());
    for line in program_lines {
        let mut commands = line.split(" ").filter(|x| !x.is_empty());
        let r = commands.next();
        let main_command = r.unwrap();
        res.push(IjvmCommand::parse(main_command) as i32);
        for command in commands {
            res.push(i32::from_str_radix(command.trim_start_matches("0x"), 16).unwrap())
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::IjvmCommand::{IADD, ILOAD};

    #[test]
    fn empty_string() {
        let res = parse("");
        assert!(res.is_empty())
    }

    #[test]
    fn iadd() {
        let res = parse("IADD");
        assert_eq!(1, res.len());
        assert_eq!(IADD as i32, *res.get(0).unwrap());
    }

    #[test]
    fn iload() {
        let res = parse("ILOAD 0x01");
        assert_eq!(2, res.len());
        assert_eq!(ILOAD as i32, *res.get(0).unwrap());
        assert_eq!(0x01, *res.get(1).unwrap());
    }

    #[test]
    fn multiple_commands() {
        let res = parse("ILOAD 0x01\nILOAD 0x02\nIADD");
        assert_eq!(5, res.len());
        assert_eq!(ILOAD as i32, *res.get(0).unwrap());
        assert_eq!(0x01, *res.get(1).unwrap());
        assert_eq!(ILOAD as i32, *res.get(2).unwrap());
        assert_eq!(0x02, *res.get(3).unwrap());
        assert_eq!(IADD as i32, *res.get(4).unwrap());
    }
}
