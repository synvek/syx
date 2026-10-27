//! Phase 79: dict runtime four-end stdout digest report.

use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct DictRuntimeReport {
    pub fixture: String,
    pub vm_ok: bool,
    pub lines: Vec<String>,
}

pub fn run_dict_runtime_report(path: &Path) -> DictRuntimeReport {
    let mut report = DictRuntimeReport {
        fixture: path.display().to_string(),
        ..Default::default()
    };
    if !path.exists() {
        return report;
    }
    syx_host_plugin::register_echo_plugin_in_process();
    if let Ok((code, lines)) = crate::run_file_printed(path, &crate::DriverOptions::default()) {
        report.vm_ok = code == 0;
        report.lines = lines;
    }
    report
}

pub fn print_dict_runtime_report(report: &DictRuntimeReport) {
    println!("SYX dict runtime report (Phase 79)");
    println!("  fixture: {}", report.fixture);
    println!("  vm ok: {}", report.vm_ok);
    println!("  stdout lines: {}", report.lines.len());
}
