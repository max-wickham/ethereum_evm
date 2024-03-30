use crate::configs::bytecode_spec::opcodes;
use crate::configs::gas_costs::{static_costs, DynamicCosts};
use crate::evm_logic::evm::call::{call, call_code, delegate_call, static_call};
use crate::evm_logic::evm::create::{create_1, create_2};
use crate::evm_logic::evm::macros::{
    pop, pop_u64, pop_usize, push, return_error_if_static, return_if_error,
    return_if_error_in_tuple, return_if_gas_too_high,
};
use crate::evm_logic::evm::EVMContext;
use crate::evm_logic::state::memory::Memory;
use crate::evm_logic::util::{
    self, h256_to_u256, int256_to_uint256, keccak256, u256_to_h256, u256_to_uint256,
    uint256_to_int256, MAX_UINT256, MAX_UINT256_COMPLEMENT, ZERO, ZERO_H256,
};
use crate::result::{ExecutionError, ExecutionResult, ExecutionSuccess};
use crate::runtime::Runtime;

use num256::Uint256;
use primitive_types::U256;
use std::ops::{Not, Rem, Shl, Shr};

#[inline]
pub fn decode_instruction(
    evm: &mut EVMContext,
    runtime: &mut impl Runtime,
    debug: bool,
) -> ExecutionResult {
    /*
    Run the next instruction, adjusting gas usage and return a bool that is true if okay, false if exception
    */
    // Should be put into a separate macros file
    // Not a function as need to be able to return from caller function

    // Provides debug data around each branches block
    if evm.program_counter > evm.program.len() - 1 {
        return ExecutionResult::Success(ExecutionSuccess::Stop);
    }

    let opcode: u8 = evm.program[evm.program_counter];
    // This macro adds print code before an after every block
    if debug {
        print!("{}", "\t".repeat(evm.nested_index as usize));
        println!(
            "PC : {:<5} | Opcode: {:<15} | Gas: {:<10}",
            evm.program_counter,
            opcodes::OPCODE_MAP[&(opcode as u8)],
            format! {"{:x}",evm.gas_input as u64 - evm.gas_recorder.clone().gas_usage as u64}
        );
    }
    match opcode {
        opcodes::STOP => return ExecutionResult::Success(ExecutionSuccess::Stop),

        opcodes::ADD => {
            let (a, b) = (pop!(evm), pop!(evm));
            push!(evm, a.overflowing_add(b).0);
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::MUL => {
            let (a, b) = (pop!(evm), pop!(evm));
            push!(evm, a.overflowing_mul(b).0);
            evm.gas_recorder.record_gas_usage(static_costs::G_LOW);
        }

        opcodes::SUB => {
            let (a, b) = (pop!(evm), pop!(evm));
            push!(evm, a.overflowing_sub(b).0);
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::DIV => {
            let (a, b) = (pop!(evm), pop!(evm));
            match b {
                ZERO => {
                    push!(evm, U256::zero());
                }
                _ => {
                    push!(evm, a.div_mod(b).0);
                }
            }
            evm.gas_recorder.record_gas_usage(static_costs::G_LOW);
        }

        opcodes::SDIV => {
            let (a, b) = (pop!(evm), pop!(evm));
            match b {
                ZERO => {
                    push!(evm, U256::zero());
                }
                _ => {
                    let a = u256_to_uint256(a);
                    let b = u256_to_uint256(b);
                    // Handle overflow case of -1 * MAX_POSITIVE
                    if a == *MAX_UINT256_COMPLEMENT && b == *MAX_UINT256 {
                        push!(evm, U256::from(MAX_UINT256_COMPLEMENT.to_be_bytes()));
                    } else {
                        let a = uint256_to_int256(a);
                        let b = uint256_to_int256(b);
                        let result: Uint256 = int256_to_uint256(a / b);
                        let result = U256::from(result.to_be_bytes());
                        push!(evm, result);
                    }
                }
            }
            evm.gas_recorder.record_gas_usage(static_costs::G_LOW);
        }

        opcodes::MOD => {
            let (a, b) = (pop!(evm), pop!(evm));
            match b {
                ZERO => {
                    push!(evm, U256::zero());
                }
                _ => {
                    push!(evm, a.rem(b));
                }
            }
            evm.gas_recorder.record_gas_usage(static_costs::G_LOW);
        }

        opcodes::SMOD => {
            let (a, b) = (pop!(evm), pop!(evm));
            match b {
                ZERO => {
                    push!(evm, U256::zero());
                }
                _ => {
                    let a = uint256_to_int256(u256_to_uint256(a));
                    let b = uint256_to_int256(u256_to_uint256(b));
                    let result: Uint256 = int256_to_uint256(a.rem(b));
                    let result = U256::from(result.to_be_bytes());
                    push!(evm, result);
                }
            }
            evm.gas_recorder.record_gas_usage(static_costs::G_LOW);
        }

        opcodes::ADDMOD => {
            let (a, b, c) = (pop!(evm), pop!(evm), pop!(evm));
            println!("a: {:?}, b: {:?}, c: {:?}", a, b, c);
            match c {
                ZERO => {
                    push!(evm, U256::zero());
                }
                _ => {
                    let result = a.checked_rem(c)
                    .unwrap()
                    .overflowing_add(b.checked_rem(c).unwrap())
                    .0;
                    println!("result: {:?}", result);
                    push!(
                        evm,
                        a.checked_rem(c)
                            .unwrap()
                            .overflowing_add(b.checked_rem(c).unwrap())
                            .0.checked_rem(c).unwrap()
                    );
                }
            }
            evm.gas_recorder.record_gas_usage(static_costs::G_MID);
        }

        opcodes::MULMOD => {
            let (a, b, c) = (pop!(evm), pop!(evm), pop!(evm));
            match c {
                ZERO => {
                    push!(evm, U256::zero());
                }
                _ => {
                    push!(
                        evm,
                        a.checked_rem(c)
                            .unwrap()
                            .overflowing_mul(b.checked_rem(c).unwrap())
                            .0
                            .checked_rem(c)
                            .unwrap()
                    );
                }
            }
            evm.gas_recorder.record_gas_usage(static_costs::G_MID);
        }

        opcodes::EXP => {
            let (a, exponent) = (pop!(evm), pop!(evm));
            push!(evm, a.overflowing_pow(exponent).0);
            evm.gas_recorder
                .record_gas_usage(DynamicCosts::Exp { power: exponent }.cost());
        }

        opcodes::SIGNEXTEND => {
            let (x, y) = (pop!(evm), pop!(evm));
            // X is the number of bytes of the input lower_mask
            if x > U256::from(31) {
                push!(evm, y);
            } else {
                let sign = y >> (x * 8 + 7) & U256::from(1 as u64);
                let lower_mask = if x == U256::from(31) {
                    U256::max_value()
                } else {
                    (U256::from(1) << ((x + 1) * 8)) - 1
                };
                if sign == ZERO {
                    push!(evm, y & lower_mask);
                } else {
                    let higher_mask = !lower_mask;
                    push!(evm, y | higher_mask);
                }
            }
            evm.gas_recorder.record_gas_usage(static_costs::G_LOW);
        }

        opcodes::LT => {
            let (a, b) = (pop!(evm), pop!(evm));
            push!(evm, U256::from((a < b) as u64));
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::GT => {
            let (a, b) = (pop!(evm), pop!(evm));
            push!(evm, U256::from((a > b) as u64));
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::SLT => {
            let (a, b) = (pop!(evm), pop!(evm));
            let a = uint256_to_int256(u256_to_uint256(a));
            let b = uint256_to_int256(u256_to_uint256(b));
            push!(evm, U256::from((a < b) as u64));
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::SGT => {
            // debug!("SGT");
            let (a, b) = (pop!(evm), pop!(evm));
            let a = uint256_to_int256(u256_to_uint256(a));
            let b = uint256_to_int256(u256_to_uint256(b));
            push!(evm, U256::from((a > b) as u64));
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::EQ => {
            let (a, b) = (pop!(evm), pop!(evm));
            push!(evm, U256::from((a == b) as u64));
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::ISZERO => {
            let data = pop!(evm);
            push!(evm, U256::from(data.eq(&U256::zero()) as u64));
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::AND => {
            let (a, b) = (pop!(evm), pop!(evm));
            push!(evm, a & b);
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::OR => {
            let (a, b) = (pop!(evm), pop!(evm));
            push!(evm, a | b);
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::XOR => {
            let (a, b) = (pop!(evm), pop!(evm));
            push!(evm, a ^ b);
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::NOT => {
            let a = pop!(evm);
            push!(evm, a.not());
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::BYTE => {
            let (i, x) = (pop!(evm), pop!(evm));
            if i > U256::from(31) {
                push!(evm, U256::zero());
            } else {
                push!(evm, (x >> (U256::from(248) - i * 8)) & (0xFF as u64).into());
            }
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::SHL => {
            let (shift, value) = (pop!(evm), pop!(evm));
            if shift > 31.into() {
                push!(evm, U256::zero());
            } else {
                push!(evm, value << shift);
            }
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::SHR => {
            let (shift, value) = (pop!(evm), pop!(evm));
            if shift > 31.into() {
                push!(evm, U256::zero());
            } else {
                push!(evm, value >> shift);
            }
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::SAR => {
            // TODO
            // let (shift, value) = (pop!(evm), pop!(evm).as_i256());
            // push!(evm,(value >> shift).as_u256());
            let (shift, value) = (pop!(evm), pop!(evm));
            let sign = value.bit(255);
            if shift >= 256.into() {
                push!(evm, if sign {U256::MAX} else {ZERO});
            }
            else if !sign {
                push!(evm, value.shr(shift.as_u64()));
            }
            else {
                let value = value.shr(shift);
                let mask = U256::MAX;
                let mask = mask.shl(256 as u64 - shift.as_u64());
                push!(evm, mask + value);
            }
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::KECCAK256 => {
            let (offset, length) = (pop_u64!(evm), pop_u64!(evm));
            evm.gas_recorder
                .record_gas_usage(DynamicCosts::Keccak256 { len: length }.cost());
            return_if_gas_too_high!(evm.gas_recorder);
            let bytes = return_if_error_in_tuple!(evm.memory.read_bytes(
                offset as usize,
                length as usize,
                &mut evm.gas_recorder
            ));
            push!(evm, U256::from(keccak256(&bytes).as_bytes()));
        }

        opcodes::ADDRESS => {
            push!(evm, evm.contract_address);
            evm.gas_recorder.record_gas_usage(2);
        }

        opcodes::BALANCE => {
            let address = pop!(evm);
            push!(evm, runtime.balance(address));
            evm.gas_recorder.record_gas_usage(
                DynamicCosts::Balance {
                    target_is_cold: runtime.is_cold(address),
                }
                .cost(),
            );
            runtime.mark_hot(address);
        }

        opcodes::ORIGIN => {
            push!(evm, evm.transaction.origin);
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::CALLER => {
            push!(evm, evm.message.caller);
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::CALLVALUE => {
            push!(evm, evm.message.value);
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::CALLDATALOAD => {
            // TODO fix
            let index = pop_u64!(evm) as usize;
            if index > evm.message.data.len() - 32 {
                push!(evm, {
                    // TODO can do better than this don't need to clone the entire message data
                    let mut bytes = evm.message.data.clone();
                    bytes.append(&mut vec![0; 32 - (evm.message.data.len() - index)]);
                    U256::from(bytes.as_slice())
                });
            } else {
                push!(evm, { U256::from(&evm.message.data[index..index + 32]) });
            }
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::CALLDATASIZE => {
            push!(evm, U256::from(evm.message.data.len() as u64));
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::CALLDATACOPY => {
            // TODO fix
            let (dest_offset, offset, size) = (pop_usize!(evm), pop_usize!(evm), pop_usize!(evm));
            evm.gas_recorder
                .record_gas_usage(DynamicCosts::Copy { size_bytes: size }.cost());
            return_if_gas_too_high!(evm.gas_recorder);
            return_if_error!(evm.memory.copy_from_bytes(
                &mut evm.message.data,
                offset,
                dest_offset,
                size,
                &mut evm.gas_recorder
            ));
        }

        opcodes::CODESIZE => {
            push!(evm, U256::from(evm.program.len() as u64));
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::CODECOPY => {
            let (dest_offset, offset, size) = (pop_usize!(evm), pop_usize!(evm), pop_usize!(evm));
            evm.gas_recorder
                .record_gas_usage(DynamicCosts::Copy { size_bytes: size }.cost());
            return_if_gas_too_high!(evm.gas_recorder);
            return_if_error!(evm.memory.copy_from(
                &mut evm.program,
                offset,
                dest_offset,
                size,
                &mut evm.gas_recorder
            ));
        }

        opcodes::GASPRICE => {
            push!(evm, evm.transaction.gas_price);
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::EXTCODESIZE => {
            let address = pop!(evm);
            push!(evm, runtime.code_size(address));
            evm.gas_recorder.record_gas_usage(
                DynamicCosts::ExtCodeSize {
                    target_is_cold: runtime.is_cold(address),
                }
                .cost(),
            );
            runtime.mark_hot(address);
        }

        opcodes::EXTCODECOPY => {
            let (addr, dest_offset, offset, size) =
                (pop!(evm), pop_usize!(evm), pop_usize!(evm), pop_usize!(evm));
            evm.gas_recorder.record_gas_usage(
                DynamicCosts::ExtCodeCopy {
                    target_is_cold: runtime.is_cold(addr),
                    size_bytes: size,
                }
                .cost(),
            );
            return_if_error!(evm.memory.copy_from_bytes(
                &runtime.code(addr),
                offset,
                dest_offset,
                size,
                &mut evm.gas_recorder
            ));
            runtime.mark_hot(addr);
        }

        opcodes::RETURNDATASIZE => {
            push!(evm, U256::from(evm.last_return_data.len() as u64));
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::RETURNDATACOPY => {
            let (dest_offset, offset, size) = (pop_usize!(evm), pop_usize!(evm), pop_usize!(evm));
            evm.gas_recorder
                .record_gas_usage(DynamicCosts::Copy { size_bytes: size }.cost());
            return_if_gas_too_high!(evm.gas_recorder);
            if offset + size > evm.last_return_data.len() {
                evm.gas_recorder.record_gas_usage(evm.gas_input as u64);
                return ExecutionResult::Error(ExecutionError::InvalidMemoryAccess);
            }
            return_if_error!(evm.memory.copy_from(
                &mut evm.last_return_data,
                offset,
                dest_offset,
                size,
                &mut evm.gas_recorder
            ));
        }

        opcodes::EXTCODEHASH => {
            let address = pop!(evm);
            evm.gas_recorder.record_gas_usage(
                DynamicCosts::ExtCodeHash {
                    target_is_cold: runtime.is_cold(address),
                }
                .cost(),
            );
            return_if_gas_too_high!(evm.gas_recorder);
            push!(evm, U256::from(util::keccak256_u256(address).as_bytes()));
            runtime.mark_hot(address);
        }

        opcodes::BLOCKHASH => {
            let block_number = pop!(evm);
            push!(evm, h256_to_u256(runtime.block_hash(block_number)));
            evm.gas_recorder
                .record_gas_usage(static_costs::G_BLOCK_HASH);
        }

        opcodes::COINBASE => {
            push!(evm, runtime.block_coinbase());
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::TIMESTAMP => {
            push!(evm, runtime.block_timestamp());
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::NUMBER => {
            push!(evm, runtime.block_number());
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::DIFFICULTY => {
            push!(evm, runtime.block_difficulty());
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::GASLIMIT => {
            push!(evm, runtime.block_gas_limit());
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::CHAINID => {
            push!(evm, runtime.chain_id());
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::SELFBALANCE => {
            push!(evm, runtime.balance(evm.contract_address));
            evm.gas_recorder.record_gas_usage(static_costs::G_LOW);
        }

        opcodes::BASEFEE => {
            push!(evm, runtime.block_base_fee_per_gas());
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::POP => {
            pop!(evm);
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::MLOAD => {
            let offset = pop_usize!(evm);
            push!(
                evm,
                return_if_error_in_tuple!(evm.memory.read_u256(offset, &mut evm.gas_recorder))
            );
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::MSTORE => {
            let (offset, value) = (pop_usize!(evm), pop!(evm));
            return_if_error!(evm.memory.write_u256(offset, value, &mut evm.gas_recorder));
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::MSTORE8 => {
            let (offset, value) = (pop_usize!(evm), pop!(evm));
            return_if_error!(evm.memory.write_u8(
                offset,
                (value & U256::from(0xFF as u64)).low_u32() as u8,
                &mut evm.gas_recorder
            ));
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::SLOAD => {
            let key = pop!(evm);
            println!("key: {:?}", key);
            evm.gas_recorder.record_gas_usage(
                DynamicCosts::SLoad {
                    target_is_cold: runtime.is_cold_index(evm.contract_address, key),
                }
                .cost(),
            );
            runtime.mark_hot_index(evm.contract_address, key);
            push!(
                evm,
                h256_to_u256(runtime.read_storage(evm.contract_address, key),)
            );
        }

        opcodes::SSTORE => {
            return_error_if_static!(evm);
            let (key, value) = (pop!(evm), pop!(evm));
            let (v_org, v_cur, v_new) = (
                runtime.read_original_storage(evm.contract_address, key),
                runtime.read_storage(evm.contract_address, key),
                u256_to_h256(value),
            );
            runtime.set_storage(evm.contract_address, key, u256_to_h256(value));
            println!("v_ord: {:?}, v_cur: {:?}, v_new: {:?}, target_is_cold: {:?}",
                     v_org, v_cur, v_new, runtime.is_cold_index(evm.contract_address, key));
            let dynamic_cost = DynamicCosts::SStore {
                original: v_org,
                current: v_cur,
                new: v_new,
                target_is_cold: runtime.is_cold_index(evm.contract_address, key),
            };
            runtime.mark_hot_index(evm.contract_address, key);
            println!("key: {:?}, value: {:?}", key, value);
            evm.gas_recorder.record_gas_usage(dynamic_cost.cost());
            evm.gas_recorder.record_refund(dynamic_cost.refund());
        }

        opcodes::JUMP => {
            let destination = pop_usize!(evm);
            evm.program_counter = destination;
            evm.program_counter -= 1;
            evm.gas_recorder.record_gas_usage(static_costs::G_MID);
        }

        opcodes::JUMPI => {
            let (destination, condition) = (pop_usize!(evm), pop!(evm));
            if !condition.eq(&U256::zero()) {
                evm.program_counter = destination - 1;
            }
            evm.gas_recorder.record_gas_usage(static_costs::G_HIGH);
        }

        opcodes::PC => {
            push!(evm, U256::from(evm.program_counter as u64));
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::MSIZE => {
            push!(evm, U256::from((evm.memory.max_index + 1) as u64));
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::GAS => {
            push!(
                evm,
                U256::from(evm.gas_input - evm.gas_recorder.gas_usage as u64)
            );
            evm.gas_recorder.record_gas_usage(static_costs::G_BASE);
        }

        opcodes::JUMPDEST => {
            evm.gas_recorder.record_gas_usage(static_costs::G_JUMP_DEST);
        }

        opcodes::PUSH_1..=opcodes::PUSH_32 => {
            // Would technically be slightly faster without this (branch for each case) but probably a negligible difference
            let push_number = opcode - opcodes::PUSH_1 + 1;
            let bytes = return_if_error_in_tuple!(evm.program.read_bytes(
                evm.program_counter + 1,
                push_number as usize,
                &mut evm.gas_recorder
            ));
            evm.program_counter += push_number as usize;
            push!(evm, U256::from_big_endian(bytes.as_slice()));
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::DUP_1..=opcodes::DUP_16 => {
            let dup_number = opcode - opcodes::DUP_1 + 1;
            let value = evm.stack.read_nth(dup_number as usize);
            push!(evm, value);
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        opcodes::SWAP_1..=opcodes::SWAP_16 => {
            let swap_number: usize = (opcode - opcodes::SWAP_1 + 1) as usize;
            let bottom_value = evm.stack.read_nth(swap_number);
            let top_value = pop!(evm);
            evm.stack.write_nth(swap_number - 1, top_value);
            push!(evm, bottom_value);
            evm.gas_recorder.record_gas_usage(static_costs::G_VERY_LOW);
        }

        // TODO log
        opcodes::LOG_0..=opcodes::LOG_4 => {
            return_error_if_static!(evm);
            // TODO implement properly
            let (offset, size) = (pop_usize!(evm), pop_usize!(evm));
            let mut topics: Vec<U256> = Vec::new();
            for _ in 0..opcode - opcodes::LOG_0 {
                topics.push(pop!(evm));
            }
            // TODO refactor this, could do something like refactor memory with a read bytes with gas cost
            let mut log_mem = Memory::new();
            return_if_error!(log_mem.copy_from_no_local_cost(
                &mut evm.memory,
                offset,
                0,
                size,
                &mut evm.gas_recorder
            ));
            evm.gas_recorder.record_gas_usage(
                DynamicCosts::Log {
                    topic_length: topics.len() as u8,
                    size: size,
                }
                .cost(),
            )
        }

        opcodes::CREATE => {
            return_error_if_static!(evm);
            return_if_error!(create_1(evm, runtime, debug));
        }

        opcodes::CALL => {
            return_error_if_static!(evm);
            return_if_error!(call(evm, runtime, debug));
        }

        opcodes::CALLCODE => {
            return_if_error!(call_code(evm, runtime, debug));
        }

        opcodes::RETURN => {
            let (offset, size) = (pop_usize!(evm), pop_usize!(evm));
            if size != 0 {
                let len = offset.checked_add(size);
                match len {
                    Some(len) => {
                        evm.gas_recorder
                            .record_memory_gas_usage(evm.memory.len(), len);
                    }
                    None => {
                        evm.gas_recorder.set_gas_usage_to_max();
                        return ExecutionResult::Error(ExecutionError::InvalidMemoryAccess);
                    }
                }
            }
            return_if_error!(evm.check_gas_usage());
            return ExecutionResult::Success(ExecutionSuccess::Return(
                evm.memory.to_sub_vec(offset, size),
            ));
        }

        opcodes::DELEGATECALL => {
            return_if_error!(delegate_call(evm, runtime, debug));
        }

        opcodes::CREATE2 => {
            return_error_if_static!(evm);
            return_if_error!(create_2(evm, runtime, debug));
        }

        opcodes::STATICCALL => {
            return_if_error!(static_call(evm, runtime, debug));
        }

        opcodes::REVERT => {
            let (offset, size) = (pop_u64!(evm) as usize, pop_u64!(evm) as usize);
            println!("offset: {:?}, size: {:?}", offset, size);
            if size != 0 {
                let len = offset.checked_add(size);
                match len {
                    Some(len) => {
                        evm.gas_recorder
                            .record_memory_gas_usage(evm.memory.len(), len);
                    }
                    None => {
                        evm.gas_recorder.set_gas_usage_to_max();
                        return ExecutionResult::Error(ExecutionError::InvalidMemoryAccess);
                    }
                }
            }
            return_if_error!(evm.check_gas_usage());
            println!("evm.memory: {:?}", evm.memory.to_sub_vec(offset, offset + size).len());
            return ExecutionResult::Error(ExecutionError::Revert(
                evm.memory.to_sub_vec(offset, offset + size),
            ));
        }

        _ => {
            panic!("Opcode not implemented {}", opcode);
        }
    };

    // return_if_error!(evm.program_counter > 1000000 || evm.nested_index > 1024);
    return_if_error!(evm.check_gas_usage());
    evm.program_counter += 1;
    return ExecutionResult::InProgress;
}
