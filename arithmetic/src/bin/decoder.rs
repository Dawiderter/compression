use std::{env::args, fs::File, io::{BufReader, BufWriter}, time::Instant};

use arithmetic::decoder::Decoder;

pub fn main() {
    let args = args().collect::<Vec<_>>();

    let input_path = args.get(1).expect("Please input the input file path");
    let output_path = args.get(2).expect("Please input the output file path");

    let input_stream = BufReader::new(File::open(input_path).unwrap());
    let mut output_stream = BufWriter::new(File::create(output_path).unwrap());

    let mut decoder = Decoder::new(input_stream, &mut output_stream);

    let time_start = Instant::now();
    decoder.decode_all();
    let time_end = Instant::now();

    let dur = (time_end - time_start).as_secs_f64();

    let output_len = decoder.get_stat();

    println!("Speed: {:.2} MB/s", output_len as f64 / (dur * 1_000_000.0));
}