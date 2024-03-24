use std::ops::Index;

use primitive_types::U256;

use crate::util::u256_to_array;

pub struct Memory {
    pub bytes: Vec<u8>,
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
        memory: &mut Memory,
        read_address: usize,
        write_address: usize,
        length: usize,
    ) -> usize {
        if write_address > self.max_index {
            self.expand(write_address)
        }
        println!("Memory {:?}", memory.bytes);
        let start_cost= memory.compute_memory_cost();
        if memory.bytes.len() < read_address + length {
            memory.expand(read_address + length)
        }
        let cost = memory.compute_memory_cost() - start_cost;
        self.bytes[write_address..write_address + length]
            .copy_from_slice(&memory.bytes[read_address..(read_address + length)]);
        cost
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
        U256::from(bytes)
    }

    #[inline]
    pub fn write(&mut self, address: usize, value: U256) {
        if address > self.max_index {
            self.expand(address)
        }
        let index = address;
        let end_index = index + 32;
        self.bytes[index..end_index].copy_from_slice(&u256_to_array(value).to_vec());
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
        self.bytes.resize(new_max_address, 0);
        let memory_size_word = (self.max_index / 4) as u64;
        self.memory_cost =
            U256::from((u64::pow(memory_size_word, 2) / 512 + (3 * memory_size_word)) as u64);
    }

    #[inline]
    fn compute_memory_cost(&mut self) -> usize {
        if self.bytes.len() == 0 {
            return 0;
        }
        self.max_index = self.bytes.len() - 1;
        let memory_size_word = (self.max_index / 4) as u64;
        self.memory_cost =
            U256::from((u64::pow(memory_size_word, 2) / 512 + (3 * memory_size_word)) as u64);
        self.memory_cost.as_usize()
    }
}

impl Index<usize> for Memory {
    type Output = u8;

    #[inline]
    fn index(&self, s: usize) -> &u8 {
        &self.bytes[s]
    }
}

// // TODO tests
// 838137708090664833
// 838137708090664833
