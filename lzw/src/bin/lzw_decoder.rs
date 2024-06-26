use std::{env::args, io::{BufReader, BufWriter}, fs::File, time::Instant};

use lzw::{decoder::LZWDecoder, universal_decoder::{FibonacciDecoder, OmegaDecoder, DeltaDecoder, GammaDecoder}};

pub fn main() {
    let args = args().collect::<Vec<_>>();

    let coding = args.get(1).expect("Input coding gamma/delta/omega/fib");
    let input_path = args.get(2).expect("Please input the input file path");
    let output_path = args.get(3).expect("Please input the output file path");

    let input_stream = BufReader::new(File::open(input_path).unwrap());
    let mut output_stream = BufWriter::new(File::create(output_path).unwrap());

    
    let time_start = Instant::now();

    let output_len = match coding.as_str() {
        "gamma" => {
            let mut coder = LZWDecoder::new(input_stream, &mut output_stream, GammaDecoder);
            coder.decode_to_end();
            coder.get_stat()
        },
        "delta" => {
            let mut coder = LZWDecoder::new(input_stream, &mut output_stream, DeltaDecoder);
            coder.decode_to_end();
            coder.get_stat()
        },
        "omega" => {
            let mut coder = LZWDecoder::new(input_stream, &mut output_stream, OmegaDecoder);
            coder.decode_to_end();
            coder.get_stat()
        },
        "fib" => {
            let mut coder = LZWDecoder::new(input_stream, &mut output_stream, FibonacciDecoder::new());
            coder.decode_to_end();
            coder.get_stat()
        },
        _ => { panic!("Input coding as first argument (gamma/delta/omega/fib)") }
    };
    
    let time_end = Instant::now();
    let dur = (time_end - time_start).as_secs_f64();
    
    
    println!("Compression statistics:");
    println!("Output size: {:.3} MB", output_len as f64 / 1_000_000.0);
    println!("Speed: {:.2} MB/s", output_len as f64 / (dur * 1_000_000.0));
}