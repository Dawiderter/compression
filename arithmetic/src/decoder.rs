#![allow(clippy::assign_op_pattern)]

use std::io::{Read, Write};

use crate::model::Model;

pub struct BitInputStream<I> {
    buf: u8,
    n: u8,
    input_stream: I,
}

impl<I: Read> BitInputStream<I> {
    pub fn new(input_stream: I) -> Self {
        Self {
            buf: 0,
            n: 0,
            input_stream,
        }
    }

    pub fn read_bit(&mut self) -> Option<bool> {
        if self.n == 0 {
            let mut tmp_buf = [0];
            let read = self.input_stream.read_exact(&mut tmp_buf);
            if read.is_err() {
                return None;
            }
            self.buf = tmp_buf[0];
        }

        let bit = self.buf & 0x80 == 0x80;
        self.buf <<= 1;
        self.n += 1;
        self.n %= 8;

        Some(bit)
    }
}

pub struct Decoder<I, O> {
    input_stream: BitInputStream<I>,
    output_stream: O,
    low: u32,
    high: u32,
    read_bits: u32,
    model: Model,

    output_len: usize,
}

const HALF_U32: u32 = u32::MAX - (u32::MAX >> 1);
const THREE_FOURTHS_U32: u32 = u32::MAX - (u32::MAX >> 2);
const ONE_FOURTH_U32: u32 = THREE_FOURTHS_U32 - HALF_U32;

impl<I: Read, O: Write> Decoder<I, O> {
    pub fn new(input_stream: I, output_stream: O) -> Self {
        Self {
            input_stream: BitInputStream::new(input_stream),
            output_stream,
            high: u32::MAX,
            low: 0,
            read_bits: 0,
            model: Model::default(),

            output_len: 0,
        }
    }

    pub fn decode_all(&mut self) {
        for _ in 0..32 {
            self.read_bit();
        }

        loop {
            let range = self.high as u64 - self.low as u64 + 1;
            let scaled_bits = ((self.read_bits - self.low + 1) as u64 * self.model.total_symbols as u64 - 1) / range;

            let symbol = self.model.get_symbol(scaled_bits as u32);

            let Some(symbol) = symbol else {
                break;
            };

            self.write_byte(symbol);

            let p_range = self.model.get_p_range(symbol);

            self.model.save_symbol(symbol);

            self.high = self.low + (((range * p_range.upper as u64) / p_range.denom as u64) - 1) as u32;
            self.low = self.low + ((range * p_range.lower as u64) / p_range.denom as u64) as u32;

            loop {
                if self.high < HALF_U32 || self.low >= HALF_U32 {
                    self.shift_range();
                    self.read_bit();
                } else if self.low >= ONE_FOURTH_U32 && self.high < THREE_FOURTHS_U32 {
                    self.low -= ONE_FOURTH_U32;
                    self.high -= ONE_FOURTH_U32;
                    self.read_bits -= ONE_FOURTH_U32;
                    self.shift_range();
                    self.read_bit();
                } else {
                    break;
                }
            }
        }
    }

    pub fn get_stat(&self) -> usize {
        self.output_len
    }

    fn read_bit(&mut self) {
        self.read_bits <<= 1;
        if let Some(input_bit) = self.input_stream.read_bit() {
            self.read_bits += if input_bit { 1 } else { 0 };
        }
    }

    fn write_byte(&mut self, byte: u8) {
        self.output_stream.write_all(&[byte]).unwrap();
        self.output_len += 1;
    }

    fn shift_range(&mut self) {
        self.high <<= 1;
        self.low <<= 1;
        self.high |= 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::coder::Coder;

    use super::*;

    #[test]
    fn decoder() {
        let input_stream =
            include_str!("../../testy/pan_tadeusz.txt").as_bytes();
        let input_len = input_stream.len();
        let mut output_stream = Vec::<u8>::new();

        let mut coder = Coder::new(input_stream, &mut output_stream);

        coder.code_all();
        let comp_len = output_stream.len();

        //dbg!(&output_stream);

        let mut decode_output_stream = Vec::<u8>::new();

        let mut decoder = Decoder::new(output_stream.as_slice(), &mut decode_output_stream);

        decoder.decode_all();

        dbg!(String::from_utf8_lossy(&decode_output_stream));
        dbg!(input_len, comp_len);
    }
}
