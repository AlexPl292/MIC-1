use std::collections::HashMap;
use std::iter::Map;

use crate::asm::IjvmCommand;
use crate::asm::IjvmCommand::*;
use crate::main_memory::fast_decode;
use crate::microasm::MicroAsm::*;

//noinspection SpellCheckingInspection
/**
 * Commands and it's locations in memory.

 * iand3 = IAND + 2 + 3   <- First number - initial command location,
 *                           second - shifting from initial command,
 *                           third - shifting if we can't put this command at this place
 */
#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Hash)]
pub enum MicroAsm {
    Main1 = 1,

    nop1 = NOP as isize,

    iadd1 = IADD as isize,
    iadd2 = IADD as isize + 1,
    iadd3 = IADD as isize + 2,

    isub1 = ISUB as isize,
    isub2 = ISUB as isize + 1,
    isub3 = ISUB as isize + 2,

    iand1 = IAND as isize,
    iand2 = IAND as isize + 1,
    iand3 = IAND as isize + 2 + 3,

    ior1 = IOR as isize,
    ior2 = IOR as isize + 1,
    ior3 = IOR as isize + 2,

    dup1 = DUP as isize,
    dup2 = DUP as isize + 1,

    pop1 = POP as isize,
    pop2 = POP as isize + 1,
    pop3 = POP as isize + 2 + 3,

    swap1 = SWAP as isize,
    swap2 = SWAP as isize + 1 + 10,
    swap3 = SWAP as isize + 2 + 10,
    swap4 = SWAP as isize + 3 + 10,
    swap5 = SWAP as isize + 4 + 10,
    swap6 = SWAP as isize + 5 + 10,

    bipush1 = BIPUSH as isize,
    bipush2 = BIPUSH as isize + 1,
    bipush3 = BIPUSH as isize + 2,

    iload1 = ILOAD as isize,
    iload2 = ILOAD as isize + 1,
    iload3 = ILOAD as isize + 2,
    iload4 = ILOAD as isize + 3,
    iload5 = ILOAD as isize + 4,

    istore1 = ISTORE as isize,
    istore2 = ISTORE as isize + 1,
    istore3 = ISTORE as isize + 2,
    istore4 = ISTORE as isize + 3,
    istore5 = ISTORE as isize + 4,
    istore6 = ISTORE as isize + 5,

    wide1 = WIDE as isize,
    wide2 = WIDE as isize + 1,
    wide_iload1 = ILOAD as isize + 0x100,
    wide_iload2 = ILOAD as isize + 0x100 + 1,
    wide_iload3 = ILOAD as isize + 0x100 + 2,
    wide_iload4 = ILOAD as isize + 0x100 + 3,
    wide_istore1 = ISTORE as isize + 0x100,
    wide_istore2 = ISTORE as isize + 0x100 + 1,
    wide_istore3 = ISTORE as isize + 0x100 + 2,
    wide_istore4 = ISTORE as isize + 0x100 + 3,

    ldc_w1 = LDC_W as isize,
    ldc_w2 = LDC_W as isize + 1,
    ldc_w3 = LDC_W as isize + 2 + 5,
    ldc_w4 = LDC_W as isize + 3 + 6,

    iinc1 = IINC as isize,
    iinc2 = IINC as isize + 1,
    iinc3 = IINC as isize + 2,
    iinc4 = IINC as isize + 3,
    iinc5 = IINC as isize + 4,
    iinc6 = IINC as isize + 5,

    goto1 = GOTO as isize,
    goto2 = GOTO as isize + 1,
    goto3 = GOTO as isize + 2,
    goto4 = GOTO as isize + 3,
    goto5 = GOTO as isize + 4 + 3,
    goto6 = GOTO as isize + 5 + 4,

    iflt1 = IFLT as isize,
    iflt2 = IFLT as isize + 1,
    iflt3 = IFLT as isize + 2,
    iflt4 = IFLT as isize + 3,

    ifeq1 = IFEQ as isize,
    ifeq2 = IFEQ as isize + 1,
    ifeq3 = IFEQ as isize + 2 + 5,
    ifeq4 = IFEQ as isize + 3 + 6,

    if_icmpeq1 = IF_ICMPEQ as isize,
    if_icmpeq2 = IF_ICMPEQ as isize + 1 + 13,
    if_icmpeq3 = IF_ICMPEQ as isize + 2 + 14,
    if_icmpeq4 = IF_ICMPEQ as isize + 3 + 15,
    if_icmpeq5 = IF_ICMPEQ as isize + 4 + 16,
    if_icmpeq6 = IF_ICMPEQ as isize + 5 + 17,

    F = 0x2,
    F2 = 0x3,
    F3 = 0x4,
    T = 0x102,

    invokevirtual1 = INVOKEVIRTUAL as isize,
    invokevirtual2 = INVOKEVIRTUAL as isize + 1,
    invokevirtual3 = INVOKEVIRTUAL as isize + 2,
    invokevirtual4 = INVOKEVIRTUAL as isize + 3,
    invokevirtual5 = INVOKEVIRTUAL as isize + 4,
    invokevirtual6 = INVOKEVIRTUAL as isize + 5,
    invokevirtual7 = INVOKEVIRTUAL as isize + 6,
    invokevirtual8 = INVOKEVIRTUAL as isize + 7,
    invokevirtual9 = INVOKEVIRTUAL as isize + 8,
    invokevirtual10 = INVOKEVIRTUAL as isize + 9,
    invokevirtual11 = INVOKEVIRTUAL as isize + 10,
    invokevirtual12 = INVOKEVIRTUAL as isize + 11,
    invokevirtual13 = INVOKEVIRTUAL as isize + 12,
    invokevirtual14 = INVOKEVIRTUAL as isize + 13,
    invokevirtual15 = INVOKEVIRTUAL as isize + 14 + 3,
    invokevirtual16 = INVOKEVIRTUAL as isize + 15 + 4,
    invokevirtual17 = INVOKEVIRTUAL as isize + 16 + 5,
    invokevirtual18 = INVOKEVIRTUAL as isize + 17 + 6,
    invokevirtual19 = INVOKEVIRTUAL as isize + 18 + 7,
    invokevirtual20 = INVOKEVIRTUAL as isize + 19 + 8,
    invokevirtual21 = INVOKEVIRTUAL as isize + 20 + 9,
    invokevirtual22 = INVOKEVIRTUAL as isize + 21 + 10,

    ireturn1 = IRETURN as isize,
    ireturn2 = IRETURN as isize + 1 + 5,
    ireturn3 = IRETURN as isize + 2 + 6,
    ireturn4 = IRETURN as isize + 3 + 39,
    ireturn5 = IRETURN as isize + 4 + 40,
    ireturn6 = IRETURN as isize + 5 + 41,
    ireturn7 = IRETURN as isize + 6 + 42,
    ireturn8 = IRETURN as isize + 7 + 43,
}

impl MicroAsm {
    //noinspection SpellCheckingInspection
    fn command(&self) -> [bool; 36] {
        match *self {
            Main1 => Cb::new().r_pc().alu_b_inc().w_pc().fetch().jmpc().get(),

            nop1 => Cb::new().next_command(Main1),

            iadd1 => Cb::new().r_sp().alu_b_dec().w_mar().w_sp().read().next_command(iadd2),
            iadd2 => Cb::new().r_tos().alu_b().w_h().next_command(iadd3),
            iadd3 => Cb::new().r_mdr().alu_sum().w_mdr().w_tos().write().finish(),

            isub1 => Cb::new().r_sp().alu_b_dec().w_mar().w_sp().read().next_command(isub2),
            isub2 => Cb::new().r_tos().alu_b().w_h().next_command(isub3),
            isub3 => Cb::new().r_mdr().alu_sub().w_mdr().w_tos().write().finish(),

            iand1 => Cb::new().r_sp().alu_b_dec().w_mar().w_sp().read().next_command(iand2),
            iand2 => Cb::new().r_tos().alu_b().w_h().next_command(iand3),
            iand3 => Cb::new().r_mdr().alu_and().w_mdr().w_tos().write().finish(),

            ior1 => Cb::new().r_sp().alu_b_dec().w_mar().w_sp().read().next_command(ior2),
            ior2 => Cb::new().r_tos().alu_b().w_h().next_command(ior3),
            ior3 => Cb::new().r_mdr().alu_or().w_mdr().w_tos().write().finish(),

            dup1 => Cb::new().r_sp().alu_b_inc().w_sp().w_mar().next_command(dup2),
            dup2 => Cb::new().r_tos().alu_b().w_mdr().write().finish(),

            pop1 => Cb::new().r_sp().alu_b_dec().w_sp().w_mar().read().next_command(pop2),
            pop2 => Cb::new().next_command(pop3), // Waiting for read
            pop3 => Cb::new().r_mdr().alu_b().w_tos().finish(),

            swap1 => Cb::new().r_sp().alu_b_dec().w_sp().w_mar().read().next_command(swap2),
            swap2 => Cb::new().r_sp().alu_b().w_mar().next_command(swap3),
            swap3 => Cb::new().r_mdr().alu_b().w_h().write().next_command(swap4),
            swap4 => Cb::new().r_tos().alu_b().w_mdr().next_command(swap5),
            swap5 => Cb::new().r_sp().alu_b_dec().w_mar().write().next_command(swap6),
            swap6 => Cb::new().alu_a().w_tos().finish(),

            bipush1 => Cb::new().r_sp().alu_b_inc().w_sp().w_mar().next_command(bipush2),
            bipush2 => Cb::new().r_pc().alu_b_inc().w_pc().fetch().next_command(bipush3),
            bipush3 => Cb::new().r_mbr().alu_b().w_tos().w_mdr().write().finish(),

            iload1 => Cb::new().r_lv().alu_b().w_h().next_command(iload2),
            iload2 => Cb::new().r_mbru().alu_sum().w_mar().read().next_command(iload3),
            iload3 => Cb::new().r_sp().alu_b_inc().w_sp().w_mar().next_command(iload4),
            iload4 => Cb::new().r_pc().alu_b_inc().w_pc().fetch().write().next_command(iload5),
            iload5 => Cb::new().r_mdr().alu_b().w_tos().finish(),

            istore1 => Cb::new().r_lv().alu_b().w_h().next_command(istore2),
            istore2 => Cb::new().r_mbru().alu_sum().w_mar().next_command(istore3),
            istore3 => Cb::new().r_tos().alu_b().w_mdr().write().next_command(istore4),
            istore4 => Cb::new().r_sp().alu_b_dec().w_sp().w_mar().read().next_command(istore5),
            istore5 => Cb::new().r_pc().alu_b_inc().w_pc().fetch().next_command(istore6),
            istore6 => Cb::new().r_mdr().alu_b().w_tos().finish(),

            wide1 => Cb::new().r_pc().alu_b_inc().w_pc().fetch().next_command(wide2),
            wide2 => Cb::new().jmpc().next_command_wide_jump(),
            wide_iload1 => Cb::new().r_pc().alu_b_inc().w_pc().fetch().next_command(wide_iload2),
            wide_iload2 => Cb::new().r_mbru().alu_b().sll8().w_h().next_command(wide_iload3),
            wide_iload3 => Cb::new().r_mbru().alu_or().w_h().next_command(wide_iload4),
            wide_iload4 => Cb::new().r_lv().alu_sum().w_mar().read().next_command(iload3),
            wide_istore1 => Cb::new().r_pc().alu_b_inc().w_pc().fetch().next_command(wide_istore2),
            wide_istore2 => Cb::new().r_mbru().alu_b().sll8().w_h().next_command(wide_istore3),
            wide_istore3 => Cb::new().r_mbru().alu_or().w_h().next_command(wide_istore4),
            wide_istore4 => Cb::new().r_lv().alu_sum().w_mar().read().next_command(istore3),

            ldc_w1 => Cb::new().r_pc().alu_b_inc().w_pc().fetch().next_command(ldc_w2),
            ldc_w2 => Cb::new().r_mbru().alu_b().sll8().w_h().next_command(ldc_w3),
            ldc_w3 => Cb::new().r_mbru().alu_or().w_h().next_command(ldc_w4),
            ldc_w4 => Cb::new().r_cpp().alu_sum().w_mar().read().next_command(iload3),

            iinc1 => Cb::new().r_lv().alu_b().w_h().next_command(iinc2),
            iinc2 => Cb::new().r_mbru().alu_sum().w_mar().read().next_command(iinc3),
            iinc3 => Cb::new().r_pc().alu_b_inc().w_pc().fetch().next_command(iinc4),
            iinc4 => Cb::new().r_mdr().alu_b().w_h().next_command(iinc5),
            iinc5 => Cb::new().r_pc().alu_b_inc().w_pc().fetch().next_command(iinc6),
            iinc6 => Cb::new().r_mbr().alu_sum().w_mdr().write().finish(),

            goto1 => Cb::new().r_pc().alu_b_dec().w_opc().next_command(goto2),
            goto2 => Cb::new().r_pc().alu_b_inc().w_pc().fetch().next_command(goto3),
            goto3 => Cb::new().r_mbr().alu_b().sll8().w_h().next_command(goto4),
            goto4 => Cb::new().r_mbru().alu_or().w_h().next_command(goto5),
            goto5 => Cb::new().r_opc().alu_sum().w_pc().fetch().next_command(goto6),
            goto6 => Cb::new().finish(),

            iflt1 => Cb::new().r_sp().alu_b_dec().w_sp().w_mar().read().next_command(iflt2),
            iflt2 => Cb::new().r_tos().alu_b().w_opc().next_command(iflt3),
            iflt3 => Cb::new().r_mdr().alu_b().w_tos().next_command(iflt4),
            iflt4 => Cb::new().r_opc().alu_b().jamn().next_command(F),

            ifeq1 => Cb::new().r_sp().alu_b_dec().w_sp().w_mar().read().next_command(ifeq2),
            ifeq2 => Cb::new().r_tos().alu_b().w_opc().next_command(ifeq3),
            ifeq3 => Cb::new().r_mdr().alu_b().w_tos().next_command(ifeq4),
            ifeq4 => Cb::new().r_opc().alu_b().jamz().next_command(F),

            if_icmpeq1 => Cb::new().r_sp().alu_b_dec().w_sp().w_mar().read().next_command(if_icmpeq2),
            if_icmpeq2 => Cb::new().r_sp().alu_b_dec().w_sp().w_mar().next_command(if_icmpeq3),
            if_icmpeq3 => Cb::new().r_mdr().alu_b().w_h().read().next_command(if_icmpeq4),
            if_icmpeq4 => Cb::new().r_tos().alu_b().w_opc().next_command(if_icmpeq5),
            if_icmpeq5 => Cb::new().r_mdr().alu_b().w_tos().next_command(if_icmpeq6),
            if_icmpeq6 => Cb::new().r_opc().alu_sub().jamz().next_command(F),

            T => Cb::new().r_pc().alu_b_dec().w_opc().fetch().next_command(goto2),
            F => Cb::new().r_pc().alu_b_inc().w_pc().next_command(F2),
            F2 => Cb::new().r_pc().alu_b_inc().w_pc().fetch().next_command(F3),
            F3 => Cb::new().finish(),

            invokevirtual1 => Cb::new().r_pc().inc().w_pc().fetch().next_command(invokevirtual2),
            invokevirtual2 => Cb::new().r_mbru().alu_b().sll8().w_h().next_command(invokevirtual3),
            invokevirtual3 => Cb::new().r_mbru().alu_or().w_h().next_command(invokevirtual4),
            invokevirtual4 => Cb::new().r_cpp().alu_sum().w_mar().read().next_command(invokevirtual5),
            invokevirtual5 => Cb::new().r_pc().alu_b_inc().w_opc().next_command(invokevirtual6),
            invokevirtual6 => Cb::new().r_mdr().alu_b().w_pc().fetch().next_command(invokevirtual7),
            invokevirtual7 => Cb::new().r_pc().alu_b_inc().w_pc().fetch().next_command(invokevirtual8),
            invokevirtual8 => Cb::new().r_mbru().alu_b().sll8().w_h().next_command(invokevirtual9),
            invokevirtual9 => Cb::new().r_mbru().alu_or().w_h().next_command(invokevirtual10),
            invokevirtual10 => Cb::new().r_pc().alu_b_inc().w_pc().fetch().next_command(invokevirtual11),
            invokevirtual11 => Cb::new().r_sp().alu_sub().w_tos().next_command(invokevirtual12),
            invokevirtual12 => Cb::new().r_tos().alu_b_inc().w_mar().w_tos().next_command(invokevirtual13),
            invokevirtual13 => Cb::new().r_pc().alu_b_inc().w_pc().fetch().next_command(invokevirtual14),
            invokevirtual14 => Cb::new().r_mbru().alu_b().sll8().w_h().next_command(invokevirtual15),
            invokevirtual15 => Cb::new().r_mbru().alu_or().w_h().next_command(invokevirtual16),
            invokevirtual16 => Cb::new().r_sp().alu_sum_inc().w_mdr().write().next_command(invokevirtual17),
            invokevirtual17 => Cb::new().r_mdr().alu_b().w_sp().w_mar().next_command(invokevirtual18),
            invokevirtual18 => Cb::new().r_opc().alu_b().w_mdr().write().next_command(invokevirtual19),
            invokevirtual19 => Cb::new().r_sp().alu_b_inc().w_sp().w_mar().next_command(invokevirtual20),
            invokevirtual20 => Cb::new().r_lv().alu_b().w_mdr().write().next_command(invokevirtual21),
            invokevirtual21 => Cb::new().r_pc().alu_b_inc().w_pc().fetch().next_command(invokevirtual22),
            invokevirtual22 => Cb::new().r_tos().alu_b().w_lv().finish(),

            ireturn1 => Cb::new().r_lv().alu_b().w_sp().w_mar().read().next_command(ireturn2),
            ireturn2 => Cb::new().next_command(ireturn3),
            ireturn3 => Cb::new().r_mdr().alu_b().w_mar().w_lv().read().next_command(ireturn4),
            ireturn4 => Cb::new().r_lv().alu_b_inc().w_mar().next_command(ireturn5),
            ireturn5 => Cb::new().r_mdr().alu_b().w_pc().read().fetch().next_command(ireturn6),
            ireturn6 => Cb::new().r_sp().alu_b().w_mar().next_command(ireturn7),
            ireturn7 => Cb::new().r_mdr().alu_b().w_lv().next_command(ireturn8),
            ireturn8 => Cb::new().r_tos().alu_b().w_mdr().write().finish(),
        }
    }
}

struct Cb {
    command: [bool; 36],
}

impl Cb {
    fn new() -> Cb { Cb { command: [false; 36] } }

    fn finish(&mut self) -> [bool; 36] {
        self.next_command(Main1);
        return self.command;
    }

    fn next_command(&mut self, addr: MicroAsm) -> [bool; 36] {
        let decoded = fast_decode(addr as i32);
        for x in 0..9 {
            self.command[x] = decoded[x];
        }
        return self.command;
    }

    fn next_command_wide_jump(&mut self) -> [bool; 36] {
        self.command[0] = true;
        return self.command;
    }

    fn jmpc(&mut self) -> &mut Cb { self.bit(9) }
    fn jamn(&mut self) -> &mut Cb { self.bit(10) }
    fn jamz(&mut self) -> &mut Cb { self.bit(11) }
    fn sll8(&mut self) -> &mut Cb { self.bit(12) }
    fn sra1(&mut self) -> &mut Cb { self.bit(13) }

    // ALU
    fn f0(&mut self) -> &mut Cb { self.bit(14) }
    fn f1(&mut self) -> &mut Cb { self.bit(15) }
    fn ena(&mut self) -> &mut Cb { self.bit(16) }
    fn enb(&mut self) -> &mut Cb { self.bit(17) }
    fn inva(&mut self) -> &mut Cb { self.bit(18) }
    fn inc(&mut self) -> &mut Cb { self.bit(19) }

    // ALU helper
    fn alu_b_dec(&mut self) -> &mut Cb { self.f0().f1().enb().inva() }
    fn alu_b_inc(&mut self) -> &mut Cb { self.f0().f1().enb().inc() }
    fn alu_sum(&mut self) -> &mut Cb { self.f0().f1().ena().enb() }
    fn alu_sum_inc(&mut self) -> &mut Cb { self.f0().f1().ena().enb().inc() }
    fn alu_sub(&mut self) -> &mut Cb { self.f0().f1().ena().enb().inva().inc() }
    fn alu_and(&mut self) -> &mut Cb { self.ena().enb() }
    fn alu_or(&mut self) -> &mut Cb { self.f1().ena().enb() }
    fn alu_b(&mut self) -> &mut Cb { self.f1().enb() }
    fn alu_a(&mut self) -> &mut Cb { self.f1().ena() }

    fn w_h(&mut self) -> &mut Cb { self.bit(20) }
    fn w_opc(&mut self) -> &mut Cb { self.bit(21) }
    fn w_tos(&mut self) -> &mut Cb { self.bit(22) }
    fn w_cpp(&mut self) -> &mut Cb { self.bit(23) }
    fn w_lv(&mut self) -> &mut Cb { self.bit(24) }
    fn w_sp(&mut self) -> &mut Cb { self.bit(25) }
    fn w_pc(&mut self) -> &mut Cb { self.bit(26) }
    fn w_mdr(&mut self) -> &mut Cb { self.bit(27) }
    fn w_mar(&mut self) -> &mut Cb { self.bit(28) }
    fn write(&mut self) -> &mut Cb { self.bit(29) }
    fn read(&mut self) -> &mut Cb { self.bit(30) }
    fn fetch(&mut self) -> &mut Cb { self.bit(31) }

    fn r_mdr(&mut self) -> &mut Cb { return self; }
    fn r_pc(&mut self) -> &mut Cb { self.bit(32) }
    fn r_mbr(&mut self) -> &mut Cb { self.bit(33) }
    fn r_mbru(&mut self) -> &mut Cb { self.bit(32).bit(33) }
    fn r_sp(&mut self) -> &mut Cb { self.bit(34) }
    fn r_lv(&mut self) -> &mut Cb { self.bit(32).bit(34) }
    fn r_cpp(&mut self) -> &mut Cb { self.bit(33).bit(34) }
    fn r_tos(&mut self) -> &mut Cb { self.bit(32).bit(33).bit(34) }
    fn r_opc(&mut self) -> &mut Cb { self.bit(35) }

    fn get(&self) -> [bool; 36] { self.command }

    fn bit(&mut self, i: usize) -> &mut Cb {
        self.command[i] = true;
        return self;
    }
}
