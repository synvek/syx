//! Hosted full-site thin-open inventory
//! Locks inventory signed; Hosted full site remains unmet; bans deploy / B `[x]`.

#[test]
fn hosted_site_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/hosted-site-vectors.md",
        "docs/lang/hosted-site-exit.md",
        "docs/lang/website-capability-vectors.md",
        "docs/lang/website-capability-exit.md",
        "docs/lang/a10-capability-source.md",
        "docs/lang/a10-capability-exit.md",
        "docs/lang/residual-product-vectors.md",
        "packages/internal/capabilities.toml",
        "packages/internal/capabilities.generated.json",
        "books/shared/capabilities-generated.md",
        "scripts/gen_capabilities.py",
        "crates/syx-driver/src/website_capabilities.rs",
        "crates/syx-driver/src/hosted_capabilities.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "hosted thin requires evidence file: {path}"
        );
    }
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    assert!(
        caps.contains("[meta]")
            && caps.contains("schema = \"syx-capability-v0\"")
            && caps.contains("ADR-723"),
        "capabilities.toml must retain v0 schema + ADR-723 hosted inventory cross-index"
    );
}

#[test]
fn hosted_site_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/hosted-site-vectors.md");
    assert!(
        spec.contains("ADR-723")
            && (spec.contains("thin-open") || spec.contains("inventory signed")),
        "spec must cite ADR-723 hosted thin-open inventory"
    );
    assert!(
        (spec.contains("Hosted inventory thin") || spec.contains("inventory thin"))
            && (spec.contains("**met** (W379)") || spec.contains("ADR-723")),
        "spec must index hosted inventory met"
    );
    assert!(
        spec.contains("Hosted full site") && spec.contains("**unmet**"),
        "spec must keep hosted full site unmet"
    );
    assert!(
        (spec.contains("deploy") || spec.contains("marketing") || spec.contains("Static"))
            && spec.contains("**unmet**"),
        "spec must index deploy/marketing gap as unmet"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("hosted") || spec.contains("deploy") || spec.contains("B-04")),
        "spec Frozen must keep hosted-site / B fences"
    );
    assert!(
        (spec.contains("Deployable") || spec.contains("ADR-725") || spec.contains("W381"))
            && (spec.contains("**met** (W381") || spec.contains("ADR-725")),
        "spec must index Deployable hosted consumer product met (narrow)"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);

    let website = include_str!("../../../../docs/lang/website-capability-vectors.md");
    assert!(
        website.contains("Hosted full site") && website.contains("**unmet**"),
        "website vectors must keep Hosted full site unmet after W381 exit"
    );
    assert!(
        website.contains("ADR-725")
            || website.contains("**met** (W381")
            || website.contains("W381"),
        "website vectors must cross-index hosted product met exit (W381)"
    );
}

#[test]
fn hosted_site_thin_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W379")
            && (roadmap.contains("hosted")
                || roadmap.contains("ADR-723")
                || roadmap.contains("inventory")),
        "roadmap must cite W379 hosted thin inventory"
    );
    let a10 = roadmap
        .lines()
        .find(|l| l.starts_with("| A-10 |"))
        .expect("A-10 row");
    assert!(
        a10.contains("| [x] |"),
        "A-10 must remain [x]: {a10}"
    );
    for id in ["B-04", "B-05", "B-06"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(
            row.contains("| [x] |"),
            "{id} is delivered after Wave B; do not re-pin the not-[x] fence: {row}"
        );
    }
    let b08 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-08 |"))
        .expect("B-08 row");
    assert!(b08.contains("| [x] |"), "B-08 must remain [x]: {b08}");
    assert!(
        roadmap.contains("Q-L5") && roadmap.contains("已签"),
        "Q-L5 must remain signed"
    );
}
