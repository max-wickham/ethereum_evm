mod decoder;

use crate::evm_logic::gas_calculator::{call_data_gas_cost, GasRecorder};
use crate::state::memory::Memory;
use crate::state::stack::Stack;
use crate::runtime::Runtime;

use primitive_types::U256;

#[derive(Clone)]
struct Transaction {
    pub origin: U256,
    pub gas_price: U256,
}

struct Message {
    pub caller: U256,
    pub value: U256,
    pub data: Memory,
}

pub struct EVMContext {
    stack: Stack,
    memory: Memory,
    program: Memory,
    program_counter: usize,
    contract_address: U256,
    transaction: Transaction,
    message: Message,
    last_return_data: Memory,
    result: Memory,
    gas_input: u64,
    gas_price: U256,
    stopped: bool,
    nested_index: usize,
    gas_recorder: GasRecorder,
}

impl EVMContext {
    #[inline]
    pub fn execute_transaction(
        runtime: &mut impl Runtime,
        contract: U256,
        origin: U256,
        gas: u64,
        gas_price: U256,
        value: U256,
        data: Vec<u8>,
        debug: bool,
    ) -> usize {
        let message = Message {
            caller: contract,
            value: value,
            data: Memory::from(data, &mut GasRecorder { gas_usage: 0 }),
        };
        let transaction = Transaction {
            origin: origin,
            gas_price: gas_price,
        };
        let mut evm = EVMContext::create_sub_context(
            contract,
            message,
            gas,
            runtime.code(contract),
            transaction,
            gas_price,
            0,
        );
        evm.gas_recorder.record_gas(21000);
        evm.execute_program(runtime, debug);
        evm.gas_recorder.gas_usage
    }

    #[inline]
    fn create_sub_context(
        address: U256,
        message: Message,
        gas: u64,
        code: Vec<u8>,
        transaction: Transaction,
        gas_price: U256,
        nested_index: usize,
    ) -> EVMContext {
        EVMContext {
            stack: Stack::new(),
            memory: Memory::new(),
            program: Memory::from(code, &mut GasRecorder { gas_usage: 0 }),
            program_counter: 0,
            contract_address: address,
            // TODO remove need to clone here
            transaction: transaction,
            message: message,
            last_return_data: Memory::new(),
            result: Memory::new(),
            gas_input: gas,
            gas_price: gas_price,
            stopped: false,
            nested_index: nested_index,
            gas_recorder: GasRecorder { gas_usage: 0 },
        }
    }

    #[inline]
    fn execute_program(&mut self, runtime: &mut impl Runtime, debug: bool) -> bool {
        runtime.add_context();

        let result = || -> bool {
            println!("Message data size : {}", self.message.data.bytes.len());
            if self.message.data.bytes.len() != 0{
                self.gas_recorder
                .record_gas(call_data_gas_cost(&self.message.data.bytes));
            }
            if debug {
                println!("Call Data Gas Cost: {}", self.gas_recorder.gas_usage);
            }
            while !self.stopped {
                let result = self.execute_next_instruction(runtime, debug);
                if !result {
                    return false;
                }
            }
            if debug {
                println!(
                    "Program Gas Usage : {:x}",
                    self.gas_input - self.gas_recorder.gas_usage as u64
                );
            }
            true
        }();
        if result {
            runtime.merge_context();
        } else {
            runtime.revert_context();
        }

        result
    }

    #[inline]
    fn execute_next_instruction(&mut self, runtime: &mut impl Runtime, debug: bool) -> bool {
        decoder::decode_instruction(self, runtime, debug)
    }
    #[inline]
    fn check_gas_usage(&self) -> bool {
        self.gas_recorder.gas_usage > self.gas_input as usize
    }
}

// copy between mem objects
// message data
// program data
// mem data
