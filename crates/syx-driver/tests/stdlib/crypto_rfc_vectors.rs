//! Crypto HostFn RFC known-answer + tamper nails (SHA/HMAC/PBKDF2/Ed25519).

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn crypto_rfc_sha256_empty_vector() {
    // FIPS 180-4 / empty message SHA-256 known answer.
    let out = run(
        r#"
func main() {
  print(__syx_crypto_sha256(""))
}
"#,
    );
    let hex = out.lines().next().unwrap_or("").trim().to_lowercase();
    assert_eq!(
        hex, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "sha256 empty vector must exact: {out}"
    );
}

#[test]
fn crypto_tamper_ed25519_verify_false() {
    // Same seed/pk as W13 roundtrip; verify against wrong message → exact false.
    let out = run(
        r#"
func main() {
  let key = "0102030405060708590a0b0c0d0e0f101112131415161718591a1b1c1d1e1f"
  let pk = "598aab146dd9c6546b96fedec96e5b0af4e665f3ca0eb00335aaae75e0dc8288"
  let sig = __syx_crypto_ed25519_sign(key, "p87")
  print(__syx_crypto_ed25519_verify(pk, "p88", sig))
}
"#,
    );
    assert_eq!(
        out.trim(),
        "false",
        "ed25519 tamper (wrong message) must exact false: {out}"
    );
}

#[test]
fn crypto_rfc_sha512_empty_vector() {
    // FIPS 180-4 / empty message SHA-512 known answer (W348 / ADR-692).
    let out = run(
        r#"
func main() {
  print(__syx_crypto_sha512(""))
}
"#,
    );
    let hex = out.lines().next().unwrap_or("").trim().to_lowercase();
    assert_eq!(
        hex,
        "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e",
        "sha512 empty vector must exact: {out}"
    );
}

#[test]
fn crypto_rfc_hmac_sha256_rfc4231_jefe() {
    // RFC 4231 test case 2 — key "Jefe", data "what do ya want for nothing?" (W348 / ADR-692).
    let out = run(
        r#"
func main() {
  print(__syx_crypto_hmac_sha256("Jefe", "what do ya want for nothing?"))
}
"#,
    );
    let hex = out.lines().next().unwrap_or("").trim().to_lowercase();
    assert_eq!(
        hex, "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843",
        "HMAC-SHA256 RFC4231 Jefe vector must exact: {out}"
    );
}

#[test]
fn crypto_rfc_pbkdf2_c2_vector() {
    // Same SHA-256-family vector as m0_pbkdf2 c=1, with iterations=2 (W349 / ADR-693).
    let out = run(
        r#"
func main() {
  print(__syx_crypto_pbkdf2("password", "salt", 2, 20))
}
"#,
    );
    let hex = out.lines().next().unwrap_or("").trim().to_lowercase();
    assert_eq!(
        hex, "ae4d0c95af6b46d32d0adff928f06dd02a303f8e",
        "pbkdf2 c=2 vector must exact: {out}"
    );
}

#[test]
fn crypto_tamper_ed25519_sig_byte_flip() {
    // Same seed/pk as W347; flip first hex digit of signature; message stays "p87" → false.
    let key = "0102030405060708590a0b0c0d0e0f101112131415161718591a1b1c1d1e1f";
    let pk = "598aab146dd9c6546b96fedec96e5b0af4e665f3ca0eb00335aaae75e0dc8288";
    let sig = run(&format!(
        r#"
func main() {{
  print(__syx_crypto_ed25519_sign("{key}", "p87"))
}}
"#
    ))
    .lines()
    .next()
    .unwrap_or("")
    .trim()
    .to_lowercase();
    assert!(
        sig.len() >= 2 && sig.chars().all(|c| c.is_ascii_hexdigit()),
        "ed25519 sign must return hex: {sig}"
    );
    let mut chars: Vec<char> = sig.chars().collect();
    chars[0] = if chars[0] == '0' { '1' } else { '0' };
    let bad: String = chars.into_iter().collect();
    let out = run(&format!(
        r#"
func main() {{
  print(__syx_crypto_ed25519_verify("{pk}", "p87", "{bad}"))
}}
"#
    ));
    assert_eq!(
        out.trim(),
        "false",
        "ed25519 tamper (sig-byte flip) must exact false: {out}"
    );
}
