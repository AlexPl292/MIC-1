#![allow(non_camel_case_types)]

use crate::asm::IjvmCommand::{*};

//noinspection SpellCheckingInspection
pub enum IjvmCommand {
    BIPUSH = 0x10,
    DUP = 0x59,
    GOTO = 0xA7,
    IADD = 0x60,
    IAND = 0x7E,
    IFEQ = 0x99,
    IFLT = 0x9B,
    IF_ICMPEQ = 0x9F,
    IINC = 0x84,
    ILOAD = 0x15,
    INVOKEVIRTUAL = 0xB6,
    IOR = 0x80,
    IRETURN = 0xAC,
    ISTORE = 0x36,
    ISUB = 0x64,
    LDC_W = 0x13,
    NOP = 0x00,
    POP = 0x57,
    SWAP = 0x5F,
    WIDE = 0xC4,
}

impl IjvmCommand {
    pub fn parse(str: &str) -> Option<IjvmCommand> {
        match str {
            "BIPUSH" => Option::Some(BIPUSH),
            "DUP" => Option::Some(DUP),
            "GOTO" => Option::Some(GOTO),
            "IADD" => Option::Some(IADD),
            "IAND" => Option::Some(IAND),
            "IFEQ" => Option::Some(IFEQ),
            "IFLT" => Option::Some(IFLT),
            "IF_ICMPEQ" => Option::Some(IF_ICMPEQ),
            "IINC" => Option::Some(IINC),
            "ILOAD" => Option::Some(ILOAD),
            "INVOKEVIRTUAL" => Option::Some(INVOKEVIRTUAL),
            "IOR" => Option::Some(IOR),
            "IRETURN" => Option::Some(IRETURN),
            "ISTORE" => Option::Some(ISTORE),
            "ISUB" => Option::Some(ISUB),
            "LDC_W" => Option::Some(LDC_W),
            "NOP" => Option::Some(NOP),
            "POP" => Option::Some(POP),
            "SWAP" => Option::Some(SWAP),
            "WIDE" => Option::Some(WIDE),
            _ => Option::None
        }
    }
}
