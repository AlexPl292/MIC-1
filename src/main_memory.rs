pub struct MainMemory {
    cells: [i32; 512]
}

impl MainMemory {
    pub fn read(&self, addr: [bool; 32]) -> [bool; 32] {
        let i_addr = fast_encode(addr);
        let data = self.cells[i_addr as usize];
        fast_decode(data)
    }
}

fn fast_decode(number: i32) -> [bool; 32] {
    let mut res = [false; 32];
    for i in 0..32 {
        res[i] = (number & (1 << (31 - i))) != 0;
    }
    res
}

fn fast_encode(data: [bool; 32]) -> i32 {
    let mut res = 0;

    for i in 0..32 {
        res = res | (if data[i] { 1 } else { 0 }) << (31 - i);
    }

    res
}

#[derive(PartialEq)]
pub enum ReadState {
    ReadInitialized,
    ReadInProgress,
    NoRead,
}
