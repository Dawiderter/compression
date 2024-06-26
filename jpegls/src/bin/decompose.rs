use std::{fs::read, env::args};

use image::{RgbImage, Rgb};
use jpegls::tga::read_tga_to_bitmap;

pub fn main() {
    let args = args().collect::<Vec<_>>();
    let input_path = args.get(1).expect("Please input the input file path");
    let input_buf = read(input_path).unwrap();

    let bitmap = read_tga_to_bitmap(&input_buf);
    let width = bitmap.red.width;
    let height = bitmap.red.height;

    let mut img_red = RgbImage::new(width as u32, height as u32);
    let mut img_green = RgbImage::new(width as u32, height as u32);
    let mut img_blue = RgbImage::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            let red = bitmap.red.get(x, y);
            let green = bitmap.green.get(x, y);
            let blue = bitmap.blue.get(x, y);
            img_red.put_pixel(x as u32, y as u32, Rgb([red, 0, 0]));
            img_green.put_pixel(x as u32, y as u32, Rgb([0, green, 0]));
            img_blue.put_pixel(x as u32, y as u32, Rgb([0,0,blue]));
        }
    }

    img_red.save(format!("{}-red.png", input_path)).unwrap();
    img_green.save(format!("{}-green.png", input_path)).unwrap();
    img_blue.save(format!("{}-blue.png", input_path)).unwrap();
}