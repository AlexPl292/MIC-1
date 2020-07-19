use std::collections::HashMap;
use std::iter::Map;

fn commands() -> HashMap<String, i32> {
    let mut commands = HashMap::new();

    // main1 : PC = PC + 1; fetch; goto(MBR)
    // commands.insert(String::from("main1"), 0b1);


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
    fn f0(&mut self) -> &mut Cb { self.bit(14) }
    fn f1(&mut self) -> &mut Cb { self.bit(15) }
    fn ena(&mut self) -> &mut Cb { self.bit(16) }
    fn enb(&mut self) -> &mut Cb { self.bit(17) }
    fn inva(&mut self) -> &mut Cb { self.bit(18) }
    fn inc(&mut self) -> &mut Cb { self.bit(19) }
    fn h(&mut self) -> &mut Cb { self.bit(20) }
    fn opc(&mut self) -> &mut Cb { self.bit(21) }
    fn tos(&mut self) -> &mut Cb { self.bit(22) }
    fn cpp(&mut self) -> &mut Cb { self.bit(23) }
    fn lv(&mut self) -> &mut Cb { self.bit(24) }
    fn sp(&mut self) -> &mut Cb { self.bit(25) }
    fn pc(&mut self) -> &mut Cb { self.bit(26) }
    fn mdr(&mut self) -> &mut Cb { self.bit(27) }
    fn mar(&mut self) -> &mut Cb { self.bit(28) }
    fn write(&mut self) -> &mut Cb { self.bit(29) }
    fn read(&mut self) -> &mut Cb { self.bit(30) }
    fn fetch(&mut self) -> &mut Cb { self.bit(31) }

    fn b_mdr(&mut self) -> &mut Cb { return self; }
    fn b_pc(&mut self) -> &mut Cb { self.bit(32) }
    fn b_mbr(&mut self) -> &mut Cb { self.bit(33) }
    fn b_mbru(&mut self) -> &mut Cb { self.bit(32).bit(33) }
    fn b_sp(&mut self) -> &mut Cb { self.bit(34) }
    fn b_lv(&mut self) -> &mut Cb { self.bit(32).bit(34) }
    fn b_cpp(&mut self) -> &mut Cb { self.bit(33).bit(34) }
    fn b_tos(&mut self) -> &mut Cb { self.bit(32).bit(33).bit(34) }
    fn b_ops(&mut self) -> &mut Cb { self.bit(35) }

    fn get(self) -> [bool; 36] { self.command }

    fn bit(&mut self, i: usize) -> &mut Cb {
        self.command[i] = true;
        return self;
    }
}
