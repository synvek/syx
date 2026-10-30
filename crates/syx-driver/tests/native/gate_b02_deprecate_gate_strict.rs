//! strict deprecate gate
//! compile/compile_into* #[deprecated]; external compile sites 0; ≠ retired / ≠ B-02 [x].

#[test]
fn b02_deprecate_gate_strict_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-codegen/src/lib.rs",
        "crates/syx-codegen/tests/float_literal.rs",
        "docs/gates/gate-b02-deprecate-gate-strict.md",
        "docs/gates/gate-b02-compile-into-first-slice.md",
        "plan/02-improvement-roadmap.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-02 strict deprecate gate requires evidence file: {path}"
        );
    }
    let cg = include_str!("../../../syx-codegen/src/lib.rs");
    assert!(
        cg.contains("GateB02DeprecateGateStrictInventory")
            && cg.contains("gate_b02_deprecate_gate_strict_inventory")
            && cg.contains("ADR-822")
            && (cg.contains("b02_deprecate_gate_strict_signed: true")
                || cg.contains("b02_deprecate_gate_strict_signed:true")),
        "syx-codegen must expose GateB02DeprecateGateStrictInventory (W478 / ADR-822)"
    );
    // W482 honesty: live public APIs may be deleted; nail historical gate inventory fields only.
    assert!(
        cg.contains("fn compile_with_mir")
            && (cg.contains("deprecated_attrs_present: true")
                || cg.contains("deprecated_attrs_present:true"))
            && (cg.contains("compile_into_api_present: true")
                || cg.contains("compile_into_api_present:true")),
        "gate inventory historical floor: deprecated_attrs + api_present true"
    );
    assert!(
        (cg.contains("b02_dual_track_retired: false")
            || cg.contains("b02_dual_track_retired:false"))
            && (cg.contains("b02_axis_exit_enabled: false")
                || cg.contains("b02_axis_exit_enabled:false"))
            && (cg.contains("deprecate_gate_strict_enabled: true")
                || cg.contains("deprecate_gate_strict_enabled:true")),
        "inventory must keep retire/exit false and gate true"
    );
    let float_lit = include_str!("../../../syx-codegen/tests/float_literal.rs");
    assert!(
        !float_lit.contains("CodeGenerator::compile(&")
            && !float_lit.contains("CodeGenerator::compile( &")
            && float_lit.contains("compile_with_mir")
            && float_lit.contains("lower_module"),
        "float_literal must use MIR path with zero AST-only compile calls"
    );
    let driver = include_str!("../../src/lib.rs");
    assert!(
        !driver.contains("CodeGenerator::compile_into(")
            && !driver.contains("CodeGenerator::compile_into_with_semantic("),
        "driver production must not call compile_into*"
    );
}

#[test]
fn b02_deprecate_gate_strict_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b02-deprecate-gate-strict.md");
    assert!(
        spec.contains("ADR-822") && (spec.contains("deprecate") || spec.contains("gate")),
        "spec must cite ADR-822 B-02 strict deprecate gate"
    );
    assert!(
        spec.contains("three-hundred-and-twenty-eighth"),
        
        "spec must cite W478 fingerprint"
    );
    assert!(
        spec.contains("GateB02DeprecateGateStrictInventory")
            && spec.contains("gate_b02_deprecate_gate_strict_inventory"),
        "spec must name deprecate-gate inventory API"
    );
    assert!(
        spec.contains("b02_deprecate_gate_strict_signed=true")
            || spec.contains("`b02_deprecate_gate_strict_signed=true`"),
        "spec must cite b02_deprecate_gate_strict_signed=true"
    );
    assert!(
        spec.contains("deprecate_gate_strict_enabled=true")
            || spec.contains("`deprecate_gate_strict_enabled=true`"),
        "spec must cite deprecate_gate_strict_enabled=true"
    );
    assert!(
        spec.contains("external_compile_call_sites_zero=true")
            || spec.contains("`external_compile_call_sites_zero=true`"),
        "spec must cite external_compile_call_sites_zero=true"
    );
    assert!(
        spec.contains("b02_dual_track_retired=false")
            || spec.contains("`b02_dual_track_retired=false`"),
        "spec must cite b02_dual_track_retired=false"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("B-02") || spec.contains("compile") || spec.contains("Export")),
        "spec Frozen must fence B-02 [x] / delete compile / Export"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b02_deprecate_gate_strict_inventory_nails() {
    let inv = syx_codegen::gate_b02_deprecate_gate_strict_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.first_slice_floor_ok);
    assert!(inv.deprecated_attrs_present);
    assert!(inv.external_compile_call_sites_zero);
    assert!(inv.compile_into_production_call_sites_zero);
    assert!(inv.compile_into_api_present);
    assert!(!inv.b02_dual_track_retired);
    assert!(!inv.b02_axis_exit_enabled);
    assert!(inv.deprecate_gate_strict_enabled);
    assert!(inv.b02_deprecate_gate_strict_signed);
    let first = syx_codegen::gate_b02_compile_into_first_slice_inventory();
    assert!(first.b02_compile_into_first_slice_signed);
    assert!(!first.b02_dual_track_retired);
}

