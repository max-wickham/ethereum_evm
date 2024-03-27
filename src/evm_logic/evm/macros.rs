

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

macro_rules! return_if_false {
    ($evm_val:expr) => {
        if !$evm_val {
            return false;
        }
    };
}
pub(crate) use return_if_false;


macro_rules! pop {
    ($evm_val:tt) => {{
        let result = $evm_val.stack.pop();
        let result = match result {
            Err(()) => {
                return false;
            }

            Ok(value) => value,
        };
        result
    }};
}
pub(crate) use pop;
