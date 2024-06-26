use std::{env::args, fs::File, io::{BufReader, BufWriter}, time::Instant};

use arithmetic::coder::Coder;

pub fn main() {
    let args = args().collect::<Vec<_>>();

    let input_path = args.get(1).expect("Please input the input file path");
    let output_path = args.get(2).expect("Please input the output file path");

    let input_stream = BufReader::new(File::open(input_path).unwrap());
    let mut output_stream = BufWriter::new(File::create(output_path).unwrap());

    let mut coder = Coder::new(input_stream, &mut output_stream);
    
    let time_start = Instant::now();
    coder.code_all();
    let time_end = Instant::now();
    let dur = (time_end - time_start).as_secs_f64();
    
    let (input_len, output_len) = coder.get_stat();
    
    println!("Compression statistics:");
    println!("Compression ratio: {}", input_len as f64 / output_len as f64);
    println!("Average code length: {}", output_len as f64 * 8.0 / input_len as f64);
    println!("Speed: {:.2} MB/s", input_len as f64 / (dur * 1_000_000.0));
    
    let entropy = entropy::quick_entropy_of_file(input_path);
    println!("Entropy of source: {}", entropy);
}