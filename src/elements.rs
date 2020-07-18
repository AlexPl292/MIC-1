fn nor(a: bool, b: bool) -> (bool) { !(a || b) }

fn half_adder(a: bool, b: bool) -> (bool, bool) { (a ^ b, a && b) }

fn adder(a: bool, b: bool, carry_in: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(a, b);
    let (sum, carry2) = half_adder(sum1, carry_in);
    let carry_out = carry1 || carry2;
    (sum, carry_out)
}

fn decoder_2(f0: bool, f1: bool) -> (bool, bool, bool, bool) {
    (!f0 && !f1, !f0 && f1, f0 && !f1, f0 && f1)
}

pub struct SrLatch {
    pub res: bool
}

impl SrLatch {
    pub fn new() -> SrLatch { Latch(false) }

    pub fn update(&mut self, setting: bool, resetting: bool) {
        let not_res = nor(setting, self.res);
        self.res = nor(not_res, resetting);
    }
}

pub struct DLatch {
    pub res: bool
}

impl DLatch {
    pub fn new() -> DLatch { DLatch(false) }

    pub fn update(&mut self, d: bool) {
        let not_res = nor(d, self.res);
        self.res = nor(not_res, !d);
    }
}
