pub fn decoder_2x4(f0: bool, f1: bool) -> [bool; 4] {
    [!f0 && !f1, f0 && !f1, !f0 && f1, f0 && f1]
}

fn decoder_4x16(f0: bool, f1: bool, f2: bool, f3: bool) -> [bool; 16] {
    let mut res = [false; 16];
    let en = decoder_2x4(f2, f3);
    let s = decoder_2x4(f0, f1);
    for i in 0..4 {
        for k in 0..4 {
            res[i * 4 + k] = en[i] && s[k];
        }
    }

    res
}

pub fn decoder_4x9(input: [bool; 4]) -> [bool; 9] {
    let mut dest = [false; 9];
    let res = decoder_4x16(input[0], input[1], input[2], input[3]);
    dest.copy_from_slice(&res[..9]);
    dest
}

pub fn decoder_9x512(input: [bool; 9]) -> [bool; 512] {
    let mut res = [false; 512];
    let first_part = decoder_4x16(input[0], input[1], input[2], input[3]);
    let second_part = decoder_4x16(input[4], input[5], input[6], input[7]);

    for i in 0..16 {
        for k in 0..16 {
            res[i * 16 + k] = first_part[k] && second_part[i];
            res[i * 16 + k + 256] = first_part[k] && second_part[i];
        }
    }

    for i in 0..256 {
        res[i] = res[i] && !input[8];
    }

    for i in 256..512 {
        res[i] = res[i] && input[8];
    }

    res
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};

    use super::*;

    #[test]
    fn dec_2x4() {
        assert_eq!([true, false, false, false], decoder_2x4(false, false));
        assert_eq!([false, true, false, false], decoder_2x4(true, false));
        assert_eq!([false, false, true, false], decoder_2x4(false, true));
        assert_eq!([false, false, false, true], decoder_2x4(true, true));
    }

    #[test]
    fn dec_all_zero() {
        let input = decoder_9x512([false; 9]);
        assert_eq!(true, input[0]);
        for x in 1..512 {
            assert_eq!(false, input[x]);
        }
    }

    #[test]
    fn dec_last_true() {
        let input = decoder_9x512([false, false, false, false, false, false, false, false, true]);
        for x in 0..512 {
            if x == 256 {
                assert_eq!(true, input[x], "Index: {}", x);
            } else {
                assert_eq!(false, input[x], "Index: {}", x);
            }
        }
    }

    #[quickcheck]
    fn dec_4_16(data: Vec<bool>) {
        let mut res = [false; 4];
        for x in 0..4 {
            res[x] = *data.get(x).unwrap_or(&false);
        }

        let mut number = 0;
        for x in 0..4 {
            number += if res[x] { 2i32.pow(x as u32) } else { 0 };
        }

        let decoded = decoder_4x16(res[0], res[1], res[2], res[3]);
        for x in 0..16 {
            assert_eq!(number == x, decoded[x as usize], "Index: {}", x)
        }
    }

    #[quickcheck]
    fn dec_9_512(data: Vec<bool>) {
        let mut res = [false; 9];
        for x in 0..9 {
            res[x] = *data.get(x).unwrap_or(&false);
        }

        let mut number = 0;
        for x in 0..9 {
            number += if res[x] { 2i32.pow(x as u32) } else { 0 };
        }

        let decoded = decoder_9x512(res);
        for x in 0..512 {
            assert_eq!(number == x, decoded[x as usize], "Index: {}", x)
        }
    }
}
