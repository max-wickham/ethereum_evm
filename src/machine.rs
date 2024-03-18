use crate::bytecode_spec::opcodes;
use ethnum::U256;
use std::collections::HashMap;

const STACK_SIZE: usize = 1024;
const MEMORY_SIZE: usize = 1024;

pub struct Transaction {
    origin: U256,
    gas_price: U256,
}

pub struct Message {
    caller: U256,
    value: U256,
    data: Vec<u8>,
}
pub struct EVM {
    stack_pointer: usize,
    stack: [U256; STACK_SIZE],
    memory: [u8; MEMORY_SIZE],
    storage: HashMap<U256, U256>,
    program: Vec<u8>,
    program_counter: usize,
    contract_address: U256,
    transaction: Transaction,
    message: Message,
}

impl EVM {
    fn run_next_instruction(&mut self) -> bool {
        let opcode = self.program[self.program_counter];

        match opcode {
            opcodes::STOP => {
                return true;
            }

            opcodes::ADD => {
                self.stack[self.stack_pointer - 1] = self.stack_nth(0) + self.stack_nth(-1);
                self.stack_pointer -= 1;
            }

            opcodes::MUL => {
                self.stack[self.stack_pointer - 1] = self.stack_nth(0) * self.stack_nth(-1);
                self.stack_pointer -= 1;
            }

            opcodes::SUB => {
                self.stack[self.stack_pointer - 1] = self.stack_nth(0) - self.stack_nth(-1);
                self.stack_pointer -= 1;
            }

            opcodes::DIV => {
                self.stack[self.stack_pointer - 1] = self.stack_nth(0) / self.stack_nth(-1);
                self.stack_pointer -= 1;
            }

            opcodes::SDIV => {
                self.stack[self.stack_pointer - 1] =
                    (self.stack_nth(0).as_i256() / self.stack_nth(-1).as_i256()).as_u256();
                self.stack_pointer -= 1;
            }

            opcodes::MOD => {
                self.stack[self.stack_pointer - 1] = self.stack_nth(0) % self.stack_nth(-1);
                self.stack_pointer -= 1;
            }

            opcodes::SMOD => {
                self.stack[self.stack_pointer - 1] =
                    (self.stack_nth(0).as_i256() % self.stack_nth(-1).as_i256()).as_u256();
                self.stack_pointer -= 1;
            }

            opcodes::ADDMOD => {
                self.stack[self.stack_pointer - 2] =
                    (self.stack_nth(0) + self.stack_nth(-1)) % self.stack_nth(-2);
                self.stack_pointer -= 2;
            }

            opcodes::MULMOD => {
                self.stack[self.stack_pointer - 2] =
                    (self.stack_nth(0) * self.stack_nth(-1)) % self.stack_nth(-2);
                self.stack_pointer -= 2;
            }

            opcodes::EXP => {
                self.stack[self.stack_pointer - 1] =
                    self.stack_nth(0).pow(self.stack_nth(-1).as_u32());
                self.stack_pointer -= 1;
            }

            opcodes::SIGNEXTEND => {
                // TODO
            }

            opcodes::LT => {
                self.stack[self.stack_pointer - 1] =
                    U256::from(self.stack_nth(0) < self.stack_nth(-1));
                self.stack_pointer -= 1;
            }

            opcodes::GT => {
                self.stack[self.stack_pointer - 1] =
                    U256::from(self.stack_nth(0) > self.stack_nth(-1));
                self.stack_pointer -= 1;
            }

            opcodes::SLT => {
                self.stack[self.stack_pointer - 1] =
                    U256::from(self.stack_nth(0).as_i256() < self.stack_nth(-1).as_i256());
                self.stack_pointer -= 1;
            }

            opcodes::SGT => {
                self.stack[self.stack_pointer - 1] =
                    U256::from(self.stack_nth(0).as_i256() > self.stack_nth(-1).as_i256());
                self.stack_pointer -= 1;
            }

            opcodes::EQ => {
                self.stack[self.stack_pointer - 1] =
                    U256::from(self.stack_nth(0) == self.stack_nth(-1));
                self.stack_pointer -= 1;
            }

            opcodes::ISZERO => {
                self.stack[self.stack_pointer] = U256::from(self.stack_nth(0) == 0);
            }

            opcodes::AND => {
                self.stack[self.stack_pointer - 1] = self.stack_nth(0) & self.stack_nth(-1);
                self.stack_pointer -= 1;
            }

            opcodes::OR => {
                self.stack[self.stack_pointer - 1] = self.stack_nth(0) | self.stack_nth(-1);
                self.stack_pointer -= 1;
            }

            opcodes::XOR => {
                self.stack[self.stack_pointer - 1] = self.stack_nth(0) ^ self.stack_nth(-1);
                self.stack_pointer -= 1;
            }

            opcodes::NOT => {
                self.stack[self.stack_pointer] = !self.stack_nth(0);
            }

            opcodes::BYTE => {
                self.stack[self.stack_pointer - 1] =
                    (self.stack_nth(-1) >> (248 - self.stack_nth(0) * 8)) & 0xFF;
                self.stack_pointer -= 1;
            }

            opcodes::SHL => {
                self.stack[self.stack_pointer - 1] = self.stack_nth(-1) << self.stack_nth(0);
                self.stack_pointer -= 1;
            }

            opcodes::SHR => {
                self.stack[self.stack_pointer - 1] = self.stack_nth(-1) >> self.stack_nth(0);
                self.stack_pointer -= 1;
            }

            opcodes::SAR => {
                self.stack[self.stack_pointer - 1] =
                    (self.stack_nth(-1).as_i256() << self.stack_nth(0)).as_u256();
                self.stack_pointer -= 1;
            }

            opcodes::SHA3 => {
                // TODO
            }

            opcodes::ADDRESS => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer] = self.contract_address;
            }

            opcodes::BALANCE => {
                // TODO get balance of the address of the top of the stack
            }

            opcodes::ORIGIN => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer] = self.transaction.origin;
            }

            opcodes::CALLER => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer] = self.message.caller;
            }

            opcodes::CALLVALUE => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer] = self.message.value;
            }

            opcodes::CALLDATALOAD => {
                let index = self.stack_nth(0).as_u64() as usize;
                let mut u256_value: U256 = U256::from(0 as u64);
                let bytes_to_copy = &self.message.data[index..index + 32];
                let mut bytes = [0; 32];
                bytes.copy_from_slice(bytes_to_copy);
                u256_value = U256::from_be_bytes(bytes);
                self.stack[self.stack_pointer] = u256_value;
            }

            opcodes::CALLDATASIZE => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer] = U256::from(self.message.data.len() as u64);
            }

            opcodes::CALLDATACOPY => {
                // TODO
            }

            opcodes::CODESIZE => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer] = U256::from(self.program.len() as u64);
            }

            opcodes::CODECOPY => {
                // TODO
            }

            opcodes::GASPRICE => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer] = self.transaction.gas_price;
            }

            _ => {}
        }

        return false;
    }

    fn stack_nth(&self, index: i64) -> U256 {
        return self.stack[(self.stack_pointer as i64 + index) as usize];
    }
}
