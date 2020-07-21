pub struct Bus36 {
    pub data: [bool; 36]
}

impl Bus36 {
    pub fn new() -> Bus36 { Bus36 { data: [false; 36] } }
    pub fn from(data: [bool; 36]) -> Bus36 { Bus36 { data } }
    pub fn connect(&mut self, lines: [bool; 36]) {
        for i in 0..36 {
            self.data[i] = self.data[i] || lines[i];
        }
    }
}

pub struct Bus32 {
    pub data: [bool; 32]
}

impl Bus32 {
    pub fn new() -> Bus32 { Bus32 { data: [false; 32] } }
    pub fn from(data: [bool; 32]) -> Bus32 { Bus32 { data } }
    pub fn connect(&mut self, lines: [bool; 32]) {
        for i in 0..32 {
            self.data[i] = self.data[i] || lines[i];
        }
    }
}

pub struct Bus9 {
    pub data: [bool; 9]
}

impl Bus9 {
    pub fn new() -> Bus9 { Bus9 { data: [false; 9] } }
    pub fn from(data: [bool; 9]) -> Bus9 { Bus9 { data } }
    pub fn connect(&mut self, lines: [bool; 9]) {
        for i in 0..9 {
            self.data[i] = self.data[i] || lines[i];
        }
    }
}

