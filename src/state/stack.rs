use ethnum::U256;

const STACK_SIZE: usize = 1024;

pub struct Stack {
    data: [u8; STACK_SIZE],
    stack_pointer: usize,
}

impl Stack {
    #[inline]
    pub fn new() -> Stack {
        Stack {
            data: [0; STACK_SIZE],
            stack_pointer: 0,
        }
    }

    #[inline]
    pub fn push(&mut self, value: U256) {
        self.push_bytes(&value.to_be_bytes().to_vec());
    }

    #[inline]
    pub fn push_bytes(&mut self, bytes: &Vec<u8>) {
        let mut padded_vec = vec![0; 32 - bytes.len()];
        let padded_vec = if bytes.len() < 32 {
            padded_vec.append(&mut bytes.clone());
            &padded_vec
        }else {
            bytes
        };
        let item = U256::from_be_bytes(padded_vec.as_slice().try_into().unwrap());
        let bytes = item.to_be_bytes();
        let len = bytes.len();
        self.data[self.stack_pointer..self.stack_pointer + len].copy_from_slice(&bytes);
        self.stack_pointer += len;
        // println!("Pushed {:?}", U256::from_be_bytes(bytes));
    }

    #[inline]
    pub fn pop(&mut self) -> U256 {
        let u256_from_bytes = U256::from_be_bytes(self.data[self.stack_pointer-32..self.stack_pointer].try_into().unwrap());
        self.stack_pointer -= 32;
        // println!("Popped {:?}", u256_from_bytes);
        u256_from_bytes
    }

    #[inline]
    pub fn read_nth(&self, offset: usize) -> U256 {
        let index = self.stack_pointer - offset * 32;
        U256::from_be_bytes(self.data[index..index+32].try_into().unwrap())
    }

    #[inline]
    pub fn write_nth(&mut self, offset: usize, value: U256) {
        let index = self.stack_pointer - offset * 32;
        let end_index = index + 32;
        self.data[index..end_index].copy_from_slice(&value.to_be_bytes().to_vec());
    }
}

// TODO tests
