//! LLVM ObjectRefWrap emit residual retire first-slice
//! migrate/cleanup false; RT ABI wrap + RC still live.

#[test]
fn b09_llvm_objectref_wrap_emit_retire_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-wasm-emit/src/mir_emit.rs",
        "crates/syx-rt/src/abi.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-llvm-objectref-wrap-emit-retire-first-slice.md",
        "docs/gates/gate-b09-objectref-arrayget-heap-slots-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 LLVM wrap emit retire requires evidence file: {path}"
        );
    }
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    let wrap_arm = format!("MirOp::{} =>", "ObjectRefWrap");
    assert!(
        (llvm.contains("ADR-858") || llvm.contains("ADR-888"))
            && !llvm.contains(&wrap_arm)
            && !llvm.contains("out.push_str(\"declare void @syx_rt_object_ref_wrap()\")")
            && !llvm.contains("out.push_str(\"  call void @syx_rt_object_ref_wrap()\\n\")"),
        "LLVM ObjectRefWrap MirOp arm must be gone (W544); emit/declare remain retired; cite ADR-858/888"
    );
    assert!(
        !llvm.contains("objectref_rc_migrate_enabled = true")
            && !llvm.contains("objectref_rc_migrate_enabled=true"),
        "must not flip objectref_rc_migrate_enabled"
    );
    let wasm = include_str!("../../../syx-wasm-emit/src/mir_emit.rs");
    assert!(
        (wasm.contains("ADR-858") || wasm.contains("ADR-888"))
            && !wasm.contains("MirOp::ObjectRefWrap"),
        "WASM mir_emit must not match ObjectRefWrap after W544 peel; ADR-858/888 ok"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    let abi_deleted = !abi.contains("pub extern \"C\" fn syx_rt_object_ref_wrap");
    let historical = rt.contains("rt_abi_object_ref_wrap_still_present: true")
        || rt.contains("rt_abi_object_ref_wrap_still_present:true");
    assert!(
        (abi_deleted || historical)
            && (abi.contains("OBJECT_REF") || abi.contains("object_handle")),
        "RT opcode wrap ABI deleted (W530) or historical; W541 peel notes remain in abi.rs"
    );
    assert!(
        rt.contains("GateB09LlvmObjectrefWrapEmitRetireFirstSliceInventory")
            && rt.contains("gate_b09_llvm_objectref_wrap_emit_retire_first_slice_inventory")
            && rt.contains("ADR-858")
            && (rt.contains("b09_llvm_objectref_wrap_emit_retire_first_slice_signed: true")
                || rt.contains("b09_llvm_objectref_wrap_emit_retire_first_slice_signed:true")),
        "syx-rt must expose GateB09LlvmObjectrefWrapEmitRetireFirstSliceInventory"
    );
    assert!(
        (rt.contains("llvm_objectref_wrap_call_emit_retired: true")
            || rt.contains("llvm_objectref_wrap_call_emit_retired:true"))
            && (rt.contains("llvm_objectref_wrap_declare_omitted: true")
                || rt.contains("llvm_objectref_wrap_declare_omitted:true"))
            && (rt.contains("wasm_objectref_wrap_aot_already_nop: true")
                || rt.contains("wasm_objectref_wrap_aot_already_nop:true"))
            && (rt.contains("rt_abi_object_ref_wrap_still_present: true")
                || rt.contains("rt_abi_object_ref_wrap_still_present:true"))
            && (rt.contains("llvm_wasm_objectref_rc_still_live: true")
                || rt.contains("llvm_wasm_objectref_rc_still_live:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must register wrap emit retire and keep migrate/cleanup/enable false + RC live"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_llvm_objectref_wrap_emit_retire_first_slice_inventory")
            && lib.contains("GateB09LlvmObjectrefWrapEmitRetireFirstSliceInventory"),
        "syx-rt lib.rs must re-export LLVM wrap emit-retire inventory"
    );
}

#[test]
fn b09_llvm_objectref_wrap_emit_retire_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-llvm-objectref-wrap-emit-retire-first-slice.md");
    assert!(
        spec.contains("ADR-858")
            && (spec.contains("ObjectRefWrap") || spec.contains("wrap"))
            && (spec.contains("emit") || spec.contains("emit-path")),
        "spec must cite ADR-858 LLVM ObjectRefWrap emit retire"
    );
    assert!(
        spec.contains("three-hundred-and-sixty-fourth"),
        
        "spec must cite W514 fingerprint"
    );
    assert!(
        spec.contains("GateB09LlvmObjectrefWrapEmitRetireFirstSliceInventory")
            && spec.contains("gate_b09_llvm_objectref_wrap_emit_retire_first_slice_inventory"),
        "spec must name wrap emit-retire inventory API"
    );
    assert!(
        spec.contains("b09_llvm_objectref_wrap_emit_retire_first_slice_signed=true")
            || spec.contains("`b09_llvm_objectref_wrap_emit_retire_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("llvm_objectref_wrap_call_emit_retired=true")
            || spec.contains("`llvm_objectref_wrap_call_emit_retired=true`"))
            && (spec.contains("llvm_objectref_wrap_declare_omitted=true")
                || spec.contains("`llvm_objectref_wrap_declare_omitted=true`"))
            && (spec.contains("wasm_objectref_wrap_aot_already_nop=true")
                || spec.contains("`wasm_objectref_wrap_aot_already_nop=true`"))
            && (spec.contains("rt_abi_object_ref_wrap_still_present=true")
                || spec.contains("`rt_abi_object_ref_wrap_still_present=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`")),
        "spec must cite wrap retire flags and migrate/cleanup false + RT ABI still"
    );
    assert!(
        !spec.contains("objectref_rc_migrate_enabled=true")
            && !spec.contains("`objectref_rc_migrate_enabled=true`")
            && !spec.contains("unified_mir_runtime_heap_enabled=true")
            && !spec.contains("`unified_mir_runtime_heap_enabled=true`"),
        "spec must not claim full migrate or unified heap enabled"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("unified") || spec.contains("cleanup") || spec.contains("B-09")),
        "spec Frozen must fence enable / cleanup / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_llvm_objectref_wrap_emit_retire_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_llvm_objectref_wrap_emit_retire_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_arrayget_heap_slots_first_slice_floor_ok);
    assert!(inv.llvm_objectref_wrap_call_emit_retired);
    assert!(inv.llvm_objectref_wrap_declare_omitted);
    assert!(inv.wasm_objectref_wrap_aot_already_nop);
    assert!(inv.rt_abi_object_ref_wrap_still_present);
    assert!(inv.objectref_rc_storage_still_present);
    assert!(inv.llvm_wasm_objectref_rc_still_live);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_llvm_objectref_wrap_emit_retire_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_arrayget_heap_slots_first_slice_inventory();
    assert!(floor.vm_mir_run_arrayget_heap_slots_tracked);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(floor.b09_objectref_arrayget_heap_slots_first_slice_signed);
}

#[test]
fn b09_llvm_objectref_wrap_emit_retire_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W514")
            && (roadmap.contains("ADR-858")
                || roadmap.contains("ObjectRefWrap")
                || roadmap.contains("wrap emit")),
        "roadmap must cite W514 / ADR-858 LLVM ObjectRefWrap emit retire"
    );
    let b02 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-02 |"))
        .expect("B-02 row");
    assert!(b02.contains("[x]"), "roadmap B-02 must remain [x]: {b02}");
    let b09 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-09 |"))
        .expect("B-09 row");
    assert!(
        b09.contains("[x]") || b09.contains("[~]"),
        "roadmap B-09 must be [x] (W558+) or historical [~]: {b09}"
    );
}
