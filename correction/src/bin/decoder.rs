use std::{env::args, fs::File, io::{BufWriter, Read, Write}};

use correction::hamming::decode;

pub fn main() {
    let args = args().collect::<Vec<_>>();

    let input_path = args.get(1).expect("Missing input path");
    let output_path = args.get(2).expect("Missing output path");

    let mut reader = File::open(input_path).expect("Missing input file");
    let mut writer = BufWriter::new(File::create(output_path).expect("Can't create output file"));

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();

    let mut count = 0;
    let mut total_count = 0;

    for bytes in buf.chunks(2) {
        let &[byte1, byte2] = bytes else { unreachable!() };

        let (upper_decoded, e1) = decode(byte1);
        let (lower_decoded, e2) = decode(byte2);

        println!("{:08b}", byte1);
        println!("{:08b}", byte2);

        if e1 {
            count += 1;
        }
        if e2 {
            count += 1;
        }

        let byte = upper_decoded | (lower_decoded >> 4); 


        writer.write_all(&[byte]).unwrap();

        total_count += 2;
    }

    println!("{count} {total_count}");
}