//! Phase 76: RT-internal exec fallback report for `syx doctor --rt-exec-report`.

#[derive(Debug, Clone, Default)]
pub struct RtExecReport {
    pub internal_exec_count: u64,
    pub fixture: String,
}

pub fn run_rt_exec_report(fixture: &str) -> RtExecReport {
    RtExecReport {
        internal_exec_count: syx_rt::rt_internal_exec_count(),
        fixture: fixture.to_string(),
    }
}

pub fn print_rt_exec_report(report: &RtExecReport) {
    println!("SYX RT exec report (Phase 76)");
    println!("  fixture: {}", report.fixture);
    println!("  RT-internal exec count: {}", report.internal_exec_count);
}
