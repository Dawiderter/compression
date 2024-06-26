use std::io::{Read, Write};

use crate::{universal_decoder::{BitInputStream, UniversalDecoder}, MAX_DICT_SIZE};

pub struct LZWDecoder<I, O, D> {
    input_stream: BitInputStream<I>,
    output_stream: O,
    universal_decoder: D,
    last_id: Option<usize>,
    dict: Vec<Vec<u8>>,
    output_len: usize,
}

impl<I: Read, O: Write, D: UniversalDecoder> LZWDecoder<I, O, D> {
    pub fn new(input_stream: I, output_stream: O, universal_decoder: D) -> Self {
        let mut dict = vec![];
        for i in 0..=255 {
            dict.push(vec![i]);
        }

        Self {
            input_stream: BitInputStream::new(input_stream),
            output_stream,
            universal_decoder,
            last_id: None,
            dict,
            output_len: 0,
        }
    }

    pub fn decode_to_end(&mut self) {
        while let Some(new_n) = self.universal_decoder.decode_number(&mut self.input_stream) {
            self.decode_one(new_n - 1);
        }
    }

    pub fn get_stat(&self) -> usize {
        self.output_len
    }

    fn decode_one(&mut self, n: usize) {
        if let Some(entry) = self.dict.get(n) {
            self.output_stream.write_all(entry).unwrap();
            self.output_len += entry.len();

            if let Some(last_id) = self.last_id {
                let first_byte = entry[0];
                self.add_entry(last_id, first_byte);
            }
        } else {
            let last_id = self.last_id.unwrap();
            let first_byte = self.dict[last_id][0];
            self.add_entry(last_id, first_byte);

            let entry = &self.dict.last().unwrap();
            
            self.output_stream.write_all(entry).unwrap();
            self.output_len += entry.len();
        }
        self.last_id = Some(n);
    }

    fn add_entry(&mut self, id: usize, new_byte: u8) {
        if self.dict.len() >= MAX_DICT_SIZE {
            return;
        }

        let mut new_entry = self.dict[id].clone();
        new_entry.push(new_byte);
        self.dict.push(new_entry);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        coder::LZWCoder,
        universal_coder::{DeltaCoder, FibonacciCoder, GammaCoder, OmegaCoder},
        universal_decoder::{DeltaDecoder, FibonacciDecoder, GammaDecoder, OmegaDecoder},
    };

    use super::*;

    #[test]
    fn lzw_gamma() {
        let buf = include_bytes!("./../../testy/test1.bin");
        let mut coded_buf = vec![];
        let mut output_buf = vec![];

        let mut coder = LZWCoder::new(buf.as_slice(), &mut coded_buf, GammaCoder);

        coder.code_to_end();

        println!("original len: {}", buf.len());
        println!("coded len: {}", coded_buf.len());

        let mut decoder = LZWDecoder::new(coded_buf.as_slice(), &mut output_buf, GammaDecoder);

        decoder.decode_to_end();

        assert_eq!(buf.as_slice(), output_buf);
        //eprintln!("{}", String::from_utf8_lossy(&output_buf));
    }

    #[test]
    fn lzw_delta() {
        let buf = include_str!("./../../testy/pride_and_prejudice.txt");
        let mut coded_buf = vec![];
        let mut output_buf = vec![];

        let mut coder = LZWCoder::new(buf.as_bytes(), &mut coded_buf, DeltaCoder);

        coder.code_to_end();

        println!("original len: {}", buf.len());
        println!("coded len: {}", coded_buf.len());

        let mut decoder = LZWDecoder::new(coded_buf.as_slice(), &mut output_buf, DeltaDecoder);

        decoder.decode_to_end();

        assert_eq!(buf.as_bytes(), output_buf);
        //eprintln!("{}", String::from_utf8_lossy(&output_buf));
    }

    #[test]
    fn lzw_omega() {
        let buf = include_str!("./../../testy/pride_and_prejudice.txt");
        let mut coded_buf = vec![];
        let mut output_buf = vec![];

        let mut coder = LZWCoder::new(buf.as_bytes(), &mut coded_buf, OmegaCoder);

        coder.code_to_end();

        println!("original len: {}", buf.len());
        println!("coded len: {}", coded_buf.len());

        let mut decoder = LZWDecoder::new(coded_buf.as_slice(), &mut output_buf, OmegaDecoder);

        decoder.decode_to_end();

        assert_eq!(buf.as_bytes(), output_buf);
        //eprintln!("{}", String::from_utf8_lossy(&output_buf));
    }

    #[test]
    fn lzw_fib() {
        let buf = include_bytes!("./../../testy/test3.bin");
        let mut coded_buf = vec![];
        let mut output_buf = vec![];

        let mut coder = LZWCoder::new(buf.as_slice(), &mut coded_buf, FibonacciCoder::new());

        coder.code_to_end();

        println!("original len: {}", buf.len());
        println!("coded len: {}", coded_buf.len());

        let mut decoder = LZWDecoder::new(
            coded_buf.as_slice(),
            &mut output_buf,
            FibonacciDecoder::new(),
        );

        decoder.decode_to_end();

        assert_eq!(buf.as_slice(), output_buf);
        //eprintln!("{}", String::from_utf8_lossy(&output_buf));
    }
}
