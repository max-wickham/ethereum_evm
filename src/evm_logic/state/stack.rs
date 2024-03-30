use primitive_types::U256;

use crate::evm_logic::util::ZERO;

const STACK_SIZE: usize = 1024;

pub struct Stack {
    data: [U256; STACK_SIZE],
    stack_pointer: usize,
}

impl Stack {
    #[inline]
    pub fn new() -> Stack {
        Stack {
            data: [ZERO; STACK_SIZE],
            stack_pointer: 0,
        }
    }

    #[inline]
    pub fn push(&mut self, value: U256) -> Result<(),()> {
        if self.stack_pointer + 1 == STACK_SIZE {
            return Err(());
        }
        self.data[self.stack_pointer] = value;
        self.stack_pointer += 1;
        return Ok(());
    }

    #[inline]
    pub fn pop(&mut self) -> Result<U256, ()> {
        if self.stack_pointer < 1 {
            return Err(());
        }
        self.stack_pointer -= 1;
        return Ok(self.data[self.stack_pointer]);
    }

    // TODO add error handling here
    #[inline]
    pub fn read_nth(&self, offset: usize) -> U256 {
        self.data[self.stack_pointer - offset]
    }

    // TODO add error handling here
    #[inline]
    pub fn write_nth(&mut self, offset: usize, value: U256) {
        self.data[self.stack_pointer - offset] = value;
    }
}
