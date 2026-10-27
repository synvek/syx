//! B-03 FrontendPipeline entry matrix (W461 / ADR-805).
//! Six entries × StopAfter documented; ≠ Pipeline API mutation / ≠ frontend exports.

#[test]
fn frontend_pipeline_entry_matrix_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-frontend/src/lib.rs",
        "docs/lang/frontend-pipeline-entry-matrix.md",
        "crates/syx-driver/tests/lang/frontend_pipeline_parity.rs",
        "plan/02-improvement-roadmap.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-03 entry matrix requires evidence file: {path}"
        );
    }
    let fe = include_str!("../../../syx-frontend/src/lib.rs");
    assert!(
        fe.contains("FrontendPipelineEntryMatrixInventory")
            && fe.contains("frontend_pipeline_entry_matrix_inventory")
            && fe.contains("ADR-805")
            && fe.contains("FrontendPipeline")
            && fe.contains("StopAfter")
            && (fe.contains("b03_pipeline_entry_matrix_signed: true")
                || fe.contains("b03_pipeline_entry_matrix_signed:true")),
        "syx-frontend must expose FrontendPipelineEntryMatrixInventory (W461 / ADR-805)"
    );
    assert!(
        fe.contains("StopAfter::Expanded")
            && fe.contains("Prepared")
            && fe.contains("Semantic")
            && fe.contains("Mir"),
        "StopAfter variants must remain present"
    );
    assert!(
        fe.contains("pipeline_api_mutation_enabled: false")
            || fe.contains("pipeline_api_mutation_enabled:false"),
        "inventory must keep pipeline_api_mutation_enabled=false"
    );
}

#[test]
fn frontend_pipeline_entry_matrix_spec_signed() {
    let spec = include_str!("../../../../docs/lang/frontend-pipeline-entry-matrix.md");
    assert!(
        spec.contains("ADR-805") && (spec.contains("entry matrix") || spec.contains("Entry ×")),
        "spec must cite ADR-805 entry matrix"
    );
    assert!(
        spec.contains("three-hundred-and-eleventh"),
        
        "spec must cite W461 fingerprint"
    );
    assert!(
        spec.contains("FrontendPipelineEntryMatrixInventory")
            && spec.contains("frontend_pipeline_entry_matrix_inventory"),
        "spec must name entry matrix inventory API"
    );
    assert!(
        spec.contains("b03_pipeline_entry_matrix_signed=true")
            || spec.contains("`b03_pipeline_entry_matrix_signed=true`"),
        "spec must cite b03_pipeline_entry_matrix_signed=true"
    );
    assert!(
        spec.contains("pipeline_api_mutation_enabled=false")
            || spec.contains("`pipeline_api_mutation_enabled=false`"),
        "spec must cite pipeline_api_mutation_enabled=false"
    );
    for entry in ["file", "project", "embed", "LSP", "REPL", "browser"] {
        assert!(spec.contains(entry), "spec matrix must name entry: {entry}");
    }
    for stop in ["Expanded", "Prepared", "Semantic", "Mir"] {
        assert!(spec.contains(stop), "spec must document StopAfter::{stop}");
    }
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("Pipeline") || spec.contains("Export") || spec.contains("daemon")),
        "spec Frozen must fence API mutation / exports / daemon"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn frontend_pipeline_entry_matrix_inventory_nails() {
    let inv = syx_frontend::frontend_pipeline_entry_matrix_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b03_pipeline_entry_matrix_signed);
    assert_eq!(inv.documented_entry_count, 6);
    assert!(inv.stop_after_variants_documented);
    assert!(inv.frontend_pipeline_api_present);
    assert!(!inv.pipeline_api_mutation_enabled);
    assert!(inv.b03_axis_exit_enabled);
}

#[test]
fn frontend_pipeline_entry_matrix_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W461")
            && (roadmap.contains("ADR-805")
                || roadmap.contains("entry matrix")
                || roadmap.contains("FrontendPipeline")
                || roadmap.contains("B-03")),
        "roadmap must cite W461 / ADR-805 B-03 entry matrix"
    );
    let b03 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-03 |"))
        .expect("B-03 row");
    assert!(
        b03.contains("| [x] |"),
        "B-03 must be [x] after W461: {b03}"
    );
    for id in ["B-04", "B-05", "B-06", "B-07", "B-08"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(row.contains("| [x] |"), "{id} must remain [x]: {row}");
    }
    for id in ["B-01", "B-02"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        if id == "B-01" {
            assert!(
                row.contains("| [x] |") || row.contains("| [~] |"),
                "B-01 must be [x] after W474 exit (or historical [~]): {row}"
            );
        } else {
            assert!(
                row.contains("| [x] |") || row.contains("| [~] |"),
                "{id} must be [x] or historical [~]: {row}"
            );
        }
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
