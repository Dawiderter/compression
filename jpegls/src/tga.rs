#[derive(Debug)]
pub struct Bitmap {
    pub red: Channel,
    pub green: Channel,
    pub blue: Channel,
}

impl Bitmap {
    pub fn iter(&self) -> impl Iterator<Item = &u8> {
        self.red.iter().chain(self.green.iter()).chain(self.blue.iter())
    }
}

#[derive(Debug)]
pub struct Channel {
    pub width: isize,
    pub height: isize,
    pub data: Vec<u8>,
}

impl Channel {
    pub fn new(width: isize, height: isize) -> Self {
        Self {
            width,
            height,
            data: Vec::with_capacity((width * height) as usize),
        }
    }

    pub fn get(&self, x: isize, y: isize) -> u8 {
        if (0 <= x && x < self.width) && (0 <= y && y < self.height) {
            self.data[(y * self.width + x) as usize]
        } else {
            0
        }

    }

    pub fn iter(&self) -> impl Iterator<Item = &u8> {
        self.data.iter()
    }
}

pub fn read_tga_to_bitmap(input: &[u8]) -> Bitmap {
    let mut input = input;

    let header = &input[..18];
    input = &input[18..];

    let width = &header[12..14];
    let width = width[0] as isize + width[1] as isize * 256;
    let height = &header[14..16];
    let height = height[0] as isize + height[1] as isize * 256;

    let mut bitmap = Bitmap {
        red: Channel::new(width, height),
        green: Channel::new(width, height),
        blue: Channel::new(width, height),
    };

    for _ in 0..height {
        for _ in 0..width {
            let colors = &input[..3];
            input = &input[3..];
            bitmap.red.data.push(colors[2]);
            bitmap.green.data.push(colors[1]);
            bitmap.blue.data.push(colors[0]);
        }
    }

    bitmap.red.data.reverse();
    bitmap.green.data.reverse();
    bitmap.blue.data.reverse();

    for y in 0..height {
        let y = y as usize;
        let width = width as usize;
        bitmap.red.data[y*width..(y+1)*width].reverse();
        bitmap.green.data[y*width..(y+1)*width].reverse();
        bitmap.blue.data[y*width..(y+1)*width].reverse();
    }

    bitmap
}

#[cfg(test)]
mod tests {
    use image::{RgbImage, Rgb, save_buffer};

    use super::*;

    #[test]
    fn test_read() {
        let bytes = include_bytes!("./../testy4/example1.tga");

        let bitmap = read_tga_to_bitmap(bytes);
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

        img_red.save("./mytest/red.png").unwrap();
        img_green.save("./mytest/green.png").unwrap();
        img_blue.save("./mytest/blue.png").unwrap();
    }
}