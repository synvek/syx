//! Channel select fairness product nail (RR 10/10).
// was: m3_a04_fairness

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_vm::{reset_select_fair_cursor, select_fair_fingerprint};

fn run(src: &str) -> Vec<String> {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
}

#[test]
fn m3_a04_fairness_select_rr_even_split() {
    assert_eq!(
        select_fair_fingerprint(),
        "select-fair-v1:round-robin",
        "Phase 107 RR fingerprint must remain"
    );
    reset_select_fair_cursor(0);
    let lines = run(
        r#"
func main() {
  let ch1 = __syx_channel_create(1)
  let ch2 = __syx_channel_create(1)
  var i = 0
  while i < 20 {
    __syx_channel_try_recv(ch1)
    __syx_channel_try_recv(ch2)
    __syx_channel_send(ch1, 11)
    __syx_channel_send(ch2, 22)
    let r = __syx_channel_select([ch1, ch2])
    print(r[1])
    i = i + 1
  }
}
"#,
    );
    let c11 = lines.iter().filter(|l| l.contains("11")).count();
    let c22 = lines.iter().filter(|l| l.contains("22")).count();
    assert_eq!(
        (c11, c22),
        (10, 10),
        "fair RR from seed 0 must split evenly: 11={c11} 22={c22} lines={lines:?}"
    );
}

