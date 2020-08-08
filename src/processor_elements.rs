use crate::alu::AluControl;
use crate::memory::Register36;

pub struct BBusControls {
    controls: [bool; 9]
}

impl BBusControls {
    pub fn new(controls: [bool; 9]) -> BBusControls { BBusControls { controls } }
    pub fn mdr(&self) -> bool { self.controls[0] }
    pub fn pc(&self) -> bool { self.controls[1] }
    pub fn mbr(&self) -> bool { self.controls[2] }
    pub fn mbru(&self) -> bool { self.controls[3] }
    pub fn sp(&self) -> bool { self.controls[4] }
    pub fn lv(&self) -> bool { self.controls[5] }
    pub fn cpp(&self) -> bool { self.controls[6] }
    pub fn tos(&self) -> bool { self.controls[7] }
    pub fn opc(&self) -> bool { self.controls[8] }
}

pub struct CBusControls {
    controls: [bool; 9]
}

impl CBusControls {
    pub fn new(controls: [bool; 9]) -> CBusControls { CBusControls { controls } }
    pub fn h(&self) -> bool { self.controls[0] }
    pub fn opc(&self) -> bool { self.controls[1] }
    pub fn tos(&self) -> bool { self.controls[2] }
    pub fn cpp(&self) -> bool { self.controls[3] }
    pub fn lv(&self) -> bool { self.controls[4] }
    pub fn sp(&self) -> bool { self.controls[5] }
    pub fn pc(&self) -> bool { self.controls[6] }
    pub fn mdr(&self) -> bool { self.controls[7] }
    pub fn mar(&self) -> bool { self.controls[8] }
}

impl Register36 {
    pub fn mir_jmpc(self) -> bool { self.get()[9] }
    pub fn mir_jamn(self) -> bool { self.get()[10] }
    pub fn mir_jamz(self) -> bool { self.get()[11] }

    pub fn mir_addr(self) -> [bool; 9] {
        let mut res = [false; 9];
        res.copy_from_slice(&self.get()[..9]);
        res
    }

    pub fn mir_b_bus_controls(self) -> [bool; 4] {
        let mut res = [false; 4];
        res.copy_from_slice(&self.get()[32..36]);
        res
    }

    pub fn mir_write(self) -> bool { self.get()[29] }
    pub fn mir_read(self) -> bool { self.get()[30] }
    pub fn mir_fetch(self) -> bool { self.get()[31] }

    pub fn mir_alu_controls(self) -> AluControl {
        let mut code = [false; 6];
        code.copy_from_slice(&self.get()[14..20]);
        AluControl::from(code)
    }

    pub fn mir_c_bus_controls(self) -> CBusControls {
        let mut code = [false; 9];
        code.copy_from_slice(&self.get()[20..29]);
        CBusControls::new(code)
    }

    pub fn mir_ssl8(self) -> bool { self.get()[12] }
    pub fn mir_sra1(self) -> bool { self.get()[13] }
}
