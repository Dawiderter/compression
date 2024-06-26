use std::io::Read;

pub trait UniversalDecoder {
    fn decode_number<I: Read>(&mut self, input_stream: &mut BitInputStream<I>) -> Option<usize>;
}

pub struct GammaDecoder;

impl UniversalDecoder for GammaDecoder {
    fn decode_number<I: Read>(&mut self, input_stream: &mut BitInputStream<I>) -> Option<usize> {
        let mut zeros = 0;
        loop { 
            let Some(bit) = input_stream.read_bit() else { return None; };
            if !bit {
                zeros += 1;
            } else {
                break;
            }
        }
        
        let mut read_number = 1;

        while zeros != 0 {
            let Some(bit) = input_stream.read_bit() else { return None; };
            read_number *= 2;
            read_number += if bit { 1 } else { 0 }; 
            zeros -= 1;
        }

        Some(read_number)
    }
}

pub struct DeltaDecoder;

impl UniversalDecoder for DeltaDecoder {
    fn decode_number<I: Read>(&mut self, input_stream: &mut BitInputStream<I>) -> Option<usize> {
        let mut n_size = GammaDecoder.decode_number(input_stream)? - 1;
        let mut read_number = 1;

        while n_size != 0 {
            let Some(bit) = input_stream.read_bit() else { return None; };
            read_number *= 2;
            read_number += if bit { 1 } else { 0 }; 
            n_size -= 1;
        }

        Some(read_number)
    }
}

pub struct OmegaDecoder;

impl UniversalDecoder for OmegaDecoder {
    fn decode_number<I: Read>(&mut self, input_stream: &mut BitInputStream<I>) -> Option<usize> {
        let mut n = 1;

        loop {
            let Some(bit) = input_stream.read_bit() else { return None; };

            if !bit {
                break;
            }

            let mut read_number = 1;
            let mut to_read = n;

            while to_read > 0 {
                let Some(bit) = input_stream.read_bit() else { return None; };
                read_number *= 2;
                read_number += if bit { 1 } else { 0 }; 
                to_read -= 1;
            }

            n = read_number;
        }

        Some(n)
    }
}

pub struct FibonacciDecoder {
    fib_numbers:  Vec<usize>,
}

impl FibonacciDecoder {
    pub fn new() -> Self {
        Self { fib_numbers: vec![0, 1] }
    }

    pub fn fib(&mut self, i: usize) -> usize {
        while i >= self.fib_numbers.len() {
            let len = self.fib_numbers.len();
            self.fib_numbers.push(self.fib_numbers[len - 1] + self.fib_numbers[len - 2])
        }

        self.fib_numbers[i]
    }
}

impl UniversalDecoder for FibonacciDecoder {
    fn decode_number<I: Read>(&mut self, input_stream: &mut BitInputStream<I>) -> Option<usize> {
        let mut read_number = 0;
        let mut i = 2;
        let mut last_bit = false;

        while let Some(bit) = input_stream.read_bit() {
            if last_bit && bit {
                return Some(read_number);
            }

            if bit {
                read_number += self.fib(i);
            }
            

            last_bit = bit;
            i += 1;
        }

        None
    }
}

impl Default for FibonacciDecoder {
    fn default() -> Self {
        Self::new()
    }
}

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

#[cfg(test)]
mod tests {
    use crate::universal_coder::{BitOutputStream, GammaCoder, UniversalCoder, DeltaCoder, OmegaCoder, FibonacciCoder};

    use super::*;

    #[test]
    fn gamma() {
        let mut stream = vec![];
        let mut coder = GammaCoder;

        let mut bitoutput = BitOutputStream::new(&mut stream);

        for i in 1..=15 {
            coder.code_number(&mut bitoutput, i);
        }
        coder.pad(&mut bitoutput);

        for num in &stream {
            eprintln!("{:#010b}", num);
        }

        let mut decoder = GammaDecoder;
        let mut bitinput = BitInputStream::new(stream.as_slice());

        while let Some(num) = decoder.decode_number(&mut bitinput) {
            eprintln!("{}", num);
        }
    }

    #[test]
    fn delta() {
        let mut stream = vec![];
        let mut coder = DeltaCoder;

        let mut bitoutput = BitOutputStream::new(&mut stream);

        for i in 1..=20 {
            coder.code_number(&mut bitoutput, i);
        }
        coder.pad(&mut bitoutput);

        for num in &stream {
            eprintln!("{:#010b}", num);
        }

        let mut decoder = DeltaDecoder;
        let mut bitinput = BitInputStream::new(stream.as_slice());

        while let Some(num) = decoder.decode_number(&mut bitinput) {
            eprintln!("{}", num);
        }
    }

    #[test]
    fn omega() {
        let mut stream = vec![];
        let mut coder = OmegaCoder;

        let mut bitoutput = BitOutputStream::new(&mut stream);

        for i in 1..=9 {
            coder.code_number(&mut bitoutput, i);
        }
        coder.pad(&mut bitoutput);

        for num in &stream {
            eprintln!("{:#010b}", num);
        }

        let mut decoder = OmegaDecoder;
        let mut bitinput = BitInputStream::new(stream.as_slice());

        while let Some(num) = decoder.decode_number(&mut bitinput) {
            eprintln!("{}", num);
        }
    }

    #[test]
    fn fib() {
        let mut stream = vec![];
        let mut coder = FibonacciCoder::new();

        let mut bitoutput = BitOutputStream::new(&mut stream);

        for i in 1..=10 {
            coder.code_number(&mut bitoutput, i);
        }
        coder.pad(&mut bitoutput);

        for num in &stream {
            eprintln!("{:#010b}", num);
        }

        let mut decoder = FibonacciDecoder::new();
        let mut bitinput = BitInputStream::new(stream.as_slice());

        while let Some(num) = decoder.decode_number(&mut bitinput) {
            eprintln!("{}", num);
        }
    }
}