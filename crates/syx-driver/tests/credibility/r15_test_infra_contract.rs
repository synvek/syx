//! R-15 test-infrastructure debt contract.
//! Nails: lib test target compiles, count_gate paths track their files, no vacuous
//! asserts or ghost inputs in the parity gates.

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

#[test]
fn r15_lib_tests_do_not_include_untracked_examples() {
    // `include_str!` on an untracked/gitignored sample breaks the whole lib test target
    // on a clean clone. R-15 inlined both probes instead.
    let driver = std::fs::read_to_string(repo_path("crates/syx-driver/src/lib.rs")).unwrap();
    assert!(
        !driver.contains("/../../examples/main.sy"),
        "syx-driver must not include_str! the never-tracked examples/main.sy"
    );
    let lsp = std::fs::read_to_string(repo_path("crates/syx-lsp/src/lib.rs")).unwrap();
    assert!(
        !lsp.contains("/../../examples/hello2.sy"),
        "syx-lsp must not include_str! the gitignored examples/hello2.sy"
    );
}

#[test]
fn r15_count_gates_point_at_existing_files() {
    // Each count_gate must read the file that actually holds the tests.
    let cases = [
        (
            "crates/syx-driver/tests/browser/browser_compile.rs",
            "tests/browser/browser_compile.rs",
        ),
        (
            "crates/syx-driver/tests/evergreen/parity_native.rs",
            "tests/evergreen/parity_native.rs",
        ),
        (
            "crates/syx-driver/tests/evergreen/registry.rs",
            "tests/evergreen/registry.rs",
        ),
        (
            "crates/syx-driver/tests/lang/polymorphic_base_ref.rs",
            "tests/lang/polymorphic_base_ref.rs",
        ),
        (
            "crates/syx-driver/tests/native/native_strict_parity_env.rs",
            "tests/native/native_strict_parity_env.rs",
        ),
    ];
    for (rel, expected) in cases {
        let src = std::fs::read_to_string(repo_path(rel)).unwrap();
        assert!(
            src.contains(expected),
            "{rel} count_gate must read {expected} (flat pre-split path would panic)"
        );
        assert!(
            repo_path(&format!("crates/syx-driver/{expected}")).is_file(),
            "{expected} must exist"
        );
    }
}

#[test]
fn r15_parity_gates_have_no_vacuous_asserts() {
    let vm = std::fs::read_to_string(repo_path(
        "crates/syx-driver/tests/evergreen/parity_vm.rs",
    ))
    .unwrap();
    assert!(
        !vm.contains("if let Ok((0, printed))"),
        "parity_vm must not silently skip the MIR actor assert"
    );

    let hard = std::fs::read_to_string(repo_path(
        "crates/syx-driver/tests/runtime/stdlib_parity_hard.rs",
    ))
    .unwrap();
    assert!(
        !hard.contains("browser_tier2k"),
        "stdlib_parity_hard must not reference the removed browser_tier2k file"
    );
    assert!(
        !hard.contains("read_to_string(&path).unwrap_or_default()"),
        "the anti-soft-assert gate must not itself read files softly"
    );
}
