use std::ops::Index;

use primitive_types::U256;

use crate::evm_logic::{gas_calculator::GasRecorder, util::u256_to_array};

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
        println!("Write addresses: {}", write_address);
        if write_address + length > self.max_index {
            println!("Expanding memory");
            println!("memory bytes length: {}", memory.bytes.len());
            println!("read address: {}", read_address);
            println!("length: {}", length);
            println!("max index: {}", self.max_index);
            self.expand(write_address + length, gas_recorder)
        }
        if memory.bytes.len() < read_address + length {
            memory.expand(read_address + length, gas_recorder)
        }
        self.bytes[write_address..write_address + length]
            .copy_from_slice(&memory.bytes[read_address..(read_address + length)]);
    }

    #[inline]
    pub fn copy_from_with_local_cost(
        &mut self,
        memory: &mut Memory,
        read_address: usize,
        write_address: usize,
        length: usize,
        gas_recorder: &mut GasRecorder,
    ) {
        println!("Write addresses: {}", write_address);
        if write_address + length > self.max_index {
            self.expand(write_address + length, &mut GasRecorder{gas_usage: 0, gas_refunds: 0});
        }
        if memory.bytes.len() < read_address + length {
            println!("Expanding memory");
            println!("memory bytes length: {}", memory.bytes.len());
            println!("read address: {}", read_address);
            println!("length: {}", length);
            println!("max index: {}", self.max_index);
            memory.expand(read_address + length, gas_recorder)
        }
        self.bytes[write_address..write_address + length]
            .copy_from_slice(&memory.bytes[read_address..(read_address + length)]);
    }

    #[inline]
    pub fn copy_from_bytes(
        &mut self,
        bytes: &Vec<u8>,
        read_address: usize,
        write_address: usize,
        length: usize,
        gas_recorder: &mut GasRecorder,
    ) {
        if write_address + length > self.max_index {
            self.expand(write_address + length, gas_recorder)
        }
        let (mut start_address, mut end_address) = (read_address, read_address + length);
        if start_address > bytes.len() {
            start_address = bytes.len();
        }
        if end_address > bytes.len() {
            end_address = bytes.len();
        }
        self.bytes[write_address..write_address + (end_address - start_address)]
            .copy_from_slice(&bytes[start_address..end_address]);
    }

    #[inline]
    pub fn set_length(&mut self, length: usize) {
        // TODO add gas recorder here?
        self.bytes.resize(length, 0);
    }

    #[inline]
    pub fn read(&mut self, address: usize, gas_recorder: &mut GasRecorder) -> U256 {
        // TODO add memory expansion cost?
        if address > {if self.max_index > 32 {self.max_index - 32} else {self.max_index}} {
            self.expand(address,gas_recorder);
        }
        let bytes_to_copy = &self.bytes[address..address + 32];
        let mut bytes = [0; 32];
        bytes.copy_from_slice(bytes_to_copy);
        U256::from(bytes)
    }

    #[inline]
    pub fn write(&mut self, address: usize, value: U256, gas_recorder: &mut GasRecorder) {
        if address > {if self.max_index > 32 {self.max_index - 32} else {self.max_index}} {
            self.expand(address + 32,gas_recorder);
        }
        let index = address;
        let end_index = index + 32;
        self.bytes[index..end_index].copy_from_slice(&u256_to_array(value).to_vec());
    }

    #[inline]
    pub fn write_u8(&mut self, address: usize, value: u8, gas_recorder: &mut GasRecorder) {
        if address > {if self.max_index > 1 {self.max_index - 1} else {self.max_index}} {
            self.expand(address, gas_recorder);
        }
        self.bytes[address] = value;
    }

    #[inline]
    pub fn read_bytes(&mut self, address: usize, length: usize, gas_recorder: &mut GasRecorder) -> Vec<u8> {
        if address > {if self.max_index > length {self.max_index - length} else {self.max_index}} {
            self.expand(address,gas_recorder);
        }
        self.bytes[address..(address + length)].to_vec()
    }

    #[inline]
    fn expand(&mut self, new_max_address: usize, gas_recorder: &mut GasRecorder) {
        if new_max_address > 10000000 {
            panic!("Memory expansion error");
        }
        if new_max_address == 0 {
            println!("New max address is 0");
            return;
        }
        println!("Self.butes.len() : {}", self.bytes.len());
        let mut new_max_address = new_max_address;
        println!("New max address : {}", new_max_address);
        // if new_max_address % 32 != 0 {
            // new_max_address= new_max_address + 32;
        // }
        // println!("New max address : {}", new_max_address);
        self.max_index = new_max_address;
        gas_recorder.record_memory_usage(self.bytes.len(), new_max_address);
        // println!("Max Address : {}", new_max_address);
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
