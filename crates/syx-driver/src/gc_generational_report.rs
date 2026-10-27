//! Phase 80: generational GC report for `syx doctor --gc-generational-report`.

#[derive(Debug, Clone, Default)]
pub struct GcGenerationalReport {
    pub generational_enabled: bool,
    pub v13_enabled: bool,
    pub young_gen_len: usize,
    pub old_gen_len: usize,
    pub write_barrier_hits: u64,
    pub v13_compact_cycles: u64,
}

pub fn run_gc_generational_report() -> GcGenerationalReport {
    let enabled = syx_runtime_core::heap_v9_generational();
    let v13 = syx_runtime_core::heap_v13_generational();
    let (young, old, barriers) = if let Ok(h) = syx_runtime_core::global_heap().lock() {
        (h.young_gen_len(), h.old_gen_len(), h.write_barrier_hits())
    } else {
        (0, 0, 0)
    };
    GcGenerationalReport {
        generational_enabled: enabled,
        v13_enabled: v13,
        young_gen_len: young,
        old_gen_len: old,
        write_barrier_hits: barriers,
        v13_compact_cycles: syx_runtime_core::gc_gen_compact_cycles(),
    }
}

pub fn print_gc_generational_report(report: &GcGenerationalReport) {
    println!("SYX GC generational report (Phase 91)");
    println!(
        "  heap v9 generational: {}",
        if report.generational_enabled {
            "yes"
        } else {
            "no"
        }
    );
    println!(
        "  heap v13 generational compact: {}",
        if report.v13_enabled { "yes" } else { "no" }
    );
    println!("  young gen len: {}", report.young_gen_len);
    println!("  old gen len: {}", report.old_gen_len);
    println!("  write barrier hits: {}", report.write_barrier_hits);
    println!("  v13 compact cycles: {}", report.v13_compact_cycles);
}
