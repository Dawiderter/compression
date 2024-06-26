use std::io::Write;

pub trait UniversalCoder {
    fn code_number<O: Write>(&mut self, output_stream: &mut BitOutputStream<O>, number: usize);
    fn pad<O: Write>(&mut self, output_stream: &mut BitOutputStream<O>);
}

pub struct GammaCoder;

impl UniversalCoder for GammaCoder {
    fn code_number<O: Write>(&mut self, output_stream: &mut BitOutputStream<O>, number: usize) {
        let num_size = number.ilog2() + 1;

        for _ in 0..(num_size - 1) {
            output_stream.write_bit(false);
        }

        let mut mask = 1 << (num_size - 1);
        while mask != 0 {
            output_stream.write_bit((mask & number) > 0);
            mask >>= 1;
        }
    }

    fn pad<O: Write>(&mut self, output_stream: &mut BitOutputStream<O>) {
        output_stream.pad_with_zeros();
    }
    
}

pub struct DeltaCoder; 

impl UniversalCoder for DeltaCoder {
    fn code_number<O: Write>(&mut self, output_stream: &mut BitOutputStream<O>, number: usize) {
        let n = number.ilog2() + 1;
        GammaCoder.code_number(output_stream, n as usize); 

        let mut mask = if n > 1 { 1 << (n - 2) } else { 0 };
        while mask != 0 {
            output_stream.write_bit((mask & number) > 0);
            mask >>= 1;
        }
    }

    fn pad<O: Write>(&mut self, output_stream: &mut BitOutputStream<O>) {
        output_stream.pad_with_zeros();
    }
} 

pub struct OmegaCoder;

impl UniversalCoder for OmegaCoder {
    fn code_number<O: Write>(&mut self, output_stream: &mut BitOutputStream<O>, number: usize) {
        let mut bits = vec![];
        bits.push(false);

        let mut rec_number = number;

        while rec_number != 1 {
            let mut n = rec_number;
            while n != 0 {
                bits.push(n % 2 == 1);
                n /= 2;
            }

            let n_size = rec_number.ilog2();
            rec_number = n_size as usize;
        }

        for bit in bits.into_iter().rev() {
            output_stream.write_bit(bit);
        }
    }

    fn pad<O: Write>(&mut self, output_stream: &mut BitOutputStream<O>) {
        output_stream.pad_with_ones()
    }
}

pub struct FibonacciCoder {
    fib_numbers:  Vec<usize>,
}

impl FibonacciCoder {
    pub fn new() -> Self {
        Self { fib_numbers: vec![0, 1] }
    }

    pub fn find_or_resize(&mut self, to: usize) -> usize {
        let mut i = 2;
        loop {
            if i == self.fib_numbers.len() {
                self.fib_numbers.push(self.fib_numbers[i - 1] + self.fib_numbers[i - 2])
            }

            if to < self.fib_numbers[i] {
                break;
            }

            i += 1;
        }
        i - 1
    }
}

impl UniversalCoder for FibonacciCoder {
    fn code_number<O: Write>(&mut self, output_stream: &mut BitOutputStream<O>, number: usize) {
        let mut bits = vec![];
        bits.push(true);

        let mut i = self.find_or_resize(number);
        let mut n = number;

        while i >= 2 {
            bits.push(n >= self.fib_numbers[i]);
            if n >= self.fib_numbers[i] {
                n -= self.fib_numbers[i];
            }
            i -= 1;
        }

        for bit in bits.into_iter().rev() {
            output_stream.write_bit(bit);
        }        
    }

    fn pad<O: Write>(&mut self, output_stream: &mut BitOutputStream<O>) {
        output_stream.pad_with_zeros();
    }
}

impl Default for FibonacciCoder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BitOutputStream<O> {
    buf: u8,
    n: u8,
    output_stream: O,
    pub written_bytes: usize,
}

impl<O: Write> BitOutputStream<O> {
    pub fn new(output_stream: O) -> Self {
        Self {
            buf: 0,
            n: 0,
            output_stream,
            written_bytes: 0,
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

    pub fn pad_with_ones(&mut self) {
        if self.n != 0 {
            self.buf <<= 8 - self.n;
            self.buf |= !0 >> self.n;
            self.output_stream.write_all(&[self.buf]).unwrap();
            self.buf = 0;
            self.n = 0;
            self.written_bytes += 1;
        }
    }

    pub fn pad_with_zeros(&mut self) {
        if self.n != 0 {
            self.buf <<= 8 - self.n;
            self.output_stream.write_all(&[self.buf]).unwrap();
            self.buf = 0;
            self.n = 0;
            self.written_bytes += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gamma() {
        let mut stream = vec![];
        let mut coder = GammaCoder;

        let mut bitoutput = BitOutputStream::new(&mut stream);

        for i in 1..10 {
            coder.code_number(&mut bitoutput, i);
        }
       coder.pad(&mut bitoutput);

        for num in stream {
            eprintln!("{:#010b}", num)
        }
    }

    #[test]
    fn delta() {
        let mut stream = vec![];
        let mut coder = DeltaCoder;

        let mut bitoutput = BitOutputStream::new(&mut stream);

        for i in 1..10 {
            coder.code_number(&mut bitoutput, i);
        }
        coder.pad(&mut bitoutput);

        for num in stream {
            eprintln!("{:#010b}", num)
        }
    }

    #[test]
    fn omega() {
        let mut stream = vec![];
        let mut coder = OmegaCoder;

        let mut bitoutput = BitOutputStream::new(&mut stream);

        for i in 1..10 {
            coder.code_number(&mut bitoutput, i);
        }
        coder.pad(&mut bitoutput);

        for num in stream {
            eprintln!("{:#010b}", num)
        }
    }

    #[test]
    fn fib() {
        let mut stream = vec![];
        let mut coder = FibonacciCoder::new();

        let mut bitoutput = BitOutputStream::new(&mut stream);

        for i in 1..10 {
            coder.code_number(&mut bitoutput, i);
        }
        coder.pad(&mut bitoutput);

        for num in stream {
            eprintln!("{:#010b}", num)
        }
    }
}
