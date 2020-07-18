use crate::bus::{Bus9, Bus36};
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

    pub fn get(self) -> [bool; 36] {
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

    pub fn get(self, address: Bus9) -> Bus36 {
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
