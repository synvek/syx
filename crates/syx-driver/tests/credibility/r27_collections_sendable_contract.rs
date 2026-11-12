//! R-27 Collections public Object → Sendable contract.
//!
//! Array / Dictionary / Set element·key façades spell `Sendable`.
//! Inventory pin: ≤180 / ≤10 (R-41); simd Object residual ≥159 (R-42 charter parked).

use std::path::PathBuf;
use std::process::Command;

fn repo_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn run_census() -> String {
    let script = repo_path("scripts/r27_collections_object_census.sh");
    let out = Command::new("bash")
        .arg(&script)
        .current_dir(repo_path(""))
        .output()
        .unwrap_or_else(|e| panic!("run census: {e}"));
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "r27_collections_object_census must exit 0\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    stdout
}

fn parse(stdout: &str, key: &str) -> i64 {
    for line in stdout.lines() {
        if let Some(rest) = line.strip_prefix(key) {
            return rest
                .parse()
                .unwrap_or_else(|_| panic!("bad int for {key}: {rest}"));
        }
    }
    panic!("missing key {key} in census output:\n{stdout}");
}

fn code_lines(path: &str) -> String {
    std::fs::read_to_string(repo_path(path))
        .unwrap_or_else(|e| panic!("read {path}: {e}"))
        .lines()
        .filter(|ln| !ln.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn r27_array_dict_set_spell_sendable() {
    let array = code_lines("packages/core/builtins/collections/array.sy");
    let dict = code_lines("packages/core/builtins/collections/dictionary.sy");
    let set = code_lines("packages/core/builtins/collections/set.sy");
    assert!(
        array.contains("contains(item: Sendable)") && array.contains("append(item: Sendable)"),
        "Array.contains/append must take Sendable"
    );
    assert!(
        !array.contains("item: Object"),
        "Array must not spell item: Object"
    );
    assert!(
        dict.contains("key: Sendable") && !dict.contains("key: Object"),
        "Dictionary keys must be Sendable"
    );
    assert!(
        set.contains("item: Sendable") && !set.contains("item: Object"),
        "Set items must be Sendable"
    );
}

#[test]
fn r27_collections_object_census_script_green() {
    let stdout = run_census();
    assert_eq!(parse(&stdout, "collections_array_dict_set_object="), 0);
    // R-41: OrderedSet Object-shell cleared → ceiling ≤180/≤10.
    assert!(parse(&stdout, "object_sig_residual_count=") <= 180);
    assert!(parse(&stdout, "object_sig_residual_files=") <= 10);
    assert!(parse(&stdout, "simd_object_residual=") >= 159);
    assert_eq!(parse(&stdout, "ordered_set_object_shell_residual="), 0);
}

#[test]
fn r27_no_ignore_on_collections_paths() {
    let paths = [
        "crates/syx-driver/tests/stdlib/collection_array_contains.rs",
        "crates/syx-driver/tests/lang/array_element_check.rs",
        "crates/syx-driver/tests/lang/set_element_check.rs",
        "crates/syx-driver/tests/lang/dict_keys_values_type.rs",
    ];
    for rel in paths {
        let p = repo_path(rel);
        if !p.is_file() {
            continue;
        }
        let src = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {rel}: {e}"));
        assert!(
            !src.contains("#[ignore]"),
            "fail-closed: do not #[ignore] collections path {rel}"
        );
    }
}
