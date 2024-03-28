use primitive_types::{H160, H256, U256};
use sha3::{Digest, Keccak256};

use crate::{
    configs::gas_costs::DynamicCosts,
    evm_logic::{evm::{call::CallArgs, macros::return_if_error_in_tuple}, util::{h256_to_u256, keccak256, u256_to_array, u256_to_h256, ZERO}},
    result::{Error, ExecutionResult},
    runtime::{self, Runtime},
};

use super::{
    call::{self, make_call},
    macros::{pop, return_if_error},
    EVMContext,
};

pub fn create(
    evm: &mut EVMContext,
    address: U256,
    runtime: &mut impl Runtime,
    debug: bool,
    value: U256,
    offset: usize,
    size: usize,
) -> ExecutionResult {
    let code = return_if_error_in_tuple!(evm.memory.read_bytes(offset, size, &mut evm.gas_recorder));
    runtime.create_contract(address, code);
    println!("Created: {:?}", address);
    // evm.stack.push(ZERO);
    // evm.stack.push(ZERO);
    // evm.stack.push(ZERO);
    // evm.stack.push(value);
    // evm.stack.push(ZERO);
    // evm.stack.push(address);
    // evm.stack.push(U256::from(
    //     evm.gas_input - evm.gas_recorder.gas_usage as u64,
    // ));
    make_call(evm, runtime, debug, CallArgs{
        gas: evm.gas_input - evm.gas_recorder.gas_usage as u64,
        contract_address: address,
        code_address: address,
        caller_address: evm.message.caller,
        value: ZERO,
        args_offset: 0,
        args_size: 0,
        ret_offset: 0,
        ret_size: 0,
    }, false);
    // return_if_error!(make_call(evm, runtime, debug, false, true));
    // Undo the call gas cost
    // evm.gas_recorder.gas_usage -= 100;
    println!("Created: {:?}", address);
    let deployed_code_size = runtime.code_size(address).as_usize();
    println!("Deployed code size: {}", deployed_code_size);
    println!(
        "Gas {:x}",
        DynamicCosts::Create {
            deployed_code_size: deployed_code_size
        }
        .cost()
    );
    evm.gas_recorder.record_gas(
        DynamicCosts::Create {
            deployed_code_size: deployed_code_size,
        }
        .cost(),
    );
    runtime.increase_nonce(evm.message.caller);
    runtime.increase_nonce(address);
    runtime.set_contract_code(address, evm.last_return_data.bytes.clone());
    ExecutionResult::Success
}

pub fn create_1(evm: &mut EVMContext, runtime: &mut impl Runtime, debug: bool) -> ExecutionResult {
    let (value, offset, size) = (pop!(evm), pop!(evm).as_usize(), pop!(evm).as_usize());
    let sender_address = evm.message.caller;
    let sender_nonce = runtime.nonce(sender_address);
    let mut encodable = u256_to_array(sender_address).to_vec();
    encodable.append(&mut u256_to_array(sender_nonce).to_vec());
    let address: U256 = h256_to_u256(keccak256(&rlp::encode(&encodable).to_vec()));
    let mut address_mod = Vec::from([0u8; 12]);
    address_mod.append(&mut u256_to_array(address).as_slice()[12..].to_vec());
    let address = U256::from_big_endian(address_mod.as_slice());
    create(evm, address, runtime, debug, value, offset, size)
}

pub fn create_2(evm: &mut EVMContext, runtime: &mut impl Runtime, debug: bool) -> ExecutionResult {
    let (value, offset, size, salt) = (
        pop!(evm),
        pop!(evm).as_usize(),
        pop!(evm).as_usize(),
        pop!(evm),
    );
    let code = return_if_error_in_tuple!(evm.memory.read_bytes(offset, size, &mut evm.gas_recorder));
    let code_hash = keccak256(&code);
    println!("Code Hash: {:x}", code_hash);
    let address: H160 = {
        let mut hasher = Keccak256::new();
        hasher.update([0xff]);
        hasher.update(&H160::from(u256_to_h256(evm.message.caller))[..]);
        hasher.update(&u256_to_h256(salt)[..]);
        hasher.update(&code_hash[..]);
        H256::from_slice(hasher.finalize().as_slice()).into()
    };
    println!(
        "Value: {}, Offset: {}, Size: {}, Salt: {}",
        value, offset, size, salt
    );
    println!("Address: {:x}", address);
    create(evm, h256_to_u256(H256::from(address)), runtime, debug, value, offset, size)
}
