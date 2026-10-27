//! TCP connect Int handle; unknown-handle read fails closed.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn tcp_stream_connect_returns_int_not_status_string() {
    // Wave A W19 / ADR-363 / A-03: refused connect product exact int (not contains soft).
    let out = run(
        r#"
func main() {
  let h = __syx_tcp_connect("127.0.0.1", 9)
  print(h)
}
"#,
    );
    assert_eq!(
        out.trim(),
        "-1",
        "tcp connect must exact int handle path, not status string: {out}"
    );
}

#[test]
fn tcp_stream_unknown_handle_read_fails() {
    // Wave A W20 / ADR-364 / A-03: try/catch product exact (not message/is_err soft).
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
  try {
    print(__syx_tcp_read(999999, 8))
  } catch (err) {
    print("caught")
  }
}
"#,
        &DriverOptions::default(),
    )
    .expect("unknown handle must catch");
    assert_eq!(
        result.printed.join("\n"),
        "caught",
        "tcp unknown-handle read product must exact: {:?}",
        result.printed
    );
}
