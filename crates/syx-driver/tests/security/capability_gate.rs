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

#[test]
fn m3_e03_capability_gate_ci_inventory_and_fence() {
    let policy = include_str!("../../../../docs/capability-sandbox-policy-v0.md");
    assert!(
        policy.contains("hostfn-capability-gate-v0:compile-unsupported"),
        "policy must declare hostfn capability gate fingerprint"
    );

    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test security"),
        "ci.yml must wire m3_e03_capability_gate"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_e03_capability_gate = \"signed-ADR-313\""),
        "inventory must note signed-ADR-313"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-313")
            || guide.contains("m3_e03_capability_gate")
            || (guide.contains("ir-capability-gate") && guide.contains("E03")),
        "UsageGuide must document M3-E03 capability gate thin"
    );


    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let e03 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-E03 |"))
        .expect("M3-E03 row");
    assert!(
        e03.contains("| [~] |") || e03.contains("| [x] |"),
        "M3-E03 must be [~] until closeout or [x] after: {e03}"
    );
    assert!(
        roadmap.contains("hostfn-capability-gate-v0:compile-unsupported")
            || roadmap.contains("ADR-313")
            || roadmap.contains("m3_e03_capability_gate"),
        "roadmap must record E03 thin / fingerprint / ADR-313"
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
        (roadmap.contains("W56")
            && (roadmap.contains("EC-M3-3") || roadmap.contains("thin")))
            || (roadmap.contains("W55")
                && (roadmap.contains("closeout")
                    || roadmap.contains("ADR-314")
                    || roadmap.contains("E03")
                    || roadmap.contains("M3-E03")))
            || (roadmap.contains("W54")
                && roadmap.contains("ADR-313")
                && (roadmap.contains("thin") || roadmap.contains("E03"))),
        "roadmap may advance W54–W55 done / W56 = EC-M3-3 thin"
    );

    let gap = include_str!("../../../../plan/archive/syx-language-stdlib-gap-analysis.md");
    assert!(
        gap.contains("ADR-313")
            || gap.contains("ADR-314")
            || gap.contains("m3_e03_capability_gate")
            || gap.contains("m3_e03_closeout")
            || (gap.contains("M3-E03") && gap.contains("capability"))
            || gap.contains("hostfn-capability-gate-v0:compile-unsupported"),
        "gap-analysis must reflect E03 capability gate thin/closeout"
    );
}
