//! C-8d: clear F20–F23 book MD honesty (doc_debt / book_ahead).
//! en+zh status words aligned; ≠ C-8 [x] · ≠ parser surface.

fn ch06_en() -> &'static str {
    include_str!("../../../../books/language-guide/06-control-flow.md")
}
fn ch06_zh() -> &'static str {
    include_str!("../../../../books/zh-cn/language-guide/06-control-flow.md")
}
fn ch14_en() -> &'static str {
    include_str!("../../../../books/language-guide/14-inheritance.md")
}
fn ch14_zh() -> &'static str {
    include_str!("../../../../books/zh-cn/language-guide/14-inheritance.md")
}
fn ch10_en() -> &'static str {
    include_str!("../../../../books/language-guide/10-structures-and-classes.md")
}
fn ch10_zh() -> &'static str {
    include_str!("../../../../books/zh-cn/language-guide/10-structures-and-classes.md")
}
fn ch17_en() -> &'static str {
    include_str!("../../../../books/language-guide/17-error-handling.md")
}
fn ch17_zh() -> &'static str {
    include_str!("../../../../books/zh-cn/language-guide/17-error-handling.md")
}

#[test]
fn c8d_ch06_labeled_break_not_full_implemented() {
    let en = ch06_en();
    let zh = ch06_zh();
    assert!(
        !en.contains("are **all Implemented**"),
        "en Ch06 must not use all-Implemented status bundle"
    );
    assert!(
        en.contains("labeled `break`/`continue` **Partially implemented**"),
        "en Ch06 status must mark labeled break/continue Partially"
    );
    assert!(
        en.contains("| `break label` | Exits the loop with the given label (**Partially implemented**)"),
        "en Ch06 mid-table break label must be Partially"
    );
    assert!(
        en.contains("Honesty (C-8d)") && en.contains("≠"),
        "en Ch06 must have Honesty (C-8d) ≠ full Implemented"
    );
    assert!(
        en.contains("| Labeled `break label` | **Partially implemented**"),
        "en Ch06 Swift-diff must keep labeled break Partially"
    );

    assert!(
        !zh.contains("**全部已实现**"),
        "zh Ch06 must not use 全部已实现 status bundle"
    );
    assert!(
        zh.contains("标签 `break`/`continue` **部分实现**"),
        "zh Ch06 status must mark labeled Partially"
    );
    assert!(
        zh.contains("| `break label` | 跳出指定标签的循环（**部分实现**）"),
        "zh Ch06 mid-table break label must be 部分实现"
    );
    assert!(
        zh.contains("诚实（C-8d）") && zh.contains("≠"),
        "zh Ch06 must have 诚实（C-8d）≠ 全量已实现"
    );
    assert!(
        zh.contains("| 标签 `break label` | **部分实现**"),
        "zh Ch06 Swift-diff must keep labeled break 部分实现"
    );
}

#[test]
fn c8d_ch14_ch15_convenience_not_partial() {
    let ch15_en = include_str!("../../../../books/language-guide/15-initialization.md");
    let ch15_zh = include_str!("../../../../books/zh-cn/language-guide/15-initialization.md");
    assert!(
        ch15_en.contains("`required init` / `convenience init` **Not implemented**"),
        "en Ch15 status must mark required/convenience Not implemented (not Partially/Not)"
    );
    assert!(
        !ch15_en.contains("**Partially implemented** / **Not implemented**"),
        "en Ch15 must drop Partially/Not ambiguity for convenience"
    );
    assert!(
        ch15_zh.contains("`required init` / `convenience init` **未实现**"),
        "zh Ch15 status must mark required/convenience 未实现"
    );
    assert!(
        !ch15_zh.contains("**部分实现** / **未实现**"),
        "zh Ch15 must drop 部分实现/未实现 ambiguity"
    );

    let en = ch14_en();
    let zh = ch14_zh();
    // Convenience row in Swift-diff must be Not implemented, not Partially.
    let en_conv = en
        .lines()
        .find(|l| l.contains("`convenience init`") && l.contains('|'))
        .expect("en Ch14 convenience row");
    assert!(
        en_conv.contains("**Not implemented**") && !en_conv.contains("Partially"),
        "en Ch14 convenience must be Not implemented; got {en_conv}"
    );
    let zh_conv = zh
        .lines()
        .find(|l| l.contains("`convenience init`") && l.contains('|'))
        .expect("zh Ch14 convenience row");
    assert!(
        zh_conv.contains("**未实现**") && !zh_conv.contains("部分实现"),
        "zh Ch14 convenience must be 未实现; got {zh_conv}"
    );
}

#[test]
fn c8d_ch10_mutating_swift_diff_partial() {
    let en = ch10_en();
    let zh = ch10_zh();
    let en_row = en
        .lines()
        .find(|l| l.contains("`mutating func`") && l.contains('|') && !l.contains("Phase 60 起"))
        .or_else(|| {
            // Prefer Differences-from-Swift table row (after feature table).
            en.lines()
                .filter(|l| l.contains("`mutating func`") && l.contains('|'))
                .last()
        })
        .expect("en Ch10 mutating row");
    // Swift-diff row must be Partially, not bare Implemented.
    let diff_en = en
        .split("## Differences from Swift")
        .nth(1)
        .expect("en Differences section")
        .lines()
        .find(|l| l.contains("`mutating func`"))
        .expect("en Swift-diff mutating row");
    assert!(
        diff_en.contains("**Partially implemented**") && !diff_en.contains("**Implemented** (Phase"),
        "en Ch10 Swift-diff mutating must be Partially; got {diff_en}"
    );
    assert!(
        en.contains("Honesty (C-8d)") && en.contains("≠"),
        "en Ch10 must have Honesty (C-8d)"
    );
    let _ = en_row;

    let diff_zh = zh
        .split("## 与 Swift 的差异")
        .nth(1)
        .expect("zh Differences section")
        .lines()
        .find(|l| l.contains("`mutating func`"))
        .expect("zh Swift-diff mutating row");
    assert!(
        diff_zh.contains("**部分实现**") && !diff_zh.contains("**已实现**"),
        "zh Ch10 Swift-diff mutating must be 部分实现; got {diff_zh}"
    );
    assert!(
        zh.contains("诚实（C-8d）") && zh.contains("≠"),
        "zh Ch10 must have 诚实（C-8d）"
    );
}

#[test]
fn c8d_ch17_typed_throws_honesty() {
    let en = ch17_en();
    let zh = ch17_zh();
    assert!(
        en.contains("Honesty (C-8d)")
            && en.contains("≠")
            && (en.contains("GA") || en.contains("residual")),
        "en Ch17 must Honesty (C-8d) typed throws ≠ GA"
    );
    assert!(
        en.contains("rethrows") && en.contains("Not implemented"),
        "en Ch17 honesty must keep rethrows Not implemented"
    );
    assert!(
        zh.contains("诚实（C-8d）") && zh.contains("≠") && (zh.contains("GA") || zh.contains("residual")),
        "zh Ch17 must 诚实（C-8d）typed throws ≠ GA"
    );
    assert!(
        zh.contains("rethrows") && zh.contains("未实现"),
        "zh Ch17 honesty must keep rethrows 未实现"
    );
}
