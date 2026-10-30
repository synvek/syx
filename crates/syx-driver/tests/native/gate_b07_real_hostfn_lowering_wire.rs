//! real HostFn lowering first-slice wire
//! HttpGet / http_get_aot only; ≠ B-07 [x] / ≠ batch rewrite / ≠ MIR blob.

#[test]
fn b07_real_hostfn_lowering_wire_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/gates/gate-b07-real-hostfn-lowering-wire.md",
        "docs/gates/gate-b07-real-hostfn-lowering-deepen.md",
        "docs/gates/gate-b07-real-hostfn-lowering.md",
        "crates/syx-rt/src/llvm_stubs.rs",
        "crates/syx-rt/src/llvm_stub_audit.rs",
        "plan/02-improvement-roadmap.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-07 wire requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/llvm_stub_audit.rs");
    assert!(
        rt.contains("GateHostFnRealLoweringHorizonFirstSliceWireInventory")
            && rt.contains("gate_hostfn_real_lowering_horizon_first_slice_wire_inventory")
            && rt.contains("b07_real_lowering_first_slice_wire_signed")
            && rt.contains("ADR-795"),
        "syx-rt must expose B-07 first-slice wire inventory (W451 / ADR-795)"
    );
}

#[test]
fn b07_real_hostfn_lowering_wire_stub_http_get_aot() {
    let stubs = include_str!("../../../syx-rt/src/llvm_stubs.rs");
    let start = stubs
        .find("fn syx_rt_http_get_aot")
        .expect("syx_rt_http_get_aot must exist");
    let after = &stubs[start..];
    let end = after.find("\n#[no_mangle]").unwrap_or(after.len());
    let body = &after[..end];
    assert!(
        body.contains("HostFnId::HttpGet") && body.contains("rt_call_host_native"),
        "http_get_aot must route to HostFnId::HttpGet: {body}"
    );
    assert!(
        !body.contains("rt_direct_exec"),
        "http_get_aot must leave rt_direct_exec(POP): {body}"
    );
}

#[test]
fn b07_real_hostfn_lowering_wire_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b07-real-hostfn-lowering-wire.md");
    assert!(
        spec.contains("ADR-795") && (spec.contains("HttpGet") || spec.contains("http_get")),
        "spec must cite ADR-795 HttpGet wire"
    );
    assert!(
        spec.contains("three-hundred-and-first"),
        
        "spec must cite W451 fingerprint"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("B-07 `[x]`")
                || spec.contains("B-07 [x]")
                || spec.contains("batch")
                || spec.contains("exports")),
        "spec Frozen must fence B-07 [x] / batch / exports"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b07_real_hostfn_lowering_wire_live_inventory() {
    use syx_rt::gate_hostfn_real_lowering_horizon_first_slice_wire_inventory;
    let inv = gate_hostfn_real_lowering_horizon_first_slice_wire_inventory();
    assert!(inv.b07_real_lowering_first_slice_wire_signed);
    assert!(inv.b07_real_lowering_thin_signed);
    assert!(inv.b07_real_lowering_deepen_thin_signed);
    assert!(inv.first_slice_wire_enabled);
    assert!(!inv.lowering_enabled);
    assert!(inv.relational_deepen_floor_ok, "{inv:?}");
    assert!(inv.relational_http_get_wire_body_ok, "{inv:?}");
    assert!(inv.relational_inventory_overlap_ok);
}


#[test]
fn b07_real_hostfn_lowering_wire_honesty_no_exit_no_blob() {
    let spec = include_str!("../../../../docs/gates/gate-b07-real-hostfn-lowering-wire.md");
    assert!(
        spec.contains("lowering_enabled=false")
            || spec.contains("`lowering_enabled=false`")
            || spec.contains("Not** B-07 `[x]`")
            || spec.contains("**Not** B-07"),
        "wire must honesty-fence B-07 [x] / global lowering off"
    );
    assert!(
        !(spec.contains("this window deleted") && spec.contains("mir_blob")),
        "must not claim MIR blob delete this window"
    );
    let caps = include_str!("../../../../packages/internal/host_fn_capabilities.generated.json");
    assert!(
        caps.contains("\"count\": 629"),
        "HostFn count must remain 629 in generated caps"
    );
}
