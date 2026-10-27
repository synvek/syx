//! Phase 135 Track A: Class Properties v3 cache fingerprints.
//! Wired into `syx-cache-v107` invalidation cluster.

/// `super.init(...)` on subclass instances preserves parent stored-field readback.
pub const SUPER_INIT_FIELD_READBACK: &str = "lang-props-v3:super-init-field-readback";

/// Class computed `{ get set }` MVP (getter + setter desugar).
pub const COMPUTED_GET_SET_MVP: &str = "lang-props-v3:computed-get-set-mvp";

/// Property `override` MVP with polymorphic getter/setter dispatch.
pub const PROPERTY_OVERRIDE_MVP: &str = "lang-props-v3:property-override-mvp";

/// Docs honesty for Properties v3; full Swift property model → 136+.
pub const DOCS_HONESTY: &str = "lang-props-v3:docs-honesty";
