//! ADR-308 / M3 W49: M3-E01 permission model docs thin start.
// was: m3_e01_permission_model

#[test]
fn m3_e01_permission_model_policy_doc() {
    let policy = include_str!("../../../../docs/capability-sandbox-policy-v0.md");
    assert!(
        policy.contains("permission-policy-v0:default-deny-intent"),
        "policy must declare fingerprint permission-policy-v0:default-deny-intent"
    );
    for domain in ["fs", "net", "env", "process", "plugin"] {
        assert!(
            policy.contains(domain),
            "policy must cover domain {domain}"
        );
    }
    assert!(
        policy.contains("No runtime")
            || policy.contains("无 runtime")
            || policy.contains("Docs-only")
            || policy.contains("docs-only"),
        "policy must honestly state docs-only / no runtime enforcement"
    );
    assert!(
        policy.contains("default deny")
            || policy.contains("default-deny")
            || policy.contains("Default deny"),
        "policy must state default-deny intent"
    );
}

#[test]
fn m3_e01_permission_model_ci_and_inventory() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test security"),
        "ci.yml must wire m3_e01_permission_model"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_e01_permission_model = \"signed-ADR-308\""),
        "inventory must note signed-ADR-308"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-308")
            || guide.contains("m3_e01_permission_model")
            || (guide.contains("E01") && guide.contains("权限")),
        "UsageGuide must document M3-E01 permission model thin start"
    );
}

#[test]
fn m3_e01_permission_model_fence() {
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let e01 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-E01 |"))
        .expect("M3-E01 row");
    assert!(
        e01.contains("| [~] |") || e01.contains("| [x] |"),
        "M3-E01 must be [~] after thin or [x] after closeout: {e01}"
    );
    assert!(
        roadmap.contains("permission-policy-v0:default-deny-intent")
            || roadmap.contains("ADR-308")
            || roadmap.contains("m3_e01_permission_model"),
        "roadmap must record E01 thin / fingerprint / ADR-308"
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
        roadmap.contains("≠ 运行时")
            || roadmap.contains("≠ EC-M3-3")
            || roadmap.contains("≠ E02")
            || e01.contains("≠"),
        "roadmap must note honest ≠ runtime / ≠ EC-M3-3 / ≠ E02–E03"
    );
    assert!(
        (roadmap.contains("W51")
            && (roadmap.contains("E02")
                || roadmap.contains("M3-E02")
                || roadmap.contains("thin")))
            || (roadmap.contains("W50")
                && (roadmap.contains("closeout")
                    || roadmap.contains("ADR-309")
                    || roadmap.contains("E01")
                    || roadmap.contains("M3-E01")))
            || (roadmap.contains("W49")
                && (roadmap.contains("ADR-308")
                    || roadmap.contains("m3_e01")
                    || roadmap.contains("thin"))),
        "roadmap may advance W49/W50 done / W51 = E02 thin"
    );
    let gap = include_str!("../../../../plan/archive/syx-language-stdlib-gap-analysis.md");
    assert!(
        gap.contains("ADR-308")
            || gap.contains("permission-policy-v0")
            || gap.contains("M3-E01")
            || gap.contains("Capability sandbox"),
        "gap-analysis must reflect E01 permission policy thin"
    );
}
