//! pool layering docs thin start.
// was: m3_a05_pool_layering

#[test]
fn m3_a05_pool_layering_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_a05_pool_layering"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_a05_pool_layering = \"signed-ADR-270\""),
        "inventory must note signed-ADR-270"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-270")
            || guide.contains("m3_a05_pool_layering")
            || guide.contains("高级运行时"),
        "UsageGuide must document M3-A05 pool layering"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    assert!(
        roadmap.contains("ADR-270")
            && (roadmap.contains("pool")
                || roadmap.contains("分层")
                || roadmap.contains("m3_a05")
                || roadmap.contains("A05")),
        "roadmap must record M3 W11 A05 docs"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    assert!(
        roadmap.contains("W12")
            && (roadmap.contains("closeout")
                || roadmap.contains("A05")
                || roadmap.contains("ADR-271"))
            || (roadmap.contains("W13")
                && (roadmap.contains("A01") || roadmap.contains("closeout"))),
        "roadmap must record W12 A05 closeout and/or lock W13 = A01 closeout"
    );
    let a05 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-A05 |"))
        .expect("M3-A05 row");
    assert!(
        a05.contains("| [~] |") || a05.contains("| [x] |"),
        "M3-A05 may be [~] pre-closeout or [x] after W12: {a05}"
    );
}

#[test]
fn m3_a05_pool_layering_docs_honesty() {
    let book = include_str!("../../../../books/language-guide/18-concurrency.md");
    assert!(
        book.contains("应用并发") || book.contains("主业务"),
        "18-concurrency must have application concurrency tier"
    );
    assert!(
        book.contains("高级运行时") || book.contains("ADR-270"),
        "18-concurrency must label pool knobs as advanced runtime"
    );
    assert!(
        !book.contains("core/max/keepAlive / bounded queue / rejection policies）→ **134+**")
            && !book.contains("拒绝策略 / Actor 共享池化 → **134+**"),
        "must not claim shipped knobs are still deferred to 134+"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("高级运行时") || guide.contains("ADR-270"),
        "UsageGuide must state advanced runtime layering"
    );
    assert!(
        !guide.contains("core/max/keepAlive / bounded queue / rejection policies）→ **134+**"),
        "UsageGuide must not mark shipped pool knobs as 134+"
    );
    let facade = include_str!("../../../../packages/core/concurrency.sy");
    assert!(
        facade.contains("advanced runtime") || facade.contains("ADR-270"),
        "Concurrency facade must comment pool knobs as advanced runtime"
    );
}
