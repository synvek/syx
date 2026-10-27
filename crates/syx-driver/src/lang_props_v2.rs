//! Phase 134 Track A: Class Properties v2 cache fingerprints.
//!
//! Wired into `syx-cache-v106` invalidation cluster.

/// Docs honesty for Class Properties v2 / SYX 112.0.
pub const DOCS_HONESTY: &str = "lang-props-v2:docs-honesty";

/// Inherited property observer lookup along `class_superclasses`.
pub const INHERITED_OBSERVERS: &str = "lang-props-v2:inherited-observers";

/// `init` body `self.field = …` skips willSet/didSet (Swift contract).
pub const INIT_SUPPRESS_OBSERVERS: &str = "lang-props-v2:init-suppress-observers";

/// Class computed get-only MVP (`var x: T { get { … } }`).
pub const COMPUTED_GET_ONLY: &str = "lang-props-v2:computed-get-only-mvp";

/// Class `static var` / `static let` MVP.
pub const STATIC_VAR: &str = "lang-props-v2:static-var-mvp";

/// Class `lazy var` MVP (first-read init; non-thread-safe).
pub const LAZY_VAR: &str = "lang-props-v2:lazy-var-mvp";
