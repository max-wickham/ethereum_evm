

#[derive(Clone, Debug)]
pub enum Error {
    InsufficientValuesOnStack,
    StackOverflow,
    InsufficientGas,
    ModifyStaticState,
    InvalidMemSize,
    InvalidMemoryAccess,
    Halted,
    Revert(Vec<u8>),
}

#[derive(Clone, Debug)]
pub enum ExecutionSuccess {
    Unknown,
    RevertedTransaction,
    Stop,
    Return(Vec<u8>),
}

#[derive(Clone, Debug)]
pub enum ExecutionResult {
    Success(ExecutionSuccess),
    Err(Error)

}

impl  ExecutionResult {
    pub fn has_return_result(&self) -> bool {
        match self {
            ExecutionResult::Err(error) => match error {
                Error::Revert(_) => true,
                _ => false
            },
            ExecutionResult::Success(success) => match success {
                ExecutionSuccess::Return(_) => true,
                _ => false
            }
        }
    }

    pub fn return_result(self) -> Option<Vec<u8>> {
        match self {
            ExecutionResult::Err(error) => match error {
                Error::Revert(result) => Some(result),
                _ => None
            },
            ExecutionResult::Success(success) => match success {
                ExecutionSuccess::Return(result) => Some(result),
                _ => None
            }
        }
    }
}
