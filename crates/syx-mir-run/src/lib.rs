#![allow(dead_code)]
#![allow(unused_imports)]
mod mir_run_diag;
pub mod op;
mod runtime;

pub use op::*;
pub use runtime::{
    execute_mir_closure, install_module_string_pool, mir_closure_runtime_active,
    mir_main_exit_code, mir_op_immediates, module_runtime_string_offsets, pin_active_mir_runtime,
    run_mir_function, run_mir_function_with_args, unpin_active_mir_runtime,
    with_active_mir_runtime, MirRuntime,
};
