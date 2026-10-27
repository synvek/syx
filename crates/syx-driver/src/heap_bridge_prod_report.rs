//! Phase 82: Heap v11 production bridge report for `syx doctor --heap-bridge-prod-report`.

#[derive(Debug, Clone, Default)]
pub struct HeapBridgeProdReport {
    pub heap_v11_enabled: bool,
    pub heap_root_bridge_enabled: bool,
    pub bridge_publish_api: bool,
    pub attach_count: u64,
}

pub fn run_heap_bridge_prod_report() -> HeapBridgeProdReport {
    HeapBridgeProdReport {
        heap_v11_enabled: syx_runtime_core::heap_v11(),
        heap_root_bridge_enabled: syx_runtime_core::heap_root_bridge_enabled(),
        bridge_publish_api: include_str!("../../syx-runtime-core/src/managed_heap.rs")
            .contains("heap_root_bridge_publish"),
        attach_count: syx_runtime_core::heap_root_bridge_attach_count(),
    }
}

pub fn print_heap_bridge_prod_report(report: &HeapBridgeProdReport) {
    println!("SYX heap v11 production bridge report (Phase 82)");
    println!(
        "  heap v11: {}",
        if report.heap_v11_enabled { "yes" } else { "no" }
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
        "  publish API: {}",
        if report.bridge_publish_api {
            "yes"
        } else {
            "no"
        }
    );
    println!("  attach count: {}", report.attach_count);
}
