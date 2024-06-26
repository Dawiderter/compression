pub trait BitGetSet {
    fn get(&self, pos: u8) -> bool;
    fn set(&mut self, pos: u8, value: bool);
}

impl BitGetSet for u8 {
    fn get(&self, pos: u8) -> bool {
        self & ((1 << 7) >> pos) > 0
    }

    fn set(&mut self, pos: u8, value: bool) {
        let x = ((value as u8) << 7) >> pos;
        let mask = (1 << 7) >> pos;
        *self = (*self & !mask) | x;
    }
}

pub fn code(data: u8) -> u8 {
    let mut code = 0;

    code.set(0, data.get(0));
    code.set(1, data.get(0) ^ data.get(1));
    code.set(2, data.get(1) ^ data.get(2));
    code.set(3, data.get(0) ^ data.get(2) ^ data.get(3));
    code.set(4, data.get(1) ^ data.get(3));
    code.set(5, data.get(2));
    code.set(6, data.get(3));

    let mut parity = false;
    for i in 0..=6 {
        parity ^= code.get(i);
    }
    code.set(7, parity);

    code
}

fn check(code: u8) -> u8 {
    let mut check = 0;

    check.set(0, code.get(2) ^ code.get(4) ^ code.get(5) ^ code.get(6));
    check.set(1, code.get(1) ^ code.get(3) ^ code.get(4) ^ code.get(5));
    check.set(2, code.get(0) ^ code.get(2) ^ code.get(3) ^ code.get(4));

    let mut parity_check = false;
    for i in 0..=7 {
        parity_check ^= code.get(i);
    }
    check.set(3, parity_check);

    check
}

fn correction(check: u8) -> Option<u8> {
    match check {
        48 => Some(0),
        80 => Some(1),
        176 => Some(2),
        112 => Some(3),
        240 => Some(4),
        208 => Some(5),
        144 => Some(6),
        16 => Some(7),
        _ => None,
    }
}

pub fn decode(mut code: u8) -> (u8, bool) {
    let check = check(code);

    let mut double_error = false;
    //println!("{:08b} -> {:08b}", code, check);

    if check != 0 {
        let correction = correction(check);
        match correction {
            Some(i) => {
                let x = code.get(i);
                //println!("\t{i} {x}");
                code.set(i, !x);
            }
            None => {
                double_error = true;
            }
        }
    }

    let mut decoded = 0;
    decoded.set(0, code.get(0));
    decoded.set(1, code.get(1) ^ code.get(0));
    decoded.set(2, code.get(5));
    decoded.set(3, code.get(6));

    (decoded, double_error)
}

fn _generate_syndromes() {
    for i in 0..8 {
        let error = (1 << 7) >> i;

        let syndrome = check(error);

        eprintln!("{} {}", syndrome, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_test() {
        let mut code = 0;

        code.set(1, true);
        code.set(4, true);
        code.set(7, true);

        eprintln!("{:08b}", code);

        code.set(7, false);
        code.set(1, false);
        eprintln!("{:08b}", code);
    }

    #[test]
    fn single_err_test() {
        for e in 0..8 {
            let err = 1 << e;
            for i in 0..16 {
                let data = i << 4;
                let code = code(data);
                let err_code = code ^ err;
                let (decoded, _) = decode(err_code);
                assert_eq!(data, decoded);
            }
        }
    }

    #[test]
    fn double_err_test() {
        for e1 in 0..8 {
            for e2 in (e1+1)..8 {
                let err1 = 1 << e1;
                let err2 = 1 << e2;
                for i in 0..16 {
                    let data = i << 4;
                    let code = code(data);
                    let err_code = (code ^ err1) ^ err2;
                    let (_, err) = decode(err_code);
                    assert!(err);
                }
            } 
        }
    }

    #[test]
    fn syndromes() {
        _generate_syndromes();
    }

    #[test]
    fn code_test() {
        let data = 0b10000000;
        eprintln!("{:08b}", code(data));

        let data = 0b01000000;
        eprintln!("{:08b}", code(data));

        let data = 0b00100000;
        eprintln!("{:08b}", code(data));

        let data = 0b00010000;
        eprintln!("{:08b}", code(data));
    }
}
