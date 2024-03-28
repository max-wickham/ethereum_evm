

#[derive(Copy, Clone, Debug)]
pub enum Error {
    InsufficientValuesOnStack,
    InsufficientGas,
    ModifyStaticState,
    InvalidMemSize,
    InvalidMemoryAccess,
    Halted,
    Revert,
}

#[derive(Copy, Clone, Debug)]
pub enum ExecutionSuccess {
    Unknown,
    RevertedTransaction,
    Stop,
    Return,
}

#[derive(Copy, Clone, Debug)]
pub enum ExecutionResult {
    Success(ExecutionSuccess),
    Err(Error)

}

impl  ExecutionResult {
    pub fn is_result_with_return(&self) -> bool {
        match self {
            ExecutionResult::Err(error) => match error {
                Error::Revert => true,
                _ => false
            },
            ExecutionResult::Success(success) => match success {
                ExecutionSuccess::Return => true,
                _ => false
            }
        }
    }
}
