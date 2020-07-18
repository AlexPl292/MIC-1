use crate::decoders::decoder_2x4;

fn adder(a: bool, b: bool, carry_in: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(a, b);
    let (sum, carry2) = half_adder(sum1, carry_in);
    let carry_out = carry1 || carry2;
    (sum, carry_out)
}


fn half_adder(a: bool, b: bool) -> (bool, bool) { (a ^ b, a && b) }

struct AluControl {
    inv_a: bool,
    en_a: bool,
    en_b: bool,
    f0: bool,
    f1: bool,
    inc: bool,
}

fn alu_unit(a: bool, b: bool, inv_a: bool, en_a: bool, en_b: bool, carry_in: bool, f0: bool, f1: bool) -> (bool, bool) {
    let a_enabled = a && en_a;
    let b_signal = b && en_b;
    let a_signal = a_enabled ^ inv_a;

    // Decode allow signals
    let allowed = decoder_2x4(f0, f1);

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

fn alu_8(a: [bool; 8], b: [bool; 8], control: AluControl) -> ([bool; 8], bool) {
    let mut result = [false; 8];

    let mut carry = control.inc;
    for i in 0..8 {
        let (res, alu_carry) = alu_unit(a[i], b[i], control.inv_a, control.en_a, control.en_b, carry, control.f0, control.f1);
        result[i] = res;
        carry = alu_carry;
    }

    (result, carry)
}

