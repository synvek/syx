#![allow(dead_code)]
#![allow(unused_imports)]
pub mod op;
mod runtime;

pub use op::*;
pub use runtime::{
    execute_mir_closure, install_module_string_pool, mir_main_exit_code, mir_op_immediates,
    module_runtime_string_offsets, run_mir_function, run_mir_function_with_args,
    mir_closure_runtime_active, pin_active_mir_runtime, unpin_active_mir_runtime,
    with_active_mir_runtime, MirRuntime,
};
