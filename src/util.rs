use ethnum::U256;
use keccak_hash::keccak_256;

fn vec_to_fixed_array(bytes: Vec<u8>) -> [u8; 32] {
    let mut result = [0u8; 32];
    let len = bytes.len().min(32); // Take minimum to avoid out-of-bounds access
    // Copy bytes into the result array
    result[..len].copy_from_slice(&bytes[..len]);
    result
}

pub fn keccak256_u256(addr: U256) -> U256 {
    keccak256(&addr.to_be_bytes().to_vec())
}


pub fn keccak256(bytes: &Vec<u8>) -> U256 {
    let mut output: Vec<u8> = vec![];
    keccak_256(&bytes, &mut output);
    U256::from_be_bytes(vec_to_fixed_array(output))
}
