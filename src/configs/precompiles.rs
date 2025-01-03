use lazy_static::lazy_static;
use primitive_types::U256;

lazy_static! {
    /// 0x01: ecrecover
    pub static ref ECRECOVER_PRECOMPILE: U256 = U256::from(1);

    /// 0x02: sha256
    pub static ref SHA256_PRECOMPILE: U256 = U256::from(2);

    /// 0x03: ripemd160
    pub static ref RIPEMD160_PRECOMPILE: U256 = U256::from(3);

    /// 0x04: identity
    pub static ref IDENTITY_PRECOMPILE: U256 = U256::from(4);

    /// 0x05: modexp (EIP-198)
    pub static ref MODEXP_PRECOMPILE: U256 = U256::from(5);

    /// 0x06: alt_bn128-add (EIP-196)
    pub static ref ALTBN128_ADD_PRECOMPILE: U256 = U256::from(6);

    /// 0x07: alt_bn128-mul (EIP-196)
    pub static ref ALTBN128_MUL_PRECOMPILE: U256 = U256::from(7);

    /// 0x08: alt_bn128-pairing (EIP-197)
    pub static ref ALTBN128_PAIRING_PRECOMPILE: U256 = U256::from(8);

    /// 0x09: blake2-f (EIP-152)
    pub static ref BLAKE2_F_PRECOMPILE: U256 = U256::from(9);
}

pub fn is_precompile(address: &U256) -> bool {
    address.ge(&*ECRECOVER_PRECOMPILE) && address.le(&*BLAKE2_F_PRECOMPILE)
}
