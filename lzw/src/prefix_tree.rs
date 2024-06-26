use std::collections::HashMap;

use crate::MAX_DICT_SIZE;

#[derive(Debug, Clone)]
pub struct PrefixNode {
    children: HashMap<u8, usize>,
}

impl PrefixNode {
    pub fn new() -> Self {
        Self { children: HashMap::new() }
    }
}

#[derive(Debug)]
pub struct PrefixTree {
    buffer: Vec<PrefixNode>,
}

impl PrefixTree {
    pub fn new() -> Self {
        let buf = vec![PrefixNode::new(); 256];

        Self { buffer: buf }
    }

    pub fn travel(&self, from: usize, char: u8) -> Option<usize> {
        let node = &self.buffer[from];
        let dest = node.children.get(&char).copied();
        dest
    }

    pub fn append(&mut self, from: usize, char: u8) -> usize {
        if self.buffer.len() >= MAX_DICT_SIZE {
            return from;
        }

        let new_node = PrefixNode::new();
        let id = self.buffer.len();
        self.buffer.push(new_node);

        let src_node = &mut self.buffer[from];
        src_node.children.insert(char, id);

        id
    }
}

impl Default for PrefixTree {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PrefixNode {
    fn default() -> Self {
        Self::new()
    }
}