// use crate::main;
use crate::state::memory::Memory;
use crate::state::stack::Stack;
use crate::util::{self, gas_usage_change, keccak256};
use crate::{bytecode_spec::opcodes, runtime::Runtime};

use ethnum::U256;
use std::collections::HashMap;
use std::num::Wrapping;

#[derive(Clone)]
pub struct Transaction {
    pub origin: U256,
    pub gas_price: U256,
}

pub struct Message {
    pub caller: U256,
    pub value: U256,
    pub data: Memory,
}
pub struct EVMContext {
    // stack_pointer: usize,
    call_data: Memory,
    stack: Stack,
    memory: Memory,
    // storage: &'b mut HashMap<U256, U256>,
    program: Memory,
    program_counter: usize,
    contract_address: U256,
    transaction: Transaction,
    message: Message,
    last_return_data: Memory,
    result: Memory,
    gas_input: u64,
    gas_usage: u64,
    gas_price: U256,
    stopped: bool,
}

impl EVMContext {
    #[inline]
    pub fn create_sub_context(
        address: U256,
        message: Message,
        gas: u64,
        code: Vec<u8>,
        transaction: Transaction,
        gas_price: U256,
    ) -> EVMContext {
        EVMContext {
            call_data: Memory::new(),
            stack: Stack::new(),
            memory: Memory::new(),
            program: Memory::from(code),
            program_counter: 0,
            contract_address: address,
            // TODO remove need to clone here
            transaction: transaction,
            message: message,
            last_return_data: Memory::new(),
            result: Memory::new(),
            gas_input: gas,
            gas_usage: 0,
            gas_price: gas_price,
            stopped: false,
        }
    }

    #[inline]
    pub fn execute(&mut self, runtime: &mut impl Runtime) -> bool {
        // TODO run code
        while !self.stopped {
            let result = self.run_next_instruction(runtime);
            if !result {
                return false;
            }
        }
        true
    }

    #[inline]
    fn run_next_instruction(&mut self, runtime: &mut impl Runtime) -> bool {
        /*
        Run the next instruction, adjusting gas usage and return a bool that is true if okay, false if exception
        */

        let opcode = self.program[self.program_counter];
        println!("Opcode:{:?}", opcode);
        match opcode {
            opcodes::STOP => {
                self.stopped = true;
                return true;
            }

            opcodes::ADD => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a.wrapping_add(b));
                self.gas_usage += 3;
            }

            opcodes::MUL => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a.wrapping_mul(b));
                self.gas_usage += 5;
            }

            opcodes::SUB => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a.wrapping_sub(b));
                println!("A-B, {}", a.wrapping_sub(b));
                self.gas_usage += 3;
            }

            opcodes::DIV => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a.wrapping_div(b));
                self.gas_usage += 5;
            }

            opcodes::SDIV => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push((a.as_i256() / b.as_i256()).as_u256());
                self.gas_usage += 5;
            }

            opcodes::MOD => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a % b);
                self.gas_usage += 5;
            }

            opcodes::SMOD => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push((a.as_i256() % b.as_i256()).as_u256());
                self.gas_usage += 5;
            }

            opcodes::ADDMOD => {
                let (a, b, c) = (self.stack.pop(), self.stack.pop(), self.stack.pop());
                self.stack.push((a + b) % c);
                self.gas_usage += 8;
            }

            opcodes::MULMOD => {
                let (a, b, c) = (self.stack.pop(), self.stack.pop(), self.stack.pop());
                self.stack.push((a * b) % c);
                self.gas_usage += 8;
            }

            opcodes::EXP => {
                let (a, exponent) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a.pow(exponent.as_u32()));
                self.gas_usage += 10 + 50 * (util::bytes_for_u256(&exponent) as u64);
            }

            opcodes::SIGNEXTEND => {
                let (x, y) = (self.stack.pop(), self.stack.pop());
                if x > 31 {
                    self.stack.push(y);
                } else {
                    let t = 248 - x * 8;
                    let sign = y & (U256::from(1 as u64) << t);
                    if sign == 0 {
                        let lower_mask = x << t - 1;
                        self.stack.push(y & lower_mask);
                    } else {
                        let higher_mask = !(x << t - 1);
                        self.stack.push(y | higher_mask);
                    }
                }
                self.gas_usage += 5;
            }

            opcodes::LT => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(U256::from(a < b));
                self.gas_usage += 3;
            }

            opcodes::GT => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(U256::from(a > b));
                self.gas_usage += 3;
            }

            opcodes::SLT => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(U256::from(a.as_i256() < b.as_i256()));
                self.gas_usage += 3;
            }

            opcodes::SGT => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(U256::from(a.as_i256() > b.as_i256()));
                self.gas_usage += 3;
            }

            opcodes::EQ => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(U256::from(a == b));
                self.gas_usage += 3;
            }

            opcodes::ISZERO => {
                let data = self.stack.pop();
                self.stack.push(U256::from(data == 0));
                self.gas_usage += 3;
            }

            opcodes::AND => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a & b);
                self.gas_usage += 3;
            }

            opcodes::OR => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a | b);
                self.gas_usage += 3;
            }

            opcodes::XOR => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a ^ b);
                self.gas_usage += 3;
            }

            opcodes::NOT => {
                let a = self.stack.pop();
                self.stack.push(!a);
                self.gas_usage += 3;
            }

            opcodes::BYTE => {
                let (i, x) = (self.stack.pop(), self.stack.pop());
                self.stack.push((x >> (248 - i * 8)) & 0xFF);
                self.gas_usage += 3;
            }

            opcodes::SHL => {
                let (shift, value) = (self.stack.pop(), self.stack.pop());
                self.stack.push(value << shift);
                self.gas_usage += 3;
            }

            opcodes::SHR => {
                let (shift, value) = (self.stack.pop(), self.stack.pop());
                self.stack.push(value >> shift);
                self.gas_usage += 3;
            }

            opcodes::SAR => {
                let (shift, value) = (self.stack.pop(), self.stack.pop().as_i256());
                self.stack.push((value >> shift).as_u256());
                self.gas_usage += 3;
            }

            opcodes::KECCAK256 => {
                let (offset, length) = (self.stack.pop().as_usize(), self.stack.pop().as_usize());
                let bytes = self.memory.read_bytes(offset, length);
                self.stack.push(keccak256(&bytes));
                // As of the Ethereum Yellow Paper (EIP-62), the gas cost for the KECCAK256 instruction is 30 gas plus an additional 6 gas for each 256-bit word (or part thereof) of input data.
                self.gas_usage += 30 + (length.div_ceil(256) as u64 * 6);
            }

            opcodes::ADDRESS => {
                self.stack.push(self.contract_address);
                self.gas_usage += 2;
            }

            opcodes::BALANCE => {
                let address = self.stack.pop();
                self.stack.push(runtime.balance(address));
                self.gas_usage += if runtime.is_hot(address) { 100 } else { 2600 };
                runtime.mark_hot(address);
            }

            opcodes::ORIGIN => {
                self.stack.push(self.transaction.origin);
                self.gas_usage += 2;
            }

            opcodes::CALLER => {
                self.stack.push(self.message.caller);
                self.gas_usage += 2;
            }

            opcodes::CALLVALUE => {
                self.stack.push(self.message.value);
                self.gas_usage += 2;
            }

            opcodes::CALLDATALOAD => {
                let index = self.stack.pop().as_u64() as usize;
                self.stack.push(self.message.data.read(index));
                self.gas_usage += 3;
            }

            opcodes::CALLDATASIZE => {
                self.stack.push(U256::from(self.message.data.len() as u64));
                self.gas_usage += 2;
            }

            opcodes::CALLDATACOPY => {
                let (dest_offset, offset, length) = (
                    self.stack.pop().as_usize(),
                    self.stack.pop().as_usize(),
                    self.stack.pop().as_usize(),
                );
                let current_memory_usage = self.memory.memory_cost;
                self.memory
                    .copy_from(&self.message.data, offset, dest_offset, length);
                let new_usage = self.memory.memory_cost;
                self.gas_usage +=
                    3 + 3 * (length as u64 + 31 / 32) + (new_usage - current_memory_usage).as_u64();
            }

            opcodes::CODESIZE => {
                self.stack.push(U256::from(self.program.len() as u64));
                self.gas_usage += 2;
            }

            opcodes::CODECOPY => {
                let (dest_offset, offset, length) = (
                    self.stack.pop().as_usize(),
                    self.stack.pop().as_usize(),
                    self.stack.pop().as_usize(),
                );

                let current_memory_usage = self.memory.memory_cost;
                self.memory
                    .copy_from(&self.program, offset, dest_offset, length);
                let new_usage = self.memory.memory_cost;
                self.gas_usage +=
                    3 + 3 * (length as u64 + 31 / 32) + (new_usage - current_memory_usage).as_u64();
            }

            opcodes::GASPRICE => {
                self.stack.push(self.transaction.gas_price);
                self.gas_usage += 2;
            }

            opcodes::EXTCODESIZE => {
                let address = self.stack.pop();
                self.stack.push(runtime.code_size(address));
                self.gas_usage += if runtime.is_hot(address) { 100 } else { 2600 };
                runtime.mark_hot(address);
            }

            opcodes::EXTCODECOPY => {
                let (addr, dest_offset, offset, length) = (
                    self.stack.pop(),
                    self.stack.pop().as_usize(),
                    self.stack.pop().as_usize(),
                    self.stack.pop().as_usize(),
                );

                let current_memory_usage = self.memory.memory_cost;
                self.memory.copy_from(
                    &Memory::from(runtime.code(addr)),
                    offset,
                    dest_offset,
                    length,
                );
                let new_usage = self.memory.memory_cost;
                self.gas_usage +=
                    3 * (length as u64 + 31 / 32) + (new_usage - current_memory_usage).as_u64();
                self.gas_usage += if runtime.is_hot(addr) { 100 } else { 2600 };
                runtime.mark_hot(addr);
            }

            opcodes::RETURNDATASIZE => {
                self.stack
                    .push(U256::from(self.last_return_data.len() as u64));
                self.gas_usage += 2;
            }

            opcodes::RETURNDATACOPY => {
                let (dest_offset, offset, length) = (
                    self.stack.pop().as_usize(),
                    self.stack.pop().as_usize(),
                    self.stack.pop().as_usize(),
                );
                let current_memory_usage = self.memory.memory_cost;
                self.memory
                    .copy_from(&self.last_return_data, offset, dest_offset, length);
                let new_usage = self.memory.memory_cost;
                self.gas_usage +=
                    3 + 3 * (length as u64 + 31 / 32) + (new_usage - current_memory_usage).as_u64();
            }

            opcodes::EXTCODEHASH => {
                let addr = self.stack.pop();
                self.stack.push(util::keccak256_u256(addr));
                self.gas_usage += if runtime.is_hot(addr) { 100 } else { 2600 };
                runtime.mark_hot(addr);
            }

            opcodes::BLOCKHASH => {
                let block_number = self.stack.pop();
                self.stack.push(runtime.block_hash(block_number));
                self.gas_usage += 20;
            }

            opcodes::COINBASE => {
                self.stack.push(runtime.block_coinbase());
                self.gas_usage += 2;
            }

            opcodes::TIMESTAMP => {
                self.stack.push(runtime.block_timestamp());
                self.gas_usage += 2;
            }

            opcodes::NUMBER => {
                self.stack.push(runtime.block_number());
                self.gas_usage += 2;
            }

            opcodes::DIFFICULTY => {
                self.stack.push(runtime.block_difficulty());
                self.gas_usage += 2;
            }

            opcodes::GASLIMIT => {
                self.stack.push(runtime.block_gas_limit());
                self.gas_usage += 2;
            }

            opcodes::CHAINID => {
                self.stack.push(runtime.chain_id());
                self.gas_usage += 2;
            }

            opcodes::SELFBALANCE => {
                self.stack.push(runtime.balance(self.contract_address));
                self.gas_usage += 5;
            }

            opcodes::BASEFEE => {
                self.stack.push(runtime.block_base_fee_per_gas());
                self.gas_usage += 2;
            }

            opcodes::POP => {
                self.stack.pop();
                self.gas_usage += 2;
            }

            opcodes::MLOAD => {
                let offset = self.stack.pop().as_usize();
                let current_memory_usage = self.memory.memory_cost;
                self.stack.push(self.memory.read(offset));
                let new_usage = self.memory.memory_cost;
                self.gas_usage += 3 + (new_usage - current_memory_usage).as_u64();
            }

            opcodes::MSTORE => {
                let (offset, value) = (self.stack.pop().as_usize(), self.stack.pop());
                let current_memory_usage = self.memory.memory_cost;
                self.memory.write(offset, value);
                let new_usage = self.memory.memory_cost;
                self.gas_usage += 3 + (new_usage - current_memory_usage).as_u64();
            }

            opcodes::MSTORE8 => {
                let (offset, value) = (self.stack.pop().as_usize(), self.stack.pop());
                let current_memory_usage = self.memory.memory_cost;
                self.memory.write_u8(offset, (value & 0xFF).as_u8());
                let new_usage = self.memory.memory_cost;
                self.gas_usage += 3 + (new_usage - current_memory_usage).as_u64();
            }

            opcodes::SLOAD => {
                let key = self.stack.pop();
                self.stack
                    .push(runtime.storage(self.contract_address)[&key]);
                // self.gas_usage += if runtime.is_hot(self.contract_address) {
                //     100
                // } else {
                //     2600
                // };
                // runtime.mark_hot(self.contract_address);
            }

            opcodes::SSTORE => {
                let (key, value) = (self.stack.pop(), self.stack.pop());
                // self.storage.insert(key, value);
                runtime.set_storage(self.contract_address, key, value);
                // self.gas_usage += if runtime.is_hot(self.contract_address) {
                //     100
                // } else {
                //     2600
                // };
                // runtime.mark_hot(self.contract_address);
            }

            opcodes::JUMP => {
                let destination = self.stack.pop().as_usize();
                self.program_counter = destination;
                self.gas_usage += 8;
            }

            opcodes::JUMPI => {
                let (destination, condition) = (self.stack.pop().as_usize(), self.stack.pop());
                if condition != 0 {
                    self.program_counter = destination;
                }
                self.gas_usage += 10;
            }

            opcodes::PC => {
                self.stack.push(U256::from(self.program_counter as u64));
                self.gas_usage += 2;
            }

            opcodes::MSIZE => {
                self.stack.push(U256::from(self.memory.max_index as u64));
                self.gas_usage += 2;
            }

            opcodes::GAS => {
                self.stack.push(U256::from(self.gas_input - self.gas_usage));
                self.gas_usage += 2;
            }

            opcodes::PUSH_1 => {
                self.push_n(1);
            }
            opcodes::PUSH_2 => {
                self.push_n(2);
            }
            opcodes::PUSH_3 => {
                self.push_n(3);
            }
            opcodes::PUSH_4 => {
                self.push_n(4);
            }
            opcodes::PUSH_5 => {
                self.push_n(5);
            }
            opcodes::PUSH_6 => {
                self.push_n(6);
            }
            opcodes::PUSH_7 => {
                self.push_n(7);
            }
            opcodes::PUSH_8 => {
                self.push_n(8);
            }
            opcodes::PUSH_9 => {
                self.push_n(9);
            }
            opcodes::PUSH_10 => {
                self.push_n(10);
            }
            opcodes::PUSH_11 => {
                self.push_n(11);
            }
            opcodes::PUSH_12 => {
                self.push_n(12);
            }
            opcodes::PUSH_13 => {
                self.push_n(13);
            }
            opcodes::PUSH_14 => {
                self.push_n(14);
            }
            opcodes::PUSH_15 => {
                self.push_n(15);
            }
            opcodes::PUSH_16 => {
                self.push_n(16);
            }
            opcodes::PUSH_17 => {
                self.push_n(17);
            }
            opcodes::PUSH_18 => {
                self.push_n(18);
            }
            opcodes::PUSH_19 => {
                self.push_n(19);
            }
            opcodes::PUSH_20 => {
                self.push_n(20);
            }
            opcodes::PUSH_21 => {
                self.push_n(21);
            }
            opcodes::PUSH_22 => {
                self.push_n(22);
            }
            opcodes::PUSH_23 => {
                self.push_n(23);
            }
            opcodes::PUSH_24 => {
                self.push_n(24);
            }
            opcodes::PUSH_25 => {
                self.push_n(25);
            }
            opcodes::PUSH_26 => {
                self.push_n(26);
            }
            opcodes::PUSH_27 => {
                self.push_n(27);
            }
            opcodes::PUSH_28 => {
                self.push_n(28);
            }
            opcodes::PUSH_29 => {
                self.push_n(29);
            }
            opcodes::PUSH_30 => {
                self.push_n(30);
            }
            opcodes::PUSH_31 => {
                self.push_n(31);
            }
            opcodes::PUSH_32 => {
                self.push_n(32);
            }

            opcodes::DUP_1 => {
                self.dup_n(1);
            }
            opcodes::DUP_2 => {
                self.dup_n(2);
            }
            opcodes::DUP_3 => {
                self.dup_n(3);
            }
            opcodes::DUP_4 => {
                self.dup_n(4);
            }
            opcodes::DUP_5 => {
                self.dup_n(5);
            }
            opcodes::DUP_6 => {
                self.dup_n(6);
            }
            opcodes::DUP_7 => {
                self.dup_n(7);
            }
            opcodes::DUP_8 => {
                self.dup_n(8);
            }
            opcodes::DUP_9 => {
                self.dup_n(9);
            }
            opcodes::DUP_10 => {
                self.dup_n(10);
            }
            opcodes::DUP_11 => {
                self.dup_n(11);
            }
            opcodes::DUP_12 => {
                self.dup_n(12);
            }
            opcodes::DUP_13 => {
                self.dup_n(13);
            }
            opcodes::DUP_14 => {
                self.dup_n(14);
            }
            opcodes::DUP_15 => {
                self.dup_n(15);
            }
            opcodes::DUP_16 => {
                self.dup_n(16);
            }

            opcodes::SWAP_1 => {
                self.swap_n(1);
            }
            opcodes::SWAP_2 => {
                self.swap_n(2);
            }
            opcodes::SWAP_3 => {
                self.swap_n(3);
            }
            opcodes::SWAP_4 => {
                self.swap_n(4);
            }
            opcodes::SWAP_5 => {
                self.swap_n(5);
            }
            opcodes::SWAP_6 => {
                self.swap_n(6);
            }
            opcodes::SWAP_7 => {
                self.swap_n(7);
            }
            opcodes::SWAP_8 => {
                self.swap_n(8);
            }
            opcodes::SWAP_9 => {
                self.swap_n(9);
            }
            opcodes::SWAP_10 => {
                self.swap_n(10);
            }
            opcodes::SWAP_11 => {
                self.swap_n(11);
            }
            opcodes::SWAP_12 => {
                self.swap_n(12);
            }
            opcodes::SWAP_13 => {
                self.swap_n(13);
            }
            opcodes::SWAP_14 => {
                self.swap_n(14);
            }
            opcodes::SWAP_15 => {
                self.swap_n(15);
            }
            opcodes::SWAP_16 => {
                self.swap_n(16);
            }

            // TODO log
            opcodes::LOG_0 => {
                // TODO
            }

            opcodes::CREATE => {
                // TODO
            }

            opcodes::CALL => {
                self.make_call(runtime, false);
                self.gas_usage += 100;
            }

            opcodes::CALLCODE => {
                self.make_call(runtime, true);
                self.gas_usage += 100;
            }

            opcodes::RETURN => {
                let (offset, size) = (self.stack.pop().as_usize(), self.stack.pop().as_usize());
                self.result.set_length(size);
                self.result.copy_from(&self.memory, offset, 0, size);
                self.gas_usage += 0;
            }

            opcodes::DELEGATECALL => {
                // TODO
                // Same as call but storage, sender and value remain the same
                self.gas_usage += 100;
            }

            opcodes::CREATE2 => {
                // TODO
                // Same as create but except the salt allows the new contract to be deployed at a consistent, deterministic address.
                // Should deployment succeed, the account's code is set to the return data resulting from executing the initialisation code.
            }

            _ => {}
        }

        self.program_counter += 1;
        if self.check_gas_usage() {
            return false;
        }
        return true;
    }

    #[inline]
    fn check_gas_usage(&self) -> bool {
        self.gas_usage > self.gas_input
    }

    // #[inline]
    // fn set_return_data(&mut self, memory: &Memory, offset: usize, length: usize) {
    //     self.last_return_data.set_length(length);
    //     self.last_return_data.copy_from(memory, offset, 0, length);
    // }

    #[inline]
    fn push_n(&mut self, num_bytes: usize) {
        let bytes = self
            .program
            .read_bytes(self.program_counter + 1, num_bytes as usize);
        self.program_counter += num_bytes as usize;
        self.stack.push_bytes(&bytes);
        self.gas_usage += 3;
        // println!("Pushn {}", num_bytes);
        // println!("{:?}",bytes);
    }

    #[inline]
    fn dup_n(&mut self, index: usize) {
        let value = self.stack.read_nth(index);
        self.stack.push(value);
        self.gas_usage += 3;
    }

    #[inline]
    fn swap_n(&mut self, index: usize) {
        let bottom_value = self.stack.read_nth(index);
        let top_value = self.stack.pop();
        self.stack.write_nth(index - 1, top_value);
        self.stack.push(bottom_value);
        self.gas_usage += 3;
    }

    #[inline]
    fn make_call(&mut self, runtime: &mut impl Runtime, maintain_storage: bool) -> bool {
        let (mut gas, address, value, args_offset, args_size, ret_offset, ret_size) = (
            self.stack.pop().as_u64(),
            self.stack.pop(),
            self.stack.pop(),
            self.stack.pop().as_usize(),
            self.stack.pop().as_usize(),
            self.stack.pop().as_usize(),
            self.stack.pop().as_usize(),
        );
        let code: Vec<u8> = runtime.code(address);

        if value != 0 {
            gas += 2300;
        }
        if gas > (self.gas_input - self.gas_usage) * 63 / 64 {
            gas = (self.gas_input - self.gas_usage) * 63 / 64;
        }
        // TODO check gas is okay
        let mut sub_evm = EVMContext::create_sub_context(
            address,
            Message {
                caller: self.contract_address,
                data: {
                    let mut memory = Memory::new();
                    memory.copy_from(&self.memory, args_offset, 0, args_size);
                    memory
                },
                value: value,
            },
            gas,
            code,
            self.transaction.clone(),
            self.gas_price,
        );
        let response = sub_evm.execute(runtime);
        self.last_return_data = sub_evm.result;
        let current_memory_cost = self.memory.memory_cost;
        self.memory
            .copy_from(&self.last_return_data, 0, ret_offset, ret_size);
        let new_memory_cost = self.memory.memory_cost;
        self.stack.push(U256::from(response));

        let memory_expansion_cost = (new_memory_cost - current_memory_cost).as_u64();
        let code_execution_cost = sub_evm.gas_usage;
        let address_access_cost = if runtime.is_hot(address) {
            100
        } else {
            runtime.mark_hot(address);
            2600
        };
        let positive_value_cost = if value != 0 { 6700 } else { 0 };
        let value_to_empty_account_cost = if value != 0
            && runtime.nonce(address) == 0
            && runtime.code_size(address) == 0
            && runtime.balance(address) == 0
        {
            25000
        } else {
            0
        };
        self.gas_usage += memory_expansion_cost
            + code_execution_cost
            + address_access_cost
            + positive_value_cost
            + value_to_empty_account_cost;
        response
    }
}

// copy between mem objects
// message data
// program data
// mem data
