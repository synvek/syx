//! Long closure-async MIR-free thin-open inventory (W414 / ADR-758).
//! Public inventory API + policy/meta; gap remains open; ≠ MIR-free / ≠ delete sidecar.

#[test]
fn long_mir_free_closure_async_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "docs/lang/long-mir-free-closure-async.md",
        "docs/lang/long-mir-free-emit-omit.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long closure-async thin requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonClosureAsyncMirFreeThinInventory")
            && mir.contains("mir_module_long_horizon_closure_async_mir_free_thin_inventory"),
        "syx-mir must expose MirModuleLongHorizonClosureAsyncMirFreeThinInventory (W414)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && llvm.contains("mir_function_needs_mir_entry")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit: empty+register; no sidecar/blob (post-W431; W414 historical)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("closure_async_mir_free_thin_signed")
            && wave_b.contains("closure_async_mir_free_gap_open"),
        "wave_b_horizon must expose closure-async thin flags (W414)"
    );
}

#[test]
fn long_mir_free_closure_async_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-closure-async.md");
    assert!(
        spec.contains("ADR-758")
            && (spec.contains("closure-async") || spec.contains("closure async")),
        "spec must cite ADR-758 W414 Long closure-async thin: missing markers"
    );
    assert!(
        spec.contains("MirModuleLongHorizonClosureAsyncMirFreeThinInventory")
            && spec.contains("mir_module_long_horizon_closure_async_mir_free_thin_inventory"),
        "spec must name public Long closure-async thin inventory API"
    );
    assert!(
        spec.contains("mir_function_needs_mir_entry")
            && (spec.contains("AwaitPoll") || spec.contains("Await")),
        "spec must cite mir_function_needs_mir_entry / Await contract"
    );
    assert!(
        spec.contains("closure_async_mir_free_thin_signed=true")
            || spec.contains("`closure_async_mir_free_thin_signed=true`"),
        "spec must cite closure_async_mir_free_thin_signed=true"
    );
    assert!(
        spec.contains("closure_async_mir_free_gap_open=true")
            || spec.contains("`closure_async_mir_free_gap_open=true`")
            || (spec.contains("still open") && spec.contains("closure")),
        "spec must cite closure_async_mir_free_gap_open still open"
    );
    assert!(
        spec.contains("mir_free_enabled=false") || spec.contains("`mir_free_enabled=false`"),
        "spec must cite mir_free_enabled=false"
    );
    assert!(
        spec.contains("sidecar_constant_still_embedded=true")
            || spec.contains("`sidecar_constant_still_embedded=true`"),
        "spec must cite sidecar constant still embedded"
    );
    for line in spec.lines() {
        if line.contains("gap")
            && (line.contains("closed") || line.contains("close"))
            && !line.contains("≠")
            && !line.contains("not")
            && !line.contains("NOT")
            && !line.contains("仍")
            && !line.contains("still")
            && !line.contains("forbidden")
            && !line.contains("Frozen")
            && !line.contains("unmet")
            && !line.contains("open")
        {
            panic!("affirmative gap close forbidden: {line}");
        }
    }
}

