//! ADR-311 / M3 W52: M3-E02 permission grants deepen (fs + [permissions] TOML + S024).
// was: m3_e02_permission_fs

use syx_config::PermissionGrants;
use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_pkg::{PermissionsConfig, SyxManifest};

fn with_permission<F: FnOnce()>(sandbox: bool, allow_net: bool, allow_fs: bool, f: F) {
    let _serial = super::permission_lock::hold();
    let prev = syx_config::runtime_config();
    let mut cfg = prev.clone();
    cfg.permission = PermissionGrants {
        sandbox,
        allow_net,
        allow_fs,
        allow_env: false,
        allow_process: false,
        allow_plugin: false,
    };
    syx_config::install_runtime_config(cfg);
    f();
    syx_config::install_runtime_config(prev);
}

#[test]
fn m3_e02_permission_fs_unauthorized_negative() {
    let src = r#"
func main() {
  let s = __syx_file_read("/tmp/syx-e02-missing.txt")
  print(s)
  return 0
}
"#;
    with_permission(true, true, false, || {
        match compile_and_run_embedded(src, &DriverOptions::default()) {
            Err(e) if e.code.as_deref() == Some("S024") => {
                assert!(
                    e.message.contains("fs")
                        || e.message.contains("allow-fs")
                        || e.message.contains("permission"),
                    "S024 message should mention fs/allow-fs: {}",
                    e.message
                );
            }
            Err(e) => panic!("expected S024 unauthorized fs, got: {e:?}"),
            Ok(r) => panic!("expected S024 unauthorized fs, ran: {:?}", r.printed),
        }
    });
}

#[test]
fn m3_e02_permission_fs_allow_fs_positive() {
    let src = r#"
func main() {
  let s = __syx_file_read("/tmp/syx-e02-missing.txt")
  print(s)
  return 0
}
"#;
    with_permission(true, false, true, || {
        // May fail at runtime (missing file) but must not emit S024 permission denial.
        match compile_and_run_embedded(src, &DriverOptions::default()) {
            Ok(_) => {}
            Err(e) => {
                assert_ne!(
                    e.code.as_deref(),
                    Some("S024"),
                    "allow-fs must not emit S024: {e:?}"
                );
            }
        }
    });
}

#[test]
fn m3_e02_permission_fs_toml_permissions_parse_and_merge() {
    let text = r#"
name = "app"
main = "main.sy"

[permissions]
sandbox = true
allow_net = true
allow_fs = true
"#;
    let manifest: SyxManifest =
        toml::from_str(text).expect("syx.toml [permissions] must deserialize");
    assert!(manifest.permissions.sandbox);
    assert!(manifest.permissions.allow_net);
    assert!(manifest.permissions.allow_fs);

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
        allow_fs: true,
        allow_env: false,
        allow_process: false,
        allow_plugin: false,
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
    assert!(!grants.allow_net);
    assert!(grants.allow_fs);
}

#[test]
fn m3_e02_permission_fs_ci_inventory_and_fence() {
    let policy = include_str!("../../../../docs/capability-sandbox-policy-v0.md");
    assert!(
        policy.contains("permission-grants-v0:allow-net-fs-toml"),
        "policy must declare deepen fingerprint"
    );
    assert!(
        policy.contains("[permissions]") || policy.contains("allow_fs"),
        "policy must document [permissions] / allow_fs"
    );

    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test security"),
        "ci.yml must wire m3_e02_permission_fs"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_e02_permission_fs = \"signed-ADR-311\""),
        "inventory must note signed-ADR-311"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-311")
            || guide.contains("m3_e02_permission_fs")
            || (guide.contains("allow-fs") && guide.contains("E02")),
        "UsageGuide must document M3-E02 permission grants deepen"
    );


    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let e02 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-E02 |"))
        .expect("M3-E02 row");
    assert!(
        e02.contains("| [~] |") || e02.contains("| [x] |"),
        "M3-E02 must be [~] after deepen or [x] after closeout: {e02}"
    );
    assert!(
        roadmap.contains("permission-grants-v0:allow-net-fs-toml")
            || roadmap.contains("ADR-311")
            || roadmap.contains("m3_e02_permission_fs"),
        "roadmap must record E02 deepen / fingerprint / ADR-311"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-3") || roadmap.contains("- [ ] EC-M3-3"),
        "EC-M3-3 exit condition must remain documented (may be checked after ADR-316)"
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
                && (roadmap.contains("ADR-311")
                    || roadmap.contains("deepen")
                    || roadmap.contains("m3_e02_permission_fs"))),
        "roadmap may advance W52/W53 done / W54 = E03 thin"
    );

    let gap = include_str!("../../../../plan/archive/syx-language-stdlib-gap-analysis.md");
    assert!(
        gap.contains("ADR-311")
            || gap.contains("allow-net-fs-toml")
            || gap.contains("allow-fs")
            || gap.contains("m3_e02_permission_fs"),
        "gap-analysis must reflect E02 fs deepen"
    );

    let cli = include_str!("../../../../crates/syx-cli/src/main.rs");
    assert!(
        cli.contains("allow-fs"),
        "CLI must expose --allow-fs"
    );
}
