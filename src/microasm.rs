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
}

impl MicroAsm {
    //noinspection SpellCheckingInspection
    fn command(&self) -> [bool; 36] {
        match *self {
            Main1 => Cb::new().r_pc().alu_b_inc().w_pc().fetch().jmpc().get(),

            nop1 => Cb::new().next_command(Main1).get(),

            iadd1 => Cb::new().r_sp().alu_b_dec().w_mar().w_sp().read().next_command(iadd2).get(),
            iadd2 => Cb::new().r_tos().alu_b().w_h().next_command(iadd3).get(),
            iadd3 => Cb::new().r_mdr().alu_sum().w_mdr().w_tos().write().finish().get(),

            isub1 => Cb::new().r_sp().alu_b_dec().w_mar().w_sp().read().next_command(isub2).get(),
            isub2 => Cb::new().r_tos().alu_b().w_h().next_command(isub3).get(),
            isub3 => Cb::new().r_mdr().alu_sub().w_mdr().w_tos().write().finish().get(),

            iand1 => Cb::new().r_sp().alu_b_dec().w_mar().w_sp().read().next_command(iand2).get(),
            iand2 => Cb::new().r_tos().alu_b().w_h().next_command(iand3).get(),
            iand3 => Cb::new().r_mdr().alu_and().w_mdr().w_tos().write().finish().get(),

            ior1 => Cb::new().r_sp().alu_b_dec().w_mar().w_sp().read().next_command(ior2).get(),
            ior2 => Cb::new().r_tos().alu_b().w_h().next_command(ior3).get(),
            ior3 => Cb::new().r_mdr().alu_or().w_mdr().w_tos().write().finish().get(),

            dup1 => Cb::new().r_sp().alu_b_inc().w_sp().w_mar().next_command(dup2).get(),
            dup2 => Cb::new().r_tos().alu_b().w_mdr().write().finish().get(),

            pop1 => Cb::new().r_sp().alu_b_dec().w_sp().w_mar().read().next_command(pop2).get(),
            pop2 => Cb::new().next_command(pop3).get(), // Waiting for read
            pop3 => Cb::new().r_mdr().alu_b().w_tos().finish().get(),
        }
    }
}

struct Cb {
    command: [bool; 36],
}

impl Cb {
    fn new() -> Cb { Cb { command: [false; 36] } }

    fn finish(&mut self) -> &mut Cb { self.next_command(Main1) }

    fn next_command(&mut self, addr: MicroAsm) -> &mut Cb {
        let decoded = fast_decode(addr as i32);
        for x in 0..9 {
            self.command[x] = decoded[x];
        }
        return self;
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
    fn alu_sub(&mut self) -> &mut Cb { self.f0().f1().ena().enb().inva().inc() }
    fn alu_and(&mut self) -> &mut Cb { self.ena().enb() }
    fn alu_or(&mut self) -> &mut Cb { self.f1().ena().enb() }
    fn alu_b(&mut self) -> &mut Cb { self.f1().enb() }

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
    fn r_ops(&mut self) -> &mut Cb { self.bit(35) }

    fn get(&self) -> [bool; 36] { self.command }

    fn bit(&mut self, i: usize) -> &mut Cb {
        self.command[i] = true;
        return self;
    }
}
