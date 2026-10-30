//! conditional conformance thin start (Array Element where → stable S010).
// was: m4_conditional_conformance

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[test]
fn m4_conditional_conformance_pass_array_int() {
    let src = r#"
protocol Marker {
  func mark() -> Int { return 0 }
}
extension Array: Marker where Element: Equatable {
  func mark() -> Int { return 1 }
}
func takeMarker<T>(_ x: T) -> Int where T: Marker {
  return 1
}
func main() {
  let xs: [Int] = [1, 2]
  print(takeMarker(xs))
  return 0
}
"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("Array<Int> must satisfy Marker where Element: Equatable")
        .printed
        .join("\n");
    assert!(
        out.contains('1'),
        "pass path must print: {out}"
    );
}

#[test]
fn m4_conditional_conformance_fail_s010() {
    let src = r#"
protocol Marker {
  func mark() -> Int { return 0 }
}
extension Array: Marker where Element: Equatable {
  func mark() -> Int { return 1 }
}
class Box {
  var x: Int
  init(_ x: Int) { self.x = x }
}
func takeMarker<T>(_ x: T) -> Int where T: Marker {
  return 1
}
func main() {
  let b = Box(1)
  let xs: [Box] = [b]
  print(takeMarker(xs))
  return 0
}
"#;
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Err(e) if e.code.as_deref() == Some("S010") => {
            assert!(
                e.message.contains("where")
                    || e.message.contains("Equatable")
                    || e.message.contains("Marker")
                    || e.message.contains("Element")
                    || e.message.contains("Box")
                    || e.message.contains("Array"),
                "S010 message should mention where/Equatable/Marker/Element/Box/Array: {}",
                e.message
            );
        }
        Err(e) => panic!("expected S010 conditional-conformance failure, got: {e:?}"),
        Ok(r) => panic!(
            "expected S010 conditional-conformance failure, ran: {:?}",
            r.printed
        ),
    }
}

