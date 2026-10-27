//! Dictionary.remove deletes keys (not contains-only).

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn dict_remove_deletes_key() {
    let out = run(
        r#"
func main() {
  var d: [String: Int] = ["a": 1, "b": 2]
  let removed = d.remove("a")
  print(removed)
  print(d.containsKey("a"))
  print(d.containsKey("b"))
  print(d.count)
}
"#,
    );
    let lines: Vec<_> = out.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
    assert_eq!(lines.get(0).copied(), Some("true"), "remove ok: {out}");
    assert_eq!(lines.get(1).copied(), Some("false"), "a gone: {out}");
    assert_eq!(lines.get(2).copied(), Some("true"), "b remains: {out}");
    assert_eq!(lines.get(3).copied(), Some("1"), "count: {out}");
}

#[test]
fn dict_remove_missing_is_false() {
    let out = run(
        r#"
func main() {
  var d: [String: Int] = ["a": 1]
  print(d.remove("missing"))
  print(d.containsKey("a"))
}
"#,
    );
    let lines: Vec<_> = out.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
    assert_eq!(lines.get(0).copied(), Some("false"), "{out}");
    assert_eq!(lines.get(1).copied(), Some("true"), "{out}");
}
