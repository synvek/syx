//! C-8e: Ch10 en+zh Compatibility notes for F18/F19 legacy_accept.
//! ≠ C-8 [x] · ≠ parser surface · ≠ remove legacy accept.

fn ch10_en() -> &'static str {
    include_str!("../../../../books/language-guide/10-structures-and-classes.md")
}
fn ch10_zh() -> &'static str {
    include_str!("../../../../books/zh-cn/language-guide/10-structures-and-classes.md")
}

fn inventory_rows() -> Vec<Vec<&'static str>> {
    // C-9g2: same public fixture as c8c (≠ docs/census).
    let census = include_str!("fixtures/c8c_book_parser_fork_inventory.md");
    let mut rows = Vec::new();
    let mut in_table = false;
    for line in census.lines() {
        if line.starts_with("| id |") {
            in_table = true;
            continue;
        }
        if in_table {
            if !line.starts_with('|') {
                break;
            }
            if line.contains("---") {
                continue;
            }
            let cols: Vec<&str> = line
                .trim()
                .trim_matches('|')
                .split('|')
                .map(|c| c.trim())
                .collect();
            if cols.len() >= 7 && cols[0].starts_with('F') {
                rows.push(cols);
            }
        }
    }
    rows
}

#[test]
fn c8e_ch10_en_compat_bare_and_mutating() {
    let en = ch10_en();
    assert!(
        en.contains("Compatibility (C-8e)"),
        "en Ch10 must have Compatibility (C-8e)"
    );
    assert!(
        en.matches("Compatibility (C-8e)").count() >= 2,
        "en Ch10 must have F18 + F19 Compatibility blocks"
    );
    assert!(
        en.contains("bare `name: Type`") || en.contains("legacy bare"),
        "en Ch10 F18 must mention bare name: Type"
    );
    assert!(
        en.contains("one release cycle") || en.contains("one-cycle"),
        "en Ch10 must mention one-cycle / one release cycle"
    );
    assert!(
        en.contains("mutating func") && en.contains("S060") && en.contains("U-30"),
        "en Ch10 F19 must mention mutating / S060 / U-30"
    );
    assert!(
        en.contains("language-book-ssot"),
        "en Ch10 Compatibility must point at language-book-ssot"
    );
    assert!(
        en.contains("≠") && en.contains("park"),
        "en Ch10 must Non-claim removing legacy (≠ / park)"
    );
}

#[test]
fn c8e_ch10_zh_compat_bare_and_mutating() {
    let zh = ch10_zh();
    assert!(
        zh.contains("兼容（C-8e）"),
        "zh Ch10 must have 兼容（C-8e）"
    );
    assert!(
        zh.matches("兼容（C-8e）").count() >= 2,
        "zh Ch10 must have F18 + F19 兼容 blocks"
    );
    assert!(
        zh.contains("name: Type") || zh.contains("裸字段"),
        "zh Ch10 F18 must mention bare / 裸字段"
    );
    assert!(
        zh.contains("一个发布周期") || zh.contains("一周期"),
        "zh Ch10 must mention one-cycle"
    );
    assert!(
        zh.contains("mutating func") && zh.contains("S060") && zh.contains("U-30"),
        "zh Ch10 F19 must mention mutating / S060 / U-30"
    );
    assert!(
        zh.contains("language-book-ssot"),
        "zh Ch10 must point at language-book-ssot"
    );
}

#[test]
fn c8e_inventory_no_live_legacy_accept() {
    let rows = inventory_rows();
    let present: std::collections::BTreeSet<&str> = rows.iter().map(|r| r[4]).collect();
    assert!(
        !present.contains("legacy_accept"),
        "inventory must have no live legacy_accept after C-8e; present={present:?}"
    );
    let f18 = rows.iter().find(|r| r[0] == "F18").expect("F18");
    let f19 = rows.iter().find(|r| r[0] == "F19").expect("F19");
    assert_eq!(f18[4], "parked_honest");
    assert_eq!(f19[4], "parked_honest");
    assert!(
        f18[5].contains("C-8e") || f18[2].contains("C-8e"),
        "F18 must cite C-8e note"
    );
    assert!(
        f19[5].contains("C-8e") || f19[2].contains("C-8e"),
        "F19 must cite C-8e note"
    );
}
