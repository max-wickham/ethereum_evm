

#[derive(Copy, Clone, Debug)]
pub enum Error {
    InsufficientValuesOnStack,
    InsufficientGas,
    ModifyStaticState,
}

#[derive(Copy, Clone, Debug)]
pub enum ExecutionResult {
    Success,
    Err(Error)
}
