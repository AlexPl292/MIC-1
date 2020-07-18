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

fn alu_unit(inv_a: bool, a: bool, en_a: bool, b: bool, en_b: bool, carry_in: bool, f0: bool, f1: bool) -> (bool, bool) {
    let a_enabled = a && en_a;
    let b_signal = b && en_b;
    let a_signal = a_enabled ^ inv_a;

    // Decode allow signals
    let (a_and_b_allowed, a_or_b_allowed, not_b_allowed, a_plus_b_allowed) = decoder_2(f0, f1);

    // Compute simple resultes
    let a_and_b_res = (a_signal && b_signal) && a_and_b_allowed;
    let a_or_b_res = (a_signal || b_signal) && a_or_b_allowed;
    let not_b_res = !b_signal && not_b_allowed;

    // A and B sum
    let (a_plus_b_res_temp, carry_temp) = adder(a_signal, b_signal, carry_in);
    let a_plus_b_res = a_plus_b_res_temp && a_plus_b_allowed;
    let carry_out = carry_temp && a_plus_b_allowed;

    // Final result
    let res = a_and_b_res || a_or_b_res || not_b_res || a_plus_b_res;

    (res, carry_out)
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
