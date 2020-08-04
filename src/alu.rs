use crate::bus::Bus32;
use crate::decoders::decoder_2x4;
use crate::main_memory::{fast_decode, fast_encode};

fn adder(a: bool, b: bool, carry_in: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(a, b);
    let (sum, carry2) = half_adder(sum1, carry_in);
    let carry_out = carry1 || carry2;
    (sum, carry_out)
}


fn half_adder(a: bool, b: bool) -> (bool, bool) { (a ^ b, a && b) }

pub struct AluControl {
    f0: bool,
    f1: bool,
    en_a: bool,
    en_b: bool,
    inv_a: bool,
    inc: bool,
}

impl AluControl {
    pub fn from(code: [bool; 6]) -> AluControl {
        AluControl {
            f0: code[0],
            f1: code[1],
            en_a: code[2],
            en_b: code[3],
            inv_a: code[4],
            inc: code[5],
        }
    }

    fn new() -> AluControl { AluControl { f0: false, f1: false, en_a: false, en_b: false, inv_a: false, inc: false } }

    fn f0(&mut self) -> &mut AluControl {
        self.f0 = true;
        self
    }
    fn f1(&mut self) -> &mut AluControl {
        self.f1 = true;
        self
    }
    fn ena(&mut self) -> &mut AluControl {
        self.en_a = true;
        self
    }
    fn enb(&mut self) -> &mut AluControl {
        self.en_b = true;
        self
    }
    fn inva(&mut self) -> &mut AluControl {
        self.inv_a = true;
        self
    }
    fn inc(&mut self) -> &mut AluControl {
        self.inc = true;
        self
    }

    fn alu_b_dec() -> AluControl { AluControl { f0: true, f1: true, en_a: false, en_b: true, inv_a: true, inc: false } }
    fn alu_b_inc() -> AluControl { AluControl { f0: true, f1: true, en_a: false, en_b: true, inv_a: false, inc: true } }
    fn alu_sum() -> AluControl { AluControl { f0: true, f1: true, en_a: true, en_b: true, inv_a: false, inc: false } }
    fn alu_sum_inc() -> AluControl { AluControl { f0: true, f1: true, en_a: true, en_b: true, inv_a: false, inc: true } }
    fn alu_sub() -> AluControl { AluControl { f0: true, f1: true, en_a: true, en_b: true, inv_a: true, inc: true } }
    fn alu_and() -> AluControl { AluControl { f0: false, f1: false, en_a: true, en_b: true, inv_a: false, inc: false } }
    fn alu_or() -> AluControl { AluControl { f0: false, f1: true, en_a: true, en_b: true, inv_a: false, inc: false } }
    fn alu_b() -> AluControl { AluControl { f0: false, f1: true, en_a: false, en_b: true, inv_a: false, inc: false } }
    fn alu_a() -> AluControl { AluControl { f0: false, f1: true, en_a: true, en_b: false, inv_a: false, inc: false } }
}

fn alu_unit(a: bool, b: bool, inv_a: bool, en_a: bool, en_b: bool, carry_in: bool, f0: bool, f1: bool) -> (bool, bool) {
    let a_enabled = a && en_a;
    let b_signal = b && en_b;
    let a_signal = a_enabled ^ inv_a;

    // Decode allow signals
    // f1 and f0 should be in this order because mic-1 uses different bit ordering
    let allowed = decoder_2x4(f1, f0);

    // Compute simple resultes
    let a_and_b_res = (a_signal && b_signal) && allowed[0];
    let a_or_b_res = (a_signal || b_signal) && allowed[1];
    let not_b_res = !b_signal && allowed[2];

    // A and B sum
    let (a_plus_b_res_temp, carry_temp) = adder(a_signal, b_signal, carry_in);
    let a_plus_b_res = a_plus_b_res_temp && allowed[3];
    let carry_out = carry_temp && allowed[3];

    // Final result
    let res = a_and_b_res || a_or_b_res || not_b_res || a_plus_b_res;

    (res, carry_out)
}

pub fn alu_32(a: Bus32, b: Bus32, control: AluControl) -> (Bus32, bool, bool) {
    let mut result = [false; 32];

    let mut carry = control.inc;
    for i in 0..32 {
        let (res, alu_carry) = alu_unit(a.data[i], b.data[i], control.inv_a, control.en_a, control.en_b, carry, control.f0, control.f1);
        result[i] = res;
        carry = alu_carry;
    }

    let n_bit = result[31];
    let mut z_bit = false;
    for i in 0..32 {
        z_bit |= result[i];
    }
    z_bit = !z_bit;

    (Bus32::from(result), n_bit, z_bit)
}

fn alu_32_i(a: i32, b: i32, control: AluControl) -> (i32, bool, bool) {
    let a_bus = Bus32::from(fast_decode(a));
    let b_bus = Bus32::from(fast_decode(b));

    let (alu_res, n, z) = alu_32(a_bus, b_bus, control);
    return (fast_encode(&alu_res.data), n, z);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decrement() {
        let (res, n, z) = alu_32_i(0, 1, AluControl::alu_b_dec());
        assert_eq!(0, res);
        assert_eq!(false, n);
        assert_eq!(true, z);
    }

    #[test]
    fn decrement_1() {
        let (res, n, z) = alu_32_i(0, 0, AluControl::alu_b_dec());
        assert_eq!(-1, res);
        assert_eq!(true, n);
        assert_eq!(false, z);
    }

    #[test]
    fn decrement_2() {
        let (res, n, z) = alu_32_i(0, 10, AluControl::alu_b_dec());
        assert_eq!(9, res);
        assert_eq!(false, n);
        assert_eq!(false, z);
    }

    #[test]
    fn increment() {
        let (res, n, z) = alu_32_i(0, 10, AluControl::alu_b_inc());
        assert_eq!(11, res);
        assert_eq!(false, n);
        assert_eq!(false, z);
    }

    #[test]
    fn increment_1() {
        let (res, n, z) = alu_32_i(0, -2, AluControl::alu_b_inc());
        assert_eq!(-1, res);
        assert_eq!(true, n);
        assert_eq!(false, z);
    }

    #[test]
    fn increment_2() {
        let (res, n, z) = alu_32_i(0, -1, AluControl::alu_b_inc());
        assert_eq!(0, res);
        assert_eq!(false, n);
        assert_eq!(true, z);
    }

    #[test]
    fn sum() {
        let (res, n, z) = alu_32_i(0, -1, AluControl::alu_sum());
        assert_eq!(-1, res);
        assert_eq!(true, n);
        assert_eq!(false, z);
    }

    #[test]
    fn sum_1() {
        let (res, n, z) = alu_32_i(1, 2, AluControl::alu_sum());
        assert_eq!(3, res);
        assert_eq!(false, n);
        assert_eq!(false, z);
    }

    #[test]
    fn sum_2() {
        let (res, n, z) = alu_32_i(0, 0, AluControl::alu_sum());
        assert_eq!(0, res);
        assert_eq!(false, n);
        assert_eq!(true, z);
    }

    #[quickcheck]
    fn quick_sum(a: i32, b: i32) {
        let (res, n, z) = alu_32_i(a, b, AluControl::alu_sum());
        let res = a + b;
        assert_eq!(res, res);
        assert_eq!(res < 0, n);
        assert_eq!(res == 0, z);
    }

    #[quickcheck]
    fn quick_dec(a: i32, b: i32) {
        let (res, n, z) = alu_32_i(a, b, AluControl::alu_b_dec());
        let res = b - 1;
        assert_eq!(res, res);
        assert_eq!(res < 0, n);
        assert_eq!(res == 0, z);
    }

    #[quickcheck]
    fn quick_inc(a: i32, b: i32) {
        let (res, n, z) = alu_32_i(a, b, AluControl::alu_b_inc());
        let res = b + 1;
        assert_eq!(res, res);
        assert_eq!(res < 0, n);
        assert_eq!(res == 0, z);
    }

    #[quickcheck]
    fn quick_sum_inc(a: i32, b: i32) {
        let (res, n, z) = alu_32_i(a, b, AluControl::alu_sum_inc());
        let res = a + b + 1;
        assert_eq!(res, res);
        assert_eq!(res < 0, n);
        assert_eq!(res == 0, z);
    }

    #[quickcheck]
    fn quick_sub(a: i32, b: i32) {
        let (res, n, z) = alu_32_i(a, b, AluControl::alu_sub());
        let res = b - a;
        assert_eq!(res, res);
        assert_eq!(res < 0, n);
        assert_eq!(res == 0, z);
    }

    #[quickcheck]
    fn quick_and(a: i32, b: i32) {
        let (res, n, z) = alu_32_i(a, b, AluControl::alu_and());
        let res = b & a;
        assert_eq!(res, res);
        assert_eq!(res < 0, n);
        assert_eq!(res == 0, z);
    }

    #[quickcheck]
    fn quick_or(a: i32, b: i32) {
        let (res, n, z) = alu_32_i(a, b, AluControl::alu_or());
        let res = b | a;
        assert_eq!(res, res);
        assert_eq!(res < 0, n);
        assert_eq!(res == 0, z);
    }

    #[quickcheck]
    fn quick_b(a: i32, b: i32) {
        let (res, n, z) = alu_32_i(a, b, AluControl::alu_b());
        let res = b;
        assert_eq!(res, res);
        assert_eq!(res < 0, n);
        assert_eq!(res == 0, z);
    }

    #[quickcheck]
    fn quick_a(a: i32, b: i32) {
        let (res, n, z) = alu_32_i(a, b, AluControl::alu_a());
        let res = a;
        assert_eq!(res, res);
        assert_eq!(res < 0, n);
        assert_eq!(res == 0, z);
    }
}
