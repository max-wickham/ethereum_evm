use std::ops::Index;

use primitive_types::U256;

use crate::evm_logic::evm::macros::{break_if_error, return_if_error};
use crate::result::ExecutionSuccess;
use crate::{
    evm_logic::{
        evm::macros::{return_if_gas_too_high, return_tuple_if_error},
        gas_calculator::GasRecorder,
        util::{u256_to_array, ZERO},
    },
    result::{Error, ExecutionResult},
};
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
        let mut memory = Memory {
            bytes: bytes,
            max_index: 0,
        };
        memory.expand(len, Some(gas_recorder));
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
    ) -> ExecutionResult {
        println!("Write addresses: {}", write_address);
        if write_address.checked_add(length).is_none() || read_address.checked_add(length).is_none()
        {
            gas_recorder.record_gas(gas_recorder.gas_input as u64);
            return_if_gas_too_high!(gas_recorder);
        }
        if write_address + length > self.max_index {
            println!("Expanding memory");
            println!("memory bytes length: {}", memory.bytes.len());
            println!("read address: {}", read_address);
            println!("length: {}", length);
            println!("max index: {}", self.max_index);
            return_if_error!(self.expand(write_address + length, Some(gas_recorder)));
        }
        if memory.bytes.len() < read_address + length {
            println!("Expanding memory");
            return_if_error!(memory.expand(read_address + length, Some(gas_recorder)));
        }
        self.bytes[write_address..write_address + length]
            .copy_from_slice(&memory.bytes[read_address..(read_address + length)]);
        ExecutionResult::Success(ExecutionSuccess::Unknown)
    }

    #[inline]
    pub fn copy_from_without_cost(
        &mut self,
        memory: &mut Memory,
        read_address: usize,
        write_address: usize,
        length: usize,
    ) -> ExecutionResult {
        if write_address + length > self.max_index {
            return_if_error!(self.expand(write_address + length, None));
        }
        if memory.bytes.len() < read_address + length {
            return_if_error!(memory.expand(read_address + length, None));
        }
        self.bytes[write_address..write_address + length]
            .copy_from_slice(&memory.bytes[read_address..(read_address + length)]);
        ExecutionResult::Success(ExecutionSuccess::Unknown)
    }

    #[inline]
    pub fn copy_from_no_local_cost(
        &mut self,
        memory: &mut Memory,
        read_address: usize,
        write_address: usize,
        length: usize,
        gas_recorder: &mut GasRecorder,
    ) -> ExecutionResult {
        if write_address.checked_add(length).is_none() || read_address.checked_add(length).is_none()
        {
            gas_recorder.record_gas(gas_recorder.gas_input as u64);
            return_if_gas_too_high!(gas_recorder);
        }
        if write_address.checked_add(length).is_none() || read_address.checked_add(length).is_none()
        {
            gas_recorder.record_gas(gas_recorder.gas_input as u64);
            return_if_gas_too_high!(gas_recorder);
        }
        if write_address + length > self.max_index {
            return_if_error!(self.expand(write_address + length, None));
        }
        if memory.bytes.len() < read_address + length {
            return_if_error!(memory.expand(read_address + length, Some(gas_recorder)));
        }
        self.bytes[write_address..write_address + length]
            .copy_from_slice(&memory.bytes[read_address..(read_address + length)]);
        ExecutionResult::Success(ExecutionSuccess::Unknown)
    }

    #[inline]
    pub fn copy_from_bytes(
        &mut self,
        bytes: &Vec<u8>,
        read_address: usize,
        write_address: usize,
        length: usize,
        gas_recorder: &mut GasRecorder,
    ) -> ExecutionResult {
        // TODO should be kept as U256 as maybe the gas input is high enough for this?
        if write_address.checked_add(length).is_none() || read_address.checked_add(length).is_none()
        {
            gas_recorder.record_gas(gas_recorder.gas_input as u64);
            println!("Bad len1");
            return_if_gas_too_high!(gas_recorder);
        }
        if write_address.checked_add(length).is_none() || read_address.checked_add(length).is_none()
        {
            gas_recorder.record_gas(gas_recorder.gas_input as u64);
            println!("Bad len2");
            return_if_gas_too_high!(gas_recorder);
        }
        if write_address + length > self.max_index {
            println!("Expand");
            return_if_error!(self.expand(write_address + length, Some(gas_recorder)));
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
        ExecutionResult::Success(ExecutionSuccess::Unknown)
    }

    #[inline]
    pub fn set_length(&mut self, length: usize) {
        // TODO add gas recorder here?
        self.bytes.resize(length, 0);
    }

    #[inline]
    pub fn read(
        &mut self,
        address: usize,
        gas_recorder: &mut GasRecorder,
    ) -> (ExecutionResult, U256) {
        // TODO add memory expansion cost?
        if address > {
            if self.max_index > 32 {
                self.max_index - 32
            } else {
                self.max_index
            }
        } {
            return_tuple_if_error!(self.expand(address, Some(gas_recorder)), ZERO);
        }
        let bytes_to_copy = &self.bytes[address..address + 32];
        let mut bytes = [0; 32];
        bytes.copy_from_slice(bytes_to_copy);
        (ExecutionResult::Success(ExecutionSuccess::Unknown), U256::from(bytes))
    }

    #[inline]
    pub fn write(
        &mut self,
        address: usize,
        value: U256,
        gas_recorder: &mut GasRecorder,
    ) -> ExecutionResult {
        if address > {
            if self.max_index > 32 {
                self.max_index - 32
            } else {
                self.max_index
            }
        } {
            return_if_error!(self.expand(address + 32, Some(gas_recorder)));
        }
        let index = address;
        let end_index = index + 32;
        self.bytes[index..end_index].copy_from_slice(&u256_to_array(value).to_vec());
        ExecutionResult::Success(ExecutionSuccess::Unknown)
    }

    #[inline]
    pub fn write_u8(
        &mut self,
        address: usize,
        value: u8,
        gas_recorder: &mut GasRecorder,
    ) -> ExecutionResult {
        if address > {
            if self.max_index > 1 {
                self.max_index - 1
            } else {
                self.max_index
            }
        } {
            return_if_error!(self.expand(address, Some(gas_recorder)));
        }
        self.bytes[address] = value;
        ExecutionResult::Success(ExecutionSuccess::Unknown)
    }

    #[inline]
    pub fn read_bytes(
        &mut self,
        address: usize,
        length: usize,
        gas_recorder: &mut GasRecorder,
    ) -> (ExecutionResult, Vec<u8>) {
        if address > {
            if self.max_index > length {
                self.max_index - length
            } else {
                self.max_index
            }
        } {
            return_tuple_if_error!(self.expand(address, Some(gas_recorder)), vec![]);
        }
        (
            ExecutionResult::Success(ExecutionSuccess::Unknown),
            self.bytes[address..(address + length)].to_vec(),
        )
    }

    #[inline]
    fn expand(
        &mut self,
        new_max_address: usize,
        gas_recorder: Option<&mut GasRecorder>,
    ) -> ExecutionResult {
        // if new_max_address > 10000000 {
        //     return ExecutionResult::Err(Error::InvalidMemSize);
        // }
        if new_max_address == 0 {
            println!("New max address is 0");
            return ExecutionResult::Success(ExecutionSuccess::Unknown);
        }
        println!("Self.butes.len() : {}", self.bytes.len());
        let mut new_max_address = new_max_address;
        println!("New max address : {}", new_max_address);
        // if new_max_address % 32 != 0 {
        // new_max_address= new_max_address + 32;
        // }
        // println!("New max address : {}", new_max_address);
        self.max_index = new_max_address;
        match gas_recorder {
            Some(gas_recorder) => {
                gas_recorder.record_memory_usage(self.bytes.len(), new_max_address);
                return_if_gas_too_high!(gas_recorder);
            }
            _ => {}
        }

        // println!("Max Address : {}", new_max_address);
        self.bytes.resize(new_max_address, 0);
        ExecutionResult::Success(ExecutionSuccess::Unknown)
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
