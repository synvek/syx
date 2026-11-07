//! SYX driver feature-gate face.
//! Extracted in S2-I231 (zero semantics).

/// Phase 22: when true, embedded `Result` stdlib uses Phase 21 Object-only String API.
/// Phase 25 ADR-024: legacy Object Result wire removed; env is ignored.
pub fn result_legacy_object_enabled() -> bool {
    false
}

/// Phase 23 DE1: default builtin Result wire (inverse of legacy Object dict).
pub fn result_builtin_wire_enabled() -> bool {
    !result_legacy_object_enabled()
}

/// Phase 28 ED2: Result<T,E> generic syntax surface enabled (`SYX_RESULT_GENERIC=1` or default on).
pub fn result_generic_syntax_enabled() -> bool {
    std::env::var("SYX_RESULT_GENERIC")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 29 EI2: Result<T,E> full monomorphization for arbitrary T/E instantiations.
pub fn result_generic_full_mono_enabled() -> bool {
    std::env::var("SYX_RESULT_GENERIC_FULL")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(result_generic_syntax_enabled())
}

/// Phase 30 EN3: Result try/`?` expression ergonomics with mono linkage.
pub fn result_try_expr_enabled() -> bool {
    std::env::var("SYX_RESULT_TRY_EXPR")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(result_generic_full_mono_enabled())
}

/// Phase 31 ES3: async/Result try/`?` deep mono linkage at async/Actor boundaries.
pub fn result_async_try_mono_enabled() -> bool {
    std::env::var("SYX_RESULT_ASYNC_TRY")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(result_try_expr_enabled())
}

/// Phase 32 EX3: async/Result try/`?` full mono at async/Actor boundaries.
pub fn result_async_actor_try_mono_enabled() -> bool {
    std::env::var("SYX_RESULT_ASYNC_ACTOR_TRY")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(result_async_try_mono_enabled())
}

/// Phase 33 FC3: Result/Actor try production complete mono (`SYX_RESULT_ASYNC_ACTOR_TRY_PROD=0` opt-out).
pub fn result_async_actor_try_prod_mono_enabled() -> bool {
    std::env::var("SYX_RESULT_ASYNC_ACTOR_TRY_PROD")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(result_async_actor_try_mono_enabled())
}

/// Phase 34 FH4: Result/Actor try E2E prod (`SYX_RESULT_ASYNC_ACTOR_TRY_E2E=0` opt-out).
pub fn result_async_actor_try_e2e_enabled() -> bool {
    std::env::var("SYX_RESULT_ASYNC_ACTOR_TRY_E2E")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(result_async_actor_try_prod_mono_enabled())
}

/// Phase 33 FE2: Net setsockopt prod host path default on.
pub fn net_setsockopt_prod_enabled() -> bool {
    std::env::var("SYX_NET_SETSOCKOPT_PROD")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 34 FH3: typealias MVP syntax surface (`SYX_TYPEALIAS_MVP=1` opt-in).
pub fn typealias_mvp_enabled() -> bool {
    std::env::var("SYX_TYPEALIAS_MVP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(typealias_prod_enabled())
}

/// Phase 35 FM1: typealias prod default on (`SYX_TYPEALIAS=0` opt-out).
pub fn typealias_prod_enabled() -> bool {
    syx_semantic::typealias_prod_enabled()
}

/// Phase 35 FM2: property wrapper MVP (`SYX_PROPERTY_WRAPPER=1` opt-in).
pub fn property_wrapper_mvp_enabled() -> bool {
    syx_semantic::property_wrapper_mvp_enabled()
}

/// Phase 35 FL2: Net setsockopt real syscall prod default on.
pub fn net_setsockopt_real_enabled() -> bool {
    std::env::var("SYX_NET_SETSOCKOPT_REAL")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(net_setsockopt_prod_enabled())
}

/// Phase 35 FL3: dist rustls crate E2E prod (Unix default on; opt-out via env).
pub fn dist_rustls_crate_e2e_prod_enabled() -> bool {
    std::env::var("SYX_DIST_RUSTLS_E2E_PROD")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(cfg!(unix))
}

/// Phase 35 FL3: dist Win named pipe E2E prod (Windows default on).
pub fn dist_winpipe_e2e_prod_enabled() -> bool {
    std::env::var("SYX_DIST_WINPIPE_E2E_PROD")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(cfg!(windows))
}

/// Phase 23 ADR-020: hygienic `@attached` v2 with `{type}` prefix in templates.
pub fn attached_hygienic_v2_enabled() -> bool {
    syx_macro::attached_hygienic_v2_enabled()
}

/// Phase 41: user `#macro` param gensym prefix (ADR-059 v1). Opt out via `--no-macro-hygiene` or `SYX_MACRO_HYGIENE=0`.
pub fn macro_hygiene_enabled() -> bool {
    syx_macro::macro_hygiene_enabled()
}

/// Phase 23 ADR-018/020: built-in `@attached` source body templates.
pub fn attached_source_template(kind: &str) -> &'static str {
    syx_macro::attached_source_template(kind)
}

/// Phase 24 DJ1: weeks until `SYX_RESULT_LEGACY_OBJECT` removal (CI artifact).
pub fn result_legacy_retirement_weeks() -> u32 {
    std::env::var("SYX_RESULT_LEGACY_RETIRE_WEEKS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

/// Phase 24 DJ3: opt-in Iterator protocol desugar for `for-in` (`SYX_ITERATOR_PROTOCOL=1`).
/// Phase 25 DO2: Iterator protocol desugar default on; `SYX_ITERATOR_PROTOCOL=0` disables.
pub fn iterator_protocol_enabled() -> bool {
    std::env::var("SYX_ITERATOR_PROTOCOL")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}
