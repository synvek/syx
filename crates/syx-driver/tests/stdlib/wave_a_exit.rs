//! Wave A structural exit charter evidence nails
//! Policy + meta flip only — top-level sign-off of plan/02 exit triad.

#[test]
fn wave_a_exit_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        // F-10 / W575: a02 pure-meta gate retired; Q-L2 behavior floor = differential Suite-1/2.
        "crates/syx-driver/tests/differential/suite1.rs",
        "crates/syx-driver/tests/differential/suite2.rs",
        "plan/archive/gates/gate-a02-ql2-floor.md",
        // F-10 / W576: a05 pure-meta gate retired; Q-S1 behavior floor = qs1 five-domain + matrix.
        "crates/syx-driver/tests/stdlib/hostfn_file.rs",
        "crates/syx-driver/tests/stdlib/hostfn_http.rs",
        "crates/syx-driver/tests/stdlib/hostfn_tcp.rs",
        "crates/syx-driver/tests/stdlib/hostfn_process.rs",
        "crates/syx-driver/tests/stdlib/hostfn_sql.rs",
        "packages/internal/hostfn_qs1_matrix.toml",
        "plan/archive/gates/gate-a05-qs1-floor.md",
        // F-10 / W577: a04 exit meta retired; Q-L4 behavior floor = 6 modules + edge-semantics (stays live).
        "docs/gates/gate-a04-edge-semantics.md",
        "crates/syx-driver/tests/stdlib/edge_deinit_nil.rs",
        "crates/syx-driver/tests/stdlib/edge_exception.rs",
        "crates/syx-driver/tests/stdlib/edge_overflow.rs",
        "crates/syx-driver/tests/stdlib/edge_nan.rs",
        "crates/syx-driver/tests/stdlib/edge_unicode.rs",
        "crates/syx-driver/tests/stdlib/edge_actor.rs",
        "docs/lang/wave-a-exit.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Wave A exit requires evidence file: {path}"
        );
    }
}
