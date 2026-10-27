use syx_config;

/// Phase 23 ADR-020: hygienic `@attached` v2 with `{type}` prefix in templates.
pub fn attached_hygienic_v2_enabled() -> bool {
    std::env::var("SYX_ATTACHED_HYGIENIC_V2")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true)
}

/// Phase 41: user `#macro` param gensym prefix (ADR-059 v1). Opt out via `--no-macro-hygiene` or `SYX_MACRO_HYGIENE=0`.
pub fn macro_hygiene_enabled() -> bool {
    syx_config::macro_hygiene_enabled()
}

/// Phase 23 ADR-018/020: built-in `@attached` source body templates.
pub fn attached_source_template(kind: &str) -> &'static str {
    if attached_hygienic_v2_enabled() {
        return match kind {
            "peer" => r#"print("peer:{type}:{name}")"#,
            "member" => r#"print("member:{type}:{name}")"#,
            "custom" => r#"print("custom:{type}:{name}")"#,
            "accessor" => r#"return self.{name}"#,
            "deinit" => r#"print("deinit:{type}")"#,
            "extension" => r#"func describe() -> String { return "{type}" }"#,
            _ => r#"print("attached:{type}:{name}")"#,
        };
    }
    match kind {
        "peer" => r#"print("peer:{name}")"#,
        "member" => r#"print("member:{name}")"#,
        "custom" => r#"print("custom:{name}")"#,
        "accessor" => r#"return self.{name}"#,
        "deinit" => r#"print("deinit")"#,
        "extension" => r#"func describe() -> String { return "attached" }"#,
        _ => r#"print("attached:{name}")"#,
    }
}

pub(crate) fn attached_use_source_templates() -> bool {
    std::env::var("SYX_ATTACHED_USE_SOURCE")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .map(|disabled| !disabled)
        .unwrap_or(true)
}

pub(crate) fn attached_body_template_for(kind: &str, env_key: &str, default: &str) -> String {
    if attached_use_source_templates() {
        attached_source_template(kind).to_string()
    } else {
        crate::attached::attached_body_template(env_key, default)
    }
}
