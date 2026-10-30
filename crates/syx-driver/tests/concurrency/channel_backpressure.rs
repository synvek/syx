//! Channel backpressure deepen (trySend Full→false + length/capacity).
// was: m3_a04_backpressure

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_a04_backpressure_try_send_full_false_and_length() {
    let src = r#"
func main() {
  let ch = __syx_channel_create(2)
  print(__syx_channel_capacity(ch))
  print(__syx_channel_length(ch))
  print(__syx_channel_try_send(ch, "A"))
  print(__syx_channel_try_send(ch, "B"))
  print(__syx_channel_length(ch))
  print(__syx_channel_try_send(ch, "C"))
  print(__syx_channel_try_recv(ch))
  print(__syx_channel_length(ch))
  print(__syx_channel_try_send(ch, "C"))
  print(__syx_channel_length(ch))
}
"#;
    let out = run(src);
    let lines: Vec<&str> = out.lines().filter(|l| !l.is_empty()).collect();
    assert!(
        lines.len() >= 10,
        "expected capacity/length/trySend/tryRecv sequence: {out}"
    );
    assert_eq!(lines[0], "2", "capacity must be 2: {out}");
    assert_eq!(lines[1], "0", "initial length must be 0: {out}");
    assert!(
        lines[2] == "true" || lines[2] == "True",
        "trySend A must succeed: {out}"
    );
    assert!(
        lines[3] == "true" || lines[3] == "True",
        "trySend B must succeed: {out}"
    );
    assert_eq!(lines[4], "2", "length after fill must be 2: {out}");
    assert!(
        lines[5] == "false" || lines[5] == "False",
        "trySend C when full must return false: {out}"
    );
    assert_eq!(lines[6], "A", "tryRecv must drain A: {out}");
    assert_eq!(lines[7], "1", "length after drain must be 1: {out}");
    assert!(
        lines[8] == "true" || lines[8] == "True",
        "trySend C after drain must succeed: {out}"
    );
    assert_eq!(lines[9], "2", "length after refill must be 2: {out}");
}

