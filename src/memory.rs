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

pub struct Register8 {
    pub registers: [DLatch; 8]
}

impl Register8 {
    pub fn new() -> Register8 { Register8 { registers: [DLatch::new(); 8] } }

    pub fn update(&mut self, input: [bool; 8]) {
        for i in 0..8 {
            self.registers[i].update(input[i]);
        }
    }

    pub fn get(&mut self) -> [bool; 8] {
        let mut res = [false; 8];
        for i in 0..8 {
            res[i] = self.registers[i].state;
        }
        res
    }
}

