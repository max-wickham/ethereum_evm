

#[derive(Copy, Clone)]
pub enum Error {
    InsufficientValuesOnStack,
    InsufficientGas
}

#[derive(Copy, Clone)]
pub enum ExecutionResult {
    Success,
    Err(Error)
}
