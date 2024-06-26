use std::{fs::read, env::args};

use quant::{tga::read_tga_to_bitmap, lbg::{LBG, Metric, Splitting}, color::ColorVector};

fn main() {
    let args = args().collect::<Vec<_>>();

    let input_path = args.get(1).expect("Please input the input file path");
    let output_path = args.get(2).expect("Please input the output file path");
    let color = args.get(3).expect("Please input number of colors").parse::<u32>().expect("Error while parsing number of colors");
    let metric = args.get(4).map(|s| match s.as_str() {
        "man" => Metric::Manhattan,
        "eucl" => Metric::Euclid,
        _ => panic!("Input metrics man/eucl"),
    }).unwrap_or(Metric::Manhattan);

    let splitting = args.get(5).map(|s| match s.as_str() {
        "rand" => Splitting::Randomized,
        "const" => Splitting::Constant(ColorVector { r: 0.001, g: 0.001, b: 0.001 }),
        _ => panic!("Input splitting methods rand/const"),
    }).unwrap_or(Splitting::Randomized);


    let input_buf = read(input_path).unwrap();
    let bitmap = read_tga_to_bitmap(&input_buf);

    let mut lbg = LBG::new(&bitmap, metric, splitting);

    for i in 0..=color {
        lbg.split();
        lbg.optimize(0.01);
        let b = lbg.code();
        b.save(&format!("{}_col={}_{:?}_{:?}.png",output_path,i,metric,splitting));

        let mse = lbg.error();
        let snr = lbg.signal_noise_ratio();
        let snr_db = 10.0 * snr.log10();
        println!("Colors: {} (2^{}), MSE: {:.4}, SNR: {:.2} ({:.2} dB)", 2u32.pow(i),i, mse, snr, snr_db);
    }
}
