use std::{env::args, fs::File, io::{BufWriter, Read, Write}};

use correction::hamming::BitGetSet;
use rand::{distributions::{Bernoulli, Distribution}, thread_rng};

pub fn main() {
    let args = args().collect::<Vec<_>>();

    let p : f64 = args.get(1).expect("Missing p").parse().expect("p not a number");
    let input_path = args.get(2).expect("Missing input path");
    let output_path = args.get(3).expect("Missing output path");

    let mut reader = File::open(input_path).expect("Missing input file");
    let mut writer = BufWriter::new(File::create(output_path).expect("Can't create output file"));

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();

    let mut rng = thread_rng();
    let bern = Bernoulli::new(p).unwrap();

    for mut byte in buf {
        for i in 0..8 {
            if bern.sample(&mut rng) {
                let x = byte.get(i);
                byte.set(i, !x);
            }
        }
        writer.write_all(&[byte]).unwrap();
    }
}