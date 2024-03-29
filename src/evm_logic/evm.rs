mod call;
mod create;
mod decoder;
pub mod macros;

use crate::configs::gas_costs::static_costs;
use crate::evm_logic::gas_calculator::GasRecorder;
use crate::result::{Error, ExecutionResult, ExecutionSuccess};
use crate::runtime::Runtime;

use super::state::memory::Memory;
use super::state::stack::Stack;

use primitive_types::U256;

#[derive(Clone)]
struct Transaction {
    pub origin: U256,
    pub gas_price: U256,
}

struct Message {
    pub caller: U256,
    pub value: U256,
    pub data: Vec<u8>,
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
    gas_input: u64,
    gas_price: U256,
    nested_index: usize,
    gas_recorder: GasRecorder,
    is_static: bool,
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
    ) -> (ExecutionResult, usize) {
        let message = Message {
            caller: contract,
            value: value,
            data: data,
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
            false,
        );
        evm.gas_recorder.record_gas_usage(static_costs::G_TRANSACTION);
        evm.gas_recorder.record_call_data_gas_usage(&evm.message.data);
        if debug {
            println!("Call Data Gas Cost: {:x}", evm.gas_recorder.gas_usage);
        }
        let result = evm.execute_program(runtime, debug);
        // TODO move this into gas_recorder
        let gas_usage = evm.gas_recorder.gas_usage
            - usize::min(evm.gas_recorder.gas_refunds, evm.gas_recorder.gas_usage / 2);
        return (result, gas_usage);
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
        is_static: bool,
    ) -> EVMContext {
        EVMContext {
            stack: Stack::new(),
            memory: Memory::new(),
            program: Memory::from(
                code,
                Some(&mut GasRecorder {
                    gas_input: gas as usize,
                    gas_usage: 0,
                    gas_refunds: 0,
                }),
            ),
            program_counter: 0,
            contract_address: address,
            // TODO remove need to clone here
            transaction: transaction,
            message: message,
            last_return_data: Memory::new(),
            gas_input: gas,
            gas_price: gas_price,
            nested_index: nested_index,
            gas_recorder: GasRecorder {
                gas_input: gas as usize,
                gas_usage: 0,
                gas_refunds: 0,
            },
            is_static: is_static,
        }
    }

    #[inline]
    fn execute_program(&mut self, runtime: &mut impl Runtime, debug: bool) -> ExecutionResult {
        runtime.add_context();

        let result = {
            let mut result;
            if self.program.len() != 0 {
                loop {
                    result = self.execute_next_instruction(runtime, debug);
                    match &result {
                        ExecutionResult::Err(_) => {
                            break;
                        }
                        ExecutionResult::Success(success) => match success {
                            ExecutionSuccess::Return(_) | ExecutionSuccess::Stop => {
                                break;
                            }
                            _ => {}
                        },
                    }
                }
            } else {
                result = ExecutionResult::Err(Error::InvalidMemSize);
            }
            if debug {
                println!(
                    "Program Gas Usage : {:x}",
                    if self.gas_recorder.gas_usage > self.gas_input as usize {
                        self.gas_input as u64
                    } else {
                        self.gas_input - self.gas_recorder.gas_usage as u64
                    }
                );
            }
            result
        };
        self.gas_recorder.gas_usage = if self.gas_recorder.gas_usage > self.gas_input as usize {
            self.gas_input as u64
        } else {
            self.gas_recorder.gas_usage as u64
        } as usize;
        println!("Sub Gas Usage {:x}", self.gas_recorder.gas_usage);
        match result {
            ExecutionResult::Success(_) => {
                runtime.merge_context();
            }
            ExecutionResult::Err(_) => {
                runtime.revert_context();
            }
        }

        result
    }

    #[inline]
    fn execute_next_instruction(
        &mut self,
        runtime: &mut impl Runtime,
        debug: bool,
    ) -> ExecutionResult {
        decoder::decode_instruction(self, runtime, debug)
    }

    #[inline]
    fn check_gas_usage(&self) -> ExecutionResult {
        match (self.gas_recorder.gas_usage
            - self
                .gas_recorder
                .gas_refunds
                .min(self.gas_recorder.gas_usage))
            > self.gas_input as usize
        {
            true => ExecutionResult::Err(crate::result::Error::InsufficientGas),
            false => ExecutionResult::Success(ExecutionSuccess::Unknown),
        }
    }
}

// copy between mem objects
// message data
// program data
// mem data
