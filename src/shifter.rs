use crate::bus::Bus32;

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
    for _ in 0..8 {
        res = shift(res, true, enabled);
    }
    return res;
}

pub fn sra1(data: Bus32, enabled: bool) -> Bus32 {
    let mut res = shift(data, false, enabled);
    res.data[31] = res.data[30];
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_ones() {
        let bus32 = Bus32::from([true; 32]);

        let shifted = sll8(bus32, true);

        let result = shifted.data;
        for x in 0..8 {
            assert_eq!(result[x], false);
        }
        for x in 8..32 {
            assert_eq!(result[x], true);
        }
    }

    #[quickcheck]
    fn ssl8_check(data: Vec<bool>) {
        let mut res = [false; 32];
        for x in 0..32 {
            res[x] = *data.get(x).unwrap_or(&false);
        }

        let bus32 = Bus32::from(res);

        let shifted = sll8(bus32, true);

        let result = shifted.data;
        for x in 0..8 {
            assert_eq!(result[x], false);
        }
        for x in 8..32 {
            assert_eq!(result[x], res[x - 8]);
        }
    }
}
