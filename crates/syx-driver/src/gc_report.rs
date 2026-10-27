//! Phase 73–75: memory / GC report for `syx doctor --gc-report`.

#[derive(Debug, Clone)]
pub struct GcReport {
    pub cache_tag: &'static str,
    pub arc_retain_release: bool,
    pub vm_mark_sweep: bool,
    pub unified_heap: bool,
    pub borrow_se0461_default: bool,
    pub browser_prelude_v3: bool,
    pub slot_count: usize,
    pub high_water: usize,
    pub collect_cycles: u32,
    pub compact_cycles: u32,
    pub bytes_reclaimed: u64,
    pub fragmentation_ratio: f64,
}

pub fn run_gc_report() -> GcReport {
    let lib = include_str!("lib.rs");
    let heap_src = include_str!("../../syx-runtime-core/src/managed_heap.rs");
    let (
        slot_count,
        high_water,
        collect_cycles,
        compact_cycles,
        bytes_reclaimed,
        fragmentation_ratio,
    ) = syx_runtime_core::global_heap()
        .lock()
        .map(|h| {
            (
                h.slot_count(),
                h.high_water(),
                h.collect_cycles(),
                h.compact_cycles(),
                h.bytes_reclaimed(),
                h.fragmentation_ratio(),
            )
        })
        .unwrap_or((0, 0, 0, 0, 0, 0.0));
    GcReport {
        cache_tag: if lib.contains("syx-cache-v47") {
            "syx-cache-v47"
        } else if lib.contains("syx-cache-v46") {
            "syx-cache-v46"
        } else if lib.contains("syx-cache-v45") {
            "syx-cache-v45"
        } else if lib.contains("syx-cache-v44") {
            "syx-cache-v44"
        } else {
            "unknown"
        },
        arc_retain_release: include_str!("../../syx-runtime-core/src/value.rs")
            .contains("object_ref_retain"),
        // Wave A W494 / ADR-838: probe for live `fn collect_mark_sweep` only
        // (comments may still name the retired API).
        vm_mark_sweep: include_str!("../../syx-vm/src/lib.rs").contains("fn collect_mark_sweep"),
        unified_heap: heap_src.contains("ManagedHeap") && heap_src.contains("collect_global"),
        borrow_se0461_default: syx_semantic::borrow_regions_se0461_complete_enabled(),
        browser_prelude_v3: lib.contains("browser-prelude-v3"),
        slot_count,
        high_water,
        collect_cycles,
        compact_cycles,
        bytes_reclaimed,
        fragmentation_ratio,
    }
}

pub fn print_gc_report(report: &GcReport) {
    println!("SYX memory / GC report (Phase 75)");
    println!("  cache tag: {}", report.cache_tag);
    println!(
        "  unified managed heap v4: {}",
        if report.unified_heap { "yes" } else { "no" }
    );
    println!("  heap slots: {}", report.slot_count);
    println!("  heap high_water: {}", report.high_water);
    println!("  heap collect_cycles: {}", report.collect_cycles);
    println!("  heap compact_cycles: {}", report.compact_cycles);
    println!("  heap bytes_reclaimed: {}", report.bytes_reclaimed);
    println!(
        "  heap fragmentation_ratio: {:.3}",
        report.fragmentation_ratio
    );
    println!(
        "  ARC retain/release: {}",
        if report.arc_retain_release {
            "yes"
        } else {
            "no"
        }
    );
    println!(
        "  VM mark-sweep GcCollect: {}",
        if report.vm_mark_sweep { "yes" } else { "no" }
    );
    println!(
        "  borrow SE-0461 default: {}",
        if report.borrow_se0461_default {
            "on"
        } else {
            "off"
        }
    );
    println!(
        "  browser prelude v3 fingerprint: {}",
        if report.browser_prelude_v3 {
            "yes"
        } else {
            "no"
        }
    );
}
