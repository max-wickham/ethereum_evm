use std::ops::Index;

use ethnum::U256;

pub struct Memory {
    bytes: Vec<u8>,
    pub max_index: usize,
    pub memory_cost: U256,
}

impl Memory {
    #[inline]
    pub fn new() -> Memory {
        Memory {
            bytes: vec![],
            max_index: 0,
            memory_cost: U256::from(0 as u64),
        }
    }

    #[inline]
    pub fn from(bytes: Vec<u8>) -> Memory {
        let len = bytes.len();
        let mut  memory = Memory {
            bytes: bytes,
            max_index: 0,
            memory_cost: U256::from(0 as u64),
        };
        memory.expand(len);
        memory
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    #[inline]
    pub fn copy_from(
        &mut self,
        memory: &Memory,
        read_address: usize,
        write_address: usize,
        length: usize,
    ) {
        if write_address > self.max_index {
            self.expand(write_address)
        }
        self.bytes[write_address..write_address + length]
            .copy_from_slice(&memory.bytes[read_address..read_address + length]);
    }

    #[inline]
    pub fn set_length(&mut self, length: usize) {
        self.bytes.resize(length, 0);
    }

    #[inline]
    pub fn read(&self, address: usize) -> U256 {
        let bytes_to_copy = &self.bytes[address..address + 32];
        let mut bytes = [0; 32];
        bytes.copy_from_slice(bytes_to_copy);
        U256::from_be_bytes(bytes)
    }

    #[inline]
    pub fn write(&mut self, address: usize, value: U256) {
        if address > self.max_index {
            self.expand(address)
        }
        let index = address;
        let end_index = index + 32;
        self.bytes[index..end_index].copy_from_slice(&value.to_be_bytes().to_vec());
    }

    #[inline]
    pub fn write_u8(&mut self, address: usize, value: u8) {
        if address > self.max_index {
            self.expand(address)
        }
        self.bytes[address] = value;
    }

    #[inline]
    pub fn read_bytes(&self, address: usize, length: usize) -> Vec<u8> {
        self.bytes[address..(address + length)].to_vec()
    }

    #[inline]
    fn expand(&mut self, new_max_address: usize) {
        self.max_index = new_max_address;
        let memory_size_word = (self.max_index / 4) as u64;
        self.memory_cost =
            U256::from((u64::pow(memory_size_word, 2) / 512 + (3 * memory_size_word)) as u64);
    }
}

impl Index<usize> for Memory {
    type Output = u8;

    #[inline]
    fn index(&self, s: usize) -> &u8 {
        &self.bytes[s]
    }
}

// TODO tests
