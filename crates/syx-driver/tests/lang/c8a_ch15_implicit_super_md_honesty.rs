//! C-8a: Ch15 must not claim compiler-inserted implicit super.init / CallSuper.
//! Omit path = U-38 inherited field defaults ≠ CallSuper (true implicit CallSuper parked).

#[test]
fn c8a_ch15_en_no_compiler_inserts_claim() {
    let en = include_str!("../../../../books/language-guide/15-initialization.md");
    assert!(
        !en.contains("the compiler inserts it"),
        "en Ch15 must not claim compiler inserts super.init"
    );
    assert!(
        !en.contains("automatically inserts"),
        "en Ch15 must not claim automatically inserts"
    );
    assert!(
        en.contains("≠ CallSuper") || en.contains("!= CallSuper") || en.contains("≠ `CallSuper`"),
        "en Ch15 must state omit path ≠ CallSuper"
    );
    assert!(
        en.contains("Not implemented") && en.contains("CallSuper"),
        "en Ch15 Swift-diff / honesty must mark implicit CallSuper not implemented"
    );
    assert!(
        en.contains("U-38") || en.contains("inherited field defaults"),
        "en Ch15 must cite U-38 / inherited field defaults mechanism"
    );
}

#[test]
fn c8a_ch15_zh_no_auto_insert_claim() {
    let zh = include_str!("../../../../books/zh-cn/language-guide/15-initialization.md");
    assert!(
        !zh.contains("编译器自动插入"),
        "zh Ch15 must not claim 编译器自动插入 super.init"
    );
    assert!(
        zh.contains("≠ CallSuper") || zh.contains("≠ `CallSuper`") || zh.contains("≠`CallSuper`"),
        "zh Ch15 must state omit path ≠ CallSuper"
    );
    assert!(
        zh.contains("未实现") && zh.contains("CallSuper"),
        "zh Ch15 must mark implicit CallSuper 未实现"
    );
    assert!(
        zh.contains("U-38") || zh.contains("字段默认"),
        "zh Ch15 must cite U-38 / field-default mechanism"
    );
}
