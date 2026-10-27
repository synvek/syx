//! Public matrix identity (no tip / ordinal fingerprint nails).
//! Tip wave+ordinal live only in the internal private packages overlay.

const MATRIX: &str = include_str!("../../../../packages/internal/hostfn_qs1_matrix.toml");

fn quoted_field(src: &str, key: &str) -> &'static str {
    let needle = format!("{key} = \"");
    let start = src
        .find(&needle)
        .unwrap_or_else(|| panic!("matrix missing `{key} = \"…\"`"));
    let rest = &src[start + needle.len()..];
    let end = rest
        .find('"')
        .unwrap_or_else(|| panic!("matrix `{key}` unterminated"));
    let abs = start + needle.len();
    &MATRIX[abs..abs + end]
}

/// Live `wave` / `fingerprint` from public hostfn_qs1_matrix.toml.
pub fn matrix_wave_and_fingerprint() -> (&'static str, &'static str) {
    (quoted_field(MATRIX, "wave"), quoted_field(MATRIX, "fingerprint"))
}

/// Nail: public matrix keeps schema + stable non-tip identity.
pub fn assert_live_matrix_fingerprint() {
    assert!(
        MATRIX.contains("schema_version = 1") || MATRIX.contains("schema_version=1"),
        "hostfn_qs1_matrix.toml must declare schema_version"
    );
    let (wave, fp) = matrix_wave_and_fingerprint();
    assert!(
        !wave.is_empty() && !wave.starts_with("A-W"),
        "public matrix wave must not be tip A-W*, got {wave:?}"
    );
    assert!(
        fp == "qs1-platform-gate-v0:public"
            || (fp.starts_with("qs1-platform-gate-v0:") && !fp.contains("thousand")),
        "public matrix fingerprint must be non-tip stable id, got {fp:?}"
    );
    assert!(
        MATRIX.contains(&format!("wave = \"{wave}\"")),
        "matrix must contain wave = \"{wave}\""
    );
}
