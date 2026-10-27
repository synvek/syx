//! HTTP transport failure must not return status=0 success.

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[test]
fn http_fail_connect_refused_is_error() {
    // Wave A W18 / ADR-362 / A-03: try/catch product exact (not is_err soft).
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"func main() {
  try {
    print(__syx_http_get("http://127.0.0.1:1/x"))
  } catch (err) {
    print("refused")
  }
}"#,
        &DriverOptions::default(),
    )
    .expect("connect refused must catch");
    assert_eq!(
        result.printed.join("\n"),
        "refused",
        "http connect refused product must exact: {:?}",
        result.printed
    );
}

#[test]
fn http_fail_package_documents_fail_closed() {
    let crypto = include_str!("../../../../docs/11_NetworkStdlib.md");
    // Soft doc anchor — NetworkStdlib should not claim status=0 success for failures.
    assert!(
        !crypto.contains("status=0 means success") && !crypto.contains("status = 0 success"),
        "docs must not claim status=0 success"
    );
}
