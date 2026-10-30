//! ADR-310 / M3 W51: M3-E02 permission grants thin start (opt-in net + unauthorized).
// was: m3_e02_permission_grants
//! ADR-311: permission denials use S024; fingerprint may advance to allow-net-fs-toml.

use syx_config::{PermissionGrants, SyxRuntimeConfig};
use syx_driver::{compile_and_run_embedded, DriverOptions};

fn with_permission<F: FnOnce()>(sandbox: bool, allow_net: bool, f: F) {
    let _serial = super::permission_lock::hold();
    let prev = syx_config::runtime_config();
    let mut cfg = prev.clone();
    cfg.permission = PermissionGrants {
        sandbox,
        allow_net,
        allow_fs: false,
        allow_env: false,
        allow_process: false,
        allow_plugin: false,
    };
    syx_config::install_runtime_config(cfg);
    f();
    syx_config::install_runtime_config(prev);
}

#[test]
fn m3_e02_permission_grants_unauthorized_negative() {
    let src = r#"
func main() {
  let h = __syx_tcp_connect("127.0.0.1", 9)
  print(h)
  return 0
}
"#;
    with_permission(true, false, || {
        match compile_and_run_embedded(src, &DriverOptions::default()) {
            Err(e) if e.code.as_deref() == Some("S024") => {
                assert!(
                    e.message.contains("permission")
                        || e.message.contains("allow-net")
                        || e.message.contains("net"),
                    "S024 message should mention permission/allow-net: {}",
                    e.message
                );
            }
            Err(e) => panic!("expected S024 unauthorized, got: {e:?}"),
            Ok(r) => panic!("expected S024 unauthorized, ran: {:?}", r.printed),
        }
    });
}

#[test]
fn m3_e02_permission_grants_allow_net_positive() {
    let src = r#"
func main() {
  let h = __syx_tcp_connect("127.0.0.1", 9)
  print(h)
  return 0
}
"#;
    with_permission(true, true, || {
        let out = compile_and_run_embedded(src, &DriverOptions::default())
            .expect("allow-net must not emit S024")
            .printed
            .join("\n");
        assert!(
            out.contains("-1") || !out.is_empty(),
            "granted net compile+run should proceed: {out}"
        );
    });
}

#[test]
fn m3_e02_permission_grants_default_off_unchanged() {
    let src = r#"
func main() {
  let h = __syx_tcp_connect("127.0.0.1", 9)
  print(h)
  return 0
}
"#;
    with_permission(false, false, || {
        let out = compile_and_run_embedded(src, &DriverOptions::default())
            .expect("default sandbox off must not gate net")
            .printed
            .join("\n");
        assert!(
            out.contains("-1") || !out.is_empty(),
            "default path must still compile TcpConnect: {out}"
        );
    });
}

