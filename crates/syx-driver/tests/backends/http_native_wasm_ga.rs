//! ADR-215 / M2 W71: EC-M2-4 HTTP native/WASM GA third deepening (HttpResponseText llvm=real).
//! Wave A W270 / ADR-614: Http types×7 + HttpGet HostFn wasm=real.
//! Wave A W271 / ADR-615: Http residual HostFn wasm=real (Post/Request/Serve/Shutdown/BodyStream).
//! Wave A W282 / ADR-626: net.http.timeout_tls + net.tls.client product wasm=real.
//! Wave A W293 / ADR-637: net.http product native=real (status still partial; client-only).
//! Wave A W294 / ADR-638: net.http.server product native=real (status still partial).
// was: ec_m2_4_http_native_wasm_ga3

use syx_driver::{compile_and_run_embedded, DriverOptions};

use crate::native_strict::run_native_strict_or_mirror_vm;

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
    let (code, native_out) = run_native_strict_or_mirror_vm(src, &vm_out, "http_ga");
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
    let (code, native_out) = run_native_strict_or_mirror_vm(src, &vm_out, "http_ga");
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
    let (code, native_out) = run_native_strict_or_mirror_vm(src, &vm_out, "http_ga");
    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim();
    let native_line = native_out.lines().next().unwrap_or("").trim();
    assert_eq!(
        native_line, vm_line,
        "native HttpMakeResponse must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(
        native_line, "created",
        "body must be created: {native_line}"
    );
}
