use crate::bus::{Bus36, Bus9};
use crate::decoders::decoder_9x512;

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

    pub fn update_from_bus(&mut self, input: Bus36) {
        self.update(input.data)
    }

    pub fn get(self) -> [bool; 36] {
        let mut res = [false; 36];
        for i in 0..36 {
            res[i] = self.registers[i].state;
        }
        res
    }

    pub fn read(self, enabled: bool) -> [bool; 36] {
        let mut res = self.get();
        for i in 0..36 {
            res[i] = res[i] && enabled
        }
        res
    }
}

#[derive(Copy, Clone)]
pub struct Register9 {
    pub registers: [DLatch; 9]
}

impl Register9 {
    pub fn new() -> Register9 { Register9 { registers: [DLatch::new(); 9] } }

    pub fn update(&mut self, input: [bool; 9]) {
        for i in 0..9 {
            self.registers[i].update(input[i]);
        }
    }

    pub fn get(self) -> [bool; 9] {
        let mut res = [false; 9];
        for i in 0..9 {
            res[i] = self.registers[i].state;
        }
        res
    }

    pub fn to_bus(self) -> Bus9 {
        let mut res = [false; 9];
        for i in 0..9 {
            res[i] = self.registers[i].state;
        }
        Bus9 { data: res }
    }
}

pub struct Memory512x36 {
    cells: [Register36; 512]
}

impl Memory512x36 {
    pub fn new() -> Memory512x36 { Memory512x36 { cells: [Register36::new(); 512] } }

    pub fn get(&self, address: Bus9) -> Bus36 {
        let mut res_array = [false; 36];
        let decoded_address = decoder_9x512(address.data);
        for i in 0..512 {
            for k in 0..36 {
                res_array[k] = res_array[k] || self.cells[i].registers[k].state && decoded_address[i]
            }
        }
        Bus36 { data: res_array }
    }
}
