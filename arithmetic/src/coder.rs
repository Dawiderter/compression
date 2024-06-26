#![allow(clippy::assign_op_pattern)]

use std::io::{Read, Write};

use crate::model::{Model, PRange};

pub struct BitOutputStream<O> {
    buf: u8,
    n: u8,
    output_stream: O,
    written_bytes: usize,
}

impl<O: Write> BitOutputStream<O> {
    pub fn new(output_stream: O) -> Self {
        Self {
            buf: 0,
            n: 0,
            output_stream,
            written_bytes: 0
        }
    }

    pub fn write_bit(&mut self, bit: bool) {
        self.buf <<= 1;
        self.buf |= if bit { 1 } else { 0 };
        self.n += 1;

        if self.n >= 8 {
            self.output_stream.write_all(&[self.buf]).unwrap();
            self.buf = 0;
            self.n = 0;
            self.written_bytes += 1;
        }
    }

    pub fn finish(&mut self) {
        if self.n != 0 {
            self.buf <<= 8 - self.n;
            self.output_stream.write_all(&[self.buf]).unwrap();
            self.buf = 0;
            self.n = 0;
            self.written_bytes += 1;
        }
    }
}

pub struct Coder<I, O> {
    input_stream: I,
    output_stream: BitOutputStream<O>,
    low: u32,
    high: u32,
    not_yet_written_bits: u32,
    model: Model,

    input_len: usize,
}

const HALF_U32: u32 = u32::MAX - (u32::MAX >> 1);
const THREE_FOURTHS_U32: u32 = u32::MAX - (u32::MAX >> 2);
const ONE_FOURTH_U32: u32 = THREE_FOURTHS_U32 - HALF_U32;

impl<I: Read, O: Write> Coder<I, O> {

    pub fn new(input_stream: I, output_stream: O) -> Self {
        Self {
            input_stream,
            output_stream: BitOutputStream::new(output_stream),
            high: u32::MAX,
            low: 0,
            not_yet_written_bits: 0,
            model: Model::default(),
            input_len: 0,
        }
    }

    pub fn code_all(&mut self) {
        while let Some(byte) = self.read_byte() {
            self.input_len += 1;

            let p_range = self.model.get_p_range(byte);

            self.write_p_range(p_range);

            self.model.save_symbol(byte);
        }

        let p_range = self.model.get_eof_range();

        self.write_p_range(p_range);

        self.not_yet_written_bits += 1;
        if self.low < ONE_FOURTH_U32 {
            self.write_all_bits(false);
        } else {
            self.write_all_bits(true);
        }
        
        self.output_stream.finish();
    }

    pub fn get_stat(&self) -> (usize, usize) {
        (self.input_len, self.output_stream.written_bytes)
    }

    fn write_p_range(&mut self, p_range: PRange) {
        let range = self.high as u64 - self.low as u64 + 1;

        self.high = self.low + (((range * p_range.upper as u64) / p_range.denom as u64) - 1) as u32;
        self.low = self.low + ((range * p_range.lower as u64) / p_range.denom as u64) as u32;

        loop {
            if self.high < HALF_U32 {
                self.write_all_bits(false);
                self.shift_range();
            } else if self.low >= HALF_U32 {
                self.write_all_bits(true);
                self.shift_range();
            } else if self.low >= ONE_FOURTH_U32 && self.high < THREE_FOURTHS_U32 {
                self.not_yet_written_bits += 1;
                self.low -= ONE_FOURTH_U32;
                self.high -= ONE_FOURTH_U32;
                self.shift_range();
            } else {
                break;
            }
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        let mut buf = [0];
        self.input_stream.read_exact(&mut buf).ok().and(Some(buf[0]))
    }

    fn write_all_bits(&mut self, bit: bool) {
        self.output_stream.write_bit(bit);
        for _ in 0..self.not_yet_written_bits {
            self.output_stream.write_bit(!bit);
        }
        self.not_yet_written_bits = 0;
    }

    fn shift_range(&mut self) {
        self.high <<= 1;
        self.low <<= 1;
        self.high |= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_check() {
        eprintln!("{:b}", HALF_U32);
        eprintln!("{:b}", THREE_FOURTHS_U32);
        eprintln!("{:b}", ONE_FOURTH_U32);
    }

    // #[test]
    // fn code_check() {
    //     let input_stream = b"ABABABABABAB";
    //     let mut output_stream = Vec::<u8>::new();

    //     let mut coder = Coder::new(input_stream.as_slice(), &mut output_stream);

    //     coder.code_all();

    //     dbg!(output_stream);
    // }
}