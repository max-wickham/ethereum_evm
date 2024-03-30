

#[derive(Clone, Debug)]
pub enum ExecutionError {
    ExcitedEarly,
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
    // This should never occur
    Unknown,
    // Transaction finished on a revert instruction
    RevertedTransaction,
    // Transaction finished on a stop instruction
    Stop,
    // Transaction finished on a return instruction, contains the returned vec
    Return(Vec<u8>),
}

// TODO refactor this to have both final execution result and execution in progress or final enum?
#[derive(Clone, Debug)]
pub enum ExecutionResult {
    InProgress,
    Success(ExecutionSuccess),
    Error(ExecutionError)

}

impl  ExecutionResult {

    pub fn return_result(self) -> Option<Vec<u8>> {
        match self {
            Self::Error(error) => match error {
                ExecutionError::Revert(result) => Some(result),
                _ => None
            },
            Self::Success(success) => match success {
                ExecutionSuccess::Return(result) => Some(result),
                _ => None
            },
            Self::InProgress => None
        }
    }
}
