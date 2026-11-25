//! ADR-310 / M3 W51: M3-E02 permission grants thin start (opt-in net + unauthorized).
// was: m3_e02_permission_grants
//! ADR-311: permission denials use S024; fingerprint may advance to allow-net-fs-toml.

use syx_config::PermissionGrants;
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
fn m3_e02_permission_grants_default_on_s024() {
    // C-5h: sandbox default-on + no grant → S024 (retired prior default-off pin).
    let src = r#"
func main() {
  let h = __syx_tcp_connect("127.0.0.1", 9)
  print(h)
  return 0
}
"#;
    with_permission(true, false, || {
        match compile_and_run_embedded(src, &DriverOptions::default()) {
            Err(e) if e.code.as_deref() == Some("S024") => {}
            Err(e) => panic!("expected S024 default-on ungated net, got: {e:?}"),
            Ok(r) => panic!("expected S024 default-on ungated net, ran: {:?}", r.printed),
        }
    });
}

#[test]
fn c5h_permission_sandbox_opt_out_allows_tcp() {
    // C-5h: --no-permission-sandbox / sandbox=false recovers TcpConnect compile.
    let src = r#"
func main() {
  let h = __syx_tcp_connect("127.0.0.1", 9)
  print(h)
  return 0
}
"#;
    with_permission(false, false, || {
        let out = compile_and_run_embedded(src, &DriverOptions::default())
            .expect("opt-out sandbox must not gate net")
            .printed
            .join("\n");
        assert!(
            out.contains("-1") || !out.is_empty(),
            "opt-out path must still compile TcpConnect: {out}"
        );
    });
}
