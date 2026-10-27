//! PBKDF2 facade/host ABI + experimental smoke.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn pbkdf2_four_arg_smoke() {
    let out = run(
        r#"
func main() {
  let dk = __syx_crypto_pbkdf2("password", "salt", 1, 20)
  print(dk)
}
"#,
    );
    let hex = out.lines().next().unwrap_or("").trim();
    assert_eq!(
        hex.to_lowercase(),
        "120fb6cffcf8b32c43e7225256c4f837a86548c9",
        "RFC6070 PBKDF2-HMAC-SHA256 vector, got {hex:?} from {out}"
    );
}

#[test]
fn pbkdf2_package_documents_experimental() {
    let src = include_str!("../../../../packages/crypto/mod.sy");
    assert!(src.contains("experimental"));
    assert!(src.contains("dkLen"));
}
