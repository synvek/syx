//! Phase 136 Track A: Class Properties v4 cache fingerprints.
//! Wired into `syx-cache-v108` invalidation cluster.

/// Extension computed `{ get }` / `{ get set }` MVP (desugar to `__get_` / `__set_`).
pub const EXTENSION_COMPUTED_MVP: &str = "lang-props-v4:extension-computed-mvp";

/// Docs honesty for Properties v4; full Swift property model → 137+.
pub const DOCS_HONESTY: &str = "lang-props-v4:docs-honesty";
