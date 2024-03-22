// use crate::main;
use crate::state::memory::Memory;
use crate::state::stack::Stack;
use crate::util::{self, keccak256};
use crate::{bytecode_spec::opcodes, runtime::Runtime};
use ethnum::U256;
use paste::paste;

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
    pub stack: Stack,
    pub memory: Memory,
    // storage: &'b mut BTreeMap<U256, U256>,
    pub program: Memory,
    pub program_counter: usize,
    pub contract_address: U256,
    pub transaction: Transaction,
    pub message: Message,
    pub last_return_data: Memory,
    pub result: Memory,
    pub gas_input: u64,
    pub gas_usage: u64,
    pub gas_price: U256,
    pub stopped: bool,
    pub nested_index: usize,
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
            nested_index: 0,
        }
    }

    #[inline]
    pub fn execute(&mut self, runtime: &mut impl Runtime) -> bool {
        // TODO run code
        let num_bytes = self.message.data.bytes.len();
        let num_none_zero = self
            .message
            .data
            .bytes
            .iter()
            .filter(|&&byte| byte != 0)
            .count();
        let num_zero_bytes = num_bytes - num_none_zero;
        let gas_cost = 4 * num_zero_bytes + 16 * num_none_zero;
        self.gas_usage += gas_cost as u64;
        println!("Call Data Gas Cost: {}", gas_cost);
        // Compute memory expansion cost for program
        // let memory_size_word = (self.program.len() / 4) as u64;
        // let memory_cost =
        //     U256::from((u64::pow(memory_size_word, 2) / 512 + (3 * memory_size_word)) as u64);
        // self.gas_usage += memory_cost.as_u64();
        // println!("Program Memory Gas Cost: {}", memory_cost);

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

        // Declared here so that self is in scope
        macro_rules! debug {
            ($($input:tt)*) => {
                let tabs = "\t".repeat(self.nested_index as usize);
                print!("{}", tabs);
                println!($($input)*);
            };
        }

        let opcode: u8 = self.program[self.program_counter];
        let current_gas_usage = self.gas_usage;

        match opcode {
            opcodes::STOP => {
                debug!("STOP");
                self.stopped = true;
                return true;
            }

            opcodes::ADD => {
                debug!("ADD");
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a.wrapping_add(b));
                self.gas_usage += 3;
            }

            opcodes::MUL => {
                debug!("MUL");
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a.wrapping_mul(b));
                self.gas_usage += 5;
            }

            opcodes::SUB => {
                debug!("SUB");
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a.wrapping_sub(b));
                self.gas_usage += 3;
            }

            opcodes::DIV => {
                debug!("DIV");
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a.wrapping_div(b));
                self.gas_usage += 5;
            }

            opcodes::SDIV => {
                debug!("SDIV");
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push((a.as_i256() / b.as_i256()).as_u256());
                self.gas_usage += 5;
            }

            opcodes::MOD => {
                debug!("MOD");
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a % b);
                self.gas_usage += 5;
            }

            opcodes::SMOD => {
                debug!("SMOD");
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push((a.as_i256() % b.as_i256()).as_u256());
                self.gas_usage += 5;
            }

            opcodes::ADDMOD => {
                debug!("ADDMOD");
                let (a, b, c) = (self.stack.pop(), self.stack.pop(), self.stack.pop());
                self.stack.push((a + b) % c);
                self.gas_usage += 8;
            }

            opcodes::MULMOD => {
                debug!("MULMOD");
                let (a, b, c) = (self.stack.pop(), self.stack.pop(), self.stack.pop());
                self.stack.push((a * b) % c);
                self.gas_usage += 8;
            }

            opcodes::EXP => {
                debug!("EXP");
                let (a, exponent) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a.pow(exponent.as_u32()));
                self.gas_usage += 10 + 50 * (util::bytes_for_u256(&exponent) as u64);
            }

            opcodes::SIGNEXTEND => {
                debug!("SIGNEXTEND");
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
                debug!("LT");
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(U256::from(a < b));
                self.gas_usage += 3;
            }

            opcodes::GT => {
                debug!("GT");
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(U256::from(a > b));
                self.gas_usage += 3;
            }

            opcodes::SLT => {
                debug!("SLT");
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(U256::from(a.as_i256() < b.as_i256()));
                self.gas_usage += 3;
            }

            opcodes::SGT => {
                debug!("SGT");
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(U256::from(a.as_i256() > b.as_i256()));
                self.gas_usage += 3;
            }

            opcodes::EQ => {
                debug!("EQ");
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(U256::from(a == b));
                self.gas_usage += 3;
            }

            opcodes::ISZERO => {
                debug!("ISZERO");
                let data = self.stack.pop();
                self.stack.push(U256::from(data.eq(&U256::ZERO)));
                self.gas_usage += 3;
            }

            opcodes::AND => {
                debug!("AND");
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a & b);
                self.gas_usage += 3;
            }

            opcodes::OR => {
                debug!("OR");
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a | b);
                self.gas_usage += 3;
            }

            opcodes::XOR => {
                debug!("XOR");
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a ^ b);
                self.gas_usage += 3;
            }

            opcodes::NOT => {
                debug!("NOT");
                let a = self.stack.pop();
                self.stack.push(!a);
                self.gas_usage += 3;
            }

            opcodes::BYTE => {
                debug!("BYTE");
                let (i, x) = (self.stack.pop(), self.stack.pop());
                self.stack.push((x >> (248 - i * 8)) & 0xFF);
                self.gas_usage += 3;
            }

            opcodes::SHL => {
                debug!("SHL");
                let (shift, value) = (self.stack.pop(), self.stack.pop());
                self.stack.push(value << shift);
                self.gas_usage += 3;
            }

            opcodes::SHR => {
                debug!("SHR");
                let (shift, value) = (self.stack.pop(), self.stack.pop());
                self.stack.push(value >> shift);
                self.gas_usage += 3;
            }

            opcodes::SAR => {
                debug!("SAR");
                let (shift, value) = (self.stack.pop(), self.stack.pop().as_i256());
                self.stack.push((value >> shift).as_u256());
                self.gas_usage += 3;
            }

            opcodes::KECCAK256 => {
                debug!("KECCAK256");
                let (offset, length) = (self.stack.pop().as_usize(), self.stack.pop().as_usize());
                let bytes = self.memory.read_bytes(offset, length);
                self.stack.push(keccak256(&bytes));
                // As of the Ethereum Yellow Paper (EIP-62), the gas cost for the KECCAK256 instruction is 30 gas plus an additional 6 gas for each 256-bit word (or part thereof) of input data.
                self.gas_usage += 30 + (length.div_ceil(256) as u64 * 6);
            }

            opcodes::ADDRESS => {
                debug!("ADDRESS");
                self.stack.push(self.contract_address);
                self.gas_usage += 2;
            }

            opcodes::BALANCE => {
                debug!("BALANCE");
                let address = self.stack.pop();
                self.stack.push(runtime.balance(address));
                self.gas_usage += if runtime.is_hot(address) { 100 } else { 2600 };
                runtime.mark_hot(address);
            }

            opcodes::ORIGIN => {
                debug!("ORIGIN");
                self.stack.push(self.transaction.origin);
                self.gas_usage += 2;
            }

            opcodes::CALLER => {
                debug!("CALLER");
                self.stack.push(self.message.caller);
                self.gas_usage += 2;
            }

            opcodes::CALLVALUE => {
                debug!("CALLVALUE");
                self.stack.push(self.message.value);
                self.gas_usage += 2;
            }

            opcodes::CALLDATALOAD => {
                debug!("CALLDATALOAD");
                // TODO fix
                let index = self.stack.pop().as_u64() as usize;
                self.stack.push(self.message.data.read(index));
                self.gas_usage += 3;
            }

            opcodes::CALLDATASIZE => {
                debug!("CALLDATASIZE");
                self.stack.push(U256::from(self.message.data.len() as u64));
                self.gas_usage += 2;
            }

            opcodes::CALLDATACOPY => {
                debug!("CALLDATACOPY");
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
                debug!("CODESIZE");
                self.stack.push(U256::from(self.program.len() as u64));
                self.gas_usage += 2;
            }

            opcodes::CODECOPY => {
                debug!("CODECOPY");
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
                debug!("GASPRICE");
                self.stack.push(self.transaction.gas_price);
                self.gas_usage += 2;
            }

            opcodes::EXTCODESIZE => {
                debug!("EXTCODESIZE");
                let address = self.stack.pop();
                self.stack.push(runtime.code_size(address));
                self.gas_usage += if runtime.is_hot(address) { 100 } else { 2600 };
                runtime.mark_hot(address);
            }

            opcodes::EXTCODECOPY => {
                debug!("EXTCODECOPY");
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
                debug!("RETURNDATASIZE");
                self.stack
                    .push(U256::from(self.last_return_data.len() as u64));
                self.gas_usage += 2;
            }

            opcodes::RETURNDATACOPY => {
                debug!("RETURNDATACOPY");
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
                debug!("EXTCODEHASH");
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
                debug!("MLOAD");
                let offset = self.stack.pop().as_usize();
                let current_memory_usage = self.memory.memory_cost;
                self.stack.push(self.memory.read(offset));
                let new_usage = self.memory.memory_cost;
                self.gas_usage += 3 + (new_usage - current_memory_usage).as_u64();
            }

            opcodes::MSTORE => {
                debug!("MSTORE");
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
                debug!("SLOAD");
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
                debug!("SSTORE");
                let (key, value) = (self.stack.pop(), self.stack.pop());
                let base_dynamic_gas = if runtime.storage(self.contract_address).contains_key(&key)
                    && runtime.storage(self.contract_address)[&key] != 0
                {
                    500
                } else {
                    20000
                };
                self.gas_usage += base_dynamic_gas;
                runtime.set_storage(self.contract_address, key, value);
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

            opcodes::PUSH_1..=opcodes::PUSH_32 => {
                let push_number = opcode - opcodes::PUSH_1 + 1;
                debug!("PUSH_{}", push_number);
                self.push_n(push_number as usize);
            }

            opcodes::DUP_1..=opcodes::DUP_16 => {
                let dup_number = opcode - opcodes::DUP_1 + 1;
                debug!("DUP_{}", dup_number);
                self.dup_n(dup_number as usize);
            }

            opcodes::SWAP_1..=opcodes::SWAP_16 => {
                let swap_number = opcode - opcodes::SWAP_1 + 1;
                debug!("SWAP_{}", swap_number);
                self.swap_n(swap_number as usize);
            }

            // TODO log
            opcodes::LOG_0 => {
                // TODO
            }

            opcodes::CREATE => {
                // TODO
            }

            opcodes::CALL => {
                debug!("Make Call");
                self.make_call(runtime, false);
            }

            opcodes::CALLCODE => {
                self.make_call(runtime, true);
            }

            opcodes::RETURN => {
                debug!("RETURN");
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
            // TODO add revert logic etc.
            return false;
        }
        let gas_usage = self.gas_usage - current_gas_usage;
        debug!("Gas Usage: {}", gas_usage);
        true
    }

    #[inline]
    fn check_gas_usage(&self) -> bool {
        self.gas_usage > self.gas_input
    }

    #[inline]
    fn push_n(&mut self, num_bytes: usize) {
        let bytes = self
            .program
            .read_bytes(self.program_counter + 1, num_bytes as usize);
        self.program_counter += num_bytes as usize;
        self.stack.push_bytes(&bytes);
        self.gas_usage += 3;
        // for _ in 0..self.nested_index {print!("\t");}println!("Pushn {}", num_bytes);
        // for _ in 0..self.nested_index {print!("\t");}println!("{:?}",bytes);
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
        // for _ in 0..self.nested_index {print!("\t");}println!("Address: {:?}", address);
        let code: Vec<u8> = runtime.code(address);
        if !value.eq(&U256::ZERO) {
            gas += 2300;
        }
        let one_64th_value = (self.gas_input - self.gas_usage) * 63 / 64;
        if gas > one_64th_value {
            gas = one_64th_value;
        }
        // TODO check gas is okay
        let mut sub_evm = EVMContext::create_sub_context(
            if maintain_storage {
                self.contract_address
            } else {
                address
            },
            Message {
                caller: self.contract_address,
                data: {
                    let mut memory: Memory = Memory::new();
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
        sub_evm.nested_index = self.nested_index + 1;
        // TODO calculate cost of call data
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
        let positive_value_cost = if !value.eq(&U256::ZERO) { 6700 } else { 0 };
        let value_to_empty_account_cost = if value != 0
            && runtime.nonce(address) == 0
            && runtime.code_size(address) == 0
            && runtime.balance(address) == 0
        {
            25000
        } else {
            0
        };
        // println!("Value Cost: {}", value);
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
