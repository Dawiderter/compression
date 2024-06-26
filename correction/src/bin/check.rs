use std::{env::args, fs::File, io::Read};

pub fn main() {
    let args = args().collect::<Vec<_>>();

    let input1_path = args.get(1).expect("Missing input path 1");
    let input2_path = args.get(2).expect("Missing input path 2");

    let mut reader1 = File::open(input1_path).expect("Missing input file 1");
    let mut reader2 = File::open(input2_path).expect("Missing input file 2");

    let mut buf1 = Vec::new();
    reader1.read_to_end(&mut buf1).unwrap();

    let mut buf2 = Vec::new();
    reader2.read_to_end(&mut buf2).unwrap();

    if buf1.len() != buf2.len() {
        panic!("Buffers have different lenth");
    }

    let mut count = 0;
    let mut count_all = 0;
    for (byte1, byte2) in buf1.into_iter().zip(buf2.into_iter()) {
        let upper1 = byte1 & 0b11110000;
        let upper2 = byte2 & 0b11110000;
        if upper1 != upper2 {
            count += 1;
        }
        
        let lower1 = byte1 << 4;
        let lower2 = byte2 << 4;

        if lower1 != lower2 {
            count += 1;
        }

        count_all += 2;
    }

    println!("{} {}", count, count_all);
}