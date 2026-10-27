//! Phase 91 + M0-A07 + W313–W315: public syx-rt stub audit / align / declare-diff inventories.
//! Counts / POP≠GA / capability cross / emit-declare diff nails; no stub semantic rewrite.

use syx_rt::{
    classify_llvm_stub_block, llvm_stub_audit_report, llvm_stub_capability_align_inventory,
    llvm_stub_declare_diff_inventory, LlvmStubKind,
};

#[test]
fn p98_llvm_stub_audit_counts_symbols() {
    let r = llvm_stub_audit_report();
    assert!(
        r.total >= 80,
        "expected many syx_rt_* symbols, found {}",
        r.total
    );
    assert!(
        r.wired_noop >= 1,
        "expected POP WiredNoop stubs classified, found {}",
        r.wired_noop
    );
    assert!(
        r.wired + r.wired_noop >= 20,
        "expected wired+wired_noop stubs, found wired={} wired_noop={}",
        r.wired,
        r.wired_noop
    );
    assert!(
        r.cold >= 1,
        "expected cold tier_reduce stubs, found {}",
        r.cold
    );
    assert!(
        r.noop <= 5,
        "expected few empty noop stubs, found {}",
        r.noop
    );
}

#[test]
fn p98_llvm_stub_audit_tracks_noop_ratio() {
    let r = llvm_stub_audit_report();
    let noop_ratio = (r.noop + r.wired_noop) as f64 / r.total as f64;
    eprintln!(
        "llvm_stubs audit: total={} wired={} wired_noop={} noop={} unknown={} cold={} noopish_ratio={:.2}",
        r.total, r.wired, r.wired_noop, r.noop, r.unknown, r.cold, noop_ratio
    );
    assert_eq!(
        r.wired + r.wired_noop + r.noop + r.unknown + r.cold,
        r.total,
        "audit buckets must cover all symbols"
    );
}

#[test]
fn m0_a07_pop_is_wired_noop_not_wired_ga() {
    let sample = r#"
        let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
    "#;
    assert_eq!(classify_llvm_stub_block(sample), LlvmStubKind::WiredNoop);
    let real = r#"
        let _ = rt_direct_exec(syx_mir_run::op::CALL_HOST, 1, 0);
    "#;
    assert_eq!(classify_llvm_stub_block(real), LlvmStubKind::Wired);
}

#[test]
fn w314_llvm_stub_capability_align_inventory_floors() {
    let r = llvm_stub_audit_report();
    let a = llvm_stub_capability_align_inventory();
    assert_eq!(a.stub_total, r.total);
    assert_eq!(a.stub_wired, r.wired);
    assert!(
        a.hostfn_llvm_real >= 100,
        "hostfn_llvm_real={}",
        a.hostfn_llvm_real
    );
    assert!(a.named_aot_pop >= 1, "named_aot_pop={}", a.named_aot_pop);
    assert!(
        a.named_aot_wired >= 1,
        "named_aot_wired={}",
        a.named_aot_wired
    );
    assert!(a.call_host_path_wired, "call_host path must be Wired");
    assert!(
        a.hostfn_bound_wired >= 1,
        "hostfn_bound_wired={}",
        a.hostfn_bound_wired
    );
    eprintln!("align inventory: {a:?}");
}

#[test]
fn w315_llvm_stub_declare_diff_inventory_floors() {
    let r = llvm_stub_audit_report();
    let d = llvm_stub_declare_diff_inventory();
    assert_eq!(
        d.defined_count, r.total,
        "defined_count must match audit total"
    );
    assert!(
        d.declared_count >= 300,
        "declared_count={}",
        d.declared_count
    );
    assert!(d.defined_count >= 80, "defined_count={}", d.defined_count);
    assert!(
        d.missing_in_stubs >= 1,
        "missing_in_stubs={}",
        d.missing_in_stubs
    );
    assert!(d.extra_in_stubs >= 1, "extra_in_stubs={}", d.extra_in_stubs);
    assert_eq!(d.missing_not_in_rt, 0, "missing_not_in_rt must be 0: {d:?}");
    // Honesty: missing_in_stubs ≠ unlinkable (abi covers most).
    assert!(
        d.abi_cover_count >= 1,
        "abi_cover_count={}",
        d.abi_cover_count
    );
    eprintln!("declare-diff inventory: {d:?}");
}
