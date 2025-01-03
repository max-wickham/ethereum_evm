use primitive_types::U256;
use sha2::{ Digest, Sha256 };

use crate::{
    configs::gas_costs::DynamicPreCompileCosts,
    evm_logic::evm::EVMContext,
    result::{ ExecutionResult, ExecutionSuccess },
};

pub fn sha2_256_contract(evm: &mut EVMContext) -> ExecutionResult {
    // Try to apply gas cost, if not enough return no bytes
    let input = evm.message.data.clone();
    let cost = (DynamicPreCompileCosts::Sha256 { data_word_size: (input.len() + 31) / 32 }).cost();
    if evm.gas_recorder.gas_available() < cost.try_into().unwrap() {
        evm.gas_recorder.record_gas_usage(evm.gas_recorder.gas_available().try_into().unwrap());
        return ExecutionResult::Success(ExecutionSuccess::Return(vec![]));
    }
    evm.gas_recorder.record_gas_usage(cost);
    let result = sha2_256(&input);
    let result_bytes: &mut [u8] = &mut [];
    result.to_big_endian(result_bytes);
    return ExecutionResult::Success(ExecutionSuccess::Return(result_bytes.to_vec()));
}

fn sha2_256(input: &[u8]) -> U256 {
    // 1) Create a Sha256 hasher
    let mut hasher = Sha256::new();
    // 2) Feed the input bytes
    hasher.update(input);
    // 3) Finalize the hash to a 32-byte array
    let digest = hasher.finalize();
    // 4) Convert those 32 bytes (big-endian) into a U256
    U256::from_big_endian(&digest)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use hex_literal::hex; // Import the macro from hex-literal

//     #[test]
//     fn test_sha2_256_precompile() {
//         let input = b"Hello, world!";
//         let output = sha2_256(input);

//         // Known SHA-256("Hello, world!") is:
//         //   09ca7e4eaa6e8ae9 c7d2611671291848
//         //   83644d07c62a3b2a 5a3052b2442c6eab
//         let expected_hex = hex!("09ca7e4eaa6e8ae9c7d261167129184883644d07c62a3b2a5a3052b2442c6eab");

//         assert_eq!(output, U256::from_big_endian(&expected_hex));
//     }
// }
