use std::{env::args, fs::read};

use filtering::{tga::read_tga_to_bitmap, coder::code};

fn main() {
    let args = args().collect::<Vec<_>>();

    let input_path = args.get(1).expect("Please input the input file path");
    let output_path = args.get(2).expect("Please input the output file path");
    let bits = args.get(3).map(|s| s.parse()).expect("Please input number of bits").expect("Please input a correct number");

    let input_buf = read(input_path).unwrap();
    let bitmap = read_tga_to_bitmap(&input_buf);

    let coded = code(&bitmap, bits);

    coded.save(output_path);
}
