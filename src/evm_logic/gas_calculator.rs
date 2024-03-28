use primitive_types::U256;

// TODO move into gas_recorder
#[inline]
pub fn call_data_gas_cost(data: &Vec<u8>) -> u64 {
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

#[derive(Copy,Clone)]
pub struct GasRecorder {
    pub gas_input: usize,
    pub gas_usage: usize,
    pub gas_refunds: usize,
}

impl GasRecorder {

    pub fn validate_gas(&self) -> bool {
        self.gas_usage <= self.gas_input
    }

    pub fn record_gas(&mut self, gas: u64) {
        self.gas_usage += gas as usize;
    }

    pub fn record_memory_usage(&mut self, current_memory_size: usize, new_memory_size: usize) {
        if new_memory_size == 0 {
            return;
        }
        let old_cost = GasRecorder::memory_cost(current_memory_size);
        let new_cost = GasRecorder::memory_cost(new_memory_size);
        // println!("Old cost: {}, New cost: {}", old_cost, new_cost);
        let len = new_memory_size - current_memory_size;
        // let memory_expansion_cost = 3 + 3 * (len as u64 + 31 / 32) as usize + (new_cost - old_cost);
        let memory_expansion_cost = new_cost - old_cost;
        println!("Memory expansion cost: {}", memory_expansion_cost);
        self.gas_usage += memory_expansion_cost;
    }

    pub fn subtract_gas(&mut self, gas: usize) {
        self.gas_refunds += gas;
    }

    fn memory_cost(current_memory_size_bytes: usize) -> usize {
        if current_memory_size_bytes == 0 {
            return 0;
        }
        println!("current_memory_size_bytes: {}", current_memory_size_bytes);
        let memory_size_word = current_memory_size_bytes.div_ceil(32);
        let memory_cost = (memory_size_word.pow(2)) / 512 + (3 * memory_size_word);
        memory_cost
        // let memory_size_word = (current_memory_size_bytes - 1) / 4;
        // let memory_cost = usize::pow(memory_size_word, 2) / 512 + (3 * memory_size_word);
        // memory_cost
    }
}
