// TODO complete PUSH instructions

use std::str::FromStr;

use ethnum::U256;

fn remove_char(input: &str, target: char) -> String {
    input.chars().filter(|&c| c != target).collect()
}

pub fn assemble(code: String) -> Vec<u8> {
    let mut binary: Vec<u8> = vec![];
    let words = remove_char(code.as_str(), '\n');
    let words: Vec<&str> = words.split_whitespace().collect();
    let mut index = 0;
    let num_words = words.len();
    while index < num_words {
        let mut word = words.get(index).unwrap().to_uppercase();
        println!("{}",word);
        if word == "PUSH" {
            word = String::from("PUSH_32");
        }
        let mut bytes: Vec<u8> = match word.as_str() {
            "STOP" => {
                vec![0x00]
            }
            "ADD" => {
                vec![0x01]
            }
            "MUL" => {
                vec![0x02]
            }
            "SUB" => {
                vec![0x03]
            }
            "DIV" => {
                vec![0x04]
            }
            "SDIV" => {
                vec![0x05]
            }
            "MOD" => {
                vec![0x06]
            }
            "SMOD" => {
                vec![0x07]
            }
            "ADDMOD" => {
                vec![0x08]
            }
            "MULMOD" => {
                vec![0x09]
            }
            "EXP" => {
                vec![0x0A]
            }
            "SIGNEXTEND" => {
                vec![0x0B]
            }
            "LT" => {
                vec![0x10]
            }
            "GT" => {
                vec![0x11]
            }
            "SLT" => {
                vec![0x12]
            }
            "SGT" => {
                vec![0x13]
            }
            "EQ" => {
                vec![0x14]
            }
            "ISZERO" => {
                vec![0x15]
            }
            "AND" => {
                vec![0x16]
            }
            "OR" => {
                vec![0x17]
            }
            "XOR" => {
                vec![0x18]
            }
            "NOT" => {
                vec![0x19]
            }
            "BYTE" => {
                vec![0x1A]
            }
            "SHL" => {
                vec![0x1B]
            }
            "SHR" => {
                vec![0x1C]
            }
            "SAR" => {
                vec![0x1D]
            }
            "KECCAK256" => {
                vec![0x20]
            }
            "ADDRESS" => {
                vec![0x30]
            }
            "BALANCE" => {
                vec![0x31]
            }
            "ORIGIN" => {
                vec![0x32]
            }
            "CALLER" => {
                vec![0x33]
            }
            "CALLVALUE" => {
                vec![0x34]
            }
            "CALLDATALOAD" => {
                vec![0x35]
            }
            "CALLDATASIZE" => {
                vec![0x36]
            }
            "CALLDATACOPY" => {
                vec![0x37]
            }
            "CODESIZE" => {
                vec![0x38]
            }
            "CODECOPY" => {
                vec![0x39]
            }
            "GASPRICE" => {
                vec![0x3A]
            }
            "EXTCODESIZE" => {
                vec![0x3B]
            }
            "EXTCODECOPY" => {
                vec![0x3C]
            }
            "RETURNDATASIZE" => {
                vec![0x3D]
            }
            "RETURNDATACOPY" => {
                vec![0x3E]
            }
            "EXTCODEHASH" => {
                vec![0x3F]
            }
            "BLOCKHASH" => {
                vec![0x40]
            }
            "COINBASE" => {
                vec![0x41]
            }
            "TIMESTAMP" => {
                vec![0x42]
            }
            "NUMBER" => {
                vec![0x43]
            }
            "DIFFICULTY" => {
                vec![0x44]
            }
            "GASLIMIT" => {
                vec![0x45]
            }
            "CHAINID" => {
                vec![0x46]
            }
            "SELFBALANCE" => {
                vec![0x47]
            }
            "BASEFEE" => {
                vec![0x48]
            }
            "POP" => {
                vec![0x50]
            }
            "MLOAD" => {
                vec![0x51]
            }
            "MSTORE" => {
                vec![0x52]
            }
            "MSTORE8" => {
                vec![0x53]
            }
            "SLOAD" => {
                vec![0x54]
            }
            "SSTORE" => {
                vec![0x55]
            }
            "JUMP" => {
                vec![0x56]
            }
            "JUMPI" => {
                vec![0x57]
            }
            "PC" => {
                vec![0x58]
            }
            "MSIZE" => {
                vec![0x59]
            }
            "GAS" => {
                vec![0x5A]
            }
            "JUMPDEST" => {
                vec![0x5B]
            }
            "PUSH_1" => {
                vec![0x60]
            }
            "PUSH_2" => {
                vec![0x61]
            }
            "PUSH_3" => {
                vec![0x62]
            }
            "PUSH_4" => {
                vec![0x63]
            }
            "PUSH_5" => {
                vec![0x64]
            }
            "PUSH_6" => {
                vec![0x65]
            }
            "PUSH_7" => {
                vec![0x66]
            }
            "PUSH_8" => {
                vec![0x67]
            }
            "PUSH_9" => {
                vec![0x68]
            }
            "PUSH_10" => {
                vec![0x69]
            }
            "PUSH_11" => {
                vec![0x6a]
            }
            "PUSH_12" => {
                vec![0x6b]
            }
            "PUSH_13" => {
                vec![0x6c]
            }
            "PUSH_14" => {
                vec![0x6d]
            }
            "PUSH_15" => {
                vec![0x6e]
            }
            "PUSH_16" => {
                vec![0x6f]
            }
            "PUSH_17" => {
                vec![0x70]
            }
            "PUSH_18" => {
                vec![0x71]
            }
            "PUSH_19" => {
                vec![0x72]
            }
            "PUSH_20" => {
                vec![0x73]
            }
            "PUSH_21" => {
                vec![0x74]
            }
            "PUSH_22" => {
                vec![0x75]
            }
            "PUSH_23" => {
                vec![0x76]
            }
            "PUSH_24" => {
                vec![0x77]
            }
            "PUSH_25" => {
                vec![0x78]
            }
            "PUSH_26" => {
                vec![0x79]
            }
            "PUSH_27" => {
                vec![0x7a]
            }
            "PUSH_28" => {
                vec![0x7b]
            }
            "PUSH_29" => {
                vec![0x7c]
            }
            "PUSH_30" => {
                vec![0x7d]
            }
            "PUSH_31" => {
                vec![0x7e]
            }
            "PUSH_32" => {
                index += 1;
                let next_word = words.get(index).unwrap().to_lowercase();
                println!("{}",next_word);
                let constant = U256::from_str(next_word.as_str()).unwrap();
                let mut result: Vec<u8> = vec![0x7f];
                result.append(&mut constant.to_be_bytes().to_vec());
                result
            }
            "DUP_1" => {
                vec![0x80]
            }
            "DUP_2" => {
                vec![0x81]
            }
            "DUP_3" => {
                vec![0x82]
            }
            "DUP_4" => {
                vec![0x83]
            }
            "DUP_5" => {
                vec![0x84]
            }
            "DUP_6" => {
                vec![0x85]
            }
            "DUP_7" => {
                vec![0x86]
            }
            "DUP_8" => {
                vec![0x87]
            }
            "DUP_9" => {
                vec![0x88]
            }
            "DUP_10" => {
                vec![0x89]
            }
            "DUP_11" => {
                vec![0x8a]
            }
            "DUP_12" => {
                vec![0x8b]
            }
            "DUP_13" => {
                vec![0x8c]
            }
            "DUP_14" => {
                vec![0x8d]
            }
            "DUP_15" => {
                vec![0x8e]
            }
            "DUP_16" => {
                vec![0x8f]
            }
            "SWAP_1" => {
                vec![0x90]
            }
            "SWAP_2" => {
                vec![0x91]
            }
            "SWAP_3" => {
                vec![0x92]
            }
            "SWAP_4" => {
                vec![0x93]
            }
            "SWAP_5" => {
                vec![0x94]
            }
            "SWAP_6" => {
                vec![0x95]
            }
            "SWAP_7" => {
                vec![0x96]
            }
            "SWAP_8" => {
                vec![0x97]
            }
            "SWAP_9" => {
                vec![0x98]
            }
            "SWAP_10" => {
                vec![0x99]
            }
            "SWAP_11" => {
                vec![0x9a]
            }
            "SWAP_12" => {
                vec![0x9b]
            }
            "SWAP_13" => {
                vec![0x9c]
            }
            "SWAP_14" => {
                vec![0x9d]
            }
            "SWAP_15" => {
                vec![0x9e]
            }
            "SWAP_16" => {
                vec![0x9f]
            }
            "LOG_0" => {
                vec![0xa0]
            }
            "LOG_1" => {
                vec![0xa1]
            }
            "LOG_2" => {
                vec![0xa2]
            }
            "LOG_3" => {
                vec![0xa3]
            }
            "LOG_4" => {
                vec![0xa4]
            }
            "PUSH" => {
                vec![0xB0]

            }
            "DUP" => {
                vec![0xB1]
            }
            "SWAP" => {
                vec![0xB2]
            }
            "CREATE" => {
                vec![0xF0]
            }
            "CALL" => {
                vec![0xF1]
            }
            "CALLCODE" => {
                vec![0xF2]
            }
            "RETURN" => {
                vec![0xF3]
            }
            "DELEGATECALL" => {
                vec![0xF4]
            }
            "CREATE2" => {
                vec![0xF5]
            }
            "STATICCALL" => {
                vec![0xFA]
            }
            "REVERT" => {
                vec![0xFD]
            }
            "SELFDESTRUCT" => {
                vec![0xFF]
            }
            _ => {
                vec![0x00]
            }
        };
        // println!{"Bytes: {:?}", bytes};
        binary.append(&mut bytes);
        index += 1;
    }

    binary
}
