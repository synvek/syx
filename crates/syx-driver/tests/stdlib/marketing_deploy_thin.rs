//! Marketing deploy thin-open inventory
//! Locks inventory signed; marketing full site / real deploy remain unmet; bans B `[x]`.
//! After : Static/marketing deploy tree is deepen partial met (≠ unmet, ≠ full met).

#[test]
fn marketing_deploy_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/marketing-deploy-vectors.md",
        "docs/lang/hosted-site-vectors.md",
        "docs/lang/hosted-site-exit.md",
        "docs/lang/website-capability-vectors.md",
        "docs/lang/website-capability-exit.md",
        "docs/lang/tls-server-ga-exit.md",
        "docs/lang/residual-product-vectors.md",
        "packages/internal/capabilities.toml",
        "packages/internal/capabilities.generated.json",
        "books/shared/capabilities-generated.md",
        "scripts/gen_capabilities.py",
        "crates/syx-driver/src/hosted_capabilities.rs",
        "crates/syx-driver/src/website_capabilities.rs",
        "crates/syx-driver/src/marketing_capabilities.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "marketing thin requires evidence file: {path}"
        );
    }
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    assert!(
        caps.contains("[meta]")
            && caps.contains("schema = \"syx-capability-v0\"")
            && (caps.contains("ADR-734")
                || caps.contains("ADR-733")
                || caps.contains("ADR-725")
                || caps.contains("ADR-723")),
        "capabilities.toml must retain v0 schema + marketing/hosted cross-index"
    );
}

#[test]
fn marketing_deploy_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/marketing-deploy-vectors.md");
    assert!(
        spec.contains("ADR-733")
            && (spec.contains("thin-open") || spec.contains("inventory thin")),
        "spec must cite ADR-733 marketing deploy thin-open inventory"
    );
    assert!(
        (spec.contains("Marketing deploy inventory thin") || spec.contains("inventory thin"))
            && (spec.contains("**met** (W389)") || spec.contains("ADR-733")),
        "spec must index marketing inventory thin met"
    );
    assert!(
        (spec.contains("Hosted full site") || spec.contains("marketing met") || spec.contains("marketing full site"))
            && spec.contains("**unmet**"),
        "spec must keep marketing full site unmet"
    );
    assert!(
        spec.contains("Static / marketing deploy tree")
            && (spec.contains("**partial met**") || spec.contains("partial met")),
        "spec must index Static/marketing deploy tree as deepen partial met (W390)"
    );
    assert!(
        !spec.contains("| **Static / marketing deploy tree** | **unmet** |")
            && !spec.contains("| **Static / marketing deploy tree** | **met** |"),
        "Static tree must not remain unmet and must not claim full met"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("marketing") || spec.contains("deploy") || spec.contains("B-04")),
        "spec Frozen must keep marketing / deploy / B fences"
    );
    assert!(
        (spec.contains("Deployable") || spec.contains("website") || spec.contains("hosted"))
            && (spec.contains("cross-met") || spec.contains("**cross-met**") || spec.contains("met (narrow)")),
        "spec must index website/hosted product as cross-met"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);

    let hosted = include_str!("../../../../docs/lang/hosted-site-vectors.md");
    assert!(
        hosted.contains("Hosted full site") && hosted.contains("**unmet**"),
        "hosted vectors must keep Hosted full site unmet after marketing thin"
    );
    assert!(
        hosted.contains("Static / marketing")
            && (hosted.contains("**partial met**") || hosted.contains("partial")),
        "hosted vectors must cross-index Static/marketing deploy as partial after W390"
    );
}

#[test]
fn marketing_deploy_thin_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        (roadmap.contains("W389") || roadmap.contains("W390"))
            && (roadmap.contains("marketing")
                || roadmap.contains("ADR-733")
                || roadmap.contains("ADR-734")
                || roadmap.contains("deploy")),
        "roadmap must cite W389/W390 marketing deploy work"
    );
    let a10 = roadmap
        .lines()
        .find(|l| l.starts_with("| A-10 |"))
        .expect("A-10 row");
    assert!(a10.contains("| [x] |"), "A-10 must remain [x]: {a10}");
    for id in ["B-04", "B-05", "B-06"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(row.contains("| [x] |"), "{id} is delivered after Wave B; do not re-pin the not-[x] fence: {row}");
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
