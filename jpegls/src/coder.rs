use crate::tga::Channel;

pub fn predict(channel: &Channel, x: isize, y: isize, id: u8) -> u8 {
    let w = channel.get(x - 1, y);
    let n = channel.get(x, y - 1);
    let nw = channel.get(x - 1, y - 1); 

    match id {
        0 => {
            w
        }
        1 => {
            n
        }
        2 => {
            nw
        }
        3 => {
            n.wrapping_add(w).wrapping_sub(nw)
        }
        4 => {
            n.wrapping_add(w.wrapping_sub(nw) / 2)
        }
        5 => {
            w.wrapping_add(n.wrapping_sub(nw) / 2)
        }
        6 => {
            n.wrapping_add(w) / 2
        }
        7 => {
            if nw >= w.max(n) {
                w.min(n)
            } else if nw <= w.min(n) {
                w.max(n)
            } else {
                w.wrapping_add(n).wrapping_sub(nw)
            }
        }
        _ => { unreachable!() }
    }
}

pub fn code_with_prediction(channel: &Channel, id: u8) -> Vec<u8> {
    let mut result = vec![];

    for y in 0..channel.height {
        for x in 0..channel.width {
            let prediction = predict(channel, x, y, id);
            let value = channel.get(x, y);
            let diff = value.wrapping_sub(prediction);
            result.push(diff);
        }
    }

    result
}