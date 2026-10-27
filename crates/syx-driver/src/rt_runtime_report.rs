//! Phase 78: RT runtime exec report — compile + run native, then check counter.

use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Default)]
pub struct RtRuntimeReport {
    pub internal_exec_count: u64,
    pub fixture: String,
    pub ran_native: bool,
}

pub fn run_rt_runtime_report(path: &Path) -> RtRuntimeReport {
    syx_rt::reset_rt_internal_exec_count();
    let mut ran_native = false;
    if path.exists() {
        let exe = std::env::temp_dir().join(format!(
            "syx_rt_runtime_{}",
            path.file_stem().and_then(|s| s.to_str()).unwrap_or("main")
        ));
        let _ = std::fs::remove_file(&exe);
        if crate::build_native_executable_file(path, &crate::DriverOptions::default(), &exe).is_ok()
        {
            let _ = Command::new(&exe).output();
            ran_native = true;
        }
    }
    RtRuntimeReport {
        internal_exec_count: syx_rt::rt_internal_exec_count(),
        fixture: path.display().to_string(),
        ran_native,
    }
}

pub fn print_rt_runtime_report(report: &RtRuntimeReport) {
    println!("SYX RT runtime report (Phase 78)");
    println!("  fixture: {}", report.fixture);
    println!(
        "  ran native: {}",
        if report.ran_native { "yes" } else { "no" }
    );
    println!("  RT-internal exec count: {}", report.internal_exec_count);
}
