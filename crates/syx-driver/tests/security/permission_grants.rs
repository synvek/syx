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

#[test]
fn m3_e02_permission_grants_ci_inventory_and_fence() {
    let policy = include_str!("../../../../docs/capability-sandbox-policy-v0.md");
    assert!(
        policy.contains("permission-grants-v0:allow-net-cli")
            || policy.contains("permission-grants-v0:allow-net-fs-toml"),
        "policy must declare grants fingerprint (thin or deepen)"
    );
    assert!(
        policy.contains("permission-policy-v0:default-deny-intent"),
        "policy must retain E01 fingerprint"
    );

    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test security"),
        "ci.yml must wire m3_e02_permission_grants"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_e02_permission_grants = \"signed-ADR-310\""),
        "inventory must note signed-ADR-310"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-310")
            || guide.contains("m3_e02_permission_grants")
            || (guide.contains("allow-net") && guide.contains("E02")),
        "UsageGuide must document M3-E02 permission grants thin"
    );


    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let e02 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-E02 |"))
        .expect("M3-E02 row");
    assert!(
        e02.contains("| [~] |") || e02.contains("| [x] |"),
        "M3-E02 must be [~] until closeout or [x] after: {e02}"
    );
    assert!(
        roadmap.contains("permission-grants-v0:allow-net-cli")
            || roadmap.contains("permission-grants-v0:allow-net-fs-toml")
            || roadmap.contains("ADR-310")
            || roadmap.contains("m3_e02_permission_grants"),
        "roadmap must record E02 thin/deepen / fingerprint / ADR-310"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-3") || roadmap.contains("- [ ] EC-M3-3"),
        "EC-M3-3 exit condition must remain documented (may be checked after ADR-316)"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-2") && roadmap.contains("- [x] EC-M3-4"),
        "EC-M3-2/4 must remain signed"
    );
    assert!(
        (roadmap.contains("W54")
            && (roadmap.contains("E03")
                || roadmap.contains("M3-E03")
                || roadmap.contains("thin")))
            || (roadmap.contains("W53")
                && (roadmap.contains("closeout")
                    || roadmap.contains("E02")
                    || roadmap.contains("M3-E02")
                    || roadmap.contains("ADR-312")))
            || (roadmap.contains("W52")
                && (roadmap.contains("deepen")
                    || roadmap.contains("ADR-311")
                    || roadmap.contains("fs")))
            || (roadmap.contains("W51")
                && (roadmap.contains("ADR-310")
                    || roadmap.contains("m3_e02")
                    || roadmap.contains("thin"))),
        "roadmap may advance W51–W53 done / W54 = E03 thin"
    );

    let gap = include_str!("../../../../plan/archive/syx-language-stdlib-gap-analysis.md");
    assert!(
        gap.contains("ADR-310")
            || gap.contains("permission-grants-v0")
            || gap.contains("M3-E02")
            || gap.contains("allow-net"),
        "gap-analysis must reflect E02 permission grants"
    );

    let cli = include_str!("../../../../crates/syx-cli/src/main.rs");
    assert!(
        cli.contains("permission-sandbox") && cli.contains("allow-net"),
        "CLI must expose --permission-sandbox and --allow-net"
    );
    let _ = SyxRuntimeConfig::default();
}
