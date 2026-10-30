//! pool layering docs thin start.
// was: m3_a05_pool_layering


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
