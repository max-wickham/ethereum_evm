use primitive_types::U256;

use crate::evm_logic::util::ZERO;

const STACK_SIZE: usize = 1024;

pub struct Stack {
    pub data: [U256; STACK_SIZE],
    pub stack_pointer: usize,
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
    pub fn push(&mut self, value:
        U256) -> Result<(),()> {
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
    pub fn read_nth(&self, offset: usize) -> Result<U256, ()> {
        if self.stack_pointer < offset + 1 {
            return Err(());
        }
        Ok(self.data[self.stack_pointer - offset - 1])
    }

    // TODO add error handling here
    #[inline]
    pub fn write_nth(&mut self, offset: usize, value: U256) -> Result<(),()> {
        if self.stack_pointer < offset + 1 {
            return Err(());
        }
        if self.stack_pointer - offset - 1 > 256 {
            return Err(());
        }
        self.data[self.stack_pointer - offset - 1] = value;
        Ok(())
    }
}
