use std::{fs::File, path::Path};

use crate::{
    lbg::LBG,
    tga::{Bitmap, Channel},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodedChannel {
    pub width: u32,
    pub height: u32,
    pub y_diff: Vec<i16>,
    pub z: Vec<i16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodedImage {
    pub red: CodedChannel,
    pub green: CodedChannel,
    pub blue: CodedChannel,
}

impl CodedImage {
    pub fn save(&self, path: impl AsRef<Path>) {
        let file = File::create(path).unwrap();
        bincode::serialize_into(file, self).unwrap();
    }
    pub fn read(path: impl AsRef<Path>) -> Self {
        let file = File::open(path).unwrap();
        bincode::deserialize_from(file).unwrap()
    }
}

pub fn code(bitmap: &Bitmap, bits: u8) -> CodedImage {
    let red = code_channel(&bitmap.red, bits);
    let green = code_channel(&bitmap.green, bits);
    let blue = code_channel(&bitmap.blue, bits);

    CodedImage { red, green, blue }
}

pub fn code_channel(channel: &Channel, bits: u8) -> CodedChannel {
    let mut y = Vec::new();
    let mut z = Vec::new();

    let size = channel.height as usize * channel.width as usize;
    let size = (size + 1) / 2;
    for i in 0..size {
        let x_ip = channel.get_lin(2 * i);
        let x_i = channel.get_lin(2 * i + 1);

        y.push((x_i as i16 + x_ip as i16) / 2);
        z.push((x_i as i16 - x_ip as i16) / 2);
    }

    let mut y_diff = vec![0; y.len()];
    y_diff[0] = y[0];

    for i in 1..y.len() {
        y_diff[i] = y[i] - y[i - 1];
    }

    let y_quantizer = LBG::new(&y_diff).with_bits(bits).quantizer();

    //println!("{:?}", y_quantizer.clusters);
    //println!("{:?}", &y_decoded[0..100]);
    
    let mut y_better_diff = vec![0; y.len()];
    y_better_diff[0] = y[0].clamp(0, 255);
    let mut current_decode_y = y_better_diff[0];
    for i in 1..y.len() {
        let diff = y[i] - current_decode_y;
        y_better_diff[i] = y_quantizer.code_one(diff);
        current_decode_y = (current_decode_y + y_better_diff[i]).clamp(0, 255);
    }
    
    let z_coded = LBG::new(&z).with_bits(bits).code();

    CodedChannel {
        width: channel.width as u32,
        height: channel.height as u32,
        y_diff: y_better_diff,
        // y_diff,
        z: z_coded,
        // z,
    }
}
