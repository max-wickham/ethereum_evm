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
pub struct ProgramMemory {
    pub bytes: Vec<u8>,
}

impl ProgramMemory {

    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    #[inline]
    pub fn from(bytes: &[u8]) -> ProgramMemory {
        let memory = ProgramMemory {
            bytes: bytes.to_vec(),
        };
        memory
    }

}

impl Index<usize> for ProgramMemory {
    type Output = u8;

    #[inline]
    fn index(&self, s: usize) -> &u8 {
        &self.bytes[s]
    }
}
