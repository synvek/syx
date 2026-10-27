//! Phase 139 Track L: language surface cache fingerprints (L1–L7).
//! Wired into `syx-cache-v1` invalidation cluster.

// L1 — multiline string `"""`
pub const STRING_MULTILINE_MVP: &str = "lang-string-v1:multiline-literal-mvp";
pub const STRING_DOCS_HONESTY: &str = "lang-string-v1:docs-honesty";

// L2 — tuple syntax
pub const TUPLE_PRODUCT_TYPE_MVP: &str = "lang-tuple-v1:product-type-mvp";
pub const TUPLE_NAMED_ELEMENTS_MVP: &str = "lang-tuple-v1:named-elements-mvp";
pub const TUPLE_DOCS_HONESTY: &str = "lang-tuple-v1:docs-honesty";

// L3 — nested type visibility
pub const NESTED_QUALIFIED_PATH_MVP: &str = "lang-nested-v1:qualified-path-mvp";
pub const NESTED_DOCS_HONESTY: &str = "lang-nested-v1:docs-honesty";

// L4 — closed range `...`
pub const RANGE_CLOSED_MVP: &str = "lang-range-v1:closed-range-mvp";
pub const RANGE_DOCS_HONESTY: &str = "lang-range-v1:docs-honesty";

// L5 — Character distinct scalar
pub const CHARACTER_DISTINCT_SCALAR_MVP: &str = "lang-character-v1:distinct-scalar-mvp";
pub const CHARACTER_DOCS_HONESTY: &str = "lang-character-v1:docs-honesty";

// L6 — named parameters (external / `_` / required labels)
pub const CALL_EXTERNAL_LABEL_MVP: &str = "lang-call-v1:external-label-mvp";
pub const CALL_OMIT_UNDERSCORE_MVP: &str = "lang-call-v1:omit-underscore-mvp";
pub const CALL_LABELED_ARGS_REQUIRED: &str = "lang-call-v1:labeled-args-required";
pub const CALL_DOCS_HONESTY: &str = "lang-call-v1:docs-honesty";

// L7 — `final` vs `sealed` docs debt
pub const FINAL_SEALED_REMOVED_DOCS: &str = "lang-final-v1:sealed-removed-docs";
pub const FINAL_ONLY_HONESTY: &str = "lang-final-v1:final-only-honesty";
