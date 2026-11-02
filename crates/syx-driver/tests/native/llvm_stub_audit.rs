//! Phase 91 + M0-A07 + W313: public syx-rt stub audit inventories.
//! Counts / POP≠GA nails; no stub semantic rewrite.
//! S2-I14: W315 declare-diff companion removed with orphan clear upstream feed.
//! S2-I17: W314 capability_align companion removed (HostFn←align decouple).

use syx_rt::{classify_llvm_stub_block, llvm_stub_audit_report, LlvmStubKind};

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
