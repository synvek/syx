//! Phase 73: native LLVM exec_op ratio report for `syx doctor --native-exec-op-report`.

use std::path::{Path, PathBuf};

use syx_llvm::{llvm_ir_exec_op_count, llvm_ir_exec_op_ratio};

use crate::{emit_llvm_text, DriverOptions};

#[derive(Debug, Clone)]
pub struct NativeExecOpFixtureReport {
    pub name: String,
    pub path: PathBuf,
    pub exec_op_calls: usize,
    pub ratio: f64,
}

#[derive(Debug, Clone)]
pub struct NativeExecOpReport {
    pub fixtures: Vec<NativeExecOpFixtureReport>,
    pub strict_max_ratio: f64,
    pub tier3_zero_exec_op: bool,
}

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

pub fn default_fixture_paths() -> Vec<(&'static str, PathBuf)> {
    let root = repo_root();
    vec![
        ("main_sync.sy", root.join("examples/main_sync.sy")),
        ("hello.sy", root.join("examples/hello.sy")),
        (
            "arc_retain_release.sy",
            root.join("crates/syx-driver/tests/fixtures/memory/arc_retain_release.sy"),
        ),
        (
            "unified_heap_retain.sy",
            root.join("crates/syx-driver/tests/fixtures/memory/unified_heap_retain.sy"),
        ),
    ]
}

pub fn llvm_ir_for_path(path: &Path) -> Option<String> {
    emit_llvm_text(path, &DriverOptions::default()).ok()
}

pub fn report_for_path(name: &str, path: &Path) -> NativeExecOpFixtureReport {
    let ir = llvm_ir_for_path(path).unwrap_or_default();
    let exec_op_calls = if ir.is_empty() {
        0
    } else {
        llvm_ir_exec_op_count(&ir)
    };
    let ratio = if ir.is_empty() {
        0.0
    } else {
        llvm_ir_exec_op_ratio(&ir)
    };
    NativeExecOpFixtureReport {
        name: name.to_string(),
        path: path.to_path_buf(),
        exec_op_calls,
        ratio,
    }
}

pub fn run_native_exec_op_report() -> NativeExecOpReport {
    let strict_max_ratio = syx_llvm::llvm_aot_phase6_exec_op_ratio_max();
    let tier3_zero_exec_op = syx_llvm::llvm_tier3_zero_exec_op_enabled();
    let fixtures = default_fixture_paths()
        .into_iter()
        .map(|(name, path)| report_for_path(name, &path))
        .collect();
    NativeExecOpReport {
        fixtures,
        strict_max_ratio,
        tier3_zero_exec_op,
    }
}

pub fn print_native_exec_op_report(report: &NativeExecOpReport) {
    println!("SYX native exec_op report (Phase 74)");
    println!("  strict max ratio: {}", report.strict_max_ratio);
    println!(
        "  LLVM tier-3 zero exec_op IR: {}",
        if report.tier3_zero_exec_op {
            "on"
        } else {
            "off"
        }
    );
    for f in &report.fixtures {
        let tier = if f.exec_op_calls == 0 {
            "tier-3-direct"
        } else {
            "exec_op-fallback"
        };
        println!(
            "  {}: exec_op={} ratio={:.4} tier={} ({})",
            f.name,
            f.exec_op_calls,
            f.ratio,
            tier,
            f.path.display()
        );
    }
}
