//! W1035 / ADR-1379: D-04 process domain thin grant (ProcessExec/ProcessSpawn → S024).

use syx_config::PermissionGrants;
use syx_driver::{compile_and_run_embedded, DriverOptions};

fn with_permission<F: FnOnce()>(sandbox: bool, allow_process: bool, f: F) {
    let _serial = super::permission_lock::hold();
    let prev = syx_config::runtime_config();
    let mut cfg = prev.clone();
    cfg.permission = PermissionGrants {
        sandbox,
        allow_net: true,
        allow_fs: true,
        allow_env: true,
        allow_process,
        allow_plugin: true,
    };
    syx_config::install_runtime_config(cfg);
    f();
    syx_config::install_runtime_config(prev);
}

#[test]
fn qs1_d04_permission_process_unauthorized_negative() {
    let src = r#"
func main() {
  let c = __syx_process_exec("/usr/bin/true", [])
  print(c)
  return 0
}
"#;
    with_permission(true, false, || {
        match compile_and_run_embedded(src, &DriverOptions::default()) {
            Err(e) if e.code.as_deref() == Some("S024") => {
                assert!(
                    e.message.contains("process")
                        || e.message.contains("allow-process")
                        || e.message.contains("permission"),
                    "S024 message should mention process/allow-process: {}",
                    e.message
                );
            }
            Err(e) => panic!("expected S024 unauthorized process, got: {e:?}"),
            Ok(r) => panic!("expected S024 unauthorized process, ran: {:?}", r.printed),
        }
    });
}

#[test]
fn qs1_d04_permission_process_allow_process_positive() {
    let src = r#"
func main() {
  let c = __syx_process_exec("/usr/bin/true", [])
  print(c)
  return 0
}
"#;
    with_permission(true, true, || {
        match compile_and_run_embedded(src, &DriverOptions::default()) {
            Ok(_) => {}
            Err(e) => {
                assert_ne!(
                    e.code.as_deref(),
                    Some("S024"),
                    "allow-process must not emit S024: {e:?}"
                );
            }
        }
    });
}

#[test]
fn qs1_d04_permission_process_cli_flag() {
    let cli = include_str!("../../../../crates/syx-cli/src/main.rs");
    assert!(
        cli.contains("allow-process"),
        "CLI must expose --allow-process"
    );
}
