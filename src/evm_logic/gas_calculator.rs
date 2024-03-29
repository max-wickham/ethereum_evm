#[derive(Copy,Clone)]
pub struct GasRecorder {
    pub gas_input: usize,
    pub gas_usage: usize,
    pub gas_refunds: usize,
}

impl GasRecorder {

    pub fn validate_gas_usage(&self) -> bool {
        self.gas_usage <= self.gas_input
    }

    pub fn record_gas_usage(&mut self, gas: u64) {
        self.gas_usage += gas as usize;
    }

    pub fn record_refund(&mut self, gas: u64) {
        self.gas_refunds += gas as usize;
    }

    pub fn record_memory_gas_usage(&mut self, current_memory_size: usize, new_memory_size: usize) {
        if new_memory_size == 0 || current_memory_size >= new_memory_size {
            return;
        }
        let old_cost = memory_cost(current_memory_size);
        let new_cost = memory_cost(new_memory_size);
        // println!("Old cost: {}, New cost: {}", old_cost, new_cost);
        let len = new_memory_size - current_memory_size;
        // let memory_expansion_cost = 3 + 3 * (len as u64 + 31 / 32) as usize + (new_cost - old_cost);
        let memory_expansion_cost = new_cost - old_cost;
        if self.gas_usage.checked_add(memory_expansion_cost).is_none() {
            self.gas_usage = u64::MAX as usize;
            return;
        }
        self.gas_usage += memory_expansion_cost;
    }


    pub fn record_call_data_gas_usage(&mut self, data: &Vec<u8>) {
        let cost = call_data_gas_cost(data);
        self.record_gas_usage(cost);
    }

    pub fn merge(&mut self, other: &GasRecorder) {
        self.gas_usage += other.gas_usage;
        self.gas_refunds += other.gas_refunds;
    }
}

#[inline]
fn call_data_gas_cost(data: &Vec<u8>) -> u64 {
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
    println!("Memory cost {:x}", memory_cost);
    memory_cost
}
