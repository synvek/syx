//! ADR-200 / M2 W56: EC-M2-4 HTTP native/WASM availability matrix first deepening.
//! Wave A W270 / ADR-614: Http types×7 + HttpGet HostFn wasm=real.
//! Wave A W271 / ADR-615: Http residual HostFn wasm=real (Http* HostFn wasm 收官).
//! Wave A W279 / ADR-623: net.http product wasm=real.
//! Wave A W293 / ADR-637: net.http product native=real (status still partial; client-only).
//! Wave A W280 / ADR-624: net.http.server product wasm=real.
//! Wave A W294 / ADR-638: net.http.server product native=real (status still partial).
//! Wave A W282 / ADR-626: net.http.timeout_tls + net.tls.client product wasm=real.
// was: ec_m2_4_http_native_wasm_matrix

const HTTP_HOSTS: &[&str] = &[
    "HttpGet",
    "HttpPost",
    "HttpRequest",
    "HttpServe",
    "HttpShutdown",
    "HttpMakeResponse",
    "HttpResponseStatus",
    "HttpResponseText",
    "HttpResponseOk",
    "HttpResponseHeader",
    "HttpResponseHeaders",
    "HttpResponseJson",
    "HttpBodyStreamOpen",
    "HttpBodyStreamOpenOpts",
    "HttpBodyStreamReadChunk",
    "HttpBodyStreamClose",
];

const W270_WASM_REAL: &[&str] = &[
    "HttpGet",
    "HttpMakeResponse",
    "HttpResponseStatus",
    "HttpResponseText",
    "HttpResponseOk",
    "HttpResponseHeader",
    "HttpResponseHeaders",
    "HttpResponseJson",
];

const W271_WASM_REAL: &[&str] = &[
    "HttpBodyStreamOpen",
    "HttpBodyStreamOpenOpts",
    "HttpBodyStreamReadChunk",
    "HttpBodyStreamClose",
    "HttpPost",
    "HttpRequest",
    "HttpServe",
    "HttpShutdown",
];

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

fn is_w270_wasm_real(name: &str) -> bool {
    W270_WASM_REAL.contains(&name)
}

fn is_w271_wasm_real(name: &str) -> bool {
    W271_WASM_REAL.contains(&name)
}

#[test]
fn ec_m2_4_http_native_wasm_matrix_host_fn_columns() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in HTTP_HOSTS {
        let block = host_block(toml, name);
        assert!(
            block.contains("vm = \"real\""),
            "{name} must keep vm=real: {block}"
        );
        assert!(
            block.contains("cranelift = \"unsupported\""),
            "{name} must declare cranelift=unsupported: {block}"
        );
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real: {block}"
        );
        // Wave A W270/W271: all listed Http* HostFn wasm=real (Http* wasm 收官).
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-614/615): {block}"
        );
        if is_w271_wasm_real(name) {
            assert!(
                block.contains("ADR-615"),
                "{name} notes must cite ADR-615: {block}"
            );
        } else if is_w270_wasm_real(name) {
            assert!(
                block.contains("ADR-614"),
                "{name} notes must cite ADR-614: {block}"
            );
        }
    }
}

#[test]
fn ec_m2_4_http_native_wasm_matrix_generated_md_columns() {
    let md = include_str!("../../../../books/shared/host-fn-capabilities.md");
    for name in [
        "HttpMakeResponse",
        "HttpResponseStatus",
        "HttpResponseText",
        "HttpGet",
        "HttpPost",
        "HttpRequest",
        "HttpServe",
        "HttpShutdown",
        "HttpBodyStreamOpen",
    ] {
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
            "{name} row must list wasm=real (ADR-614/615): {row}"
        );
        assert!(
            !row.contains("| host-assisted |"),
            "{name} must not list host-assisted after ADR-615: {row}"
        );
    }
}

#[test]
fn ec_m2_4_http_native_wasm_matrix_product_columns() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");

    // Wave A W293 / ADR-637: net.http client native=real; status stays partial.
    let client = product_block(caps, "net.http");
    assert!(
        client.contains("native = \"real\""),
        "net.http must declare native=real (ADR-637): {client}"
    );
    assert!(
        client.contains("wasm = \"real\""),
        "net.http must declare wasm=real (ADR-623): {client}"
    );
    assert!(
        client.contains("status = \"partial\""),
        "net.http must keep status=partial (≠ implemented): {client}"
    );
    assert!(
        client.contains("ADR-637") && client.contains("W293"),
        "net.http notes must cite ADR-637+W293: {client}"
    );
    assert!(
        client.contains("ec_m2_4_http_native_wasm_matrix"),
        "net.http tests must include ec_m2_4_http_native_wasm_matrix: {client}"
    );

    // Wave A W294 / ADR-638: net.http.server native=real; status stays partial.
    let server = product_block(caps, "net.http.server");
    assert!(
        server.contains("native = \"real\""),
        "net.http.server must declare native=real (ADR-638): {server}"
    );
    assert!(
        server.contains("wasm = \"real\""),
        "net.http.server must declare wasm=real (ADR-624): {server}"
    );
    assert!(
        server.contains("status = \"partial\""),
        "net.http.server must keep status=partial (≠ implemented): {server}"
    );
    assert!(
        server.contains("ADR-638") && server.contains("W294"),
        "net.http.server notes must cite ADR-638+W294: {server}"
    );
    assert!(
        server.contains("ec_m2_4_http_native_wasm_matrix"),
        "net.http.server tests must include ec_m2_4_http_native_wasm_matrix: {server}"
    );

    // Wave A W343 / ADR-687: timeout_tls + tls.client native=real; wasm stay real.
    let timeout_tls = product_block(caps, "net.http.timeout_tls");
    assert!(
        timeout_tls.contains("native = \"real\""),
        "net.http.timeout_tls must declare native=real (ADR-687): {timeout_tls}"
    );
    assert!(
        timeout_tls.contains("wasm = \"real\""),
        "net.http.timeout_tls must declare wasm=real (ADR-626): {timeout_tls}"
    );
    assert!(
        !timeout_tls.contains("native = \"partial\""),
        "net.http.timeout_tls must not remain native=partial: {timeout_tls}"
    );
    assert!(
        timeout_tls.contains("ADR-626") && timeout_tls.contains("W282"),
        "net.http.timeout_tls notes must cite ADR-626+W282: {timeout_tls}"
    );
    assert!(
        timeout_tls.contains("ADR-687") && timeout_tls.contains("W343"),
        "net.http.timeout_tls notes must cite ADR-687+W343: {timeout_tls}"
    );
    assert!(
        timeout_tls.contains("m2_s4_02b_timeout_hang_loopback_native_aot"),
        "net.http.timeout_tls tests must list native AOT nail: {timeout_tls}"
    );

    let tls_client = product_block(caps, "net.tls.client");
    assert!(
        tls_client.contains("native = \"real\""),
        "net.tls.client must declare native=real (ADR-687): {tls_client}"
    );
    assert!(
        tls_client.contains("wasm = \"real\""),
        "net.tls.client must declare wasm=real (ADR-626): {tls_client}"
    );
    assert!(
        !tls_client.contains("native = \"partial\""),
        "net.tls.client must not remain native=partial: {tls_client}"
    );
    assert!(
        tls_client.contains("ADR-626") && tls_client.contains("W282"),
        "net.tls.client notes must cite ADR-626+W282: {tls_client}"
    );
    assert!(
        tls_client.contains("ADR-687") && tls_client.contains("W343"),
        "net.tls.client notes must cite ADR-687+W343: {tls_client}"
    );
    assert!(
        tls_client.contains("m2_s4_05_https_loopback_ok_with_extra_root_native_aot")
            && tls_client.contains("m2_s4_05_self_signed_fail_closed_native_aot"),
        "net.tls.client tests must list native AOT nails: {tls_client}"
    );

    // Wave A W216 / ADR-560: offline types facade promoted; keep client/server partial.
    let types = product_block(caps, "net.http.types");
    assert!(
        types.contains("native = \"real\""),
        "net.http.types must declare native=real (ADR-560): {types}"
    );
    assert!(
        types.contains("wasm = \"real\""),
        "net.http.types must declare wasm=real (ADR-560): {types}"
    );
    assert!(
        types.contains("ADR-560"),
        "net.http.types notes must cite ADR-560: {types}"
    );
}

#[test]
fn ec_m2_4_http_native_wasm_matrix_honesty_fences() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    assert!(
        !toml.contains("name = \"HttpRequestAsync\""),
        "HttpRequestAsync must remain uncovered (fail-closed default); found override"
    );

    let stubs = include_str!("../../../../crates/syx-rt/src/llvm_stubs.rs");
    let idx = stubs
        .find("fn syx_rt_http_get_aot")
        .expect("syx_rt_http_get_aot must exist");
    let after = &stubs[idx..];
    let end = after.find("\n#[no_mangle]").unwrap_or(after.len().min(200));
    let body = &after[..end.min(200)];
    assert!(
        body.contains("HostFnId::HttpGet") && body.contains("rt_call_host_native"),
        "syx_rt_http_get_aot must route HostFnId::HttpGet (W451 wire): {body}"
    );
    assert!(
        !body.contains("rt_direct_exec"),
        "syx_rt_http_get_aot must leave rt_direct_exec after W451: {body}"
    );

    let oracle = include_str!("../fixtures/differential/suite3/http_get_ok.oracle.json");
    assert!(
        oracle.contains("\"wasm\": true") || oracle.contains("\"wasm\":true"),
        "Suite-3 http_get_ok must still skip wasm: {oracle}"
    );
}

#[test]
fn ec_m2_4_http_native_wasm_matrix_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_http_native_wasm_matrix_tier"),
        "inventory must contain ec_m2_4_http_native_wasm_matrix_tier: {inv}"
    );
    assert!(
        inv.contains("ADR-200") || inv.contains("signed-ADR-200"),
        "inventory must note ADR-200: {inv}"
    );
}

#[test]
fn ec_m2_4_http_native_wasm_matrix_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test evergreen"),
        "ci.yml must wire ec_m2_4_http_native_wasm_matrix"
    );
}
