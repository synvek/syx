//! ADR-215 / M2 W71: EC-M2-4 HTTP native/WASM GA third deepening (HttpResponseText llvm=real).
//! Wave A W270 / ADR-614: Http types×7 + HttpGet HostFn wasm=real.
//! Wave A W271 / ADR-615: Http residual HostFn wasm=real (Post/Request/Serve/Shutdown/BodyStream).
//! Wave A W282 / ADR-626: net.http.timeout_tls + net.tls.client product wasm=real.
//! Wave A W293 / ADR-637: net.http product native=real (status still partial; client-only).
//! Wave A W294 / ADR-638: net.http.server product native=real (status still partial).
// was: ec_m2_4_http_native_wasm_ga3

use std::fs;
use std::process::Command;

use syx_driver::{build_native_executable, compile_and_run_embedded, DriverOptions};

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

fn run_native_strict(src: &str) -> (i32, String) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_m2_w71_http_ga3_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native exe");
    (
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

#[test]
fn ec_m2_4_http_native_wasm_ga3_response_text_columns() {
    // Wave A W71 / ADR-215 llvm + Wave A W270 / ADR-614 wasm=real
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "HttpResponseText");
    assert!(
        block.contains("vm = \"real\""),
        "HttpResponseText must keep vm=real: {block}"
    );
    assert!(
        block.contains("llvm = \"real\""),
        "HttpResponseText must declare llvm=real (ADR-215): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "HttpResponseText must declare wasm=real (ADR-614): {block}"
    );
    assert!(
        block.contains("ADR-614"),
        "HttpResponseText notes must cite ADR-614: {block}"
    );
}

#[test]
fn ec_m2_4_http_native_wasm_ga3_prior_ga_hosts_remain_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["HttpMakeResponse", "HttpResponseStatus"] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must keep llvm=real: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-614): {block}"
        );
    }
}

#[test]
fn ec_m2_4_http_hosts_wasm_real_w271() {
    // Wave A W271 / ADR-615 — Http residual HostFn wasm 收官 (no remaining Http host-assisted).
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in W271_WASM_REAL {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must keep llvm=real: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-615): {block}"
        );
        assert!(
            block.contains("ADR-615"),
            "{name} notes must cite ADR-615: {block}"
        );
    }
}

#[test]
fn ec_m2_4_http_hosts_wasm_real_w270() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "HttpGet",
        "HttpMakeResponse",
        "HttpResponseStatus",
        "HttpResponseText",
        "HttpResponseOk",
        "HttpResponseHeader",
        "HttpResponseHeaders",
        "HttpResponseJson",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-614): {block}"
        );
        assert!(
            block.contains("ADR-614"),
            "{name} notes must cite ADR-614: {block}"
        );
    }
}


#[test]
fn ec_m2_4_http_native_wasm_ga_client_product_columns_w293() {
    // Wave A W293 / ADR-637: net.http client native=real; status stays partial.
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "net.http");
    assert!(
        block.contains("native = \"real\""),
        "net.http must keep native=real (ADR-637): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "net.http must declare wasm=real (ADR-623): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "net.http must keep status=partial (≠ implemented): {block}"
    );
    assert!(
        block.contains("ADR-637") && block.contains("W293"),
        "net.http notes must cite ADR-637+W293: {block}"
    );
    assert!(
        block.contains("ec_m2_4_http_native_wasm_ga"),
        "net.http tests must include ec_m2_4_http_native_wasm_ga: {block}"
    );
}

#[test]
fn ec_m2_4_http_native_wasm_ga3_product_columns() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "net.http.server");
    assert!(
        block.contains("native = \"real\""),
        "net.http.server must keep native=real (ADR-638): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "net.http.server must declare wasm=real (ADR-624): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "net.http.server must keep status=partial (≠ implemented): {block}"
    );
    assert!(
        block.contains("ADR-638") && block.contains("W294"),
        "net.http.server notes must cite ADR-638+W294: {block}"
    );
    assert!(
        block.contains("ec_m2_4_http_native_wasm_ga3"),
        "net.http.server tests must include ec_m2_4_http_native_wasm_ga3: {block}"
    );
}

#[test]
fn ec_m2_4_http_native_wasm_ga_timeout_tls_client_w282() {
    // Wave A W343 / ADR-687: timeout_tls + tls.client product native=real (wasm=real since W282).
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    for id in ["net.http.timeout_tls", "net.tls.client"] {
        let block = product_block(caps, id);
        assert!(
            block.contains("native = \"real\""),
            "{id} must keep native=real (ADR-687): {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{id} must declare wasm=real (ADR-626): {block}"
        );
        assert!(
            !block.contains("native = \"partial\""),
            "{id} must not remain native=partial: {block}"
        );
        assert!(
            block.contains("ADR-626") && block.contains("W282"),
            "{id} notes must cite ADR-626+W282: {block}"
        );
        assert!(
            block.contains("ADR-687") && block.contains("W343"),
            "{id} notes must cite ADR-687+W343: {block}"
        );
        assert!(
            block.contains("ec_m2_4_http_native_wasm_ga"),
            "{id} tests must include ec_m2_4_http_native_wasm_ga: {block}"
        );
    }
}

#[test]
fn ec_m2_4_http_native_wasm_ga3_native_response_text_exec() {
    let src = r#"func main() {
    let r = __syx_http_make_response(200, "ok")
    print(__syx_http_response_text(r))
    return 0
}"#;
    let vm_out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("vm run")
        .printed
        .join("\n");
    let (code, native_out) = run_native_strict(src);
    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim();
    let native_line = native_out.lines().next().unwrap_or("").trim();
    assert_eq!(
        native_line, vm_line,
        "native HttpResponseText must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(native_line, "ok", "text must be ok: {native_line}");
}

#[test]
fn ec_m2_4_http_native_wasm_ga_native_http_response_status_exec() {
    let src = r#"func main() {
    let r = __syx_http_make_response(200, "ok")
    print(__syx_http_response_status(r))
    return 0
}"#;
    let vm_out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("vm run")
        .printed
        .join("\n");
    let (code, native_out) = run_native_strict(src);
    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim();
    let native_line = native_out.lines().next().unwrap_or("").trim();
    assert_eq!(
        native_line, vm_line,
        "native HttpResponseStatus must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(native_line, "200", "status must be 200: {native_line}");
}

#[test]
fn ec_m2_4_http_native_wasm_ga_native_http_make_response_exec() {
    let src = r#"func main() {
    print(__syx_http_response_text(__syx_http_make_response(201, "created")))
    return 0
}"#;
    let vm_out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("vm run")
        .printed
        .join("\n");
    let (code, native_out) = run_native_strict(src);
    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim();
    let native_line = native_out.lines().next().unwrap_or("").trim();
    assert_eq!(
        native_line, vm_line,
        "native HttpMakeResponse must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(native_line, "created", "body must be created: {native_line}");
}

#[test]
fn ec_m2_4_http_native_wasm_ga3_honesty_fences() {
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
}

#[test]
fn ec_m2_4_http_native_wasm_ga3_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_http_native_wasm_ga3_tier"),
        "inventory must contain ec_m2_4_http_native_wasm_ga3_tier: {inv}"
    );
    assert!(
        inv.contains("ADR-215") || inv.contains("signed-ADR-215"),
        "inventory must note ADR-215: {inv}"
    );
}

#[test]
fn ec_m2_4_http_native_wasm_ga3_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test evergreen"),
        "ci.yml must wire ec_m2_4_http_native_wasm_ga3"
    );
}
