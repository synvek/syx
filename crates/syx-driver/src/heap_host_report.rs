//! Phase 77: heap host materialize report for `syx doctor --heap-host-report`.

#[derive(Debug, Clone, Default)]
pub struct HeapHostReport {
    pub materialize_array_api: bool,
    pub materialize_dict_api: bool,
    pub write_back_heap_slot_api: bool,
}

pub fn run_heap_host_report() -> HeapHostReport {
    let value_src = include_str!("../../syx-runtime-core/src/value.rs");
    let hosts_src = include_str!("../../syx-vm/src/hosts.rs");
    HeapHostReport {
        materialize_array_api: value_src.contains("materialize_array"),
        materialize_dict_api: value_src.contains("materialize_dict"),
        write_back_heap_slot_api: value_src.contains("write_back_heap_slot")
            && hosts_src.contains("write_back_heap_slot"),
    }
}

pub fn print_heap_host_report(report: &HeapHostReport) {
    println!("SYX heap host report (Phase 77)");
    println!(
        "  materialize_array: {}",
        if report.materialize_array_api {
            "yes"
        } else {
            "no"
        }
    );
    println!(
        "  materialize_dict: {}",
        if report.materialize_dict_api {
            "yes"
        } else {
            "no"
        }
    );
    println!(
        "  write_back_heap_slot: {}",
        if report.write_back_heap_slot_api {
            "yes"
        } else {
            "no"
        }
    );
}
