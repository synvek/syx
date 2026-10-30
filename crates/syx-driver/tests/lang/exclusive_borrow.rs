//! static exclusive borrow thin start (stable S011).
// was: m4_b01_exclusive_borrow

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[test]
fn m4_b01_exclusive_borrow_pass_single_mut() {
    let src = r#"
func main() {
  var a: Array<Int> = [1]
  let p = &mut a
  print(1)
  return 0
}
"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("single &mut must compile and run")
        .printed
        .join("\n");
    assert!(
        out.contains('1'),
        "pass path must print 1: {out}"
    );
}

#[test]
fn m4_b01_exclusive_borrow_fail_s011_double_mut() {
    let src = r#"
func main() {
  var a: Array<Int> = [1]
  let p = &mut a
  let q = &mut a
  print(p)
  return 0
}
"#;
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Err(e) if e.code.as_deref() == Some("S011") => {
            assert!(
                e.message.contains("mutable")
                    || e.message.contains("borrow")
                    || e.message.contains("more than once")
                    || e.message.contains("mut"),
                "S011 message should mention mutable/borrow/more than once: {}",
                e.message
            );
        }
        Err(e) => panic!("expected S011 double-mut failure, got: {e:?}"),
        Ok(r) => panic!("expected S011 double-mut failure, ran: {:?}", r.printed),
    }
}

