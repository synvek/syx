//! real HostFn lowering fourth-slice wire
//! TcpWrite / tcp_write only; ≠ B-07 [x] / ≠ batch rewrite / ≠ MIR blob.

#[test]
fn b07_real_hostfn_lowering_tcp_write_wire_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/gates/gate-b07-real-hostfn-lowering-tcp-write-wire.md",
        "docs/gates/gate-b07-real-hostfn-lowering-tcp-read-wire.md",
        "docs/gates/gate-b07-real-hostfn-lowering-file-read-wire.md",
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
            "B-07 TcpWrite wire requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/llvm_stub_audit.rs");
    assert!(
        rt.contains("GateHostFnRealLoweringHorizonFourthSliceWireInventory")
            && rt.contains("gate_hostfn_real_lowering_horizon_fourth_slice_wire_inventory")
            && rt.contains("b07_real_lowering_fourth_slice_wire_signed")
            && rt.contains("ADR-798"),
        "syx-rt must expose B-07 fourth-slice wire inventory (W454 / ADR-798)"
    );
}

#[test]
fn b07_real_hostfn_lowering_tcp_write_wire_stub_tcp_write() {
    let stubs = include_str!("../../../syx-rt/src/llvm_stubs.rs");
    let start = stubs
        .find("fn syx_rt_tcp_write")
        .expect("syx_rt_tcp_write must exist");
    let after = &stubs[start..];
    let end = after.find("\n#[no_mangle]").unwrap_or(after.len());
    let body = &after[..end];
    assert!(
        body.contains("HostFnId::TcpWrite") && body.contains("rt_call_host_native"),
        "tcp_write must route to HostFnId::TcpWrite: {body}"
    );
    assert!(
        !body.contains("rt_direct_exec"),
        "tcp_write must leave rt_direct_exec(POP): {body}"
    );
}

#[test]
fn b07_real_hostfn_lowering_tcp_write_wire_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b07-real-hostfn-lowering-tcp-write-wire.md");
    assert!(
        spec.contains("ADR-798") && (spec.contains("TcpWrite") || spec.contains("tcp_write")),
        "spec must cite ADR-798 TcpWrite wire"
    );
    assert!(
        spec.contains("three-hundred-and-fourth"),
        
        "spec must cite W454 fingerprint"
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
fn b07_real_hostfn_lowering_tcp_write_wire_live_inventory() {
    use syx_rt::gate_hostfn_real_lowering_horizon_fourth_slice_wire_inventory;
    let inv = gate_hostfn_real_lowering_horizon_fourth_slice_wire_inventory();
    assert!(inv.b07_real_lowering_fourth_slice_wire_signed);
    assert!(inv.b07_real_lowering_third_slice_wire_signed);
    assert!(inv.b07_real_lowering_second_slice_wire_signed);
    assert!(inv.b07_real_lowering_first_slice_wire_signed);
    assert!(inv.b07_real_lowering_thin_signed);
    assert!(inv.b07_real_lowering_deepen_thin_signed);
    assert!(inv.first_slice_wire_enabled);
    assert!(inv.second_slice_wire_enabled);
    assert!(inv.third_slice_wire_enabled);
    assert!(inv.fourth_slice_wire_enabled);
    assert!(!inv.lowering_enabled);
    assert!(inv.relational_third_slice_floor_ok, "{inv:?}");
    assert!(inv.relational_tcp_write_wire_body_ok, "{inv:?}");
    assert!(inv.relational_inventory_overlap_ok);
}

#[test]
fn b07_real_hostfn_lowering_tcp_write_wire_roadmap_still_tilde() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W454")
            && (roadmap.contains("ADR-798")
                || (roadmap.contains("TcpWrite") && roadmap.contains("wire"))),
        "roadmap must cite W454 / ADR-798 TcpWrite wire"
    );
    let b07 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-07 |"))
        .expect("B-07 row");
    assert!(
        b07.contains("| [~] |") || b07.contains("| [x] |"),
        "B-07 must be [~] or [x] after narrow exit / W456"
    );
    // W456: B-07 may be [x] after narrow exit charter (ADR-800)
    for id in ["B-04", "B-05", "B-06", "B-08"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(row.contains("| [x] |"), "{id} must remain [x]: {row}");
    }
    let b09 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-09 |"))
        .expect("B-09 row");
    assert!(
        b09.contains("| [x] |") || b09.contains("| [~] |") || b09.contains("| [ ] |"),
        "B-09 must be [x] (W558+) or historical [~] (or historical [ ]): {b09}"
    );
}

#[test]
fn b07_real_hostfn_lowering_tcp_write_wire_honesty_no_exit_no_blob() {
    let spec = include_str!("../../../../docs/gates/gate-b07-real-hostfn-lowering-tcp-write-wire.md");
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
