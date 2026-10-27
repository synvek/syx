//! ADR-141 / M1 W5: workspace from initialize + LSP top-level array shapes.

use super::*;
use std::fs;
use std::path::PathBuf;

fn temp_syx_project(tag: &str, main_src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_m1_w5_{tag}_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("syx.toml"), "name = \"m1w5\"\nmain = \"main.sy\"\n").unwrap();
    fs::write(dir.join("main.sy"), main_src).unwrap();
    dir
}

#[test]
fn m1_w5_parse_initialize_root_uri() {
    let params = json!({
        "rootUri": "file:///tmp/syx_ws_root",
        "capabilities": {}
    });
    let roots = parse_initialize_workspace_roots(Some(&params));
    assert_eq!(roots.len(), 1);
    assert!(roots[0].ends_with("syx_ws_root") || roots[0].to_string_lossy().contains("syx_ws_root"));
}

#[test]
fn m1_w5_parse_initialize_workspace_folders_fallback() {
    let params = json!({
        "rootUri": null,
        "workspaceFolders": [
            { "uri": "file:///tmp/folder_a", "name": "a" },
            { "uri": "file:///tmp/folder_b", "name": "b" }
        ]
    });
    let roots = parse_initialize_workspace_roots(Some(&params));
    assert_eq!(roots.len(), 1);
    assert!(roots[0].to_string_lossy().contains("folder_a"));
}

#[test]
fn m1_w5_workspace_symbol_empty_without_roots() {
    let syms = workspace_symbols("main", &[], &DriverOptions::default());
    assert_eq!(syms, json!([]));
}

#[test]
fn m1_w5_initialize_sets_workspace_root_and_finds_symbol() {
    let dir = temp_syx_project(
        "sym",
        "func helperAlpha() -> Int { return 1 }\nfunc main() { print(helperAlpha()) }\n",
    );
    let uri = format!("file://{}", dir.display());
    let params = json!({ "rootUri": uri, "capabilities": {} });
    let roots = parse_initialize_workspace_roots(Some(&params));
    assert_eq!(roots, vec![dir.clone()]);

    let syms = workspace_symbols("helperAlpha", &roots, &DriverOptions::default());
    let arr = syms.as_array().expect("top-level array");
    assert!(
        arr.iter().any(|s| s["name"].as_str() == Some("helperAlpha")),
        "symbols={arr:?}"
    );
    // Must not require conformance_cross_module fixture path.
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../syx-driver/tests/fixtures/projects/conformance_cross_module/app");
    assert_ne!(roots[0], fixture);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn m1_w5_code_action_result_is_top_level_array() {
    let actions = code_actions_for_document("file:///t.sy", "func main() {}", &DriverOptions::default());
    assert!(actions.is_array(), "{actions}");
}

#[test]
fn m1_w5_inlay_hint_result_is_top_level_array() {
    let hints =
        inlay_hints_for_document("file:///t.sy", "func main() { let x = 1 }", 0, &DriverOptions::default());
    assert!(hints.is_array(), "{hints}");
}

#[test]
fn m1_w5_references_result_is_top_level_array() {
    let refs = references_for_position(
        "file:///t.sy",
        "func main() { let x = 1\n print(x) }",
        1,
        8,
        &DriverOptions::default(),
    );
    assert!(refs.is_array(), "{refs}");
}

#[test]
fn m1_w5_workspace_symbol_result_is_top_level_array() {
    let dir = temp_syx_project("shape", "func main() {}\n");
    let syms = workspace_symbols("main", &[dir.clone()], &DriverOptions::default());
    assert!(syms.is_array(), "{syms}");
    let _ = fs::remove_dir_all(&dir);
}
