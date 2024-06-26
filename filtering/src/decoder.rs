use crate::{coder::{CodedImage, CodedChannel}, tga::{Bitmap, Channel}};

pub fn decode(image: CodedImage) -> Bitmap {
    let red = decode_channel(image.red);
    let green = decode_channel(image.green);
    let blue = decode_channel(image.blue);

    Bitmap { red, green, blue }
}

fn decode_channel(channel: CodedChannel) -> Channel {
    let width = channel.width as isize;
    let height = channel.height as isize;
    let size = width as usize * height as usize;
    
    let mut y_decoded = vec![0; channel.y_diff.len()];
    y_decoded[0] = channel.y_diff[0].clamp(0, 255);
    for i in 1..channel.y_diff.len() {
        y_decoded[i] = (y_decoded[i-1] + channel.y_diff[i]).clamp(0, 255);
    }

    let mut data = vec![0; size];
    
    for i in 0..size {
        if i % 2 == 0 {
            data[i] = (y_decoded[i / 2] - channel.z[i / 2]).clamp(0, 255) as u8;
        } else {
            data[i] = (y_decoded[i / 2] + channel.z[i / 2]).clamp(0, 255) as u8;
        }
    }
    
    Channel { width, height, data }

}