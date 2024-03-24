use primitive_types::{H256,U256};
use sha3::{Digest, Keccak256};

fn vec_to_fixed_array(bytes: Vec<u8>) -> [u8; 32] {
    let mut result = [0u8; 32];
    let len = bytes.len().min(32); // Take minimum to avoid out-of-bounds access
    // Copy bytes into the result array
    result[..len].copy_from_slice(&bytes[..len]);
    result
}

pub fn keccak256_u256(addr: U256) -> H256 {
    keccak256(&u256_to_array(addr).to_vec())
}


pub fn keccak256(bytes: &Vec<u8>) -> H256 {
    let result: Vec<u8> = Keccak256::digest(bytes).to_vec();
    H256::from_slice(vec_to_fixed_array(result).as_slice())
}

pub fn bytes_for_u256(num: &U256) -> usize {
    let num_zeros = num.leading_zeros();
    let num_bits = 256 - num_zeros;
    let bytes_needed = (num_bits + 7) / 8; // divide by 8 and round up
    bytes_needed as usize
}

pub fn h256_to_u256(v: H256) -> U256 {
	U256::from_big_endian(&v[..])
}

pub fn u256_to_h256(v: U256) -> H256 {
	let mut r = H256::default();
	v.to_big_endian(&mut r[..]);
	r
}

pub fn u256_to_array(v: U256) -> [u8; 32] {
    let mut x:  [u8;32] = [0;32];
    v.to_big_endian(&mut x);
    x
}



#[macro_export]
macro_rules! gas_usage_change {
    ($($code:tt)*) => {
        {
            let current_usage = self.memory.memory_cost;
            {
                $($code)*
            }
            let new_usage = self.memory.memory_cost;
            new_usage - current_usage
        }
    };
}
pub(crate) use gas_usage_change;
