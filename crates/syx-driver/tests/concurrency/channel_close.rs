//! Channel close thin start (real disconnect / drain / C010).
// was: m3_a04_channel_close

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_a04_channel_close_drain_then_nil_and_send_c010() {
    let src = r#"
func main() {
  let ch = __syx_channel_create(4)
  __syx_channel_send(ch, "A")
  __syx_channel_close(ch)
  print(__syx_channel_try_recv(ch))
  print(__syx_channel_try_recv(ch))
  try {
    __syx_channel_send(ch, "B")
    print("SEND_OK")
  } catch (err) {
    print("SEND_CLOSED")
    print(err.code)
  }
}
"#;
    let out = run(src);
    let lines: Vec<&str> = out.lines().filter(|l| !l.is_empty()).collect();
    assert!(
        lines.len() >= 3,
        "expected drain A, Nil, SEND_CLOSED/C010: {out}"
    );
    assert_eq!(lines[0], "A", "must drain buffered value after close: {out}");
    assert!(
        lines[1] == "nil" || lines[1] == "Nil",
        "second recv after drain must be Nil: {out}"
    );
    assert!(
        out.contains("SEND_CLOSED") && out.contains("C010"),
        "post-close send must surface C010: {out}"
    );
    assert!(!out.contains("SEND_OK"), "post-close send must not succeed: {out}");
}

