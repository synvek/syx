//! Phase 81: heap v10 bridge report for `syx doctor --heap-bridge-report`.

#[derive(Debug, Clone, Default)]
pub struct HeapBridgeReport {
    pub heap_v10_enabled: bool,
    pub heap_root_bridge_enabled: bool,
    pub card_table_api: bool,
    pub bridged_roots: usize,
}

pub fn run_heap_bridge_report() -> HeapBridgeReport {
    let bridged = syx_runtime_core::heap_root_bridge_peek().is_some();
    HeapBridgeReport {
        heap_v10_enabled: syx_runtime_core::heap_v10(),
        heap_root_bridge_enabled: syx_runtime_core::heap_root_bridge_enabled(),
        card_table_api: include_str!("../../syx-runtime-core/src/managed_heap.rs")
            .contains("card_table"),
        bridged_roots: usize::from(bridged),
    }
}

pub fn print_heap_bridge_report(report: &HeapBridgeReport) {
    println!("SYX heap v10 bridge report (Phase 81)");
    println!(
        "  heap v10: {}",
        if report.heap_v10_enabled { "yes" } else { "no" }
    );
    println!(
        "  heap root bridge: {}",
        if report.heap_root_bridge_enabled {
            "yes"
        } else {
            "no"
        }
    );
    println!(
        "  card table API: {}",
        if report.card_table_api { "yes" } else { "no" }
    );
    println!("  bridged roots: {}", report.bridged_roots);
}
