//! Capability manifest honesty nails (source TOML ↔ generated JSON/docs).

use syx_driver::capabilities_report::{
    print_capabilities_report, print_ir_capabilities_report, run_capabilities_report,
    run_ir_capabilities_report,
};

#[test]
fn product_jit_is_experimental_real_subset() {
    let report = run_capabilities_report();
    let jit = report
        .rows
        .iter()
        .find(|r| r.id == "runtime.jit.tier2")
        .expect("runtime.jit.tier2");
    assert_eq!(jit.status, "experimental-real-subset");
    assert_ne!(jit.vm, "stub", "JIT vm backend must not remain stub");
}

#[test]
fn no_implemented_with_native_noop() {
    let report = run_capabilities_report();
    let bad: Vec<_> = report
        .rows
        .iter()
        .filter(|r| r.status == "implemented" && r.native == "noop")
        .map(|r| r.id.as_str())
        .collect();
    assert!(
        bad.is_empty(),
        "implemented + native=noop dishonest: {bad:?}"
    );
}

#[test]
fn host_fn_table_every_row_has_backends() {
    let json = include_str!("../../../../packages/internal/host_fn_capabilities.generated.json");
    let v: serde_json::Value = serde_json::from_str(json).expect("host fn json");
    let hosts = v["hosts"].as_array().expect("hosts array");
    assert!(hosts.len() >= 500);
    for host in hosts {
        let name = host["name"].as_str().unwrap_or("?");
        let backends = host["backends"].as_object().unwrap_or_else(|| {
            panic!("{name}: missing backends object")
        });
        for key in ["vm", "mir", "llvm", "wasm", "cranelift"] {
            assert!(
                backends.contains_key(key),
                "{name}: missing backend column {key}"
            );
        }
    }
}

#[test]
fn doctor_reports_smoke() {
    let caps = run_capabilities_report();
    print_capabilities_report(&caps);
    let ir = run_ir_capabilities_report();
    print_ir_capabilities_report(&ir);
    assert!(!caps.rows.is_empty());
    assert!(ir.host_count > 0);
}

#[test]
fn ci_wires_capability_generators() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("gen_capabilities.py --check"),
        "ci must freshness-check product capability manifest"
    );
    assert!(
        ci.contains("gen_ir_capabilities.py --check"),
        "ci must freshness-check HostFn/MirOp capability tables"
    );
}
