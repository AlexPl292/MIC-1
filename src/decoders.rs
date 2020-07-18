pub fn decoder_2x4(f0: bool, f1: bool) -> [bool; 4] {
    [!f0 && !f1, !f0 && f1, f0 && !f1, f0 && f1]
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

fn decoder_4x9(f0: bool, f1: bool, f2: bool, f3: bool) -> [bool; 9] {
    let mut dest = [false; 9];
    let res = decoder_4x16(f0, f1, f2, f3);
    dest.copy_from_slice(&res[..9]);
    dest
}

fn decoder_9x512(input: [bool; 9]) -> [bool; 512] {
    let mut res = [false; 512];
    let first_part = decoder_4x16(input[0], input[1], input[2], input[3]);
    let second_part = decoder_4x16(input[4], input[5], input[6], input[7]);

    for i in 0..16 {
        for k in 0..16 {
            res[i * 16 + k] = first_part[i] && second_part[k];
            res[i * 16 + k + 256] = first_part[i] && second_part[k];
        }
    }

    for i in 256..512 {
        res[i] = res[i] && input[8];
    }

    res
}
