pub mod opcodes {

    pub const STOP: u8 = 0x00;
    pub const ADD: u8 = 0x01;
    pub const MUL: u8 = 0x02;
    pub const SUB: u8 = 0x03;
    pub const DIV: u8 = 0x04;
    pub const SDIV: u8 = 0x05;
    pub const MOD: u8 = 0x06;
    pub const SMOD: u8 = 0x07;
    pub const ADDMOD: u8 = 0x08;
    pub const MULMOD: u8 = 0x09;
    pub const EXP: u8 = 0x0A;
    pub const SIGNEXTEND: u8 = 0x0B;

    pub const LT: u8 = 0x10;
    pub const GT: u8 = 0x11;
    pub const SLT: u8 = 0x12;
    pub const SGT: u8 = 0x13;
    pub const EQ: u8 = 0x14;
    pub const ISZERO: u8 = 0x15;
    pub const AND: u8 = 0x16;
    pub const OR: u8 = 0x17;
    pub const XOR: u8 = 0x18;
    pub const NOT: u8 = 0x19;
    pub const BYTE: u8 = 0x1A;
    pub const SHL: u8 = 0x1B;
    pub const SHR: u8 = 0x1C;
    pub const SAR: u8 = 0x1D;

    pub const SHA3: u8 = 0x20;

    pub const ADDRESS: u8 = 0x30;
    pub const BALANCE: u8 = 0x31;
    pub const ORIGIN: u8 = 0x32;
    pub const CALLER: u8 = 0x33;
    pub const CALLVALUE: u8 = 0x34;
    pub const CALLDATALOAD: u8 = 0x35;
    pub const CALLDATASIZE: u8 = 0x36;
    pub const CALLDATACOPY: u8 = 0x37;
    pub const CODESIZE: u8 = 0x38;
    pub const CODECOPY: u8 = 0x39;
    pub const GASPRICE: u8 = 0x3A;
    pub const EXTCODESIZE: u8 = 0x3B;
    pub const EXTCODECOPY: u8 = 0x3C;
    pub const RETURNDATASIZE: u8 = 0x3D;
    pub const RETURNDATACOPY: u8 = 0x3E;
    pub const EXTCODEHASH: u8 = 0x3F;
    pub const BLOCKHASH: u8 = 0x40;
    pub const COINBASE: u8 = 0x41;
    pub const TIMESTAMP: u8 = 0x42;
    pub const NUMBER: u8 = 0x43;
    pub const DIFFICULTY: u8 = 0x44;
    pub const GASLIMIT: u8 = 0x45;
    pub const CHAINID: u8 = 0x46;
    pub const SELFBALANCE: u8 = 0x47;
    pub const BASEFEE: u8 = 0x48;

    pub const POP: u8 = 0x50;
    pub const MLOAD: u8 = 0x51;
    pub const MSTORE: u8 = 0x52;
    pub const MSTORE8: u8 = 0x53;
    pub const SLOAD: u8 = 0x54;
    pub const SSTORE: u8 = 0x55;
    pub const JUMP: u8 = 0x56;
    pub const JUMPI: u8 = 0x57;
    pub const PC: u8 = 0x58;
    pub const MSIZE: u8 = 0x59;
    pub const GAS: u8 = 0x5A;
    pub const JUMPDEST: u8 = 0x5B;

    pub const PUSH_1: u8 = 0x60;
    pub const PUSH_2: u8 = 0x61;
    pub const PUSH_3: u8 = 0x62;
    pub const PUSH_4: u8 = 0x63;
    pub const PUSH_5: u8 = 0x64;
    pub const PUSH_6: u8 = 0x65;
    pub const PUSH_7: u8 = 0x66;
    pub const PUSH_8: u8 = 0x67;
    pub const PUSH_9: u8 = 0x68;
    pub const PUSH_10: u8 = 0x69;
    pub const PUSH_11: u8 = 0x6a;
    pub const PUSH_12: u8 = 0x6b;
    pub const PUSH_13: u8 = 0x6c;
    pub const PUSH_14: u8 = 0x6d;
    pub const PUSH_15: u8 = 0x6e;
    pub const PUSH_16: u8 = 0x6f;
    pub const PUSH_17: u8 = 0x70;
    pub const PUSH_18: u8 = 0x71;
    pub const PUSH_19: u8 = 0x72;
    pub const PUSH_20: u8 = 0x73;
    pub const PUSH_21: u8 = 0x74;
    pub const PUSH_22: u8 = 0x75;
    pub const PUSH_23: u8 = 0x76;
    pub const PUSH_24: u8 = 0x77;
    pub const PUSH_25: u8 = 0x78;
    pub const PUSH_26: u8 = 0x79;
    pub const PUSH_27: u8 = 0x7a;
    pub const PUSH_28: u8 = 0x7b;
    pub const PUSH_29: u8 = 0x7c;
    pub const PUSH_30: u8 = 0x7d;
    pub const PUSH_31: u8 = 0x7e;
    pub const PUSH_32: u8 = 0x7f;

    pub const DUP_1: u8 = 0x80;
    pub const DUP_2: u8 = 0x81;
    pub const DUP_3: u8 = 0x82;
    pub const DUP_4: u8 = 0x83;
    pub const DUP_5: u8 = 0x84;
    pub const DUP_6: u8 = 0x85;
    pub const DUP_7: u8 = 0x86;
    pub const DUP_8: u8 = 0x87;
    pub const DUP_9: u8 = 0x88;
    pub const DUP_10: u8 = 0x89;
    pub const DUP_11: u8 = 0x8a;
    pub const DUP_12: u8 = 0x8b;
    pub const DUP_13: u8 = 0x8c;
    pub const DUP_14: u8 = 0x8d;
    pub const DUP_15: u8 = 0x8e;
    pub const DUP_16: u8 = 0x8f;

    pub const SWAP_1: u8 = 0x90;
    pub const SWAP_2: u8 = 0x91;
    pub const SWAP_3: u8 = 0x92;
    pub const SWAP_4: u8 = 0x93;
    pub const SWAP_5: u8 = 0x94;
    pub const SWAP_6: u8 = 0x95;
    pub const SWAP_7: u8 = 0x96;
    pub const SWAP_8: u8 = 0x97;
    pub const SWAP_9: u8 = 0x98;
    pub const SWAP_10: u8 = 0x99;
    pub const SWAP_11: u8 = 0x9a;
    pub const SWAP_12: u8 = 0x9b;
    pub const SWAP_13: u8 = 0x9c;
    pub const SWAP_14: u8 = 0x9d;
    pub const SWAP_15: u8 = 0x9e;
    pub const SWAP_16: u8 = 0x9f;

    pub const LOG_0: u8 = 0xa0;
    pub const LOG_1: u8 = 0xa1;
    pub const LOG_2: u8 = 0xa2;
    pub const LOG_3: u8 = 0xa3;
    pub const LOG_4: u8 = 0xa4;

    pub const PUSH: u8 = 0xB0;
    pub const DUP: u8 = 0xB1;
    pub const SWAP: u8 = 0xB2;

    pub const CREATE: u8 = 0xF0;
    pub const CALL: u8 = 0xF1;
    pub const CALLCODE: u8 = 0xF2;
    pub const RETURN: u8 = 0xF3;
    pub const DELEGATECALL: u8 = 0xF4;
    pub const CREATE2: u8 = 0xF5;

    pub const STATICCALL: u8 = 0xFA;

    pub const REVERT: u8 = 0xFD;

    pub const SELFDESTRUCT: u8 = 0xFF;
}
