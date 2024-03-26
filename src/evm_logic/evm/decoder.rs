use crate::bytecode_spec::opcodes;
use crate::evm_logic::evm::{EVMContext, Message};
use crate::runtime::Runtime;
use crate::state::memory::Memory;
use crate::util::{
    self, h256_to_u256, int256_to_uint256, keccak256, u256_to_h256, u256_to_uint256,
    uint256_to_int256, MAX_UINT256, MAX_UINT256_COMPLEMENT, ZERO,
};

use num256::Uint256;
use primitive_types::U256;
use std::ops::Rem;

#[inline]
pub fn decode_instruction(evm: &mut EVMContext, runtime: &mut impl Runtime, debug: bool) -> bool {
    /*
    Run the next instruction, adjusting gas usage and return a bool that is true if okay, false if exception
    */

    // Not a function as need to be able to return from caller function
    macro_rules! pop {
        ($($input:tt)*) => {{
            let result = evm.stack.pop();
            let result = match result {
                Err(()) => {
                    return false;
                }

                Ok(value) => value,
            };
            result
        }};
    }

    // TODO move into separate function instead of macro
    macro_rules! make_call {
        ($maintain_storage:tt) => {{
            let (mut gas, address, value, args_offset, args_size, ret_offset, ret_size) = (
                pop!().as_u64(),
                pop!(),
                pop!(),
                pop!().as_usize(),
                pop!().as_usize(),
                pop!().as_usize(),
                pop!().as_usize(),
            );
            let code: Vec<u8> = runtime.code(address);
            if !value.eq(&U256::zero()) {
                evm.gas_recorder.record_gas(2300);
            }
            let one_64th_value =
                (evm.gas_input - evm.gas_recorder.gas_usage.clone() as u64) * 63 / 64;
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
                if $maintain_storage {
                    evm.contract_address
                } else {
                    address
                },
                Message {
                    caller: evm.contract_address,
                    data: {
                        let mut memory: Memory = Memory::new();
                        memory.copy_from(
                            &mut evm.memory,
                            args_offset,
                            0,
                            args_size,
                            &mut evm.gas_recorder,
                        );
                        memory
                    },
                    value: value,
                },
                gas,
                code,
                evm.transaction.clone(),
                evm.gas_price,
                evm.nested_index + 1,
            );
            // TODO calculate cost of call data

            let response = sub_evm.execute_program(runtime, debug);
            evm.last_return_data = sub_evm.result;
            // let current_memory_cost = evm.memory.memory_cost;
            evm.memory.copy_from(
                &mut evm.last_return_data,
                0,
                ret_offset,
                ret_size,
                &mut evm.gas_recorder,
            );
            evm.stack.push(U256::from(response as u64));
            let code_execution_cost = sub_evm.gas_recorder.gas_usage;
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
            evm.gas_recorder.record_gas(
                (code_execution_cost
                    + address_access_cost
                    + positive_value_cost
                    + value_to_empty_account_cost) as usize,
            );
            response
        }};
    }

    // Provides debug data around each branches block
    macro_rules! debug_match {
        ($opcode:expr, { $( $pat:pat => $block:block ),* }) => {
            match $opcode {
                $(
                    $pat => {
                        #[allow(unreachable_code)]
                        #[allow(unused_variables)]{
                        {
                            if debug {
                                print!("{}", "\t".repeat(evm.nested_index as usize));
                                println!(
                                    "PC : {:<5} | Opcode: {:<15} | Gas: {:<10}",
                                    evm.program_counter,
                                    opcodes::OPCODE_MAP[&($opcode as u8)],
                                    format!{"{:x}",evm.gas_input - evm.gas_recorder.clone().gas_usage as u64}
                                );
                            }
                            $block
                        }
                    }
                }),*
                _ => {}
            }
        };
    }

    let opcode: u8 = evm.program[evm.program_counter];
    debug_match!(opcode, {

        opcodes::STOP => {
            evm.stopped = true;
            return true;
        },

        opcodes::ADD => {
            let (a, b) = (pop!(), pop!());
            evm.stack.push(a.overflowing_add(b).0);
            evm.gas_recorder.record_gas(3);
        },

        opcodes::MUL => {
            let (a, b) = (pop!(), pop!());
            evm.stack.push(a.overflowing_mul(b).0);
            evm.gas_recorder.record_gas(5);
        },

        opcodes::SUB => {
            let (a, b) = (pop!(), pop!());
            evm.stack.push(a.overflowing_sub(b).0);
            evm.gas_recorder.record_gas(3);
        },

        opcodes::DIV => {
            let (a, b) = (pop!(), pop!());
            match b {
                ZERO => {
                    evm.stack.push(U256::zero());
                },
                _ => {
                    evm.stack.push(a.div_mod(b).0);
                }
            }
            evm.gas_recorder.record_gas(5);
        },

        opcodes::SDIV => {
            let (a, b) = (pop!(), pop!());
            match b {
                ZERO => {
                    evm.stack.push(U256::zero());
                },
                _ => {
                    let a  = u256_to_uint256(a);
                    let b  = u256_to_uint256(b);
                    // Handle overflow case of -1 * MAX_POSITIVE
                    if a == *MAX_UINT256_COMPLEMENT && b == *MAX_UINT256 {
                        evm.stack.push(U256::from(MAX_UINT256_COMPLEMENT.to_be_bytes()));
                    }
                    else {
                        let a = uint256_to_int256(a);
                        let b = uint256_to_int256(b);
                        let result: Uint256 = int256_to_uint256(a / b);
                        let result = U256::from(result.to_be_bytes());
                        evm.stack.push(result);
                    }

                }
            }
            evm.gas_recorder.record_gas(5);
        },

        opcodes::MOD => {
            let (a, b) = (pop!(), pop!());
            match b {
                ZERO => {
                    evm.stack.push(U256::zero());
                },
                _ => {
                    evm.stack.push(a.rem(b));
                }
            }
            evm.gas_recorder.record_gas(5);
        },

        opcodes::SMOD => {
            let (a, b) = (pop!(), pop!());
            match b {
                ZERO => {
                    evm.stack.push(U256::zero());
                },
                _ => {
                    let a = uint256_to_int256(u256_to_uint256(a));
                    let b = uint256_to_int256(u256_to_uint256(b));
                    let result: Uint256 = int256_to_uint256(a.rem(b));
                    let result = U256::from(result.to_be_bytes());
                    evm.stack.push(result);
                }
            }
            evm.gas_recorder.record_gas(5);

        },

        opcodes::ADDMOD => {
            let (a, b, c) = (pop!(), pop!(), pop!());
            match c {
                ZERO => {
                    evm.stack.push(U256::zero());
                },
                _ => {
                    evm.stack.push(a.checked_rem(c).unwrap().overflowing_add(b.checked_rem(c).unwrap()).0);
                }
            }
            evm.gas_recorder.record_gas(8);
        },

        opcodes::MULMOD => {
            let (a, b, c) = (pop!(), pop!(), pop!());
            match c {
                ZERO => {
                    evm.stack.push(U256::zero());
                },
                _ => {
                    evm.stack.push(a.checked_rem(c).unwrap().overflowing_mul(b.checked_rem(c).unwrap()).0.checked_rem(c).unwrap());
                }
            }
            // println!("a: {}, b: {}, c: {}", a, b, c);
            evm.gas_recorder.record_gas(8);
        },

        opcodes::EXP => {
            let (a, exponent) = (pop!(), pop!());
            evm.stack.push(a.overflowing_pow(exponent).0);
            evm.gas_recorder.record_gas(10 + 50 * (util::bytes_for_u256(&exponent) as usize));
        },

        opcodes::SIGNEXTEND => {
            let (x, y) = (pop!(), pop!());
            // X is the number of bytes of the input lower_mask
            if x > U256::from(31) {
                evm.stack.push(y);
            } else {
                let sign = y >> (x*8 + 7) & U256::from(1 as u64);
                let lower_mask = if x == U256::from(31) {
                    U256::max_value()
                } else {
                    (U256::from(1) << ((x+1)*8)) - 1
                };
                if sign == ZERO {
                    evm.stack.push(y & lower_mask);
                } else {
                    let higher_mask = !lower_mask;
                    evm.stack.push(y | higher_mask);
                }
            }
            evm.gas_recorder.record_gas(5);
        },

        opcodes::LT => {
            let (a, b) = (pop!(), pop!());
            evm.stack.push(U256::from((a < b) as u64));
            evm.gas_recorder.record_gas(3);
        },

        opcodes::GT => {
            let (a, b) = (pop!(), pop!());
            evm.stack.push(U256::from((a > b) as u64));
            evm.gas_recorder.record_gas(3);
        },

        opcodes::SLT => {
            let (a, b) = (pop!(), pop!());
            let a = uint256_to_int256(u256_to_uint256(a));
            let b = uint256_to_int256(u256_to_uint256(b));
            evm.stack.push(U256::from((a < b) as u64));
            evm.gas_recorder.record_gas(3);
        },

        opcodes::SGT => {
            // debug!("SGT");
            let (a, b) = (pop!(), pop!());
            let a = uint256_to_int256(u256_to_uint256(a));
            let b = uint256_to_int256(u256_to_uint256(b));
            evm.stack.push(U256::from((a > b) as u64));
            evm.gas_recorder.record_gas(3);
        },

        opcodes::EQ => {
            let (a, b) = (pop!(), pop!());
            evm.stack.push(U256::from((a == b) as u64));
            evm.gas_recorder.record_gas(3);
        },

        opcodes::ISZERO => {
            let data = pop!();
            evm.stack.push(U256::from(data.eq(&U256::zero()) as u64));
            evm.gas_recorder.record_gas(3);
        },

        opcodes::AND => {
            // debug!("AND");
            let (a, b) = (pop!(), pop!());
            evm.stack.push(a & b);
            evm.gas_recorder.record_gas(3);
        },

        opcodes::OR => {
            let (a, b) = (pop!(), pop!());
            evm.stack.push(a | b);
            evm.gas_recorder.record_gas(3);
        },

        opcodes::XOR => {
            let (a, b) = (pop!(), pop!());
            evm.stack.push(a ^ b);
            evm.gas_recorder.record_gas(3);
        },

        opcodes::NOT => {
            let a = pop!();
            evm.stack.push(!a);
            evm.gas_recorder.record_gas(3);
        },

        opcodes::BYTE => {
            let (i, x) = (pop!(), pop!());
            if i > U256::from(31) {
                evm.stack.push(U256::zero());
            } else {
            evm.stack.push((x >> (U256::from(248) - i * 8)) & (0xFF as u64).into());
            }
            evm.gas_recorder.record_gas(3);
        },

        opcodes::SHL => {
            let (shift, value) = (pop!(), pop!());
            if shift > 31.into() {
                evm.stack.push(U256::zero());
            } else {
                evm.stack.push(value << shift);
            }
            evm.gas_recorder.record_gas(3);
        },

        opcodes::SHR => {
            let (shift, value) = (pop!(), pop!());
            if shift > 31.into() {
                evm.stack.push(U256::zero());
            } else {
                evm.stack.push(value >> shift);
            }
            evm.gas_recorder.record_gas(3);
        },

        opcodes::SAR => {
            // TODO
            // let (shift, value) = (pop!(), pop!().as_i256());
            // evm.stack.push((value >> shift).as_u256());
            // evm.gas_usage += 3;
        },

        opcodes::KECCAK256 => {
            let (offset, length) = (pop!().as_usize(), pop!().as_usize());
            let bytes = evm.memory.read_bytes(offset, length);
            evm.stack.push(U256::from(keccak256(&bytes).as_bytes()));
            // As of the Ethereum Yellow Paper (EIP-62), the gas cost for the KECCAK256 instruction is 30 gas plus an additional 6 gas for each 256-bit word (or part thereof) of input data.
            evm.gas_recorder.record_gas(30 + (length.div_ceil(256) as u64 * 6) as usize);
        },

        opcodes::ADDRESS => {
            evm.stack.push(evm.contract_address);
            evm.gas_recorder.record_gas(2);
        },

        opcodes::BALANCE => {
            let address = pop!();
            evm.stack.push(runtime.balance(address));
            if runtime.is_hot(address) { evm.gas_recorder.record_gas(100); } else { evm.gas_recorder.record_gas(2600); };
            runtime.mark_hot(address);
        },

        opcodes::ORIGIN => {
            evm.stack.push(evm.transaction.origin);
            evm.gas_recorder.record_gas(2);
        },

        opcodes::CALLER => {
            evm.stack.push(evm.message.caller);
            evm.gas_recorder.record_gas(2);
        },

        opcodes::CALLVALUE => {
            evm.stack.push(evm.message.value);
            evm.gas_recorder.record_gas(2);
        },

        opcodes::CALLDATALOAD => {
            // TODO fix
            let index = pop!().as_u64() as usize;
            evm.stack.push(evm.message.data.read(index));
            evm.gas_recorder.record_gas(3);
        },

        opcodes::CALLDATASIZE => {
            evm.stack.push(U256::from(evm.message.data.len() as u64));
            evm.gas_recorder.record_gas(2);
        },

        opcodes::CALLDATACOPY => {
            let (dest_offset, offset, length) = (
                pop!().as_usize(),
                pop!().as_usize(),
                pop!().as_usize(),
            );
            // let current_memory_usage = evm.memory.memory_cost;
                evm.memory
                    .copy_from(&mut evm.message.data, offset, dest_offset, length, &mut evm.gas_recorder);
                // let new_usage = evm.memory.memory_cost;
                // evm.gas_usage +=
                //     3 + 3 * (length as u64 + 31 / 32) + (new_usage - current_memory_usage).as_u64();
        },

        opcodes::CODESIZE => {
            evm.stack.push(U256::from(evm.program.len() as u64));
            evm.gas_recorder.record_gas(2);
        },

        opcodes::CODECOPY => {
            let (dest_offset, offset, length) = (
                pop!().as_usize(),
                pop!().as_usize(),
                pop!().as_usize(),
            );

            evm.memory
                .copy_from(&mut evm.program, offset, dest_offset, length, &mut evm.gas_recorder);
        },

        opcodes::GASPRICE => {
            evm.stack.push(evm.transaction.gas_price);
            evm.gas_recorder.record_gas(2);
        },

        opcodes::EXTCODESIZE => {
            let address = pop!();
            evm.stack.push(runtime.code_size(address));
            if runtime.is_hot(address) { evm.gas_recorder.record_gas(100); } else { evm.gas_recorder.record_gas(2600); };
            runtime.mark_hot(address);
        },

        opcodes::EXTCODECOPY => {
            let (addr, dest_offset, offset, length) = (
                pop!(),
                pop!().as_usize(),
                pop!().as_usize(),
                pop!().as_usize(),
            );

            evm.memory.copy_from(
                &mut Memory::from(runtime.code(addr), &mut evm.gas_recorder),
                offset,
                dest_offset,
                length,
                &mut evm.gas_recorder
            );
            runtime.mark_hot(addr);
        },

        opcodes::RETURNDATASIZE => {
            evm.stack
                .push(U256::from(evm.last_return_data.len() as u64));
            evm.gas_recorder.record_gas(2);
        },

        opcodes::RETURNDATACOPY => {
            let (dest_offset, offset, length) = (
                pop!().as_usize(),
                pop!().as_usize(),
                pop!().as_usize(),
            );
            evm.memory
                .copy_from(&mut evm.last_return_data, offset, dest_offset, length, &mut evm.gas_recorder);
        },

        opcodes::EXTCODEHASH => {
            let addr = pop!();
            evm.stack.push(U256::from(util::keccak256_u256(addr).as_bytes()));
            if runtime.is_hot(addr) { evm.gas_recorder.record_gas(100); } else { evm.gas_recorder.record_gas(2600); };
            runtime.mark_hot(addr);
        },

        opcodes::BLOCKHASH => {
            let block_number = pop!();
            evm.stack.push(h256_to_u256(runtime.block_hash(block_number)));
            evm.gas_recorder.record_gas(20);
        },

        opcodes::COINBASE => {
            evm.stack.push(runtime.block_coinbase());
            evm.gas_recorder.record_gas(2);
        },

        opcodes::TIMESTAMP => {
            evm.stack.push(runtime.block_timestamp());
            evm.gas_recorder.record_gas(2);
        },

        opcodes::NUMBER => {
            evm.stack.push(runtime.block_number());
            evm.gas_recorder.record_gas(2);
        },

        opcodes::DIFFICULTY => {
            evm.stack.push(runtime.block_difficulty());
            evm.gas_recorder.record_gas(2);
        },

        opcodes::GASLIMIT => {
            evm.stack.push(runtime.block_gas_limit());
            evm.gas_recorder.record_gas(2);
        },

        opcodes::CHAINID => {
            evm.stack.push(runtime.chain_id());
            evm.gas_recorder.record_gas(2);
        },

        opcodes::SELFBALANCE => {
            evm.stack.push(runtime.balance(evm.contract_address));
            evm.gas_recorder.record_gas(5);
        },

        opcodes::BASEFEE => {
            evm.stack.push(runtime.block_base_fee_per_gas());
            evm.gas_recorder.record_gas(2);
        },

        opcodes::POP => {
            pop!();
            evm.gas_recorder.record_gas(2);
        },

        opcodes::MLOAD => {
            let offset = pop!().as_usize();
            evm.stack.push(evm.memory.read(offset));
            evm.gas_recorder.record_gas(3);
        },

        opcodes::MSTORE => {
            let (offset, value) = (pop!().as_usize(), pop!());
            evm.memory.write(offset, value, &mut evm.gas_recorder);
            evm.gas_recorder.record_gas(3);
        },

        opcodes::MSTORE8 => {
            let (offset, value) = (pop!().as_usize(), pop!());
            evm.memory.write_u8(offset, (value & U256::from(0xFF as u64)).low_u32() as u8, &mut evm.gas_recorder);
            evm.gas_recorder.record_gas(3);
        },

        opcodes::SLOAD => {
            let key = pop!();
            if runtime.is_hot_index(evm.contract_address, key) {
                evm.gas_recorder.record_gas(100);
            } else {
                evm.gas_recorder.record_gas(2100);
                runtime.mark_hot_index(evm.contract_address, key);
            }
            evm.stack
                .push(h256_to_u256(runtime.storage(evm.contract_address)[&u256_to_h256(key)]));
            // evm.gas_usage += if runtime.is_hot(evm.contract_address) {
            //     100
            // } else {
            //     2600
            // };
            // runtime.mark_hot(evm.contract_address);
        },

        opcodes::SSTORE => {
            let (key, value) = (pop!(), pop!());
            if !runtime.is_hot_index(evm.contract_address, key){
                evm.gas_recorder.record_gas(2100);
                runtime.mark_hot_index(evm.contract_address, key);
            }
            let base_dynamic_gas;
            if !runtime.storage(evm.contract_address).contains_key(&u256_to_h256(key)) && value.eq(&U256::zero())  {
                base_dynamic_gas = 100;
            }
            else {
                base_dynamic_gas = if (runtime.storage(evm.contract_address).contains_key(&u256_to_h256(key))
                    && !h256_to_u256(runtime.storage(evm.contract_address)[&u256_to_h256(key)]).eq(&U256::zero())) || value.eq(&U256::zero())
                {
                    5000
                } else {
                    20000
                };
                runtime.set_storage(evm.contract_address, key, u256_to_h256(value));
            }
            // TODO already written slot should always be 100
            evm.gas_recorder.record_gas(base_dynamic_gas);
        },

        opcodes::JUMP => {
            let destination = pop!().as_usize();
            evm.program_counter = destination;
            evm.program_counter -= 1;
            evm.gas_recorder.record_gas(8);
        },


        opcodes::JUMPI => {
            let (destination, condition) = (pop!().as_usize(), pop!());
            if !condition.eq(&U256::zero()) {
                evm.program_counter = destination - 1;
            }
            evm.gas_recorder.record_gas(10);
        },

        opcodes::PC => {
            evm.stack.push(U256::from(evm.program_counter as u64));
            evm.gas_recorder.record_gas(2);
        },

        opcodes::MSIZE => {
            evm.stack.push(U256::from((evm.memory.max_index + 1) as u64));
            evm.gas_recorder.record_gas(2);
        },

        opcodes::GAS => {
            evm.stack.push(U256::from(evm.gas_input - evm.gas_recorder.gas_usage as u64));
            evm.gas_recorder.record_gas(2);
        },

        opcodes::JUMPDEST => {
            evm.gas_recorder.record_gas(1);
        },

        opcodes::PUSH_1..=opcodes::PUSH_32 => {
            let push_number = opcode - opcodes::PUSH_1 + 1;
            let bytes = evm
                .program
                .read_bytes(evm.program_counter + 1, push_number as usize);
            evm.program_counter += push_number as usize;
            evm.stack.push_bytes(&bytes);
            evm.gas_recorder.record_gas(3);
        },

        opcodes::DUP_1..=opcodes::DUP_16 => {
            let dup_number = opcode - opcodes::DUP_1 + 1;
            let value = evm.stack.read_nth(dup_number as usize);
            evm.stack.push(value);
            evm.gas_recorder.record_gas(3);
        },

        opcodes::SWAP_1..=opcodes::SWAP_16 => {
            let swap_number: usize = (opcode - opcodes::SWAP_1 + 1) as usize;
            let bottom_value = evm.stack.read_nth(swap_number);
            let top_value = pop!();
            evm.stack.write_nth(swap_number - 1, top_value);
            evm.stack.push(bottom_value);
            evm.gas_recorder.record_gas(3);
        },

        // TODO log
        opcodes::LOG_0 => {
            // TODO
        },

        opcodes::CREATE => {
            // TODO
        },

        opcodes::CALL => {
            make_call!(false);
        },

        opcodes::CALLCODE => {
            make_call!(true);
        },

        opcodes::RETURN => {
            let (offset, size) = (pop!().as_usize(), pop!().as_usize());
            evm.result.set_length(size);
            evm.result.copy_from(&mut evm.memory, offset, 0, size, &mut evm.gas_recorder);
            evm.stopped = true;
            return true;
        },

        opcodes::DELEGATECALL => {
            // TODO
            // Same as call but storage, sender and value remain the same
            evm.gas_recorder.record_gas(100);
        },

        opcodes::CREATE2 => {
            // TODO
            // Same as create but except the salt allows the new contract to be deployed at a consistent, deterministic address.
            // Should deployment succeed, the account's code is set to the return data resulting from executing the initialisation code.
        }
    });

    evm.program_counter += 1;

    if evm.check_gas_usage() {
        // TODO add revert logic etc.
        return false;
    }

    true
}
