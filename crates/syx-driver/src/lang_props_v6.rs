//! Phase 138 Track A: Class Properties v6 cache fingerprints.
//! Wired into `syx-cache-v110` invalidation cluster.

/// `static var` willSet/didSet observers MVP (not instance `@Published` deepen).
pub const STATIC_OBSERVERS_MVP: &str = "lang-props-v6:static-observers-mvp";

/// Docs honesty for Properties v6; full Swift property model → 139+.
pub const DOCS_HONESTY: &str = "lang-props-v6:docs-honesty";
