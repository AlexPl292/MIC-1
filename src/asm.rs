#![allow(non_camel_case_types)]
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
