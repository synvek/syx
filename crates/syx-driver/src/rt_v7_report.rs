//! Phase 82: RT native stack v7 report for `syx doctor --rt-v7-report`.

#[derive(Debug, Clone, Default)]
pub struct RtV7Report {
    pub native_stack_dispatch_v7: bool,
    pub dispatch_tag: String,
    pub full_opcode_families: bool,
    pub internal_exec_counter_api: bool,
}

pub fn run_rt_v7_report() -> RtV7Report {
    let abi = include_str!("../../syx-rt/src/abi.rs");
    RtV7Report {
        native_stack_dispatch_v7: abi.contains("NATIVE_STACK_DISPATCH_V7"),
        dispatch_tag: "native-stack-dispatch-v7-real".to_string(),
        full_opcode_families: abi.contains("CALL_HOST") && abi.contains("MAKE_CLOSURE"),
        internal_exec_counter_api: abi.contains("reset_rt_internal_exec_count")
            || abi.contains("rt_internal_exec_count"),
    }
}

pub fn print_rt_v7_report(report: &RtV7Report) {
    println!("SYX RT native stack v7 report (Phase 82)");
    println!(
        "  NATIVE_STACK_DISPATCH_V7: {}",
        if report.native_stack_dispatch_v7 {
            "yes"
        } else {
            "no"
        }
    );
    println!("  dispatch tag: {}", report.dispatch_tag);
    println!(
        "  full opcode families: {}",
        if report.full_opcode_families {
            "yes"
        } else {
            "no"
        }
    );
    println!(
        "  exec counter API: {}",
        if report.internal_exec_counter_api {
            "yes"
        } else {
            "no"
        }
    );
}
