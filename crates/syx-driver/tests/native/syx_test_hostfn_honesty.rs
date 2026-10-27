//! TestingAssert / TestingAssertEq capability honesty (vm=real, partial).

fn host_toml_block(caps: &str, host: &str) -> String {
    let marker = format!("name = \"{host}\"");
    let idx = caps
        .find(&marker)
        .unwrap_or_else(|| panic!("caps missing {host}"));
    caps[idx..].lines().take(5).collect::<Vec<_>>().join("\n")
}

fn host_json_window(generated: &str, host: &str) -> String {
    let marker = format!("\"name\": \"{host}\"");
    let idx = generated
        .find(&marker)
        .unwrap_or_else(|| panic!("generated missing {host}"));
    generated[idx..].chars().take(320).collect()
}

#[test]
fn syx_test_hostfn_testing_assert_partial_vm_real() {
    let caps = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let generated =
        include_str!("../../../../packages/internal/host_fn_capabilities.generated.json");
    for host in ["TestingAssert", "TestingAssertEq"] {
        let slice = host_toml_block(caps, host);
        assert!(
            slice.contains("status = \"partial\"")
                && slice.contains("vm = \"real\"")
                && slice.contains("mir = \"unsupported\"")
                && slice.contains("llvm = \"unsupported\"")
                && slice.contains("wasm = \"unsupported\""),
            "{host} must be status=partial with vm=real only:\n{slice}"
        );
        let window = host_json_window(generated, host);
        assert!(
            window.contains("\"status\": \"partial\"") && window.contains("\"vm\": \"real\""),
            "{host} generated row must be partial + vm=real:\n{window}"
        );
    }
    // Still fail-closed for non-product Testing hosts this window.
    for host in ["TestingTest", "TestingExpectEqual", "TestingRunAll"] {
        let window = host_json_window(generated, host);
        assert!(
            window.contains("\"status\": \"unsupported\""),
            "{host} must remain unsupported this window:\n{window}"
        );
    }
}
