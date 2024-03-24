use std::{ops::Not, str::FromStr};

use lazy_static::lazy_static;
use num256::{Int256, Uint256};
use primitive_types::{H256, U256};
use sha3::{Digest, Keccak256};

pub const ZERO: U256 = U256::zero();
lazy_static! {
    pub static ref MAX_UINT256_COMPLEMENT: Uint256 = Uint256::from_str(
        "57896044618658097711785492504343953926634992332820282019728792003956564819968"
    )
    .unwrap();
    pub static ref MAX_UINT256: Uint256 = Uint256::from_str(
        "115792089237316195423570985008687907853269984665640564039457584007913129639935"
    )
    .unwrap();
    pub static ref MIN_INT256: Int256 = Int256::from_str(
        "-57896044618658097711785492504343953926634992332820282019728792003956564819968"
    )
    .unwrap();
}

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
    let mut x: [u8; 32] = [0; 32];
    v.to_big_endian(&mut x);
    x
}

pub fn u256_to_uint256(v: U256) -> Uint256 {
    Uint256::from(TryInto::<[u8; 32]>::try_into(u256_to_array(v)).unwrap())
        .try_into()
        .unwrap()
}

pub fn uint256_to_int256(v: Uint256) -> Int256 {
    if v == *MAX_UINT256_COMPLEMENT {
        *MIN_INT256
    } else if v > *MAX_UINT256_COMPLEMENT {
        let mut twos_complement = v.to_be_bytes();
        for elem in twos_complement.iter_mut() {
            *elem = !*elem;
        }
        let twos_complement = Uint256::from(twos_complement) + Uint256::from(1 as u64);
        twos_complement.to_int256().unwrap() * Int256::from(-1)
    } else {
        v.to_int256().unwrap()
    }
}

pub fn int256_to_uint256(v: Int256) -> Uint256 {
    if v < Int256::from(0 as i64) {
        let twos_complement = v * Int256::from(-1);
        let twos_complement = twos_complement.to_uint256().unwrap();
        let mut twos_complement = twos_complement.to_be_bytes();
        for elem in twos_complement.iter_mut() {
            *elem = !*elem;
        }
        let twos_complement = Uint256::from(twos_complement) + Uint256::from(1 as u64);
        twos_complement
    } else {
        v.to_uint256().unwrap()
    }
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
