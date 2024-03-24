use std::ops::{Add, Div, Mul, Rem, Sub};

use crate::gas_calculator::call_data_gas_cost;
use crate::runtime;
// use crate::main;
use crate::state::memory::Memory;
use crate::state::stack::Stack;
use crate::util::{
    self, h256_to_u256, int256_to_uint256, keccak256, u256_to_array, u256_to_h256, u256_to_uint256, uint256_to_int256, MAX_UINT256, MAX_UINT256_COMPLEMENT, ZERO
};
use crate::{bytecode_spec::opcodes, runtime::Runtime};
use num256::{Int256, Uint256};
use paste::paste;
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
            data: Memory::from(data),
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
        evm.gas_usage = 21000;
        evm.execute(runtime, debug);
        return evm.gas_usage as usize;
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
            nested_index: nested_index,
        }
    }

    #[inline]
    pub fn execute(&mut self, runtime: &mut impl Runtime, debug: bool) -> bool {
        // TODO run code
        self.gas_usage += call_data_gas_cost(&self.message.data.bytes) as u64;
        if debug {
            println!("Call Data Gas Cost: {}", self.gas_usage);
        }
        let x: U256 = U256::zero();
        // Compute memory expansion cost for program
        // let memory_size_word = (self.program.len() / 4) as u64;
        // let memory_cost =
        //     U256::from((u64::pow(memory_size_word, 2) / 512 + (3 * memory_size_word)) as u64);
        // self.gas_usage += memory_cost.as_u64();
        // println!("Program Memory Gas Cost: {}", memory_cost);

        while !self.stopped {
            let result = self.run_next_instruction(runtime, debug);
            if !result {
                return false;
            }
        }
        if debug {
            println!("Gas : {:x}", self.gas_input - self.gas_usage);
        }

        true
    }

    #[inline]
    fn run_next_instruction(&mut self, runtime: &mut impl Runtime, debug: bool) -> bool {
        /*
        Run the next instruction, adjusting gas usage and return a bool that is true if okay, false if exception
        */

        // Declared here so that self is in scope
        macro_rules! debug {
            ($($input:tt)*) => {
                if debug {
                    let tabs = "\t".repeat(self.nested_index as usize);
                    print!("{}", tabs);
                    print!($($input)*);
                    println!(" Gas: {:x}", self.gas_input - self.gas_usage);
                }
            };
        }

        macro_rules! debug_match {
            ($opcode:expr, { $( $pat:pat => $block:block ),* }) => {
                match $opcode {
                    $(
                        $pat => {
                            #[allow(unreachable_code)]
                            #[allow(unused_variables)]{
                            {
                                print!("PC: {} ", self.program_counter);
                                let current_gas_usage = self.gas_usage;
                                if !(stringify!($pat).contains("PUSH") ||
                                    stringify!($pat).contains("DUP") ||
                                    stringify!($pat).contains("SWAP"))  {
                                    debug!(stringify!($pat));
                                    // print!(" Gas: {:x}", self.gas_input - self.gas_usage);
                                }
                                // print!(" Gas: {:x}", self.gas_input - self.gas_usage);
                                $block
                            // TODO create more elegant solution to this problem
                            // println!(" Gas: {}", self.gas_usage - current_gas_usage);
                            }
                        }
                    }),*
                    _ => {}
                }
            };
        }

        let opcode: u8 = self.program[self.program_counter];
        debug_match!(opcode, {

            opcodes::STOP => {
                self.stopped = true;
                return true;
            },

            opcodes::ADD => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a.overflowing_add(b).0);
                self.gas_usage += 3;
            },

            opcodes::MUL => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a.overflowing_mul(b).0);
                self.gas_usage += 5;
            },

            opcodes::SUB => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a.overflowing_sub(b).0);
                self.gas_usage += 3;
            },

            opcodes::DIV => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                match b {
                    ZERO => {
                        self.stack.push(U256::zero());
                    },
                    _ => {
                        self.stack.push(a.div_mod(b).0);
                    }
                }
                self.gas_usage += 5;
            },

            opcodes::SDIV => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                match b {
                    ZERO => {
                        self.stack.push(U256::zero());
                    },
                    _ => {
                        let a  = u256_to_uint256(a);
                        let b  = u256_to_uint256(b);
                        // Handle overflow case of -1 * MAX_POSITIVE
                        if a == *MAX_UINT256_COMPLEMENT && b == *MAX_UINT256 {
                            self.stack.push(U256::from(MAX_UINT256_COMPLEMENT.to_be_bytes()));
                        }
                        else {
                            let a = uint256_to_int256(a);
                            let b = uint256_to_int256(b);
                            let result: Uint256 = int256_to_uint256(a / b);
                            let result = U256::from(result.to_be_bytes());
                            self.stack.push(result);
                        }

                    }
                }
                self.gas_usage += 5;
            },

            opcodes::MOD => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                match b {
                    ZERO => {
                        self.stack.push(U256::zero());
                    },
                    _ => {
                        self.stack.push(a.rem(b));
                    }
                }
                self.gas_usage += 5;
            },

            opcodes::SMOD => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                match b {
                    ZERO => {
                        self.stack.push(U256::zero());
                    },
                    _ => {
                        let a = uint256_to_int256(u256_to_uint256(a));
                        let b = uint256_to_int256(u256_to_uint256(b));
                        let result: Uint256 = int256_to_uint256(a.rem(b));
                        let result = U256::from(result.to_be_bytes());
                        self.stack.push(result);
                    }
                }
                self.gas_usage += 5;

            },

            opcodes::ADDMOD => {
                let (a, b, c) = (self.stack.pop(), self.stack.pop(), self.stack.pop());
                match c {
                    ZERO => {
                        self.stack.push(U256::zero());
                    },
                    _ => {
                        self.stack.push(a.checked_rem(c).unwrap().overflowing_add(b.checked_rem(c).unwrap()).0);
                    }
                }
                self.gas_usage += 8;
            },

            opcodes::MULMOD => {
                let (a, b, c) = (self.stack.pop(), self.stack.pop(), self.stack.pop());
                match c {
                    ZERO => {
                        self.stack.push(U256::zero());
                    },
                    _ => {
                        self.stack.push(a.checked_rem(c).unwrap().overflowing_mul(b.checked_rem(c).unwrap()).0.checked_rem(c).unwrap());
                    }
                }
                // println!("a: {}, b: {}, c: {}", a, b, c);
                self.gas_usage += 8;
            },

            opcodes::EXP => {
                let (a, exponent) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a.overflowing_pow(exponent).0);
                self.gas_usage += 10 + 50 * (util::bytes_for_u256(&exponent) as u64);
            },

            opcodes::SIGNEXTEND => {
                let (x, y) = (self.stack.pop(), self.stack.pop());
                // X is the number of bytes of the input lower_mask
                if x > U256::from(31) {
                    self.stack.push(y);
                } else {
                    let sign = y >> (x*8 + 7) & U256::from(1 as u64);
                    let lower_mask = if x == U256::from(31) {
                        U256::max_value()
                    } else {
                        (U256::from(1) << ((x+1)*8)) - 1
                    };
                    if sign == ZERO {
                        self.stack.push(y & lower_mask);
                    } else {
                        let higher_mask = !lower_mask;
                        self.stack.push(y | higher_mask);
                    }
                }
                self.gas_usage += 5;
            },

            opcodes::LT => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(U256::from((a < b) as u64));
                self.gas_usage += 3;
            },

            opcodes::GT => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(U256::from((a > b) as u64));
                self.gas_usage += 3;
            },

            opcodes::SLT => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                let a = uint256_to_int256(u256_to_uint256(a));
                let b = uint256_to_int256(u256_to_uint256(b));
                self.stack.push(U256::from((a < b) as u64));
                self.gas_usage += 3;
            },

            opcodes::SGT => {
                // debug!("SGT");
                let (a, b) = (self.stack.pop(), self.stack.pop());
                let a = uint256_to_int256(u256_to_uint256(a));
                let b = uint256_to_int256(u256_to_uint256(b));
                self.stack.push(U256::from((a > b) as u64));
                self.gas_usage += 3;
            },

            opcodes::EQ => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(U256::from((a == b) as u64));
                self.gas_usage += 3;
            },

            opcodes::ISZERO => {
                let data = self.stack.pop();
                self.stack.push(U256::from(data.eq(&U256::zero()) as u64));
                self.gas_usage += 3;
            },

            opcodes::AND => {
                // debug!("AND");
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a & b);
                self.gas_usage += 3;
            },

            opcodes::OR => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a | b);
                self.gas_usage += 3;
            },

            opcodes::XOR => {
                let (a, b) = (self.stack.pop(), self.stack.pop());
                self.stack.push(a ^ b);
                self.gas_usage += 3;
            },

            opcodes::NOT => {
                let a = self.stack.pop();
                self.stack.push(!a);
                self.gas_usage += 3;
            },

            opcodes::BYTE => {
                let (i, x) = (self.stack.pop(), self.stack.pop());
                println!("i: {}, x: {}", i, x);
                if i > U256::from(31) {
                    self.stack.push(U256::zero());
                } else {
                self.stack.push((x >> (U256::from(248) - i * 8)) & (0xFF as u64).into());
                }
                self.gas_usage += 3;
            },

            opcodes::SHL => {
                let (shift, value) = (self.stack.pop(), self.stack.pop());
                if shift > 31.into() {
                    self.stack.push(U256::zero());
                } else {
                    self.stack.push(value << shift);
                }
                self.gas_usage += 3;
            },

            opcodes::SHR => {
                let (shift, value) = (self.stack.pop(), self.stack.pop());
                if shift > 31.into() {
                    self.stack.push(U256::zero());
                } else {
                    self.stack.push(value >> shift);
                }
                self.gas_usage += 3;
            },

            opcodes::SAR => {
                // TODO
                // let (shift, value) = (self.stack.pop(), self.stack.pop().as_i256());
                // self.stack.push((value >> shift).as_u256());
                // self.gas_usage += 3;
            },

            opcodes::KECCAK256 => {
                let (offset, length) = (self.stack.pop().as_usize(), self.stack.pop().as_usize());
                let bytes = self.memory.read_bytes(offset, length);
                self.stack.push(U256::from(keccak256(&bytes).as_bytes()));
                // As of the Ethereum Yellow Paper (EIP-62), the gas cost for the KECCAK256 instruction is 30 gas plus an additional 6 gas for each 256-bit word (or part thereof) of input data.
                self.gas_usage += 30 + (length.div_ceil(256) as u64 * 6);
            },

            opcodes::ADDRESS => {
                self.stack.push(self.contract_address);
                self.gas_usage += 2;
            },

            opcodes::BALANCE => {
                let address = self.stack.pop();
                self.stack.push(runtime.balance(address));
                self.gas_usage += if runtime.is_hot(address) { 100 } else { 2600 };
                runtime.mark_hot(address);
            },

            opcodes::ORIGIN => {
                self.stack.push(self.transaction.origin);
                self.gas_usage += 2;
            },

            opcodes::CALLER => {
                self.stack.push(self.message.caller);
                self.gas_usage += 2;
            },

            opcodes::CALLVALUE => {
                self.stack.push(self.message.value);
                self.gas_usage += 2;
            },

            opcodes::CALLDATALOAD => {
                // TODO fix
                let index = self.stack.pop().as_u64() as usize;
                self.stack.push(self.message.data.read(index));
                self.gas_usage += 3;
            },

            opcodes::CALLDATASIZE => {
                self.stack.push(U256::from(self.message.data.len() as u64));
                self.gas_usage += 2;
            },

            opcodes::CALLDATACOPY => {
                let (dest_offset, offset, length) = (
                    self.stack.pop().as_usize(),
                    self.stack.pop().as_usize(),
                    self.stack.pop().as_usize(),
                );
                let current_memory_usage = self.memory.memory_cost;
                    self.memory
                        .copy_from(&mut self.message.data, offset, dest_offset, length);
                    let new_usage = self.memory.memory_cost;
                    self.gas_usage +=
                        3 + 3 * (length as u64 + 31 / 32) + (new_usage - current_memory_usage).as_u64();
            },

            opcodes::CODESIZE => {
                self.stack.push(U256::from(self.program.len() as u64));
                self.gas_usage += 2;
            },

            opcodes::CODECOPY => {
                let (dest_offset, offset, length) = (
                    self.stack.pop().as_usize(),
                    self.stack.pop().as_usize(),
                    self.stack.pop().as_usize(),
                );

                let current_memory_usage = self.memory.memory_cost;
                self.memory
                    .copy_from(&mut self.program, offset, dest_offset, length);
                let new_usage = self.memory.memory_cost;
                self.gas_usage +=
                    3 + 3 * (length as u64 + 31 / 32) + (new_usage - current_memory_usage).as_u64();
            },

            opcodes::GASPRICE => {
                self.stack.push(self.transaction.gas_price);
                self.gas_usage += 2;
            },

            opcodes::EXTCODESIZE => {
                let address = self.stack.pop();
                self.stack.push(runtime.code_size(address));
                self.gas_usage += if runtime.is_hot(address) { 100 } else { 2600 };
                runtime.mark_hot(address);
            },

            opcodes::EXTCODECOPY => {
                let (addr, dest_offset, offset, length) = (
                    self.stack.pop(),
                    self.stack.pop().as_usize(),
                    self.stack.pop().as_usize(),
                    self.stack.pop().as_usize(),
                );

                let current_memory_usage = self.memory.memory_cost;
                self.memory.copy_from(
                    &mut Memory::from(runtime.code(addr)),
                    offset,
                    dest_offset,
                    length,
                );
                let new_usage = self.memory.memory_cost;
                self.gas_usage +=
                    3 * (length as u64 + 31 / 32) + (new_usage - current_memory_usage).as_u64();
                self.gas_usage += if runtime.is_hot(addr) { 100 } else { 2600 };
                runtime.mark_hot(addr);
            },

            opcodes::RETURNDATASIZE => {
                self.stack
                    .push(U256::from(self.last_return_data.len() as u64));
                self.gas_usage += 2;
            },

            opcodes::RETURNDATACOPY => {
                let (dest_offset, offset, length) = (
                    self.stack.pop().as_usize(),
                    self.stack.pop().as_usize(),
                    self.stack.pop().as_usize(),
                );
                let current_memory_usage = self.memory.memory_cost;
                self.memory
                    .copy_from(&mut self.last_return_data, offset, dest_offset, length);
                let new_usage = self.memory.memory_cost;
                self.gas_usage +=
                    3 + 3 * (length as u64 + 31 / 32) + (new_usage - current_memory_usage).as_u64();
            },

            opcodes::EXTCODEHASH => {
                let addr = self.stack.pop();
                self.stack.push(U256::from(util::keccak256_u256(addr).as_bytes()));
                self.gas_usage += if runtime.is_hot(addr) { 100 } else { 2600 };
                runtime.mark_hot(addr);
            },

            opcodes::BLOCKHASH => {
                let block_number = self.stack.pop();
                self.stack.push(h256_to_u256(runtime.block_hash(block_number)));
                self.gas_usage += 20;
            },

            opcodes::COINBASE => {
                self.stack.push(runtime.block_coinbase());
                self.gas_usage += 2;
            },

            opcodes::TIMESTAMP => {
                self.stack.push(runtime.block_timestamp());
                self.gas_usage += 2;
            },

            opcodes::NUMBER => {
                self.stack.push(runtime.block_number());
                self.gas_usage += 2;
            },

            opcodes::DIFFICULTY => {
                self.stack.push(runtime.block_difficulty());
                self.gas_usage += 2;
            },

            opcodes::GASLIMIT => {
                self.stack.push(runtime.block_gas_limit());
                self.gas_usage += 2;
            },

            opcodes::CHAINID => {
                self.stack.push(runtime.chain_id());
                self.gas_usage += 2;
            },

            opcodes::SELFBALANCE => {
                self.stack.push(runtime.balance(self.contract_address));
                self.gas_usage += 5;
            },

            opcodes::BASEFEE => {
                self.stack.push(runtime.block_base_fee_per_gas());
                self.gas_usage += 2;
            },

            opcodes::POP => {
                self.stack.pop();
                self.gas_usage += 2;
            },

            opcodes::MLOAD => {
                let offset = self.stack.pop().as_usize();
                let current_memory_usage = self.memory.memory_cost;
                self.stack.push(self.memory.read(offset));
                let new_usage = self.memory.memory_cost;
                self.gas_usage += 3 + (new_usage - current_memory_usage).as_u64();
            },

            opcodes::MSTORE => {
                let (offset, value) = (self.stack.pop().as_usize(), self.stack.pop());
                let current_memory_usage = self.memory.memory_cost;
                self.memory.write(offset, value);
                let new_usage = self.memory.memory_cost;
                self.gas_usage += 3 + (new_usage - current_memory_usage).as_u64();
            },

            opcodes::MSTORE8 => {
                let (offset, value) = (self.stack.pop().as_usize(), self.stack.pop());
                let current_memory_usage = self.memory.memory_cost;
                self.memory.write_u8(offset, (value & U256::from(0xFF as u64)).low_u32() as u8);
                let new_usage = self.memory.memory_cost;
                self.gas_usage += 3 + (new_usage - current_memory_usage).as_u64();
            },

            opcodes::SLOAD => {
                let key = self.stack.pop();
                if runtime.is_hot_index(self.contract_address, key) {
                    self.gas_usage += 100;
                } else {
                    self.gas_usage += 2100;
                    runtime.mark_hot_index(self.contract_address, key);
                }
                self.stack
                    .push(h256_to_u256(runtime.storage(self.contract_address)[&u256_to_h256(key)]));
                // self.gas_usage += if runtime.is_hot(self.contract_address) {
                //     100
                // } else {
                //     2600
                // };
                // runtime.mark_hot(self.contract_address);
            },

            opcodes::SSTORE => {
                let (key, value) = (self.stack.pop(), self.stack.pop());
                println!("key value {}, {}", key, value);
                if !runtime.is_hot_index(self.contract_address, key){
                    self.gas_usage += 2100;
                    runtime.mark_hot_index(self.contract_address, key);
                }
                let base_dynamic_gas;
                if !runtime.storage(self.contract_address).contains_key(&u256_to_h256(key)) && value.eq(&U256::zero())  {
                    println!("This case");
                            base_dynamic_gas = 100;
                    // runtime.set_storage(self.contract_address, key, u256_to_h256(value));
                }
                else {
                    base_dynamic_gas = if (runtime.storage(self.contract_address).contains_key(&u256_to_h256(key))
                        && !h256_to_u256(runtime.storage(self.contract_address)[&u256_to_h256(key)]).eq(&U256::zero())) || value.eq(&U256::zero())
                    {
                        5000
                    } else {
                        20000
                    };
                    runtime.set_storage(self.contract_address, key, u256_to_h256(value));
                }
                // TODO already written slot should always be 100
                self.gas_usage += base_dynamic_gas;
            },

            opcodes::JUMP => {

                println!("Jumping");
                let destination = self.stack.pop().as_usize();
                println!("Destination: {}", destination);
                self.program_counter = destination;
                self.program_counter -= 1;
                println!("Program Counter: {}", self.program_counter);
                self.gas_usage += 8;
            },


            opcodes::JUMPI => {
                let (destination, condition) = (self.stack.pop().as_usize(), self.stack.pop());
                if !condition.eq(&U256::zero()) {
                    self.program_counter = destination - 1;
                }
                self.gas_usage += 10;
            },

            opcodes::PC => {
                self.stack.push(U256::from(self.program_counter as u64));
                self.gas_usage += 2;
            },

            opcodes::MSIZE => {
                self.stack.push(U256::from(self.memory.max_index as u64));
                self.gas_usage += 2;
            },

            opcodes::GAS => {
                self.stack.push(U256::from(self.gas_input - self.gas_usage));
                self.gas_usage += 2;
            },

            opcodes::JUMPDEST => {
                self.gas_usage += 1;
            },

            opcodes::PUSH_1..=opcodes::PUSH_32 => {
                let push_number = opcode - opcodes::PUSH_1 + 1;
                debug!("opcodes::PUSH_{}", push_number);
                self.push_n(push_number as usize);
            },

            opcodes::DUP_1..=opcodes::DUP_16 => {
                let dup_number = opcode - opcodes::DUP_1 + 1;
                debug!("opcodes::DUP_{}", dup_number);
                self.dup_n(dup_number as usize);
            },

            opcodes::SWAP_1..=opcodes::SWAP_16 => {
                let swap_number = opcode - opcodes::SWAP_1 + 1;
                debug!("opcodes::SWAP_{}", swap_number);
                self.swap_n(swap_number as usize);
            },

            // TODO log
            opcodes::LOG_0 => {
                // TODO
            },

            opcodes::CREATE => {
                // TODO
            },

            opcodes::CALL => {
                self.make_call(runtime, false, debug);
            },

            opcodes::CALLCODE => {
                self.make_call(runtime, true, debug);
            },

            opcodes::RETURN => {
                let (offset, size) = (self.stack.pop().as_usize(), self.stack.pop().as_usize());
                println!("Return: {}, {}", offset, size);
                self.result.set_length(size);
                // self.result.copy_from(&mut self.memory, offset, 0, size);
                self.gas_usage += self.result.copy_from(&mut self.memory, offset, 0, size) as u64;
                self.stopped = true;
                return true;
            },

            opcodes::DELEGATECALL => {
                // TODO
                // Same as call but storage, sender and value remain the same
                self.gas_usage += 100;
            },

            opcodes::CREATE2 => {
                // TODO
                // Same as create but except the salt allows the new contract to be deployed at a consistent, deterministic address.
                // Should deployment succeed, the account's code is set to the return data resulting from executing the initialisation code.
            }
        });

        self.program_counter += 1;
        if self.check_gas_usage() {
            // TODO add revert logic etc.
            return false;
        }

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
    fn make_call(
        &mut self,
        runtime: &mut impl Runtime,
        maintain_storage: bool,
        debug: bool,
    ) -> bool {
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
        if !value.eq(&U256::zero()) {
            gas += 2300;
        }
        let one_64th_value = (self.gas_input - self.gas_usage) * 63 / 64;
        if gas > one_64th_value {
            gas = one_64th_value;
        }
        let address_access_cost = if runtime.is_hot(address) {
            100
        } else {
            runtime.mark_hot(address);
            2600
        };
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
                    memory.copy_from(&mut self.memory, args_offset, 0, args_size);
                    memory
                },
                value: value,
            },
            gas,
            code,
            self.transaction.clone(),
            self.gas_price,
            self.nested_index + 1,
        );
        // TODO calculate cost of call data
        let response = sub_evm.execute(runtime, debug);
        self.last_return_data = sub_evm.result;
        let current_memory_cost = self.memory.memory_cost;
        self.memory
            .copy_from(&mut self.last_return_data, 0, ret_offset, ret_size);
        let new_memory_cost = self.memory.memory_cost;
        self.stack.push(U256::from(response as u64));
        let memory_expansion_cost = (new_memory_cost - current_memory_cost).as_u64();
        let code_execution_cost = sub_evm.gas_usage;
        let positive_value_cost = if !value.eq(&U256::zero()) { 6700 } else { 0 };
        let value_to_empty_account_cost = if !value.eq(&U256::zero())
            && runtime.nonce(address).eq(&U256::zero())
            && runtime.code_size(address).eq(&U256::zero())
            && runtime.balance(address).eq(&U256::zero())
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
