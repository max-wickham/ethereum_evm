use std::ops::Div;

use primitive_types::{H256, U256};

use crate::evm_logic::util::{ZERO, ZERO_H256};

pub mod static_costs {
    pub const G_ZERO: u64 = 0;
    pub const G_JUMP_DEST: u64 = 1;
    pub const G_BASE: u64 = 2;
    pub const G_VERY_LOW: u64 = 3;
    pub const G_LOW: u64 = 5;
    pub const G_MID: u64 = 8;
    pub const G_HIGH: u64 = 10;
    pub const G_WARM_ACCESS: u64 = 100;
    pub const G_ACCESS_LIST_ADDRESS: u64 = 2400;
    pub const G_ACCESS_LIST_STORAGE: u64 = 1900;
    pub const G_COLD_ACCOUNT_ACCESS: u64 = 2600;
    pub const G_COLDS_LOAD: u64 = 2100;
    pub const G_SSET: u64 = 20000;
    pub const G_SRESET: u64 = 2900;
    pub const R_SCLEAR: u64 = 15000;
    pub const G_SELF_DESTRUCT: u64 = 5000;
    pub const G_CREATE: u64 = 32000;
    pub const G_CODE_DEPOSIT: u64 = 200;
    pub const G_CALL_VALUE: u64 = 9000;
    pub const G_CALL_STIPEND: u64 = 2300;
    pub const G_NEW_ACCOUNT: u64 = 25000;
    pub const G_EXP: u64 = 10;
    pub const G_EXP_BYTE: u64 = 50;
    pub const G_MEMORY: u64 = 3;
    pub const G_TX_CREATE: u64 = 32000;
    pub const G_TX_DATA_ZERO: u64 = 4;
    pub const G_TX_DATA_NON_ZERO: u64 = 16;
    pub const G_TRANSACTION: u64 = 21000;
    pub const G_LOG: u64 = 375;
    pub const G_LOG_DATA: u64 = 8;
    pub const G_LOG_TOPIC: u64 = 375;
    pub const G_KECCAK256: u64 = 30;
    pub const G_KECCAK256_WORD: u64 = 6;
    pub const G_COPY: u64 = 3;
    pub const G_BLOCK_HASH: u64 = 20;
}

pub enum DynamicCosts {
    ExtCodeSize {
        target_is_cold: bool,
    },
    Balance {
        target_is_cold: bool,
    },
    ExtCodeHash {
        target_is_cold: bool,
    },
    Call {
        value: U256,
        target_is_cold: bool,
        empty_account: bool,
        is_delegate: bool,
    },
    StaticCall {
        /// Call gas.
        gas: U256,
        /// True if target has not been previously accessed in this transaction
        target_is_cold: bool,
        /// Whether the target exists.
        target_exists: bool,
    },
    /// Gas cost for `SHA3`.
    Keccak256 {
        /// Length of the data.
        len: u64,
    },
    /// Gas cost for `LOG`.
    Log {
        /// Topic length.
        topic_length: u8,
        /// Data length.
        size: usize,
    },
    /// Gas cost for `EXTCODECOPY`.
    Exp {
        /// Power of `EXP`.
        power: U256,
    },
    Create2 {
        /// Length.
        len: U256,
    },
    Create {
        deployed_code_size: usize,
    },
    /// Gas cost for `SLOAD`.
    SLoad {
        /// True if target has not been previously accessed in this transaction
        target_is_cold: bool,
    },
    SStore {
        original: H256,
        current: H256,
        new: H256,
        target_is_cold: bool,
    },
    Copy {
        size_bytes: usize,
    },
    ExtCodeCopy {
        target_is_cold: bool,
        size_bytes: usize,
    }
}

impl DynamicCosts {
    // TODO add error here
    pub fn cost(&self) -> u64 {
        match self {
            DynamicCosts::Balance { target_is_cold } => {
                if *target_is_cold {
                    static_costs::G_COLDS_LOAD
                } else {
                    static_costs::G_WARM_ACCESS
                }
            }
            DynamicCosts::ExtCodeSize { target_is_cold } => {
                if *target_is_cold {
                    static_costs::G_COLDS_LOAD
                } else {
                    static_costs::G_WARM_ACCESS
                }
            }
            DynamicCosts::ExtCodeHash { target_is_cold } => {
                if *target_is_cold {
                    static_costs::G_COLDS_LOAD
                } else {
                    static_costs::G_WARM_ACCESS
                }
            }
            DynamicCosts::Call {
                value,
                empty_account,
                target_is_cold,
                is_delegate,
            } => {

                0 + if *target_is_cold {
                    static_costs::G_COLD_ACCOUNT_ACCESS
                } else {
                    static_costs::G_WARM_ACCESS
                } +
                if !value.eq(&ZERO) & !is_delegate {
                    static_costs::G_CALL_VALUE - static_costs::G_CALL_STIPEND
                } else {0} +
                if *empty_account {
                    static_costs::G_NEW_ACCOUNT
                } else {0}
            }
            DynamicCosts::Keccak256 { len } => {
                println!("Len in Keccak256: {}", len);
                static_costs::G_KECCAK256 + (len.div_ceil(32)) * static_costs::G_KECCAK256_WORD
            }
            DynamicCosts::Log { topic_length, size } => {
                static_costs::G_LOG
                    + static_costs::G_LOG_TOPIC * (*topic_length as u64)
                    + static_costs::G_LOG_DATA * (*size as u64)
            }

            DynamicCosts::Exp { power } => {
                // bytes_for_u256
                let bytes = (power.bits() + 7) / 8;
                static_costs::G_EXP + static_costs::G_EXP_BYTE * bytes as u64
            }

            DynamicCosts::Copy { size_bytes } => {
                static_costs::G_VERY_LOW  + static_costs::G_COPY * (size_bytes.div_ceil(32) as u64)
            }

            DynamicCosts::ExtCodeCopy { target_is_cold, size_bytes } => {
                println!("size_bytes: {}", size_bytes);
                println!("size_bytes.div_ceil(32): {}", size_bytes.div_ceil(32) as u64);
                static_costs::G_COPY * (size_bytes.div_ceil(32) as u64) +
                if *target_is_cold {
                    static_costs::G_COLD_ACCOUNT_ACCESS
                } else {
                    static_costs::G_WARM_ACCESS
                }
            }
            DynamicCosts::Create {deployed_code_size} => {

                println!("Code Size {}",deployed_code_size);
                static_costs::G_CREATE + static_costs::G_KECCAK256_WORD * (*deployed_code_size as u64).div_ceil(32)
            }
            DynamicCosts::SLoad { target_is_cold } => {
                println!("Is cold: {}", target_is_cold);
                if *target_is_cold {
                    static_costs::G_COLDS_LOAD
                } else {
                    static_costs::G_WARM_ACCESS
                }
            }
            DynamicCosts::SStore { original, current, new, target_is_cold } => {
                let mut gas_cost = if *target_is_cold { static_costs::G_COLDS_LOAD } else { 0};
                gas_cost  += if current.eq(&new) | !original.eq(&current) {
                    println!("Warm access");
                    static_costs::G_WARM_ACCESS
                } else if original.eq(&ZERO_H256) {
                    static_costs::G_SSET
                } else {
                    static_costs::G_SRESET
                };
                gas_cost

            }
            _ => 0,
        }
    }
    pub fn refund(&self) -> u64 {
        match self {
            DynamicCosts::SStore { original, current, new, target_is_cold } => {
                if !original.eq(&ZERO_H256) && new.eq(&ZERO_H256) {
                    static_costs::R_SCLEAR
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}
