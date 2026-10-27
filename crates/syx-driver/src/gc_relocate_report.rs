//! Phase 78: GC relocate report for `syx doctor --gc-relocate-report`.

#[derive(Debug, Clone, Default)]
pub struct GcRelocateReport {
    pub relocate_api: bool,
    pub relocate_count: u64,
}

pub fn run_gc_relocate_report() -> GcRelocateReport {
    GcRelocateReport {
        relocate_api: include_str!("../../syx-runtime-core/src/managed_heap.rs")
            .contains("relocate_global"),
        relocate_count: syx_runtime_core::relocate_count(),
    }
}

pub fn print_gc_relocate_report(report: &GcRelocateReport) {
    println!("SYX GC relocate report (Phase 78)");
    println!(
        "  relocate_global API: {}",
        if report.relocate_api { "yes" } else { "no" }
    );
    println!("  relocate count: {}", report.relocate_count);
}
