//! W1036 / ADR-1380: D-04 plugin domain thin grant (`__syx_plugin_*` → S024).

use syx_config::PermissionGrants;
use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_pkg::{PermissionsConfig, SyxManifest};

fn ensure_echo_plugin_host() {
    syx_host_plugin::register_echo_plugin_in_process();
}

fn with_permission<F: FnOnce()>(sandbox: bool, allow_plugin: bool, f: F) {
    let _serial = super::permission_lock::hold();
    ensure_echo_plugin_host();
    let prev = syx_config::runtime_config();
    let mut cfg = prev.clone();
    cfg.permission = PermissionGrants {
        sandbox,
        allow_net: true,
        allow_fs: true,
        allow_env: true,
        allow_process: true,
        allow_plugin,
    };
    syx_config::install_runtime_config(cfg);
    f();
    syx_config::install_runtime_config(prev);
}

#[test]
fn qs1_d04_permission_plugin_unauthorized_negative() {
    let src = r#"
func main() {
  let s = __syx_plugin_echo_echo("hi")
  print(s)
  return 0
}
"#;
    with_permission(true, false, || {
        match compile_and_run_embedded(src, &DriverOptions::default()) {
            Err(e) if e.code.as_deref() == Some("S024") => {
                assert!(
                    e.message.contains("plugin")
                        || e.message.contains("allow-plugin")
                        || e.message.contains("permission"),
                    "S024 message should mention plugin/allow-plugin: {}",
                    e.message
                );
            }
            Err(e) => panic!("expected S024 unauthorized plugin, got: {e:?}"),
            Ok(r) => panic!("expected S024 unauthorized plugin, ran: {:?}", r.printed),
        }
    });
}

#[test]
fn qs1_d04_permission_plugin_allow_plugin_positive() {
    let src = r#"
func main() {
  let s = __syx_plugin_echo_echo("hi")
  print(s)
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
                    "allow-plugin must not emit S024: {e:?}"
                );
            }
        }
    });
}

#[test]
fn qs1_d04_permission_plugin_toml_permissions_parse_and_merge() {
    let text = r#"
name = "app"
main = "main.sy"

[permissions]
sandbox = true
allow_plugin = true
"#;
    let manifest: SyxManifest =
        toml::from_str(text).expect("syx.toml [permissions] must deserialize");
    assert!(manifest.permissions.sandbox);
    assert!(manifest.permissions.allow_plugin);

    let mut grants = PermissionGrants {
        sandbox: false,
        allow_net: false,
        allow_fs: false,
        allow_env: false,
        allow_process: false,
        allow_plugin: false,
    };
    let p = PermissionsConfig {
        sandbox: true,
        allow_net: false,
        allow_fs: false,
        allow_env: false,
        allow_process: false,
        allow_plugin: true,
    };
    grants.merge_flags(
        p.sandbox,
        p.allow_net,
        p.allow_fs,
        p.allow_env,
        p.allow_process,
        p.allow_plugin,
    );
    assert!(grants.sandbox);
    assert!(grants.allow_plugin);
}

#[test]
fn qs1_d04_permission_plugin_cli_flag() {
    let cli = include_str!("../../../../crates/syx-cli/src/main.rs");
    assert!(
        cli.contains("allow-plugin"),
        "CLI must expose --allow-plugin"
    );
}
