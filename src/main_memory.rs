use crate::main_memory::ReadState::{NoRead, ReadInitialized, ReadInProgress};

pub struct MainMemory {
    cells: [i32; 512],

    pub first_reading: Vec<(i32, ReadState)>,
    pub second_reading: Vec<(i32, ReadState)>,
}

impl MainMemory {
    pub fn initialize() -> MainMemory { MainMemory { cells: [0; 512], first_reading: Vec::new(), second_reading: Vec::new() } }

    pub fn write_data(&mut self, data: i32, addr: usize) {
        self.cells[addr] = data
    }

    pub fn write(&mut self, data: [bool; 32], addr: [bool; 32], enabled: bool) {
        if !enabled { return; }
        self.cells[fast_encode(&addr) as usize] = fast_encode(&data)
    }

    pub fn read(&self, addr: [bool; 32]) -> [bool; 32] {
        let i_addr = fast_encode(&addr);
        let data = self.cells[i_addr as usize];
        fast_decode(data)
    }

    pub fn request_first_read(&mut self, addr: [bool; 32], enabled: bool) {
        if !enabled { return; }
        self.first_reading.push((fast_encode(&addr), ReadInitialized));
    }

    pub fn request_second_read(&mut self, addr: [bool; 32], enabled: bool) {
        if !enabled { return; }
        self.second_reading.push((fast_encode(&addr), ReadInitialized));
    }

    pub fn check_first_read(&mut self) -> ([bool; 32], bool) {
        let mut res = [false; 32];
        let mut enabled = false;
        for i in 0..self.first_reading.len() {
            if self.first_reading[i].1 == ReadInProgress {
                self.first_reading[i].1 = NoRead;
                res = self.read(fast_decode(self.first_reading[i].0));
                enabled = true;
            } else if self.first_reading[i].1 == ReadInitialized {
                self.first_reading[i].1 = ReadInProgress;
            }
        }
        self.first_reading.retain(|x| x.1 != NoRead);
        return (res, enabled);
    }

    pub fn check_second_read(&mut self) -> ([bool; 32], bool) {
        let mut res = [false; 32];
        let mut enabled = false;
        for i in 0..self.second_reading.len() {
            if self.second_reading[i].1 == ReadInProgress {
                self.second_reading[i].1 = NoRead;
                res = self.read(fast_decode(self.second_reading[i].0));
                enabled = true;
            } else if self.second_reading[i].1 == ReadInitialized {
                self.second_reading[i].1 = ReadInProgress;
            }
        }
        self.second_reading.retain(|x| x.1 != NoRead);
        return (res, enabled);
    }
}

pub fn fast_decode(number: i32) -> [bool; 32] {
    let mut res = [false; 32];
    for i in 0..32 {
        res[i] = (number & (1 << (i))) != 0;
    }
    res
}

pub fn fast_encode(data: &[bool; 32]) -> i32 {
    let mut res = 0;

    for i in 0..32 {
        res = res | (if data[i] { 1 } else { 0 }) << i;
    }

    res
}

#[derive(PartialEq)]
pub enum ReadState {
    ReadInitialized,
    ReadInProgress,
    NoRead,
}
