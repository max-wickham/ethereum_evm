use std::ops::Index;

use primitive_types::U256;

use crate::evm_logic::evm::macros::return_if_error;
use crate::evm_logic::{
    evm::macros::{return_if_gas_too_high, return_tuple_if_error},
    gas_recorder::GasRecorder,
    util::{u256_to_array, ZERO},
};
use crate::result::{ExecutionError, ExecutionResult};

#[derive(Default)]
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
    pub fn from(bytes: &[u8], gas_recorder: Option<&mut GasRecorder>) -> Memory {
        let len = bytes.len();
        let mut memory = Memory {
            bytes: bytes.to_vec(),
            max_index: 0,
        };
        memory.expand(len, gas_recorder);
        memory
    }

    // TODO need to add unit tests for this
    pub fn to_sub_vec(&self, start: usize, end: usize) -> Vec<u8> {
        /*
        Extract a sub vector, padded with 0s if the range is not contained within the memory length
        */
        if self.len() == 0 {
            return vec![0; end.max(start) - start];
        }
        let len: usize = self.bytes.len();
        let sub_len = end.max(start) - start;
        let mut result = self.bytes[start.min(len - 1)..end.min(len)].to_vec();
        let mut padding_bytes: Vec<u8> = vec![0; sub_len - result.len()];
        result.append(&mut padding_bytes);
        assert!(result.len() == end - start);
        result
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
        if write_address.checked_add(length).is_none() || read_address.checked_add(length).is_none()
        {
            gas_recorder.record_gas_usage(gas_recorder.gas_input as u64);
            return ExecutionResult::Error(ExecutionError::InsufficientGas);
        }
        if write_address + length > self.max_index {
            return_if_error!(self.expand(write_address + length, Some(gas_recorder)));
        }
        if memory.bytes.len() < read_address + length {
            return_if_error!(memory.expand(read_address + length, Some(gas_recorder)));
        }
        self.bytes[write_address..write_address + length]
            .copy_from_slice(&memory.bytes[read_address..(read_address + length)]);
        ExecutionResult::InProgress
    }

    // TODO refactor this to be cleaner, (currently just here because of logs)
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
            gas_recorder.record_gas_usage(gas_recorder.gas_input as u64);
            return ExecutionResult::Error(ExecutionError::InsufficientGas);
        }
        if write_address + length > self.max_index {
            return_if_error!(self.expand(write_address + length, None));
        }
        if memory.bytes.len() < read_address + length {
            return_if_error!(memory.expand(read_address + length, Some(gas_recorder)));
        }
        self.bytes[write_address..write_address + length]
            .copy_from_slice(&memory.bytes[read_address..(read_address + length)]);
        ExecutionResult::InProgress
    }

    #[inline]
    pub fn copy_from_bytes(
        &mut self,
        bytes: &[u8],
        read_address: U256,
        write_address: usize,
        length: usize,
        gas_recorder: &mut GasRecorder,
    ) -> ExecutionResult {
        // TODO should be kept as U256 as maybe the gas input is high enough for this?
        println!("Read from {:x}", read_address);
        if write_address.checked_add(length).is_none()
        {
            println!("Checked add failed");
            gas_recorder.record_gas_usage(gas_recorder.gas_input as u64);
            return ExecutionResult::Error(ExecutionError::InsufficientGas);
        }

        if write_address + length > self.bytes.len() {
            return_if_error!(self.expand(write_address + length, Some(gas_recorder)));
        }
        // let (mut start_address, mut end_address) = (read_address, read_address + length);
        // if start_address > bytes.len() {
        //     start_address = bytes.len();
        // }
        // if end_address > bytes.len() {
        //     end_address = bytes.len();
        // }

        // self.bytes[write_address..write_address + (end_address - start_address)]
        //     .copy_from_slice(&bytes[start_address..end_address]);
        // unsafe  {

            // let destination = bytes.as_ptr() + read_address.as_usize();
            // copy_bytes_with_padding(self.bytes.as_mut_ptr() + read_address, , length);
        // }
        let read_address = if read_address > (std::usize::MAX - length).into() {
            std::usize::MAX - length
        } else {
            read_address.as_usize()
        };
        copy_bytes(bytes, read_address, &mut self.bytes, write_address, length);
        ExecutionResult::InProgress
    }
    // #[inline]
    // pub fn copy_from_bytes(
    //     &mut self,
    //     bytes: &[u8],
    //     read_address: U256,
    //     write_address: usize,
    //     length: usize,
    //     gas_recorder: &mut GasRecorder,
    // ) -> ExecutionResult {
    //     // TODO should be kept as U256 as maybe the gas input is high enough for this?
    //     if write_address.checked_add(length).is_none() || read_address.checked_add(U256::from(length)).is_none()
    //     {
    //         gas_recorder.record_gas_usage(gas_recorder.gas_input as u64);
    //         return ExecutionResult::Error(ExecutionError::InsufficientGas);
    //     }
    //     if write_address + length > self.max_index {
    //         return_if_error!(self.expand(write_address + length, Some(gas_recorder)));
    //     }
    //     let (mut start_address, mut end_address) = (read_address, read_address + length);
    //     if start_address > bytes.len() {
    //         start_address = bytes.len();
    //     }
    //     if end_address > bytes.len() {
    //         end_address = bytes.len();
    //     }
    //     self.bytes[write_address..write_address + (end_address - start_address)]
    //         .copy_from_slice(&bytes[start_address..end_address]);
    //     ExecutionResult::InProgress
    // }

    #[inline]
    pub fn read_u256(
        &mut self,
        address: usize,
        gas_recorder: &mut GasRecorder,
    ) -> (ExecutionResult, U256) {
        // TODO add memory expansion cost?
        if self.bytes.len() < 32 || address > self.bytes.len() - 32 {
            println!("Memory read out of bounds: {:?}", address);
            return_tuple_if_error!(self.expand(address + 32, Some(gas_recorder)), ZERO);
        }
        let bytes_to_copy = &self.bytes[address..address + 32];
        let mut bytes = [0; 32];
        bytes.copy_from_slice(bytes_to_copy);
        (ExecutionResult::InProgress, U256::from(bytes))
    }

    #[inline]
    pub fn write_u256(
        &mut self,
        address: usize,
        value: U256,
        gas_recorder: &mut GasRecorder,
    ) -> ExecutionResult {
        if address >= self.bytes.len().max(32) - 32 {
            return_if_error!(self.expand(address + 32, Some(gas_recorder)));
        }
        let index = address;
        let end_index = index + 32;
        self.bytes[index..end_index].copy_from_slice(&u256_to_array(value).to_vec());
        ExecutionResult::InProgress
    }

    #[inline]
    pub fn write_u8(
        &mut self,
        address: usize,
        value: u8,
        gas_recorder: &mut GasRecorder,
    ) -> ExecutionResult {
        if address >= self.bytes.len().max(1) - 1{
            return_if_error!(self.expand(address + 1, Some(gas_recorder)));
        }
        self.bytes[address] = value;
        ExecutionResult::InProgress
    }

    #[inline]
    pub fn read_bytes(
        &mut self,
        address: usize,
        length: usize,
        gas_recorder: &mut GasRecorder,
    ) -> (ExecutionResult, Vec<u8>) {
        if address >  self.bytes.len().max(length)  - length {
            return_tuple_if_error!(self.expand(address + length, Some(gas_recorder)), vec![]);
        }
        (
            ExecutionResult::InProgress,
            self.bytes[address..(address + length)].to_vec(),
        )
    }

    #[inline]
    fn expand(
        &mut self,
        new_length: usize,
        gas_recorder: Option<&mut GasRecorder>,
    ) -> ExecutionResult {
        // TODO add max memory size checks
        if new_length == 0 {
            return ExecutionResult::InProgress;
        }
        self.max_index = new_length;
        match gas_recorder {
            Some(gas_recorder) => {
                gas_recorder.record_memory_gas_usage(self.bytes.len(), new_length);
                return_if_gas_too_high!(gas_recorder);
            }
            _ => {}
        }

        self.bytes.resize(new_length, 0);
        ExecutionResult::InProgress
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
fn copy_bytes(src: &[u8], src_index: usize, dest: &mut [u8], dest_index: usize, length: usize) {
    for i in 0..length {
        let src_byte = if src_index + i < src.len() {
            src[src_index + i]
        } else {
            0 // If index is out of bounds, write 0
        };
        dest[dest_index + i] = src_byte;
    }
}
