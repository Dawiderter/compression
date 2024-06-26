use std::io::{Read, Write};

use crate::{
    prefix_tree::PrefixTree,
    universal_coder::{BitOutputStream, UniversalCoder},
};

pub struct LZWCoder<I, O, C> {
    input_stream: I,
    output_stream: BitOutputStream<O>,
    universal_coder: C,
    saved_byte: Option<u8>,
    tree: PrefixTree,

    input_len: usize
}

impl<I: Read, O: Write, C: UniversalCoder> LZWCoder<I, O, C> {
    pub fn new(input_stream: I, output_stream: O, universal_coder: C) -> Self {
        Self {
            tree: PrefixTree::new(),
            input_stream,
            saved_byte: None,
            universal_coder,
            output_stream: BitOutputStream::new(output_stream),
            input_len : 0,
        }
    }

    pub fn code_to_end(&mut self) {
        while let Some(code) = self.code() {
            self.universal_coder.code_number(&mut self.output_stream, code + 1);
        }
        self.universal_coder.pad(&mut self.output_stream);
    }

    pub fn get_stat(&self) -> (usize, usize) {
        (self.input_len, self.output_stream.written_bytes)
    }

    fn code(&mut self) -> Option<usize> {
        let start = self.read()?;
        let mut current_node = start as usize;

        while let Some(byte) = self.read() {
            let maybe_next_node = self.tree.travel(current_node, byte);
            if let Some(next_node) = maybe_next_node {
                current_node = next_node;
            } else {
                self.save_byte(byte);
                self.tree.append(current_node, byte);
                break;
            }
        }

        Some(current_node)
    }

    fn save_byte(&mut self, byte: u8) {
        self.saved_byte = Some(byte)
    }

    fn read(&mut self) -> Option<u8> {
        if self.saved_byte.is_some() {
            self.saved_byte.take()
        } else {
            self.read_new_byte()
        }
    }

    fn read_new_byte(&mut self) -> Option<u8> {
        let mut buf = [0];
        self.input_stream
            .read_exact(&mut buf)
            .ok()
            .map(|_| {
                self.input_len += 1;
                buf[0]
            })
    }
}

