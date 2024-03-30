use core::panic;

use crate::result::ExecutionResult;


#[derive(Copy,Clone)]
pub struct GasRecorder {
    pub gas_input: usize,
    pub gas_usage: usize,
    pub gas_refunds: usize,
}

impl GasRecorder {

    pub fn usage_with_refunds(&self) -> usize {
        self.gas_usage - self.gas_refunds.min(self.gas_usage / 2)
    }

    pub fn is_valid(&self) -> bool {
        self.gas_usage <= self.gas_input
    }

    pub fn set_gas_usage_to_max(&mut self) {
        self.gas_usage = self.gas_input;
    }
    // TODO unit test
    pub fn is_valid_with_refunds(&self) -> bool {
        (self.gas_usage - self.gas_refunds.min(self.gas_usage / 5)) <= self.gas_input
    }

    pub fn record_gas_usage(&mut self, gas: u64) {
        self.gas_usage += gas as usize;
    }

    pub fn record_refund(&mut self, gas: u64) {
        self.gas_refunds += gas as usize;
    }

    // TODO unit test
    pub fn record_memory_gas_usage(&mut self, current_memory_size: usize, new_memory_size: usize) {
        if new_memory_size == 0 || current_memory_size >= new_memory_size {
            return;
        }
        let old_cost = memory_cost(current_memory_size);
        let new_cost = memory_cost(new_memory_size);
        let memory_expansion_cost = new_cost - old_cost;
        if self.gas_usage.checked_add(memory_expansion_cost).is_none() {
            self.gas_usage = u64::MAX as usize;
            return;
        }
        self.gas_usage += memory_expansion_cost;
    }


    pub fn record_call_data_gas_usage(&mut self, data: &[u8]) {
        let cost = call_data_gas_cost(data);
        self.record_gas_usage(cost);
    }

    pub fn merge(&mut self, other: &GasRecorder, execution_result: &ExecutionResult) {
        match execution_result {
            ExecutionResult::Error(_) => {
                self.gas_usage += other.gas_usage;
            }
            ExecutionResult::Success(_) => {
                self.gas_usage += other.gas_usage;
                self.gas_refunds += other.gas_refunds;
            }
            ExecutionResult::InProgress => {
                panic!("Cannot merge in progress execution results");
            }
        }
    }
}

#[inline]
fn call_data_gas_cost(data: &[u8]) -> u64 {
    let mut cost = 0;
    for byte in data {
        if *byte == 0 {
            cost += 4;
        } else {
            cost += 16;
        }
    }
    cost
}

#[inline]
fn memory_cost(current_memory_size_bytes: usize) -> usize {
    if current_memory_size_bytes == 0 {
        return 0;
    }

    let memory_size_word = current_memory_size_bytes.div_ceil(32);
    if memory_size_word.checked_mul(3).is_none() {
        return u64::MAX as usize;
    }
    if memory_size_word.checked_pow(2).is_none() {
        return u64::MAX as usize;
    }
    let memory_cost = (memory_size_word.pow(2)) / 512 + (3 * memory_size_word);
    memory_cost
}
