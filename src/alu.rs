use crate::bus::{Bus32};
use crate::decoders::decoder_2x4;

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

    (Bus32::from(result), n_bit, z_bit)
}
