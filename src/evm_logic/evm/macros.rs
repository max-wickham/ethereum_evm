

macro_rules! return_if_error {
    ($evm_val:expr) => {
        match $evm_val {
            ExecutionResult::Error(err) => {
                return ExecutionResult::Error(err)},
            _ => {}
        }
    };
}
pub(crate) use return_if_error;

macro_rules! return_tuple_if_error {
    ($evm_val:expr, $val:expr) => {
        match $evm_val {
            ExecutionResult::Error(err) => {
                println!("Error: {:?}", err);
                return (ExecutionResult::Error(err), $val)},
            _ => {}
        }
    };
}
pub(crate) use return_tuple_if_error;

macro_rules! return_if_error_in_tuple {
    ($evm_val:expr) => {
        match $evm_val.0 {
            ExecutionResult::Error(err) => {
                return ExecutionResult::Error(err)}
            _ => {$evm_val.1}
        }
    };
}
pub(crate) use return_if_error_in_tuple;

macro_rules! return_error_if_static {
    ($evm_val:expr) => {
        if $evm_val.is_static {
            return ExecutionResult::Error(ExecutionError::ModifyStaticState);
        }

    };
}
pub(crate) use return_error_if_static;


macro_rules! pop {
    ($evm:tt) => {{
        let result = $evm.stack.pop();
        let result = match result {
            Err(()) => {
                println!("Error: {:?}", ExecutionError::InsufficientValuesOnStack);
                $evm.gas_recorder.set_gas_usage_to_max();
                return ExecutionResult::Error(ExecutionError::InsufficientValuesOnStack);
            }
            Ok(value) => value,
        };
        result
    }};
}
pub(crate) use pop;

macro_rules! push {
    ($evm:expr, $value:expr) => {{
        let result = $evm.stack.push($value);
        match result {
            Err(()) => {
                return ExecutionResult::Error(ExecutionError::StackOverflow);
            }
            _ => {}
        }
    }};
}
pub(crate) use push;

macro_rules! pop_u64 {
    ($evm:tt) => {{
        let result = $evm.stack.pop();
        let result = match result {
            Err(()) => {
                println!("Error: {:?}", ExecutionError::InsufficientValuesOnStack);
                $evm.gas_recorder.set_gas_usage_to_max();
                return ExecutionResult::Error(ExecutionError::InsufficientValuesOnStack);
            }
            Ok(value) => value,
        };
        if result > U256::from(u64::MAX) {
            println!("U256 to large: {:?}", ExecutionError::InsufficientGas);
            // This would cause an out of gas error
            $evm.gas_recorder.gas_usage = $evm.gas_input as usize;
            // TODO refactor this away as unclear
            return ExecutionResult::Error(ExecutionError::InsufficientGas);
        }
        result.as_u64()
    }};
}
pub(crate) use pop_u64;

macro_rules! pop_usize {
    ($evm_val:tt) => {{
        pop_u64!($evm_val) as usize
    }};
}
pub(crate) use pop_usize;


macro_rules! return_if_gas_too_high {
    ($gas_recorder:expr) => {
        if !$gas_recorder.is_valid() {
            println!("Error gas: {:?}", ExecutionError::InsufficientGas);
            $gas_recorder.gas_usage = $gas_recorder.gas_input;
            return ExecutionResult::Error(ExecutionError::InsufficientGas);
        }
    };
}
pub(crate) use return_if_gas_too_high;
