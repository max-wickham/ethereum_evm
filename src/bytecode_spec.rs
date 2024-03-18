pub mod opcodes {
    // Done
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

    // Done
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

    // TODO
    pub const SHA3: u8 = 0x20;

    // TODO
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

    // TODO
    pub const POP: u8 = 0x50;
    pub const MLOAD: u8 = 0x51;
    pub const MSTORE: u8 = 0x52;
    pub const MSTORE8: u8 = 0x53;
    pub const SLOAD: u8 = 0x54;
    pub const SSTORE: u8 = 0x55;
    pub const JUMP: u8 = 0x56;
    pub const JUMP1: u8 = 0x57;
    pub const PC: u8 = 0x58;
    pub const MSIZE: u8 = 0x59;
    pub const GAS: u8 = 0x5A;
    pub const JUMPDEST: u8 = 0x5B;

    // TODO
    pub const PUSH_N: [u8; 32] = [
        0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e,
        0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d,
        0x7e, 0x7f,
    ];
    pub const DUP_N: [u8; 16] = [
        0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e,
        0x8f,
    ];
    pub const SWAP_N: [u8; 16] = [
        0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e,
        0x9f,
    ];
    pub const LOG_N: [u8; 5] = [0xa0, 0xa1, 0xa2, 0xa3, 0xa4];

    // TODO
    pub const PUSH: u8 = 0xB0;
    pub const DUP: u8 = 0xB1;
    pub const SWAP: u8 = 0xB2;

    // TODO
    pub const CREATE: u8 = 0xF0;
    pub const CALL: u8 = 0xF1;
    pub const CALLCODE: u8 = 0xF2;
    pub const RETURN: u8 = 0xF3;
    pub const DELEGATECALL: u8 = 0xF4;
    pub const RETURN2: u8 = 0xF5;

    // TODO
    pub const STATICCALL: u8 = 0xFA;

    // TODO
    pub const REVERT: u8 = 0xFD;

    // TODO
    pub const SELFDESTRUCT: u8 = 0xFF;

}
