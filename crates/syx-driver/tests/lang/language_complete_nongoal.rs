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

#[test]
fn m4_b03_nongoal_fence() {
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let row = roadmap
        .lines()
        .find(|l| l.starts_with("| M4-B03 |"))
        .expect("M4-B03 row");
    assert!(
        row.contains("| [~] |") || row.contains("| [x] |"),
        "M4-B03 must be [~] after thin or [x] after closeout: {row}"
    );
    assert!(
        roadmap.contains("ADR-334")
            || roadmap.contains("m4_b03_nongoal")
            || row.contains("nongoal")
            || row.contains("非目标")
            || row.contains("thin"),
        "roadmap must footnote M4-B03 nongoal thin / ADR-334"
    );
    assert!(
        roadmap.contains("borrow-lang-complete-v0:nongoal")
            || row.contains("borrow-lang-complete-v0:nongoal"),
        "roadmap must retain fingerprint borrow-lang-complete-v0:nongoal"
    );
    assert!(
        roadmap.contains("- [x] EC-M4-B") || roadmap.contains("- [ ] EC-M4-B"),
        "EC-M4-B must remain documented (may be checked after ADR-337)"
    );
    assert!(
        roadmap.contains("≠ EC-M4")
            || roadmap.contains("≠ NLL")
            || roadmap.contains("≠ M4 退出")
            || roadmap.contains("nongoal")
            || row.contains("≠")
            || row.contains("nongoal"),
        "roadmap must note honest ≠ EC-M4-B / NLL / M4 exit / nongoal"
    );
    assert!(
        (roadmap.contains("M4 W17")
            && (roadmap.contains("EC-M4-B")
                || roadmap.contains("NLL")
                || roadmap.contains("thin")))
            || (roadmap.contains("M4 W16")
                && (roadmap.contains("closeout")
                    || roadmap.contains("ADR-335")
                    || roadmap.contains("m4_b03_closeout")
                    || roadmap.contains("B03")
                    || roadmap.contains("M4-B03")))
            || (roadmap.contains("M4 W15")
                && (roadmap.contains("ADR-334")
                    || roadmap.contains("m4_b03_nongoal")
                    || roadmap.contains("nongoal")
                    || roadmap.contains("thin"))),
        "roadmap must lock next M4 W17 = EC-M4-B thin (or record W16/W15 done)"
    );
    let gap = include_str!("../../../../plan/archive/syx-language-stdlib-gap-analysis.md");
    assert!(
        gap.contains("ADR-334")
            || gap.contains("m4_b03_nongoal")
            || gap.contains("ADR-335")
            || gap.contains("m4_b03_closeout")
            || gap.contains("borrow-lang-complete-v0:nongoal")
            || (gap.contains("M4-B03")
                && (gap.contains("nongoal")
                    || gap.contains("非目标")
                    || gap.contains("thin")
                    || gap.contains("closeout")
                    || gap.contains("[~]")
                    || gap.contains("[x]"))),
        "gap-analysis must reflect M4-B03 nongoal thin/closeout"
    );
}
