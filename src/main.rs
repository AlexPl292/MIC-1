fn main() {
    println!("Hello, world!");
}

fn half_adder(a: bool, b: bool) -> (bool, bool) { (a ^ b, a && b) }

fn adder(a: bool, b: bool, carry_in: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(a, b);
    let (sum, carry2) = half_adder(sum1, carry_in);
    let carry_out = carry1 || carry2;
    (sum, carry_out)
}
