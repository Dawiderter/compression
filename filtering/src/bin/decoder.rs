use std::env::args;

use filtering::{coder::CodedImage, decoder::decode};

fn main() {
    let args = args().collect::<Vec<_>>();

    let input_path = args.get(1).expect("Please input the input file path");
    let output_path = args.get(2).expect("Please input the output file path");

    let coded = CodedImage::read(input_path);

    let bitmap = decode(coded);

    bitmap.save(output_path);
}