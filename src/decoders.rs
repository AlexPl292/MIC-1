fn decoder_4x16(f0: bool, f1: bool, f2: bool, f3: bool) -> [bool; 16] {
    let mut res = [false; 16];
    let en = decoder_2x4(f2, f3);
    let s = decoder_2x4(f0, f1);
    for i in 0..4 {
        res[i * 4 + 0] = en[i] && s[0];
        res[i * 4 + 1] = en[i] && s[1];
        res[i * 4 + 2] = en[i] && s[2];
        res[i * 4 + 3] = en[i] && s[3];
    }

    res
}

pub fn decoder_2x4(f0: bool, f1: bool) -> [bool; 4] {
    [!f0 && !f1, !f0 && f1, f0 && !f1, f0 && f1]
}

fn decoder_4x9(f0: bool, f1: bool, f2: bool, f3: bool) -> [bool; 9] {
    let mut dest = [false; 9];
    let res = decoder_4x16(f0, f1, f2, f3);
    dest.copy_from_slice(&res[..9]);
    dest
}

