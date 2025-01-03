use primitive_types::U256;
use secp256k1::{ Secp256k1, ecdsa::{ RecoverableSignature, RecoveryId }, Message };
// use keccak_hash::keccak;
use crate::{
    configs::gas_costs::precompile_costs::G_ECRECOVER,
    evm_logic::evm::EVMContext,
    result::{ ExecutionResult, ExecutionSuccess },
    util::{ h256_to_u256, keccak256, u256_to_array },
};

pub fn ecrecover_contract(evm: &mut EVMContext) -> ExecutionResult {
    println!("ecrecover_contract Ecrecover");
    // Try to apply gas cost, if not enough return no bytes
    if evm.gas_recorder.gas_available() < G_ECRECOVER.try_into().unwrap() {
        // TODO record max gas usage
        evm.gas_recorder.record_gas_usage(evm.gas_recorder.gas_available().try_into().unwrap());
        return ExecutionResult::Success(ExecutionSuccess::Return(vec![]));
    }
    // TODO move into dynamic function
    evm.gas_recorder.record_gas_usage(G_ECRECOVER);

    let mut input = evm.message.data.clone();
    if input.len() < 4 * (256 / 8) {
        input.resize(4 * (256 / 8), 0);
    }
    let hash = U256::from_big_endian(&input[0..32]);
    let v = U256::from_big_endian(&input[32..64]);
    let r = U256::from_big_endian(&input[64..96]);
    let s = U256::from_big_endian(&input[96..128]);
    let result = ecrecover(hash, v, r, s);
    println!("result: {:?}", result);
    let result = u256_to_array(result);

    return ExecutionResult::Success(ExecutionSuccess::Return(result.to_vec()));
}

fn ecrecover(hash: U256, v: U256, r: U256, s: U256) -> U256 {
    let mut v = v.clone();
    // 1) Convert 'hash' to 32-byte array
    let mut msg_bytes = [0u8; 32];
    hash.to_big_endian(&mut msg_bytes);

    // 2) Convert r, s to 32-byte arrays
    let mut r_bytes = [0u8; 32];
    let mut s_bytes = [0u8; 32];
    r.to_big_endian(&mut r_bytes);
    s.to_big_endian(&mut s_bytes);

    // 3) Convert 'v' (27 or 28) -> RecoveryId (0 or 1)
    if !v.eq(&U256::from(27)) && !v.eq(&U256::from(28)) {
        return U256::zero();
    }
    let mut vv = v.low_u64();
    let rec_id: i32 = (vv - 27).try_into().unwrap();
    println!("found message Ecrecover");
    // 4) Build secp256k1 objects
    let recid = match RecoveryId::try_from(rec_id) {
        Ok(id) => id,
        Err(_) => {
            return U256::zero();
        }
    };
    let mut sig64 = [0u8; 64];
    sig64[..32].copy_from_slice(&r_bytes);
    sig64[32..].copy_from_slice(&s_bytes);
    println!("found message Ecrecover");
    let signature = match RecoverableSignature::from_compact(&sig64, recid) {
        Ok(sig) => sig,
        Err(_) => {
            return U256::zero();
        }
    };
    let message = Message::from_digest(msg_bytes);
    println!("found message Ecrecover");
    // 5) Recover public key, hash to get address
    let secp = Secp256k1::new();
    let pubkey = match secp.recover_ecdsa(&message, &signature) {
        Ok(pk) => pk.serialize_uncompressed(),
        Err(_) => {
            return U256::zero();
        }
    };
    println!("found message Ecrecover");
    // pubkey[0] is the 0x04 prefix; the next 64 bytes are X, Y coords
    let mut hash = keccak256(&pubkey[1..65]);
    // set the first 12 bytes to 0
    hash[0..12].copy_from_slice(&[0u8; 12]);
    println!("Hash Ecrecover: {:?}", hash);
    // let hash = U256::from_big_endian(hash[12..].as_ref());
    h256_to_u256(hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    #[test]
    fn test_ecrecover() {
        // Quick check: invalid v => zero
        let hash = U256::from_big_endian(
            &hex!("456e9aea5e197a1f1af7a3e85a3212fa4049a3ba34c2289b4c860fc0b0c64ef3")
        );
        let v = U256::from(28); // invalid
        let r = U256::from_big_endian(
            &hex!("9242685bf161793cc25603c231bc2f568eb630ea16aa137d2664ac8038825608")
        );
        let s = U256::from_big_endian(
            &hex!("4f8ae3bd7535248d0bd448298cc2e2071e56992d0774dc340c368ae950852ada")
        );
        let recovered = ecrecover(hash, v, r, s);
        assert_eq!(
            recovered,
            U256::from_big_endian(
                &hex!("0000000000000000000000007156526fbd7a3c72969b54f64e42c10fbb768c8a")
            )
        );
    }
}
