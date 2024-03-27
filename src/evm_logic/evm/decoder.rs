use crate::configs::bytecode_spec::opcodes;
use crate::configs::gas_costs::{static_costs, DynamicCosts};
use crate::evm_logic::evm::call::make_call;
use crate::evm_logic::evm::macros::{debug_match, pop, return_if_error};
use crate::evm_logic::evm::EVMContext;
use crate::evm_logic::util::{
    self, h256_to_u256, int256_to_uint256, keccak256, u256_to_h256, u256_to_uint256,
    uint256_to_int256, MAX_UINT256, MAX_UINT256_COMPLEMENT, ZERO, ZERO_H256,
};
use crate::result::{ExecutionResult, Error};
use crate::runtime::Runtime;
use crate::state::memory::Memory;

use num256::Uint256;
use primitive_types::U256;
use std::ops::Rem;

#[inline]
pub fn decode_instruction(evm: &mut EVMContext, runtime: &mut impl Runtime, debug: bool) -> ExecutionResult {
    /*
    Run the next instruction, adjusting gas usage and return a bool that is true if okay, false if exception
    */
    // Should be put into a separate macros file
    // Not a function as need to be able to return from caller function

    // Provides debug data around each branches block

    let opcode: u8 = evm.program[evm.program_counter];
    debug_match!(evm, debug, opcode, {

        opcodes::STOP => {
            println!("STOP");
            evm.stopped = true;
            return ExecutionResult::Success;
        },

        opcodes::ADD => {
            let (a, b) = (pop!(evm), pop!(evm));
            evm.stack.push(a.overflowing_add(b).0);
            evm.gas_recorder.record_gas(static_costs::G_VERY_LOW);
            // evm.gas_recorder.record_gas(3);
        },

        opcodes::MUL => {
            let (a, b) = (pop!(evm), pop!(evm));
            evm.stack.push(a.overflowing_mul(b).0);
            evm.gas_recorder.record_gas(static_costs::G_LOW);
        },

        opcodes::SUB => {
            let (a, b) = (pop!(evm), pop!(evm));
            evm.stack.push(a.overflowing_sub(b).0);
            evm.gas_recorder.record_gas(static_costs::G_VERY_LOW);
        },

        opcodes::DIV => {
            let (a, b) = (pop!(evm), pop!(evm));
            match b {
                ZERO => {
                    evm.stack.push(U256::zero());
                },
                _ => {
                    evm.stack.push(a.div_mod(b).0);
                }
            }
            evm.gas_recorder.record_gas(static_costs::G_LOW);
        },

        opcodes::SDIV => {
            let (a, b) = (pop!(evm), pop!(evm));
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
            evm.gas_recorder.record_gas(static_costs::G_LOW);
        },

        opcodes::MOD => {
            let (a, b) = (pop!(evm), pop!(evm));
            match b {
                ZERO => {
                    evm.stack.push(U256::zero());
                },
                _ => {
                    evm.stack.push(a.rem(b));
                }
            }
            evm.gas_recorder.record_gas(static_costs::G_LOW);
        },

        opcodes::SMOD => {
            let (a, b) = (pop!(evm), pop!(evm));
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
            evm.gas_recorder.record_gas(static_costs::G_LOW);

        },

        opcodes::ADDMOD => {
            let (a, b, c) = (pop!(evm), pop!(evm), pop!(evm));
            match c {
                ZERO => {
                    evm.stack.push(U256::zero());
                },
                _ => {
                    evm.stack.push(a.checked_rem(c).unwrap().overflowing_add(b.checked_rem(c).unwrap()).0);
                }
            }
            evm.gas_recorder.record_gas(static_costs::G_MID);
        },

        opcodes::MULMOD => {
            let (a, b, c) = (pop!(evm), pop!(evm), pop!(evm));
            match c {
                ZERO => {
                    evm.stack.push(U256::zero());
                },
                _ => {
                    evm.stack.push(a.checked_rem(c).unwrap().overflowing_mul(b.checked_rem(c).unwrap()).0.checked_rem(c).unwrap());
                }
            }
            evm.gas_recorder.record_gas(static_costs::G_MID);
        },

        opcodes::EXP => {
            let (a, exponent) = (pop!(evm), pop!(evm));
            evm.stack.push(a.overflowing_pow(exponent).0);
            evm.gas_recorder.record_gas(DynamicCosts::Exp { power: exponent }.cost());
            // evm.gas_recorder.record_gas(10 + 50 * (util::bytes_for_u256(&exponent) as u64));
        },

        opcodes::SIGNEXTEND => {
            let (x, y) = (pop!(evm), pop!(evm));
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
            evm.gas_recorder.record_gas(static_costs::G_LOW);
        },

        opcodes::LT => {
            let (a, b) = (pop!(evm), pop!(evm));
            evm.stack.push(U256::from((a < b) as u64));
            evm.gas_recorder.record_gas(static_costs::G_VERY_LOW);
        },

        opcodes::GT => {
            let (a, b) = (pop!(evm), pop!(evm));
            evm.stack.push(U256::from((a > b) as u64));
            evm.gas_recorder.record_gas(static_costs::G_VERY_LOW);
        },

        opcodes::SLT => {
            let (a, b) = (pop!(evm), pop!(evm));
            let a = uint256_to_int256(u256_to_uint256(a));
            let b = uint256_to_int256(u256_to_uint256(b));
            evm.stack.push(U256::from((a < b) as u64));
            evm.gas_recorder.record_gas(static_costs::G_VERY_LOW);
        },

        opcodes::SGT => {
            // debug!("SGT");
            let (a, b) = (pop!(evm), pop!(evm));
            let a = uint256_to_int256(u256_to_uint256(a));
            let b = uint256_to_int256(u256_to_uint256(b));
            evm.stack.push(U256::from((a > b) as u64));
            evm.gas_recorder.record_gas(static_costs::G_VERY_LOW);
        },

        opcodes::EQ => {
            let (a, b) = (pop!(evm), pop!(evm));
            evm.stack.push(U256::from((a == b) as u64));
            evm.gas_recorder.record_gas(static_costs::G_VERY_LOW);
        },

        opcodes::ISZERO => {
            let data = pop!(evm);
            evm.stack.push(U256::from(data.eq(&U256::zero()) as u64));
            evm.gas_recorder.record_gas(static_costs::G_VERY_LOW);
        },

        opcodes::AND => {
            // debug!("AND");
            let (a, b) = (pop!(evm), pop!(evm));
            evm.stack.push(a & b);
            evm.gas_recorder.record_gas(static_costs::G_VERY_LOW);
        },

        opcodes::OR => {
            let (a, b) = (pop!(evm), pop!(evm));
            evm.stack.push(a | b);
            evm.gas_recorder.record_gas(static_costs::G_VERY_LOW);
        },

        opcodes::XOR => {
            let (a, b) = (pop!(evm), pop!(evm));
            evm.stack.push(a ^ b);
            evm.gas_recorder.record_gas(static_costs::G_VERY_LOW);
        },

        opcodes::NOT => {
            let a = pop!(evm);
            evm.stack.push(!a);
            evm.gas_recorder.record_gas(static_costs::G_VERY_LOW);
        },

        opcodes::BYTE => {
            let (i, x) = (pop!(evm), pop!(evm));
            if i > U256::from(31) {
                evm.stack.push(U256::zero());
            } else {
            evm.stack.push((x >> (U256::from(248) - i * 8)) & (0xFF as u64).into());
            }
            evm.gas_recorder.record_gas(static_costs::G_VERY_LOW);
        },

        opcodes::SHL => {
            let (shift, value) = (pop!(evm), pop!(evm));
            if shift > 31.into() {
                evm.stack.push(U256::zero());
            } else {
                evm.stack.push(value << shift);
            }
            evm.gas_recorder.record_gas(static_costs::G_VERY_LOW);
        },

        opcodes::SHR => {
            let (shift, value) = (pop!(evm), pop!(evm));
            if shift > 31.into() {
                evm.stack.push(U256::zero());
            } else {
                evm.stack.push(value >> shift);
            }
            evm.gas_recorder.record_gas(static_costs::G_VERY_LOW);
        },

        opcodes::SAR => {
            // TODO
            // let (shift, value) = (pop!(evm), pop!(evm).as_i256());
            // evm.stack.push((value >> shift).as_u256());
            // evm.gas_usage += 3;
        },

        opcodes::KECCAK256 => {
            let (offset, length) = (pop!(evm).as_usize(), pop!(evm).as_usize());
            println!("offset: {}, length: {}", offset, length);
            let bytes = evm.memory.read_bytes(offset, length, &mut evm.gas_recorder);
            evm.stack.push(U256::from(keccak256(&bytes).as_bytes()));
            // As of the Ethereum Yellow Paper (EIP-62), the gas cost for the KECCAK256 instruction is 30 gas plus an additional 6 gas for each 256-bit word (or part thereof) of input data.

            evm.gas_recorder.record_gas(DynamicCosts::Keccak256 { len: length as u64}.cost());
            // evm.gas_recorder.record_gas(30 + (length.div_ceil(32) as u64 * 6) as u64);
        },

        opcodes::ADDRESS => {
            evm.stack.push(evm.contract_address);
            evm.gas_recorder.record_gas(2);
        },

        opcodes::BALANCE => {
            let address = pop!(evm);
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
            let index = pop!(evm).as_u64() as usize;
            if index > evm.message.data.len() - 32 {
                evm.stack.push_bytes(
                    &{
                        let mut bytes = evm.message.data.clone();
                        bytes.append(&mut vec![0; 32 - (evm.message.data.len() - index)]);
                        bytes
                    }
                );
            } else {
                evm.stack.push_bytes(&evm.message.data[index..index + 32].to_vec());
            }
            evm.gas_recorder.record_gas(3);
        },

        opcodes::CALLDATASIZE => {
            evm.stack.push(U256::from(evm.message.data.len() as u64));
            evm.gas_recorder.record_gas(2);
        },

        opcodes::CALLDATACOPY => {
            // TODO fix
            let (dest_offset, offset, length) = (
                pop!(evm).as_usize(),
                pop!(evm).as_usize(),
                pop!(evm).as_usize(),
            );
        },

        opcodes::CODESIZE => {
            evm.stack.push(U256::from(evm.program.len() as u64));
            evm.gas_recorder.record_gas(2);
        },

        opcodes::CODECOPY => {
            let (dest_offset, offset, length) = (
                pop!(evm).as_usize(),
                pop!(evm).as_usize(),
                pop!(evm).as_usize(),
            );

            evm.memory
                .copy_from(&mut evm.program, offset, dest_offset, length, &mut evm.gas_recorder);
        },

        opcodes::GASPRICE => {
            evm.stack.push(evm.transaction.gas_price);
            evm.gas_recorder.record_gas(2);
        },

        opcodes::EXTCODESIZE => {
            let address = pop!(evm);
            evm.stack.push(runtime.code_size(address));
            if runtime.is_hot(address) { evm.gas_recorder.record_gas(100); } else { evm.gas_recorder.record_gas(2600); };
            runtime.mark_hot(address);
        },

        opcodes::EXTCODECOPY => {
            let (addr, dest_offset, offset, length) = (
                pop!(evm),
                pop!(evm).as_usize(),
                pop!(evm).as_usize(),
                pop!(evm).as_usize(),
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
                pop!(evm).as_usize(),
                pop!(evm).as_usize(),
                pop!(evm).as_usize(),
            );
            evm.memory
                .copy_from(&mut evm.last_return_data, offset, dest_offset, length, &mut evm.gas_recorder);
        },

        opcodes::EXTCODEHASH => {
            let addr = pop!(evm);
            evm.stack.push(U256::from(util::keccak256_u256(addr).as_bytes()));
            if runtime.is_hot(addr) { evm.gas_recorder.record_gas(100); } else { evm.gas_recorder.record_gas(2600); };
            runtime.mark_hot(addr);
        },

        opcodes::BLOCKHASH => {
            let block_number = pop!(evm);
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
            pop!(evm);
            evm.gas_recorder.record_gas(2);
        },

        opcodes::MLOAD => {
            let offset = pop!(evm).as_usize();
            evm.stack.push(evm.memory.read(offset, &mut evm.gas_recorder));
            evm.gas_recorder.record_gas(3);
        },

        opcodes::MSTORE => {
            let (offset, value) = (pop!(evm).as_usize(), pop!(evm));
            evm.memory.write(offset, value, &mut evm.gas_recorder);
            evm.gas_recorder.record_gas(3);
        },

        opcodes::MSTORE8 => {
            let (offset, value) = (pop!(evm).as_usize(), pop!(evm));
            evm.memory.write_u8(offset, (value & U256::from(0xFF as u64)).low_u32() as u8, &mut evm.gas_recorder);
            evm.gas_recorder.record_gas(3);
        },

        opcodes::SLOAD => {
            let key = pop!(evm);
            if runtime.is_hot_index(evm.contract_address, key) {
                evm.gas_recorder.record_gas(100);
            } else {
                evm.gas_recorder.record_gas(2100);
                runtime.mark_hot_index(evm.contract_address, key);
            }
            evm.stack
                .push(h256_to_u256(runtime.read_storage(evm.contract_address, key)));
        },

        opcodes::SSTORE => {
            let (key, value) = (pop!(evm), pop!(evm));
            if !runtime.is_hot_index(evm.contract_address, key){
                evm.gas_recorder.record_gas(2100);
                runtime.mark_hot_index(evm.contract_address, key);
            }
            let (v_org, v_cur, v_new) = (
                runtime.read_original_storage(evm.contract_address,key),
                runtime.read_storage(evm.contract_address,key),
                u256_to_h256(value),
            );
            let dynamic_gas = if v_cur.eq(&v_new) | !v_org.eq(&v_cur) {
                 100
            } else if v_org.eq(&ZERO_H256) {
                 20000
            } else {
                2900
            };
            let refund = if !v_org.eq(&ZERO_H256) && v_new.eq(&ZERO_H256) {
                15000
            } else {
                0
            };
            runtime.set_storage(evm.contract_address, key, u256_to_h256(value));
            evm.gas_recorder.record_gas(dynamic_gas);
            evm.gas_recorder.subtract_gas(refund);
        },

        opcodes::JUMP => {
            let destination = pop!(evm).as_usize();
            evm.program_counter = destination;
            evm.program_counter -= 1;
            evm.gas_recorder.record_gas(8);
        },

        opcodes::JUMPI => {
            let (destination, condition) = (pop!(evm).as_usize(), pop!(evm));
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
            // Would technically be slightly faster without this (branch for each case) but probably a negligible difference
            let push_number = opcode - opcodes::PUSH_1 + 1;
            let bytes = evm
                .program
                .read_bytes(evm.program_counter + 1, push_number as usize, &mut evm.gas_recorder);
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
            let top_value = pop!(evm);
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
            return_if_error!(make_call(evm, runtime, debug, false));
        },

        opcodes::CALLCODE => {
            return_if_error!(make_call(evm, runtime, debug, true));
        },

        opcodes::RETURN => {
            let (offset, size) = (pop!(evm).as_usize(), pop!(evm).as_usize());
            evm.result.set_length(size);
            evm.result.copy_from(&mut evm.memory, offset, 0, size, &mut evm.gas_recorder);
            evm.stopped = true;
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

    // return_if_error!(evm.program_counter > 1000000 || evm.nested_index > 1024);
    return_if_error!(evm.check_gas_usage());
    evm.program_counter += 1;
    return ExecutionResult::Success;
}
