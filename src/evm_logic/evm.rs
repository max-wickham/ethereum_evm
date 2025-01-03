mod call;
mod create;
mod decoder;
pub mod macros;
pub mod precompiles;

use std::f32::consts::E;

use crate::configs::gas_costs::static_costs;
use crate::configs::precompiles::{ self as precompile_addresses, is_precompile };
use crate::evm_logic::gas_recorder::GasRecorder;
use crate::result::{ ExecutionError, ExecutionResult, ExecutionSuccess };
use crate::runtime::Runtime;

use super::state::memory::Memory;
use super::state::program_memory::ProgramMemory;
use super::state::stack::Stack;
use super::util::ZERO;

use precompiles::ecrecover::{ ecrecover_contract };
use precompiles::sha2_256::sha2_256_contract;
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
    program: ProgramMemory,
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
    fn create_sub_context(
        address: U256,
        message: Message,
        gas: u64,
        code: Vec<u8>,
        transaction: Transaction,
        gas_price: U256,
        nested_index: usize,
        is_static: bool
    ) -> EVMContext {
        EVMContext {
            stack: Stack::new(),
            memory: Memory::new(),
            program: ProgramMemory::from(&code),
            program_counter: 0,
            contract_address: address,
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

        // let num_none_zero_calldata = self.message.data.iter().filter(|x| **x != 0).count() as u64;
        // let num_zero_calldata = self.message.data.len() as u64 - num_none_zero_calldata as u64;

        // let calldata_cost = static_costs::G_ZERO + static_costs::G_TX_DATA_NON_ZERO * num_none_zero_calldata + static_costs::G_TX_DATA_ZERO * num_zero_calldata;
        // self.gas_recorder.record_gas_usage(calldata_cost as u64);

        // TODO here we could check if a precompile and if so run the precompile contract
        let mut result;
        println!("Contract Address: {:x} Ecrecover", self.contract_address);
        match self.contract_address {
            x if x.eq(&precompile_addresses::ECRECOVER_PRECOMPILE) => {
                result = ecrecover_contract(self);
            }
            x if x == *precompile_addresses::SHA256_PRECOMPILE => {
                result = sha2_256_contract(self);
            }
            _ => {
                let jump_dests = decoder::calculate_jump_dests(self);
                if self.program.len() != 0 {
                    loop {
                        result = self.execute_next_instruction(runtime, &jump_dests, debug);
                        match &result {
                            ExecutionResult::InProgress => {}
                            _ => {
                                break;
                            }
                        }
                    }
                } else {
                    result = ExecutionResult::Error(ExecutionError::InvalidMemSize);
                }
            }
        }

        // TODO move this into gas_recorder
        self.gas_recorder.gas_usage = (if self.gas_recorder.gas_usage > (self.gas_input as usize) {
            self.gas_input as u64
        } else {
            self.gas_recorder.gas_usage as u64
        }) as usize;

        if debug {
            println!("Sub Gas Usage {:x}", self.gas_recorder.gas_usage);
        }
        match result {
            ExecutionResult::Success(_) => {
                runtime.merge_context();
            }
            ExecutionResult::Error(_) => {
                runtime.revert_context();
            }
            ExecutionResult::InProgress => {
                panic!("Program shouldn't have excited in progress");
            }
        }
        result
    }

    #[inline]
    fn execute_next_instruction(
        &mut self,
        runtime: &mut impl Runtime,
        jump_dests: &[usize],
        debug: bool
    ) -> ExecutionResult {
        decoder::decode_instruction(self, runtime, jump_dests, debug)
    }

    #[inline]
    fn check_gas_usage(&self) -> ExecutionResult {
        match !self.gas_recorder.is_valid_with_refunds() {
            true => ExecutionResult::Error(ExecutionError::InsufficientGas),
            false => ExecutionResult::InProgress,
        }
    }
}

// copy between mem objects
// message data
// program data
// mem data
#[inline]
pub fn execute_transaction(
    runtime: &mut impl Runtime,
    contract_address: U256,
    origin: U256,
    gas: u64,
    gas_price: U256,
    value: U256,
    data: &[u8],
    debug: bool
) -> (ExecutionResult, usize) {
    let message = Message {
        caller: contract_address,
        value: value,
        data: data.to_vec(),
    };

    let transaction = Transaction {
        origin: origin,
        gas_price: gas_price,
    };
    if !is_precompile(&contract_address) {
        runtime.mark_hot(contract_address);
    }

    // println!("Origin: {:x}", origin);
    // println!("Contract Address: {:x}", contract_address);
    let mut evm = EVMContext::create_sub_context(
        contract_address,
        message,
        gas,
        runtime.code(contract_address),
        transaction,
        gas_price,
        0,
        false
    );

    evm.gas_recorder.record_gas_usage(static_costs::G_TRANSACTION);
    evm.gas_recorder.record_call_data_gas_usage(&evm.message.data);
    if debug {
        println!("Call Data Gas Cost: {:x}", evm.gas_recorder.gas_usage);
    }

    // println!("Value: {:x}", value);
    // TODO checks here on balance
    runtime.deposit(contract_address, value);
    // withdraw the value from the sender
    runtime.withdrawal(origin, value);
    let result = evm.execute_program(runtime, debug);
    let gas_usage = evm.gas_recorder.usage_with_refunds();

    // Increase Nonce and deposit the value
    runtime.increase_nonce(origin);

    match result {
        ExecutionResult::Success(_) => {}
        _ => {
            // Undo the value send, TODO fix this up
            runtime.deposit(origin, value);
            // withdraw the value from the sender
            runtime.withdrawal(contract_address, value);
        }
    }

    // Withdraw the gas from the wallet
    let eth_usage = gas_usage * gas_price.as_usize();
    runtime.withdrawal(origin, U256::from(eth_usage as u64));
    runtime.deposit(runtime.block_coinbase(), U256::from(eth_usage as u64));

    // TODO handle not enough eth for gas and value

    runtime.merge_context();
    return (result, gas_usage);
}
