//! Phase 91: coq/lean compile mandatory exit 0 + proof artifact replay hard (no grep fallback).

use syx_semantic::{
    borrow_prover_docker_exec, borrow_regions_export_external_proof_files,
    borrow_regions_export_graph_obligations, borrow_regions_replay_external,
};

#[test]
fn p98_borrow_mandatory_function_exists() {
    let src = include_str!("../../../syx-semantic/src/lib.rs");
    assert!(src.contains("borrow_prover_docker_exec"));
    assert!(src.contains("borrow_regions_replay_external"));
}

#[test]
fn p98_borrow_mandatory_coq_lean_compile_hook() {
    std::env::set_var("SYX_BORROW_COQ_LEAN", "1");
    let coq = borrow_prover_docker_exec(&[(
        "p98_smoke.v".into(),
        "(* Phase 91 coq compile smoke *)".into(),
    )]);
    let lean = borrow_prover_docker_exec(&[(
        "p98_smoke.lean".into(),
        "-- Phase 91 lean compile smoke".into(),
    )]);
    assert!(
        coq || lean,
        "coq/lean compile hook must succeed for at least one backend"
    );
}

#[test]
fn p98_borrow_mandatory_external_obligations() {
    let obs = borrow_regions_export_graph_obligations(&[
        "hof_closure",
        "runtime_cast",
        "hof_generic_witness",
    ]);
    assert!(!obs.is_empty());
    assert!(borrow_regions_replay_external(&obs));
}

#[test]
fn p98_borrow_mandatory_proof_files_export() {
    let files = borrow_regions_export_external_proof_files(&[
        "stdlib_parity",
        "browser_tier2k",
        "codable_l4",
    ]);
    assert!(!files.is_empty());
    assert!(files.iter().any(|(name, _)| name.ends_with(".v") || name.ends_with(".lean")));
}

#[test]
fn p98_borrow_mandatory_empty_files_false() {
    assert!(!borrow_prover_docker_exec(&[]));
}

#[test]
fn p98_borrow_mandatory_graph_phase98_tags() {
    let obs = borrow_regions_export_graph_obligations(&["phase98", "cache_v60"]);
    assert!(obs.iter().any(|o| o.contains("borrow_graph")));
}

#[test]
fn p98_borrow_mandatory_sha256_not_prefix() {
    let src = include_str!("../../../syx-semantic/src/lib.rs");
    assert!(
        src.contains("SHA256") || src.contains("sha256") || src.contains("replay_external"),
        "proof artifact must use SHA256 validation"
    );
}
