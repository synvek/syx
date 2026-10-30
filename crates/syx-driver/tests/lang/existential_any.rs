//! existential `any` binding thin start (stable S014).
// was: m4_l03_existential_any

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[test]
fn m4_l03_existential_any_pass_int_equatable() {
    let src = r#"
func main() {
  let x: any Equatable = 1
  print(1)
  return 0
}
"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("Int must assign to any Equatable")
        .printed
        .join("\n");
    assert!(
        out.contains('1'),
        "pass path must print: {out}"
    );
}

#[test]
fn m4_l03_existential_any_fail_s014() {
    let src = r#"
class Box {
  var x: Int
  init(_ x: Int) { self.x = x }
}
func main() {
  let x: any Equatable = Box(1)
  print(1)
  return 0
}
"#;
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Err(e) if e.code.as_deref() == Some("S014") => {
            assert!(
                e.message.contains("Equatable")
                    || e.message.contains("Box")
                    || e.message.contains("conform"),
                "S014 message should mention Equatable/Box/conform: {}",
                e.message
            );
        }
        Err(e) => panic!("expected S014 existential-any failure, got: {e:?}"),
        Ok(r) => panic!(
            "expected S014 existential-any failure, ran: {:?}",
            r.printed
        ),
    }
}

