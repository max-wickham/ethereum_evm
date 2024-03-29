use primitive_types::{H160, H256, U256};
use sha3::{Digest, Keccak256};

use crate::{
    configs::gas_costs::DynamicCosts,
    evm_logic::{
        evm::{call::CallArgs, macros::return_if_error_in_tuple},
        util::{h256_to_u256, keccak256, u256_to_array, u256_to_h256, ZERO},
    },
    result::{Error, ExecutionResult, ExecutionSuccess},
    runtime::{self, Runtime},
};

use super::{
    call::{self, make_call},
    macros::{pop, pop_u64, return_if_error},
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
    let code =
        return_if_error_in_tuple!(evm.memory.read_bytes(offset, size, &mut evm.gas_recorder));
    println!("Code {:?}", code);
    runtime.create_contract(address, code);
    println!("Created: {:?}", address);
    let result = make_call(
        evm,
        runtime,
        debug,
        CallArgs {
            gas: evm.gas_input - evm.gas_recorder.gas_usage as u64,
            contract_address: address,
            code_address: address,
            caller_address: evm.message.caller,
            value: ZERO,
            args_offset: 0,
            args_size: 0,
            ret_offset: 0,
            ret_size: 0,
        },
        false,
    );
    // return_if_error!(make_call(evm, runtime, debug, false, true));
    // Undo the call gas cost
    // evm.gas_recorder.gas_usage -= 100;
    runtime.increase_nonce(evm.message.caller);
    runtime.increase_nonce(address);
    if result.has_return_result() {
        runtime.set_contract_code(address, evm.last_return_data.bytes.clone());
    }
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
    evm.gas_recorder.record_gas_usage(
        DynamicCosts::Create {
            deployed_code_size: deployed_code_size,
        }
        .cost(),
    );
    ExecutionResult::Success(ExecutionSuccess::Unknown)
}

pub fn create_1(evm: &mut EVMContext, runtime: &mut impl Runtime, debug: bool) -> ExecutionResult {
    let (value, offset, size) = (pop!(evm), pop_u64!(evm) as usize, pop_u64!(evm) as usize);
    let sender_address = evm.message.caller;
    let sender_nonce = runtime.nonce(sender_address);
	let mut stream = rlp::RlpStream::new_list(2);
    stream.append(&sender_address);
	stream.append(&sender_nonce);
	let address: H160 = H256::from_slice(Keccak256::digest(&stream.out()).as_slice()).into();
    create(evm, h256_to_u256(H256::from(address)), runtime, debug, value, offset, size)
}

pub fn create_2(evm: &mut EVMContext, runtime: &mut impl Runtime, debug: bool) -> ExecutionResult {
    let (value, offset, size, salt) = (
        pop!(evm),
        pop_u64!(evm) as usize,
        pop_u64!(evm) as usize,
        pop!(evm),
    );
    let code =
        return_if_error_in_tuple!(evm.memory.read_bytes(offset, size, &mut evm.gas_recorder));
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
    create(
        evm,
        h256_to_u256(H256::from(address)),
        runtime,
        debug,
        value,
        offset,
        size,
    )
}
