//! EC-M4-B Named move-consume thin start (stable S011).
// was: ec_m4_b_named_move

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[test]
fn ec_m4_b_named_move_pass_once() {
    let src = r#"
class Box {
  var n: Int
  init(n: Int) { self.n = n }
}
func take(_ b: Box) {
  print(1)
}
func main() {
  let b = Box(n: 1)
  take(b)
  return 0
}
"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("Named by-value consume-once must compile and run")
        .printed
        .join("\n");
    assert!(
        out.contains('1'),
        "pass path must print 1: {out}"
    );
}

#[test]
fn ec_m4_b_named_move_fail_s011_use_after_move() {
    let src = r#"
class Box {
  var n: Int
  init(n: Int) { self.n = n }
}
func take(_ b: Box) {
  print(1)
}
func main() {
  let b = Box(n: 1)
  take(b)
  print(b)
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

