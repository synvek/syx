//! M1 model v1 conformance nails — architecture-decisions SSOT + runtime gates.
// was: model_v1_nails

use std::path::PathBuf;
use std::sync::Arc;

use syx_async::{CancellationToken, TaskGroup};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn m1_w3_architecture_documents_model_v1() {
    let arch = std::fs::read_to_string(repo_root().join("docs/adr/architecture-decisions.md")).unwrap();
    assert!(
        (arch.contains("## Memory & Safety") || arch.contains("## Memory Model"))
            && arch.contains("## Error Model")
            && arch.contains("## Concurrency Model"),
        "architecture-decisions must document Memory / Error / Concurrency Model sections"
    );
    assert!(
        arch.contains("M1-C04") || arch.contains("ARC") || arch.contains("Weak"),
        "memory model section must retain M1 memory contracts"
    );
}

#[test]
fn m1_w3_error_migration_inventory_lists_tcp_facade() {
    let toml = std::fs::read_to_string(
        repo_root().join("packages/internal/error_model_migration.toml"),
    )
    .unwrap();
    assert!(toml.contains("packages/net/tcp.sy"));
    assert!(toml.contains("packages/io/file_handle.sy"));
}

#[test]
fn m1_w3_cancellation_parent_propagates_to_child_token() {
    let parent = CancellationToken::default();
    let child = CancellationToken::with_parent(parent.clone());
    assert!(!child.is_cancelled());
    parent.cancel();
    assert!(
        child.is_cancelled(),
        "child token must observe parent cancel (task-tree contract)"
    );
}

#[test]
fn m1_w3_taskgroup_cancel_all_cancels_group_token() {
    let parent = CancellationToken::default();
    let group = Arc::new(TaskGroup::new(parent.clone()));
    assert!(!group.parent_cancel.is_cancelled());
    group.cancel_all();
    assert!(group.parent_cancel.is_cancelled());
}

#[test]
fn m1_w3_weak_hosts_registered() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("__syx_weak_from"));
    assert!(reg.contains("__syx_weak_alive"));
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/lang/weak_nil_on_drop.sy");
    assert!(path.is_file(), "weak conformance fixture for future green");
}
