use std::ops::Index;

use primitive_types::U256;

use crate::{gas_calculator::GasRecorder, util::u256_to_array};

pub struct Memory {
    pub bytes: Vec<u8>,
    pub max_index: usize,
}

impl Memory {
    #[inline]
    pub fn new() -> Memory {
        Memory {
            bytes: vec![],
            max_index: 0,
        }
    }

    #[inline]
    pub fn from(bytes: Vec<u8>, gas_recorder: &mut GasRecorder) -> Memory {
        let len = bytes.len();
        let mut  memory = Memory {
            bytes: bytes,
            max_index: 0,
        };
        memory.expand(len, gas_recorder);
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
        gas_recorder: &mut GasRecorder,
    ) {
        if write_address > self.max_index {
            self.expand(write_address, gas_recorder)
        }
        if memory.bytes.len() < read_address + length {
            memory.expand(read_address + length, gas_recorder)
        }
        self.bytes[write_address..write_address + length]
            .copy_from_slice(&memory.bytes[read_address..(read_address + length)]);
    }

    #[inline]
    pub fn set_length(&mut self, length: usize) {
        // TODO add gas recorder here?
        self.bytes.resize(length, 0);
    }

    #[inline]
    pub fn read(&self, address: usize) -> U256 {
        // TODO add memory expansion cost?
        let bytes_to_copy = &self.bytes[address..address + 32];
        let mut bytes = [0; 32];
        bytes.copy_from_slice(bytes_to_copy);
        U256::from(bytes)
    }

    #[inline]
    pub fn write(&mut self, address: usize, value: U256, gas_recorder: &mut GasRecorder) {
        if address > self.max_index {
            self.expand(address,gas_recorder);
        }
        let index = address;
        let end_index = index + 32;
        self.bytes[index..end_index].copy_from_slice(&u256_to_array(value).to_vec());
    }

    #[inline]
    pub fn write_u8(&mut self, address: usize, value: u8, gas_recorder: &mut GasRecorder) {
        if address > self.max_index {
            self.expand(address, gas_recorder);
        }
        self.bytes[address] = value;
    }

    #[inline]
    pub fn read_bytes(&self, address: usize, length: usize) -> Vec<u8> {
        self.bytes[address..(address + length)].to_vec()
    }

    #[inline]
    fn expand(&mut self, new_max_address: usize, gas_recorder: &mut GasRecorder) {
        self.max_index = new_max_address;
        gas_recorder.record_memory_usage(self.bytes.len(), new_max_address);
        self.bytes.resize(new_max_address, 0);
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
