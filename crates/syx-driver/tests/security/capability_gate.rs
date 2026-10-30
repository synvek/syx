//! ADR-313 / M3 W54: M3-E03 HostFn capability-table compile gate thin start.
// was: m3_e03_capability_gate

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn with_capability_gate<F: FnOnce()>(gate: bool, f: F) {
    let _serial = super::permission_lock::hold();
    let prev = syx_config::runtime_config();
    let mut cfg = prev.clone();
    cfg.ir_capability_gate = gate;
    syx_config::install_runtime_config(cfg);
    f();
    syx_config::install_runtime_config(prev);
}

#[test]
fn m3_e03_capability_gate_unsupported_negative() {
    // FileExists is table status=unsupported (no TOML override).
    let src = r#"
func main() {
  let ok = __syx_file_exists("/tmp/syx-e03-missing")
  print(ok)
  return 0
}
"#;
    with_capability_gate(true, || {
        match compile_and_run_embedded(src, &DriverOptions::default()) {
            Err(e) if e.code.as_deref() == Some("S026") => {
                assert!(
                    e.message.contains("unsupported")
                        || e.message.contains("capability")
                        || e.message.contains("FileExists"),
                    "S026 message should mention capability/unsupported/FileExists: {}",
                    e.message
                );
            }
            Err(e) => panic!("expected S026 unsupported, got: {e:?}"),
            Ok(r) => panic!("expected S026 unsupported, ran: {:?}", r.printed),
        }
    });
}

#[test]
fn m3_e03_capability_gate_default_off_unchanged() {
    let src = r#"
func main() {
  let ok = __syx_file_exists("/tmp/syx-e03-missing")
  print(ok)
  return 0
}
"#;
    with_capability_gate(false, || {
        let out = compile_and_run_embedded(src, &DriverOptions::default())
            .expect("default capability gate off must not emit S026")
            .printed
            .join("\n");
        assert!(
            out.contains("false") || out.contains("0") || !out.is_empty(),
            "default path must still compile FileExists: {out}"
        );
    });
}

