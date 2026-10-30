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

