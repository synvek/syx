//! move-consume ownership thin start (stable S011).
// was: m4_b02_move_consume

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[test]
fn m4_b02_move_consume_pass_once() {
    let src = r#"
func take(_ s: String) {
  print(1)
}
func main() {
  let s = "hi"
  take(s)
  return 0
}
"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("String by-value consume-once must compile and run")
        .printed
        .join("\n");
    assert!(
        out.contains('1'),
        "pass path must print 1: {out}"
    );
}

#[test]
fn m4_b02_move_consume_fail_s011_use_after_move() {
    let src = r#"
func take(_ s: String) {
  print(1)
}
func main() {
  let s = "hi"
  take(s)
  print(s)
  return 0
}
"#;
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Err(e) if e.code.as_deref() == Some("S011") => {
            assert!(
                e.message.contains("moved")
                    || e.message.contains("Use of moved")
                    || e.message.to_lowercase().contains("moved value"),
                "S011 message should mention moved value: {}",
                e.message
            );
        }
        Err(e) => panic!("expected S011 use-after-move failure, got: {e:?}"),
        Ok(r) => panic!("expected S011 use-after-move failure, ran: {:?}", r.printed),
    }
}

