//! GC narrative deepen
//! Construction checklist ADR-only; ≠ unified heap enable / ≠ GC GA / ≠ B-09 [x].

#[test]
fn b09_gc_narrative_deepen_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-gc-narrative-deepen.md",
        "docs/gates/gate-b09-gc-narrative-thin.md",
        "plan/topics/gc-parity.md",
        "plan/02-improvement-roadmap.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 GC narrative deepen requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09GcNarrativeDeepenInventory")
            && rt.contains("gate_b09_gc_narrative_deepen_inventory")
            && rt.contains("ADR-829")
            && (rt.contains("b09_gc_narrative_deepen_signed: true")
                || rt.contains("b09_gc_narrative_deepen_signed:true")),
        "syx-rt must expose GateB09GcNarrativeDeepenInventory (W485 / ADR-829)"
    );
    assert!(
        (rt.contains("unified_heap_construction_adr_documented: true")
            || rt.contains("unified_heap_construction_adr_documented:true"))
            && (rt.contains("mir_runtime_root_set_documented: true")
                || rt.contains("mir_runtime_root_set_documented:true"))
            && (rt.contains("vm_gccollect_converge_path_documented: true")
                || rt.contains("vm_gccollect_converge_path_documented:true"))
            && (rt.contains("wasm_gc_types_deferred_documented: true")
                || rt.contains("wasm_gc_types_deferred_documented:true"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false"))
            && (rt.contains("b09_axis_exit_enabled: false")
                || rt.contains("b09_axis_exit_enabled:false"))
            && (rt.contains("gc_ga_claimed: false") || rt.contains("gc_ga_claimed:false")),
        "deepen inventory must document checklist and keep product flags false"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_gc_narrative_deepen_inventory")
            && lib.contains("GateB09GcNarrativeDeepenInventory"),
        "syx-rt lib.rs must re-export B-09 GC narrative deepen inventory"
    );
}

#[test]
fn b09_gc_narrative_deepen_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b09-gc-narrative-deepen.md");
    assert!(
        spec.contains("ADR-829")
            && (spec.contains("deepen")
                || spec.contains("construction")
                || spec.contains("checklist")),
        "spec must cite ADR-829 B-09 GC narrative deepen"
    );
    assert!(
        spec.contains("three-hundred-and-thirty-fifth"),
        
        "spec must cite W485 fingerprint"
    );
    assert!(
        spec.contains("GateB09GcNarrativeDeepenInventory")
            && spec.contains("gate_b09_gc_narrative_deepen_inventory"),
        "spec must name B-09 deepen inventory API"
    );
    assert!(
        spec.contains("b09_gc_narrative_deepen_signed=true")
            || spec.contains("`b09_gc_narrative_deepen_signed=true`"),
        "spec must cite b09_gc_narrative_deepen_signed=true"
    );
    assert!(
        (spec.contains("mir_runtime_root_set_documented=true")
            || spec.contains("`mir_runtime_root_set_documented=true`"))
            && (spec.contains("vm_gccollect_converge_path_documented=true")
                || spec.contains("`vm_gccollect_converge_path_documented=true`"))
            && (spec.contains("wasm_gc_types_deferred_documented=true")
                || spec.contains("`wasm_gc_types_deferred_documented=true`"))
            && (spec.contains("unified_mir_runtime_heap_enabled=false")
                || spec.contains("`unified_mir_runtime_heap_enabled=false`")),
        "spec must cite checklist flags true and unified heap false"
    );
    assert!(
        spec.contains("stack")
            && spec.contains("GcCollect")
            && (spec.contains("wasm-gc") || spec.contains("wasm_gc") || spec.contains("ADR-137")),
        "spec construction checklist must name roots / GcCollect / wasm-gc or ADR-137"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("GC") || spec.contains("unified") || spec.contains("B-09")),
        "spec Frozen must fence GC GA / unified heap / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_gc_narrative_deepen_inventory_nails() {
    let inv = syx_rt::gate_b09_gc_narrative_deepen_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_gc_narrative_thin_floor_ok);
    assert!(inv.unified_heap_construction_adr_documented);
    assert!(inv.mir_runtime_root_set_documented);
    assert!(inv.vm_gccollect_converge_path_documented);
    assert!(inv.wasm_gc_types_deferred_documented);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_gc_narrative_deepen_signed);
    let floor = syx_rt::gate_b09_gc_narrative_thin_inventory();
    assert!(!floor.unified_mir_runtime_heap_enabled);
    assert!(!floor.b09_axis_exit_enabled);
    assert!(!floor.gc_ga_claimed);
    assert!(floor.b09_gc_narrative_thin_signed);
}

#[test]
fn b09_gc_narrative_deepen_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W485")
            && (roadmap.contains("ADR-829")
                || roadmap.contains("deepen")
                || roadmap.contains("b09_gc_narrative_deepen")
                || roadmap.contains("B-09")),
        "roadmap must cite W485 / ADR-829 B-09 GC narrative deepen"
    );
    let b09 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-09 |"))
        .expect("B-09 row");
    assert!(
        b09.contains("| [x] |") || b09.contains("| [~] |"),
        "B-09 must be [x] (W558+) or historical [~] after W485 deepen: {b09}"
    );
    let b02 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-02 |"))
        .expect("B-02 row");
    assert!(b02.contains("| [x] |"), "B-02 must remain [x]: {b02}");
}
