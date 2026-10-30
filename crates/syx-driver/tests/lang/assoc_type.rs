//! associated-type typealias witness + stable S012 thin start.
// was: m4_l02_assoc_type

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[test]
fn m4_l02_assoc_type_pass_int_equatable() {
    let src = r#"
protocol P {
  associatedtype Item: Equatable
}
class C: P {
  typealias Item = Int
  init() {}
}
func main() {
  let _ = C()
  print(1)
  return 0
}
"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("typealias Item = Int must satisfy Item: Equatable")
        .printed
        .join("\n");
    assert!(
        out.contains('1'),
        "pass path must print: {out}"
    );
}

#[test]
fn m4_l02_assoc_type_fail_s012() {
    let src = r#"
protocol P {
  associatedtype Item: Equatable
}
class Box {
  var x: Int
  init(_ x: Int) { self.x = x }
}
class C: P {
  typealias Item = Box
  init() {}
}
func main() {
  let _ = C()
  print(1)
  return 0
}
"#;
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Err(e) if e.code.as_deref() == Some("S012") => {
            assert!(
                e.message.contains("Associated type")
                    || e.message.contains("Item")
                    || e.message.contains("Equatable")
                    || e.message.contains("Box"),
                "S012 message should mention Associated type/Item/Equatable/Box: {}",
                e.message
            );
        }
        Err(e) => panic!("expected S012 assoc-type failure, got: {e:?}"),
        Ok(r) => panic!("expected S012 assoc-type failure, ran: {:?}", r.printed),
    }
}

