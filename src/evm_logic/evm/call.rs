/*
Convenient to keep this as a macro as allows for early returns and error handling
Should be converted to function once proper error handling is introduced
*/


use super::macros::pop_u64;
use super::{macros::pop, EVMContext, Message};
use crate::configs::gas_costs::DynamicCosts;
use crate::evm_logic::evm::macros::{push, return_if_gas_too_high};
use crate::evm_logic::state::memory::Memory;
use crate::evm_logic::util::ZERO;
use crate::result::{ExecutionError, ExecutionResult, ExecutionSuccess};
use crate::runtime::Runtime;

use primitive_types::U256;


#[inline]
pub fn call(evm: &mut EVMContext, runtime: &mut impl Runtime, debug: bool) -> ExecutionResult {
    let (mut gas, address, value, args_offset, args_size, ret_offset, ret_size) = (
        pop!(evm).as_u64(),
        pop!(evm),
        pop!(evm),
        pop_u64!(evm) as usize,
        pop_u64!(evm) as usize,
        pop_u64!(evm) as usize,
        pop_u64!(evm) as usize,
    );
    let call_args = CallArgs {
        gas: gas,
        code_address: address,
        contract_address: address,
        caller_address: evm.contract_address,
        value: value,
        args_offset: args_offset,
        args_size: args_size,
        ret_offset: ret_offset,
        ret_size: ret_size,
    };
    evm.gas_recorder.record_gas_usage(
        DynamicCosts::Call {
            value: value,
            target_is_cold: runtime.is_cold(address),
            empty_account: !value.eq(&U256::zero())
                && runtime.nonce(address).eq(&U256::zero())
                && runtime.code_size(address).eq(&U256::zero())
                && runtime.balance(address).eq(&U256::zero()),
            is_delegate: false,
        }
        .cost(),
    );
    if evm.gas_recorder.gas_input > evm.gas_recorder.gas_usage {
        println!("Gas usage {:x}", evm.gas_recorder.gas_input - evm.gas_recorder.gas_usage);
    }
    return_if_gas_too_high!(evm.gas_recorder);
    match make_call(evm, runtime, debug, call_args, false) {
        ExecutionResult::Error(_) => ExecutionResult::Success(ExecutionSuccess::RevertedTransaction),
        ExecutionResult::Success(_) => ExecutionResult::InProgress,
        ExecutionResult::InProgress => panic!("Call should not be still in progress"),
    }
}

#[inline]
pub fn call_code(evm: &mut EVMContext, runtime: &mut impl Runtime, debug: bool) -> ExecutionResult {
    let (mut gas, address, value, args_offset, args_size, ret_offset, ret_size) = (
        pop!(evm).as_u64(),
        pop!(evm),
        pop!(evm),
        pop_u64!(evm) as usize,
        pop_u64!(evm) as usize,
        pop_u64!(evm) as usize,
        pop_u64!(evm) as usize,
    );
    // if value.eq(&ZERO) {
    //     gas += static_costs::G_CALL_STIPEND;
    // }
    let call_args = CallArgs {
        gas: gas,
        code_address: address,
        contract_address: evm.contract_address,
        caller_address: evm.contract_address,
        value: value,
        args_offset: args_offset,
        args_size: args_size,
        ret_offset: ret_offset,
        ret_size: ret_size,
    };
    evm.gas_recorder.record_gas_usage(
        DynamicCosts::Call {
            value: value,
            target_is_cold: runtime.is_cold(address),
            empty_account: !value.eq(&U256::zero())
                && runtime.nonce(address).eq(&U256::zero())
                && runtime.code_size(address).eq(&U256::zero())
                && runtime.balance(address).eq(&U256::zero()),
            is_delegate: false,
        }
        .cost(),
    );
    return_if_gas_too_high!(evm.gas_recorder);
    match make_call(evm, runtime, debug, call_args, false) {
        ExecutionResult::Error(_) => ExecutionResult::Success(ExecutionSuccess::RevertedTransaction),
        ExecutionResult::Success(_) => ExecutionResult::InProgress,
        ExecutionResult::InProgress => panic!("Call should not be still in progress"),
    }
}

#[inline]
pub fn delegate_call(
    evm: &mut EVMContext,
    runtime: &mut impl Runtime,
    debug: bool,
) -> ExecutionResult {
    let (gas, address, args_offset, args_size, ret_offset, ret_size) = (
        pop!(evm).as_u64(),
        pop!(evm),
        pop_u64!(evm) as usize,
        pop_u64!(evm) as usize,
        pop_u64!(evm) as usize,
        pop_u64!(evm) as usize,
    );
    let call_args = CallArgs {
        gas: gas,
        code_address: address,
        contract_address: evm.contract_address,
        caller_address: evm.message.caller,
        value: evm.message.value,
        args_offset: args_offset,
        args_size: args_size,
        ret_offset: ret_offset,
        ret_size: ret_size,
    };
    evm.gas_recorder.record_gas_usage(
        DynamicCosts::Call {
            value: evm.message.value,
            target_is_cold: runtime.is_cold(address),
            empty_account: !evm.message.value.eq(&U256::zero())
                && runtime.nonce(address).eq(&U256::zero())
                && runtime.code_size(address).eq(&U256::zero())
                && runtime.balance(address).eq(&U256::zero()),
            is_delegate: true,
        }
        .cost(),
    );
    return_if_gas_too_high!(evm.gas_recorder);
    match make_call(evm, runtime, debug, call_args, false) {
        ExecutionResult::Error(_) => ExecutionResult::Success(ExecutionSuccess::RevertedTransaction),
        ExecutionResult::Success(_) => ExecutionResult::InProgress,
        ExecutionResult::InProgress => panic!("Call should not be still in progress"),
    }
}


#[inline]
pub fn static_call(evm: &mut EVMContext, runtime: &mut impl Runtime, debug: bool) -> ExecutionResult {
    let (mut gas, address, args_offset, args_size, ret_offset, ret_size) = (
        pop!(evm).as_u64(),
        pop!(evm),
        pop_u64!(evm) as usize,
        pop_u64!(evm) as usize,
        pop_u64!(evm) as usize,
        pop_u64!(evm) as usize,
    );
    let mut call_args = CallArgs {
        gas: gas,
        code_address: address,
        contract_address: address,
        caller_address: evm.contract_address,
        value: ZERO,
        args_offset: args_offset,
        args_size: args_size,
        ret_offset: ret_offset,
        ret_size: ret_size,
    };
    evm.gas_recorder.record_gas_usage(
        DynamicCosts::Call {
            value: ZERO,
            target_is_cold: runtime.is_cold(address),
            empty_account: false,
            is_delegate: false,
        }
        .cost(),
    );
    return_if_gas_too_high!(evm.gas_recorder);
    runtime.mark_hot(address);
    match make_call(evm, runtime, debug, call_args, true) {
        ExecutionResult::Error(_) => ExecutionResult::Success(ExecutionSuccess::RevertedTransaction),
        ExecutionResult::Success(_) => ExecutionResult::InProgress,
        ExecutionResult::InProgress => panic!("Call should not be still in progress"),
    }
}


pub struct CallArgs {
    pub gas: u64,
    pub code_address: U256,
    pub contract_address: U256,
    pub caller_address: U256,
    pub value: U256,
    pub args_offset: usize,
    pub args_size: usize,
    pub ret_offset: usize,
    pub ret_size: usize,
}

#[inline]
pub fn make_call(
    evm: &mut EVMContext,
    runtime: &mut impl Runtime,
    debug: bool,
    args: CallArgs,
    is_static: bool
) -> ExecutionResult {
    let code: Vec<u8> = runtime.code(args.code_address);
    let gas = args
        .gas
        .min((evm.gas_input - evm.gas_recorder.gas_usage.clone() as u64) * 63 / 64);
    if args.args_offset.checked_add( args.args_size).is_none() || (args.args_offset + args.args_size > evm.memory.bytes.len()) {
        evm.gas_recorder.record_gas_usage(evm.gas_recorder.gas_input as u64);
        return ExecutionResult::Error(ExecutionError::InvalidMemSize);
    }
    let mut sub_evm = EVMContext::create_sub_context(
        args.contract_address,
        Message {
            caller: args.caller_address,
            data: evm.memory.bytes[args.args_offset..args.args_offset + args.args_size].to_vec(),
            value: args.value,
        },
        gas,
        code,
        evm.transaction.clone(),
        evm.gas_price,
        evm.nested_index + 1,
        is_static
    );
    runtime.add_context();
    let execution_result = sub_evm.execute_program(runtime, debug);
    match &execution_result {
        ExecutionResult::Error(error) => {
            runtime.revert_context();
            match &error {
            ExecutionError::Revert(result) => {
                handle_return_data(evm, result, args.ret_offset, args.ret_size);
            }
            _ => {
                evm.last_return_data = Memory::new();
            }
        }},
        ExecutionResult::Success(success) => {
            runtime.merge_context();
            match success {
            ExecutionSuccess::Return(result) => {
                handle_return_data(evm, result, args.ret_offset, args.ret_size);
            }
            _ => {
                evm.last_return_data = Memory::new();
            }
        }},
        ExecutionResult::InProgress => {
            panic!("Program shouldn't have excited whilst in progress");
        }
    }
    evm.gas_recorder.merge(&sub_evm.gas_recorder, &execution_result);
    push!(evm,U256::from(match execution_result {
        ExecutionResult::Success(_) => true,
        _ => false,
    } as u64));
    if evm.gas_recorder.gas_input > evm.gas_recorder.gas_usage {
        println!("Gas usage 1 {:x}", evm.gas_recorder.gas_input - evm.gas_recorder.gas_usage);
    }
    execution_result
}

fn handle_return_data(evm: &mut EVMContext, return_data: &Vec<u8>, ret_offset: usize, ret_size: usize) {
    evm.last_return_data = Memory::from(&return_data.clone(), None);
    evm.memory.copy_from_bytes(
        &return_data,
        0,
        ret_offset,
        ret_size,
        &mut evm.gas_recorder,
    );
}

// #[inline]
// pub fn _(
//     evm: &mut EVMContext,
//     runtime: &mut impl Runtime,
//     debug: bool,
//     maintain_storage: bool,
//     maintain_caller: bool,
// ) -> ExecutionResult {
//     let (mut gas, address, value, args_offset, args_size, ret_offset, ret_size);
//     println!("Calling");
//     if maintain_caller {
//         (gas, address, args_offset, args_size, ret_offset, ret_size) = (
//             pop!(evm).as_u64(),
//             pop!(evm),
//             pop_u64!(evm) as usize,
//             pop_u64!(evm) as usize,
//             pop_u64!(evm) as usize,
//             pop_u64!(evm) as usize,
//         );
//         value = evm.message.value;
//     } else {
//         (
//             gas,
//             address,
//             value,
//             args_offset,
//             args_size,
//             ret_offset,
//             ret_size,
//         ) = (
//             pop!(evm).as_u64(),
//             pop!(evm),
//             pop!(evm),
//             pop_u64!(evm) as usize,
//             pop_u64!(evm) as usize,
//             pop_u64!(evm) as usize,
//             pop_u64!(evm) as usize,
//         );
//     }
//     println!("Calling");
//     let code: Vec<u8> = runtime.code(address);
//     if !value.eq(&U256::zero()) & !maintain_caller {
//         evm.gas_recorder.record_gas(2300);
//     }
//     let one_64th_value = (evm.gas_input - evm.gas_recorder.gas_usage.clone() as u64) * 63 / 64;
//     if gas > one_64th_value {
//         gas = one_64th_value;maintain_caller
//     }
//     let address_access_cost = if runtime.is_hot(address) {
//         100
//     } else {
//         runtime.mark_hot(address);
//         2600
//     };
//     println!("Gas: {:x}", gas);
//     println!("args_size: {:x}", args_size);
//     // TODO check gas is okay
//     let mut sub_evm = EVMContext::create_sub_context(
//         if maintain_storage {
//             evm.contract_address
//         } else {
//             address
//         },
//         Message {
//             caller: if maintain_caller {
//                 evm.message.caller
//             } else {
//                 evm.contract_address
//             },
//             data: evm.memory.bytes[args_offset..args_offset + args_size].to_vec(),
//             value: value,
//         },
//         gas,
//         code,
//         evm.transaction.clone(),
//         evm.gas_price,
//         evm.nested_index + 1,
//     );
//     // TODO calculate cost of call data

//     let execution_result = sub_evm.execute_program(runtime, debug);
//     evm.last_return_data = sub_evm.result;
//     // let current_memory_cost = evm.memory.memory_cost;
//     evm.memory.copy_from(
//         &mut evm.last_return_data,
//         0,
//         ret_offset,
//         ret_size,
//         &mut evm.gas_recorder,
//     );
//     evm.stack.push(U256::from(match execution_result {
//         ExecutionResult::Success => true,
//         _ => false,
//     } as u64));
//     let code_execution_cost = sub_evm.gas_recorder.gas_usage;
//     let positive_value_cost = if !value.eq(&U256::zero()) & !maintain_caller {
//         6700
//     } else {
//         0
//     };
//     let value_to_empty_account_cost = if !value.eq(&U256::zero())
//         && runtime.nonce(address).eq(&U256::zero())
//         && runtime.code_size(address).eq(&U256::zero())
//         && runtime.balance(address).eq(&U256::zero())
//     {
//         25000
//     } else {
//         0
//     };
//     evm.gas_recorder.record_gas(
//         (code_execution_cost
//             + address_access_cost
//             + positive_value_cost
//             + value_to_empty_account_cost) as u64,
//     );
//     println!("execution_result: {:?}", execution_result);
//     execution_result
// }
