use crate::result::{ExecutionResult, Error};

macro_rules! debug_match {
    ($evm_val:expr, $debug:expr, $opcode:expr, { $( $pat:pat => $block:block ),* }) => {
        match $opcode {
            $(
                $pat => {
                    #[allow(unreachable_code,unused_variables)]{
                    {
                        if $debug {
                            print!("{}", "\t".repeat($evm_val.nested_index as usize));
                            println!(
                                "PC : {:<5} | Opcode: {:<15} | Gas: {:<10}",
                                $evm_val.program_counter,
                                opcodes::OPCODE_MAP[&($opcode as u8)],
                                format!{"{:x}",$evm_val.gas_input - $evm_val.gas_recorder.clone().gas_usage as u64}
                            );
                        }
                        $block
                    }
                }
            }),*
            _ => {}
        }
    };
}
pub(crate) use debug_match;

macro_rules! return_if_error {
    ($evm_val:expr) => {
        match $evm_val {
            ExecutionResult::Err(err) => {
                println!("Error: {:?}", err);
                return ExecutionResult::Err(err)},
            _ => {}
        }
    };
}
pub(crate) use return_if_error;

macro_rules! break_if_error {
    ($evm_val:expr) => {
        #[allow(dead_code)]
        match $evm_val {
            ExecutionResult::Err(_) => {break;},
            _ => {}
        }
    };
}
pub(crate) use break_if_error;

macro_rules! return_error_if_static {
    ($evm_val:expr) => {
        if $evm_val.is_static {
            return ExecutionResult::Err(Error::ModifyStaticState);
        }

    };
}
pub(crate) use return_error_if_static;



macro_rules! pop {
    ($evm_val:tt) => {{
        let result = $evm_val.stack.pop();
        let result = match result {
            Err(()) => {
                println!("Error: {:?}", Error::InsufficientValuesOnStack);
                return ExecutionResult::Err(Error::InsufficientValuesOnStack);
            }

            Ok(value) => value,
        };
        result
    }};
}
pub(crate) use pop;
