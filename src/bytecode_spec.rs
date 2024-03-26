pub mod opcodes {
    use lazy_static::lazy_static;
    use util::opcode_map;
    opcode_map! {
        pub const STOP: u8 = 0x00u8;
        pub const ADD: u8 = 0x01u8;
        pub const MUL: u8 = 0x02u8;
        pub const SUB: u8 = 0x03u8;
        pub const DIV: u8 = 0x04u8;
        pub const SDIV: u8 = 0x05u8;
        pub const MOD: u8 = 0x06u8;
        pub const SMOD: u8 = 0x07u8;
        pub const ADDMOD: u8 = 0x08u8;
        pub const MULMOD: u8 = 0x09u8;
        pub const EXP: u8 = 0x0Au8;
        pub const SIGNEXTEND: u8 = 0x0Bu8;

        pub const LT: u8 = 0x10u8;
        pub const GT: u8 = 0x11u8;
        pub const SLT: u8 = 0x12u8;
        pub const SGT: u8 = 0x13u8;
        pub const EQ: u8 = 0x14u8;
        pub const ISZERO: u8 = 0x15u8;
        pub const AND: u8 = 0x16u8;
        pub const OR: u8 = 0x17u8;
        pub const XOR: u8 = 0x18u8;
        pub const NOT: u8 = 0x19u8;
        pub const BYTE: u8 = 0x1Au8;
        pub const SHL: u8 = 0x1Bu8;
        pub const SHR: u8 = 0x1Cu8;
        pub const SAR: u8 = 0x1Du8;

        pub const KECCAK256: u8 = 0x20u8;

        pub const ADDRESS: u8 = 0x30u8;
        pub const BALANCE: u8 = 0x31u8;
        pub const ORIGIN: u8 = 0x32u8;
        pub const CALLER: u8 = 0x33u8;
        pub const CALLVALUE: u8 = 0x34u8;
        pub const CALLDATALOAD: u8 = 0x35u8;
        pub const CALLDATASIZE: u8 = 0x36u8;
        pub const CALLDATACOPY: u8 = 0x37u8;
        pub const CODESIZE: u8 = 0x38u8;
        pub const CODECOPY: u8 = 0x39u8;
        pub const GASPRICE: u8 = 0x3Au8;
        pub const EXTCODESIZE: u8 = 0x3Bu8;
        pub const EXTCODECOPY: u8 = 0x3Cu8;
        pub const RETURNDATASIZE: u8 = 0x3Du8;
        pub const RETURNDATACOPY: u8 = 0x3Eu8;
        pub const EXTCODEHASH: u8 = 0x3Fu8;
        pub const BLOCKHASH: u8 = 0x40u8;
        pub const COINBASE: u8 = 0x41u8;
        pub const TIMESTAMP: u8 = 0x42u8;
        pub const NUMBER: u8 = 0x43u8;
        pub const DIFFICULTY: u8 = 0x44u8;
        pub const GASLIMIT: u8 = 0x45u8;
        pub const CHAINID: u8 = 0x46u8;
        pub const SELFBALANCE: u8 = 0x47u8;
        pub const BASEFEE: u8 = 0x48u8;

        pub const POP: u8 = 0x50u8;
        pub const MLOAD: u8 = 0x51u8;
        pub const MSTORE: u8 = 0x52u8;
        pub const MSTORE8: u8 = 0x53u8;
        pub const SLOAD: u8 = 0x54u8;
        pub const SSTORE: u8 = 0x55u8;
        pub const JUMP: u8 = 0x56u8;
        pub const JUMPI: u8 = 0x57u8;
        pub const PC: u8 = 0x58u8;
        pub const MSIZE: u8 = 0x59u8;
        pub const GAS: u8 = 0x5Au8;
        pub const JUMPDEST: u8 = 0x5Bu8;

        pub const PUSH_1: u8 = 0x60u8;
        pub const PUSH_2: u8 = 0x61u8;
        pub const PUSH_3: u8 = 0x62u8;
        pub const PUSH_4: u8 = 0x63u8;
        pub const PUSH_5: u8 = 0x64u8;
        pub const PUSH_6: u8 = 0x65u8;
        pub const PUSH_7: u8 = 0x66u8;
        pub const PUSH_8: u8 = 0x67u8;
        pub const PUSH_9: u8 = 0x68u8;
        pub const PUSH_10: u8 = 0x69u8;
        pub const PUSH_11: u8 = 0x6au8;
        pub const PUSH_12: u8 = 0x6bu8;
        pub const PUSH_13: u8 = 0x6cu8;
        pub const PUSH_14: u8 = 0x6du8;
        pub const PUSH_15: u8 = 0x6eu8;
        pub const PUSH_16: u8 = 0x6fu8;
        pub const PUSH_17: u8 = 0x70u8;
        pub const PUSH_18: u8 = 0x71u8;
        pub const PUSH_19: u8 = 0x72u8;
        pub const PUSH_20: u8 = 0x73u8;
        pub const PUSH_21: u8 = 0x74u8;
        pub const PUSH_22: u8 = 0x75u8;
        pub const PUSH_23: u8 = 0x76u8;
        pub const PUSH_24: u8 = 0x77u8;
        pub const PUSH_25: u8 = 0x78u8;
        pub const PUSH_26: u8 = 0x79u8;
        pub const PUSH_27: u8 = 0x7au8;
        pub const PUSH_28: u8 = 0x7bu8;
        pub const PUSH_29: u8 = 0x7cu8;
        pub const PUSH_30: u8 = 0x7du8;
        pub const PUSH_31: u8 = 0x7eu8;
        pub const PUSH_32: u8 = 0x7fu8;

        pub const DUP_1: u8 = 0x80u8;
        pub const DUP_2: u8 = 0x81u8;
        pub const DUP_3: u8 = 0x82u8;
        pub const DUP_4: u8 = 0x83u8;
        pub const DUP_5: u8 = 0x84u8;
        pub const DUP_6: u8 = 0x85u8;
        pub const DUP_7: u8 = 0x86u8;
        pub const DUP_8: u8 = 0x87u8;
        pub const DUP_9: u8 = 0x88u8;
        pub const DUP_10: u8 = 0x89u8;
        pub const DUP_11: u8 = 0x8au8;
        pub const DUP_12: u8 = 0x8bu8;
        pub const DUP_13: u8 = 0x8cu8;
        pub const DUP_14: u8 = 0x8du8;
        pub const DUP_15: u8 = 0x8eu8;
        pub const DUP_16: u8 = 0x8fu8;

        pub const SWAP_1: u8 = 0x90u8;
        pub const SWAP_2: u8 = 0x91u8;
        pub const SWAP_3: u8 = 0x92u8;
        pub const SWAP_4: u8 = 0x93u8;
        pub const SWAP_5: u8 = 0x94u8;
        pub const SWAP_6: u8 = 0x95u8;
        pub const SWAP_7: u8 = 0x96u8;
        pub const SWAP_8: u8 = 0x97u8;
        pub const SWAP_9: u8 = 0x98u8;
        pub const SWAP_10: u8 = 0x99u8;
        pub const SWAP_11: u8 = 0x9au8;
        pub const SWAP_12: u8 = 0x9bu8;
        pub const SWAP_13: u8 = 0x9cu8;
        pub const SWAP_14: u8 = 0x9du8;
        pub const SWAP_15: u8 = 0x9eu8;
        pub const SWAP_16: u8 = 0x9fu8;

        pub const LOG_0: u8 = 0xa0u8;
        pub const LOG_1: u8 = 0xa1u8;
        pub const LOG_2: u8 = 0xa2u8;
        pub const LOG_3: u8 = 0xa3u8;
        pub const LOG_4: u8 = 0xa4u8;

        pub const PUSH: u8 = 0xB0u8;
        pub const DUP: u8 = 0xB1u8;
        pub const SWAP: u8 = 0xB2u8;

        pub const CREATE: u8 = 0xF0u8;
        pub const CALL: u8 = 0xF1u8;
        pub const CALLCODE: u8 = 0xF2u8;
        pub const RETURN: u8 = 0xF3u8;
        pub const DELEGATECALL: u8 = 0xF4u8;
        pub const CREATE2: u8 = 0xF5u8;

        pub const STATICCALL: u8 = 0xFAu8;

        pub const REVERT: u8 = 0xFDu8;

        pub const SELFDESTRUCT: u8 = 0xFFu8;
    }
}
