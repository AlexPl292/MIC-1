fn nor(a: bool, b: bool) -> (bool) { !(a || b) }

fn half_adder(a: bool, b: bool) -> (bool, bool) { (a ^ b, a && b) }

fn adder(a: bool, b: bool, carry_in: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(a, b);
    let (sum, carry2) = half_adder(sum1, carry_in);
    let carry_out = carry1 || carry2;
    (sum, carry_out)
}


pub struct DLatch {
    pub state: bool
}

impl DLatch {
    pub fn new() -> DLatch { DLatch(false) }

    pub fn update(&mut self, d: bool) {
        self.state = d
    }
}

pub struct Register8 {
    pub registers: [DLatch; 8]
}

impl Register8 {
    pub fn new() -> Register8 { Register8([DLatch::new(); 8]) }

    pub fn update(&mut self, input: [bool; 8]) {
        for i in 0..8 {
            self.registers[i].update(input[i]);
        }
    }

    pub fn get(&mut self) -> [bool; 8] {
        self.registers.iter().map(|x| x.state).collect()
    }
}
