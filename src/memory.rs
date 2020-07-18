use crate::bus::{Bus9, Bus36};

#[derive(Copy, Clone)]
pub struct DLatch {
    pub state: bool
}

impl DLatch {
    pub fn new() -> DLatch { DLatch { state: false } }

    pub fn update(&mut self, d: bool) {
        self.state = d
    }
}

#[derive(Copy, Clone)]
pub struct Register36 {
    pub registers: [DLatch; 36]
}

impl Register36 {
    pub fn new() -> Register36 { Register36 { registers: [DLatch::new(); 36] } }

    pub fn update(&mut self, input: [bool; 36]) {
        for i in 0..36 {
            self.registers[i].update(input[i]);
        }
    }

    pub fn get(&mut self) -> [bool; 36] {
        let mut res = [false; 36];
        for i in 0..36 {
            res[i] = self.registers[i].state;
        }
        res
    }
}

pub struct Memory512x36 {
    cells: [Register36; 512]
}

impl Memory512x36 {
    pub fn new() -> Memory512x36 { Memory512x36 { cells: [Register36::new(); 512] } }

}
