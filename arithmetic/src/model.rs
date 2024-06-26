#![allow(clippy::needless_range_loop)]

pub const CHARACTER_BUFFER_SIZE: usize = 256;

pub const MAX_CHARACTER_COUNT: usize = 1 << 30;

#[derive(Debug, Clone, Copy)]
pub struct PRange {
    pub upper: u32,
    pub lower: u32,
    pub denom: u32,
}

pub struct Model {
    symbol_upper_range: [u32; 257],
    character_update_buffer: Vec<u8>,
    pub total_symbols: u32,
}

impl Model {
    pub fn new() -> Self {
        Self {
            character_update_buffer: Vec::new(),
            symbol_upper_range: std::array::from_fn(|i| i as u32 + 1),
            total_symbols: 257,
        }
    }

    pub fn get_p_range(&self, symbol: u8) -> PRange {
        let upper = self.symbol_upper_range[symbol as usize];
        let lower = (symbol as usize)
            .checked_sub(1)
            .map(|i| self.symbol_upper_range[i])
            .unwrap_or(0);

        PRange {
            upper,
            lower,
            denom: self.total_symbols,
        }
    }

    pub fn get_eof_range(&self) -> PRange {
        let upper = self.symbol_upper_range[256];
        let lower = self.symbol_upper_range[255];

        PRange {
            upper,
            lower,
            denom: self.total_symbols,
        }
    }

    pub fn get_symbol(&self, value: u32) -> Option<u8> {
        let mut s = 0;
        while value >= self.symbol_upper_range[s] {
            s += 1;
        }
        if s == 256 {
            None
        } else {
            Some(s as u8)
        }
    }

    pub fn save_symbol(&mut self, symbol: u8) {
        if (self.total_symbols as usize) < MAX_CHARACTER_COUNT {
            self.character_update_buffer.push(symbol);
            if self.character_update_buffer.len() >= CHARACTER_BUFFER_SIZE {
                self.update_model();
            }
        }

    }

    pub fn update_model(&mut self) {
        let mut new_ranges = [0; 257];

        for symbol in self.character_update_buffer.drain(..) {
            new_ranges[symbol as usize] += 1;
        }

        for i in 1..257 {
            new_ranges[i] += new_ranges[i - 1];
        }

        for i in 0..257 {
            self.symbol_upper_range[i] += new_ranges[i];
        }

        self.total_symbols = self.symbol_upper_range[256];
        //dbg!(self.symbol_upper_range);
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}
