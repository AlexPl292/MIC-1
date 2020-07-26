use crate::bus::Bus32;
use std::fs::rename;

fn shift(data: Bus32, left: bool, enabled: bool) -> Bus32 {
    let mut res = Bus32::new();

    for i in 0..31 {
        res.data[i + 1] = left && data.data[i] && enabled || !enabled && data.data[i + 1];
    }

    for i in 1..32 {
        res.data[i - 1] = (res.data[i - 1] || !left && data.data[i]) && enabled || !enabled && data.data[i - 1];
    }

    res
}

pub fn sll8(data: Bus32, enabled: bool) -> Bus32 {
    let mut res: Bus32 = data;
    for x in 0..8 {
        res = shift(res, true, enabled);
    }
    return res;
}

pub fn sra1(data: Bus32, enabled: bool) -> Bus32 {
    let mut res = shift(data, false, enabled);
    res.data[31] = res.data[30];
    res
}
