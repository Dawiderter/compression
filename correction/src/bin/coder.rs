use std::{env::args, fs::File, io::{BufWriter, Read, Write}};

use correction::hamming::code;

pub fn main() {
    let args = args().collect::<Vec<_>>();

    let input_path = args.get(1).expect("Missing input path");
    let output_path = args.get(2).expect("Missing output path");

    let mut reader = File::open(input_path).expect("Missing input file");
    let mut writer = BufWriter::new(File::create(output_path).expect("Can't create output file"));

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();

    for byte in buf {
        let upper = byte & 0b11110000;
        let coded_upper = code(upper);

        
        let lower = byte << 4;
        let coded_lower = code(lower);
        
        // println!("{:08b}", coded_upper);
        // println!("{:08b}", coded_lower);

        writer.write_all(&[coded_upper, coded_lower]).unwrap();
    }
}