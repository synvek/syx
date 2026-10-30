//! language-complete borrow explicit non-goal thin start.
// was: m4_b03_nongoal

#[test]
fn m4_b03_nongoal_ci_and_inventory() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test lang"),
        "ci.yml must wire m4_b03_nongoal"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m4_b03_nongoal = \"signed-ADR-334\""),
        "inventory must note signed-ADR-334"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-334")
            || guide.contains("m4_b03_nongoal")
            || guide.contains("borrow-lang-complete-v0:nongoal")
            || (guide.contains("M4-B03")
                && (guide.contains("nongoal") || guide.contains("非目标"))),
        "UsageGuide must document M4-B03 nongoal thin start"
    );
}

