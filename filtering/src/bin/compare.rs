use std::{env::args, fs::read};

use filtering::tga::{read_tga_to_bitmap, read_img_to_bitmap, Bitmap, Channel};

pub fn error(input: &Bitmap, output: &Bitmap) -> f32 {
    let mut err_sum = 0.0;

    let mut len = 0;
    for (i,o) in input.iter_zip().zip(output.iter_zip()) {
        let mut sum = 0.0;
        sum += (i.0 as f32 - o.0 as f32).abs();
        sum += (i.1 as f32 - o.1 as f32).abs();
        sum += (i.2 as f32 - o.2 as f32).abs();
        err_sum += sum.powi(2);
        len += 1;
    }

    err_sum / len as f32
}

pub fn error_channel(input: &Channel, output: &Channel) -> f32 {
    let mut err_sum = 0.0;

    let mut len = 0;
    for (&i,&o) in input.iter().zip(output.iter()) {
        let mut sum = 0.0;
        sum += (i as f32 - o as f32).abs();
        err_sum += sum.powi(2);
        len += 1;
    }

    err_sum / len as f32
}

pub fn signal_noise_ratio(input: &Bitmap, output: &Bitmap) -> f32 {
    let mse = error(input,output);

    let mut sum = 0.0;

    let mut len = 0;
    for i in input.iter_zip() {
        let mut sum_i = 0.0;
        sum_i += i.0 as f32;
        sum_i += i.1 as f32;
        sum_i += i.2 as f32;
        sum += sum_i.powi(2);
        len += 1;
    }

    (sum / len as f32) / mse
}

pub fn signal_noise_ratio_channel(input: &Channel, output: &Channel) -> f32 {
    let mse = error_channel(input,output);

    let mut sum = 0.0;

    let mut len = 0;
    for &i in input.iter() {
        let mut sum_i = 0.0;
        sum_i += i as f32;
        sum += sum_i.powi(2);
        len += 1;
    }

    (sum / len as f32) / mse
}

fn main() {
    let args = args().collect::<Vec<_>>();

    let input_path = args.get(1).expect("Please input the input file path");
    let output_path = args.get(2).expect("Please input the output file path");

    let input_buf = read(input_path).unwrap();
    let bitmap_input = read_tga_to_bitmap(&input_buf);

    let output = image::open(output_path).unwrap();
    let bitmap_output = read_img_to_bitmap(output);

    // bitmap_output.save("./test.png");

    assert_eq!(bitmap_input.red.width, bitmap_output.red.width);
    assert_eq!(bitmap_input.red.height, bitmap_output.red.height);

    println!("MSE: {}", error(&bitmap_input, &bitmap_output));
    let snr = signal_noise_ratio(&bitmap_input, &bitmap_output);
    let snr_db = 10.0 * snr.log10();
    println!("SNR: {} ({} dB)", snr, snr_db);

    println!("MSE Red: {}", error_channel(&bitmap_input.red, &bitmap_output.red));
    let snr = signal_noise_ratio_channel(&bitmap_input.red, &bitmap_output.red);
    let snr_db = 10.0 * snr.log10();
    println!("SNR Red: {} ({} dB)", snr, snr_db);

    println!("MSE Green: {}", error_channel(&bitmap_input.green, &bitmap_output.green));
    let snr = signal_noise_ratio_channel(&bitmap_input.green, &bitmap_output.green);
    let snr_db = 10.0 * snr.log10();
    println!("SNR Green: {} ({} dB)", snr, snr_db);

    println!("MSE Blue: {}", error_channel(&bitmap_input.blue, &bitmap_output.blue));
    let snr = signal_noise_ratio_channel(&bitmap_input.blue, &bitmap_output.blue);
    let snr_db = 10.0 * snr.log10();
    println!("SNR Blue: {} ({} dB)", snr, snr_db);
}