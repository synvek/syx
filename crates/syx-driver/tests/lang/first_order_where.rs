//! first-order where constraint thin start (stable S010).
// was: m4_l01_first_order_where

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[test]
fn m4_l01_first_order_where_pass_int_equatable() {
    let src = r#"
func idEq<T>(_ a: T) -> T where T: Equatable {
  return a
}
func main() {
  print(idEq(1))
  return 0
}
"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("Int must satisfy where T: Equatable")
        .printed
        .join("\n");
    assert!(
        out.contains('1'),
        "pass path must print Int identity: {out}"
    );
}

#[test]
fn m4_l01_first_order_where_fail_s010() {
    let src = r#"
class Box {
  var x: Int
  init(_ x: Int) { self.x = x }
}
func idEq<T>(_ a: T) -> T where T: Equatable {
  return a
}
func main() {
  let b = Box(1)
  print(idEq(b))
  return 0
}
"#;
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Err(e) if e.code.as_deref() == Some("S010") => {
            assert!(
                e.message.contains("where")
                    || e.message.contains("Equatable")
                    || e.message.contains("Box")
                    || e.message.contains("constraint"),
                "S010 message should mention where/Equatable/Box/constraint: {}",
                e.message
            );
        }
        Err(e) => panic!("expected S010 where failure, got: {e:?}"),
        Ok(r) => panic!("expected S010 where failure, ran: {:?}", r.printed),
    }
}

