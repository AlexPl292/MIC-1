use std::collections::HashMap;
use std::iter::Map;

pub fn commands() -> HashMap<String, [bool; 36]> {
    let mut commands = HashMap::new();

    // iadd
    // MAR = SP = SP — 1; rd
    commands.insert(String::from("iadd1"), Cb::new().r_sp().alu_b_dec().w_mar().w_sp().read().next_addr([true, false, false, false, false, false, false, false, false]).get());
    // H = TOS
    commands.insert(String::from("iadd2"), Cb::new().r_tos().alu_b().w_h().next_addr([false, true, false, false, false, false, false, false, false]).get());
    // MDR = TOS = MDR + H; wr; goto Main1
    // todo!("goto missing");
    commands.insert(String::from("iadd3"), Cb::new().r_mdr().alu_sum().w_mdr().w_tos().write().next_addr([false, false, false, false, false, false, false, false, false]).get());


    return commands;
}

struct Cb {
    command: [bool; 36],
}

impl Cb {
    fn new() -> Cb { Cb { command: [false; 36] } }

    fn next_addr(&mut self, addr: [bool; 9]) -> &mut Cb {
        for i in 0..9 {
            self.command[i] = addr[i]
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
    fn alu_sum(&mut self) -> &mut Cb { self.f0().f1().ena().enb() }
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
