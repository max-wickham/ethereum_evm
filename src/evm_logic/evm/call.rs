/*
Convenient to keep this as a macro as allows for early returns and error handling
Should be converted to function once proper error handling is introduced
*/

use primitive_types::U256;

use crate::runtime::Runtime;
use super::{macros::pop, EVMContext, Message};

#[inline]
pub fn make_call(evm: &mut EVMContext, runtime: &mut impl Runtime, debug: bool, maintain_storage: bool) -> bool {
        let (mut gas, address, value, args_offset, args_size, ret_offset, ret_size) = (
            pop!(evm).as_u64(),
            pop!(evm),
            pop!(evm),
            pop!(evm).as_usize(),
            pop!(evm).as_usize(),
            pop!(evm).as_usize(),
            pop!(evm).as_usize(),
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
            if maintain_storage {
                evm.contract_address
            } else {
                address
            },
            Message {
                caller: evm.contract_address,
                data: evm.memory.bytes[args_offset..args_offset + args_size].to_vec(),
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
}
