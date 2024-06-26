use std::{env::args, io::{BufReader, BufWriter}, fs::File, time::Instant};

use lzw::{coder::LZWCoder, universal_coder::{GammaCoder, DeltaCoder, OmegaCoder, FibonacciCoder}};

pub fn main() {
    let args = args().collect::<Vec<_>>();

    let coding = args.get(1).expect("Input coding gamma/delta/omega/fib");
    let input_path = args.get(2).expect("Please input the input file path");
    let output_path = args.get(3).expect("Please input the output file path");

    let input_stream = BufReader::new(File::open(input_path).unwrap());
    let mut output_stream = BufWriter::new(File::create(output_path).unwrap());

    
    let time_start = Instant::now();

    let (input_len, output_len) = match coding.as_str() {
        "gamma" => {
            let mut coder = LZWCoder::new(input_stream, &mut output_stream, GammaCoder);
            coder.code_to_end();
            coder.get_stat()
        },
        "delta" => {
            let mut coder = LZWCoder::new(input_stream, &mut output_stream, DeltaCoder);
            coder.code_to_end();
            coder.get_stat()
        },
        "omega" => {
            let mut coder = LZWCoder::new(input_stream, &mut output_stream, OmegaCoder);
            coder.code_to_end();
            coder.get_stat()
        },
        "fib" => {
            let mut coder = LZWCoder::new(input_stream, &mut output_stream, FibonacciCoder::new());
            coder.code_to_end();
            coder.get_stat()
        },
        _ => { panic!("Input coding as first argument (gamma/delta/omega/fib)") }
    };
    
    let time_end = Instant::now();
    let dur = (time_end - time_start).as_secs_f64();
    
    
    println!("Compression statistics:");
    println!("Input size: {:.3} MB", input_len as f64 / 1_000_000.0);
    println!("Output size: {:.3} MB", output_len as f64 / 1_000_000.0);
    println!("Compression ratio: {:.3}", input_len as f64 / output_len as f64);
    println!("Average code length: {:.3}", output_len as f64 * 8.0 / input_len as f64);
    println!("Speed: {:.2} MB/s", input_len as f64 / (dur * 1_000_000.0));
    
    let entropy = entropy::quick_entropy_of_file(input_path);
    println!("Entropy of source: {}", entropy);

    let entropy = entropy::quick_entropy_of_file(output_path);
    println!("Entropy of code: {}", entropy);
}