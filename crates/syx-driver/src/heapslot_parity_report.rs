//! Phase 82: HeapSlot write-back parity report for `syx doctor --heapslot-parity-report`.

#[derive(Debug, Clone, Default)]
pub struct HeapslotParityReport {
    pub write_back_api: bool,
    pub materialize_receiver_api: bool,
    pub mir_run_write_back: bool,
    pub vm_array_set_write_back: bool,
}

pub fn run_heapslot_parity_report() -> HeapslotParityReport {
    let value_src = include_str!("../../syx-runtime-core/src/value.rs");
    let mir_src = include_str!("../../syx-mir-run/src/runtime.rs");
    let vm_src = include_str!("../../syx-vm/src/lib.rs");
    HeapslotParityReport {
        write_back_api: value_src.contains("write_back_heap_slot")
            || value_src.contains("write_back_receiver"),
        materialize_receiver_api: value_src.contains("materialize_array")
            || value_src.contains("materialize_dict"),
        mir_run_write_back: mir_src.contains("write_back_receiver"),
        vm_array_set_write_back: vm_src.contains("write_back_heap_slot")
            && vm_src.contains("ArraySet"),
    }
}

pub fn print_heapslot_parity_report(report: &HeapslotParityReport) {
    println!("SYX HeapSlot write-back parity report (Phase 82)");
    println!(
        "  write-back API: {}",
        if report.write_back_api { "yes" } else { "no" }
    );
    println!(
        "  materialize receiver: {}",
        if report.materialize_receiver_api {
            "yes"
        } else {
            "no"
        }
    );
    println!(
        "  mir-run write-back: {}",
        if report.mir_run_write_back {
            "yes"
        } else {
            "no"
        }
    );
    println!(
        "  VM ArraySet write-back: {}",
        if report.vm_array_set_write_back {
            "yes"
        } else {
            "no"
        }
    );
}
