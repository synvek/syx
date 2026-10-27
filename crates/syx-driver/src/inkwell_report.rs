//! Phase 73: inkwell report for `syx doctor --inkwell-report`.

#[derive(Debug, Clone)]
pub struct InkwellReport {
    pub cache_tag: &'static str,
    pub inkwell_default: bool,
    pub text_emit_opt_in: bool,
    pub phase75_ready: bool,
    pub heap_compact_decl: bool,
}

pub fn run_inkwell_report() -> InkwellReport {
    let driver = include_str!("lib.rs");
    let llvm = include_str!("../../syx-llvm/src/lib.rs");
    InkwellReport {
        cache_tag: if driver.contains("syx-cache-v47") {
            "syx-cache-v47"
        } else if driver.contains("syx-cache-v46") {
            "syx-cache-v46"
        } else {
            "unknown"
        },
        inkwell_default: syx_llvm::inkwell_default_enabled(),
        text_emit_opt_in: syx_llvm::llvm_text_emit_enabled(),
        phase75_ready: syx_llvm::llvm_inkwell_phase75_default_ready(),
        heap_compact_decl: llvm.contains("syx_rt_heap_compact"),
    }
}

pub fn print_inkwell_report(report: &InkwellReport) {
    println!("SYX inkwell / LLVM IR report (Phase 75)");
    println!("  cache tag: {}", report.cache_tag);
    println!(
        "  inkwell default backend: {}",
        if report.inkwell_default { "on" } else { "off" }
    );
    println!(
        "  text IR emit (SYX_LLVM_TEXT_EMIT): {}",
        if report.text_emit_opt_in { "on" } else { "off" }
    );
    println!(
        "  phase 75 inkwell GA ready: {}",
        if report.phase75_ready { "yes" } else { "no" }
    );
    println!(
        "  heap compact RT decl: {}",
        if report.heap_compact_decl {
            "yes"
        } else {
            "no"
        }
    );
}
