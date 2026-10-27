//! ADR-202 / M2 W58: EC-M2-4 Tcp native/WASM availability matrix first deepening.
//! Wave A W274 / ADR-618: Tcp×4 HostFn wasm=real 收官（非 Task 余债）.
//! Wave A W279 / ADR-623: net.tcp product wasm=real.
//! Wave A W291 / ADR-635: net.tcp product native=real (status still partial).
// was: ec_m2_4_tcp_native_wasm_matrix

const TCP_HOSTS: &[&str] = &["TcpConnect", "TcpRead", "TcpWrite", "TcpHandleRelease"];

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

fn product_block<'a>(caps: &'a str, id: &str) -> &'a str {
    let needle = format!("id = \"{id}\"");
    let rest = caps
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing capability {id}"));
    rest.split("[[capability]]").next().unwrap_or(rest)
}

#[test]
fn ec_m2_4_tcp_native_wasm_matrix_host_fn_columns() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in TCP_HOSTS {
        let block = host_block(toml, name);
        assert!(
            block.contains("vm = \"real\""),
            "{name} must keep vm=real: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-618): {block}"
        );
        assert!(
            block.contains("cranelift = \"unsupported\""),
            "{name} must declare cranelift=unsupported: {block}"
        );
        assert!(
            !block.contains("wasm = \"host-assisted\""),
            "{name} must not remain wasm=host-assisted: {block}"
        );
        // ADR-207/212/217/219: all four ADR-188 Tcp HostFns may declare llvm=real (CallHost GA).
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (GA): {block}"
        );
        assert!(
            block.contains("ADR-618"),
            "{name} notes must cite ADR-618: {block}"
        );
    }
}

#[test]
fn ec_m2_4_tcp_native_wasm_matrix_generated_md_columns() {
    let md = include_str!("../../../../books/shared/host-fn-capabilities.md");
    // ADR-618: all four ADR-188 Tcp HostFns declare wasm=real.
    for name in TCP_HOSTS {
        let row = md
            .lines()
            .find(|l| l.starts_with(&format!("| {name} |")))
            .unwrap_or_else(|| panic!("{name} row missing"));
        assert!(
            row.contains("| unsupported |"),
            "{name} row must list unsupported: {row}"
        );
        assert!(
            row.contains("| real | real | real | real |"),
            "{name} row must list wasm=real (ADR-618): {row}"
        );
        assert!(
            !row.contains("| host-assisted |"),
            "{name} must not list host-assisted after ADR-618: {row}"
        );
    }
}

#[test]
fn ec_m2_4_tcp_native_wasm_matrix_product_columns() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "net.tcp");
    assert!(
        block.contains("native = \"real\""),
        "net.tcp must declare native=real (ADR-635): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "net.tcp must declare wasm=real (ADR-623): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "net.tcp must keep status=partial (≠ implemented): {block}"
    );
    assert!(
        block.contains("ADR-635") && block.contains("W291"),
        "net.tcp notes must cite ADR-635+W291: {block}"
    );
    assert!(
        block.contains("ec_m2_4_tcp_native_wasm_matrix"),
        "net.tcp tests must include ec_m2_4_tcp_native_wasm_matrix: {block}"
    );
}

#[test]
fn ec_m2_4_tcp_native_wasm_matrix_honesty_fences() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["TcpStreamTls", "TcpSockbuf", "TcpHandleRefcount"] {
        assert!(
            !toml.contains(&format!("name = \"{name}\"")),
            "{name} must remain uncovered (fail-closed default); found override"
        );
    }

    let stubs = include_str!("../../../../crates/syx-rt/src/llvm_stubs.rs");
    let idx = stubs
        .find("fn syx_rt_tcp_read")
        .expect("syx_rt_tcp_read must exist");
    let after = &stubs[idx..];
    let end = after.find("\n#[no_mangle]").unwrap_or(after.len().min(200));
    let body = &after[..end.min(200)];
    assert!(
        body.contains("HostFnId::TcpRead") && body.contains("rt_call_host_native"),
        "syx_rt_tcp_read must route HostFnId::TcpRead (W453 wire): {body}"
    );
    assert!(
        !body.contains("rt_direct_exec"),
        "syx_rt_tcp_read must leave rt_direct_exec after W453: {body}"
    );
    let idx_w = stubs
        .find("fn syx_rt_tcp_write")
        .expect("syx_rt_tcp_write must exist");
    let after_w = &stubs[idx_w..];
    let end_w = after_w.find("\n#[no_mangle]").unwrap_or(after_w.len().min(200));
    let body_w = &after_w[..end_w.min(200)];
    assert!(
        body_w.contains("HostFnId::TcpWrite") && body_w.contains("rt_call_host_native"),
        "syx_rt_tcp_write must route HostFnId::TcpWrite (W454 wire): {body_w}"
    );
    assert!(
        !body_w.contains("rt_direct_exec"),
        "syx_rt_tcp_write must leave rt_direct_exec after W454: {body_w}"
    );

    let idx_r = stubs
        .find("fn syx_rt_tcp_handle_release")
        .expect("syx_rt_tcp_handle_release must exist");
    let after_r = &stubs[idx_r..];
    let end_r = after_r.find("\n#[no_mangle]").unwrap_or(after_r.len().min(200));
    let body_r = &after_r[..end_r.min(200)];
    assert!(
        body_r.contains("HostFnId::TcpHandleRelease") && body_r.contains("rt_call_host_native"),
        "syx_rt_tcp_handle_release must route HostFnId::TcpHandleRelease (W455 wire): {body_r}"
    );
    assert!(
        !body_r.contains("rt_direct_exec"),
        "syx_rt_tcp_handle_release must leave rt_direct_exec after W455: {body_r}"
    );
}

#[test]
fn ec_m2_4_tcp_native_wasm_matrix_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_tcp_native_wasm_matrix_tier"),
        "inventory must contain ec_m2_4_tcp_native_wasm_matrix_tier: {inv}"
    );
    assert!(
        inv.contains("ADR-202") || inv.contains("signed-ADR-202"),
        "inventory must note ADR-202: {inv}"
    );
}

#[test]
fn ec_m2_4_tcp_native_wasm_matrix_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test evergreen"),
        "ci.yml must wire ec_m2_4_tcp_native_wasm_matrix"
    );
}
