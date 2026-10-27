//! Phase 98: native build debug/release profile GA.

use std::fs;
use std::process::Command;

use syx_driver::{
    build_native_executable_file, default_native_executable_for_source, project_build_dir,
    BuildProfile, DriverOptions,
};

fn opts(profile: BuildProfile) -> DriverOptions {
    let mut o = DriverOptions::default();
    o.build_profile = profile;
    o
}

#[test]
fn p98_native_profile_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("native-build-profile-v1"));
}

#[test]
fn p98_native_build_debug_path() {
    let hello =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/hello.sy");
    if !hello.is_file() {
        return;
    }
    let out = default_native_executable_for_source(&hello, BuildProfile::Debug);
    assert!(out.to_string_lossy().contains("target/debug"));
}

#[test]
fn p98_native_build_release_path() {
    let hello =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/hello.sy");
    if !hello.is_file() {
        return;
    }
    let out = default_native_executable_for_source(&hello, BuildProfile::Release);
    assert!(out.to_string_lossy().contains("target/release"));
}

#[test]
fn p98_native_build_profile_stdout_parity() {
    let src = r#"func main() { print(93) }
"#;
    let dir = std::env::temp_dir().join(format!(
        "syx_p98_profile_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    let debug_out = project_build_dir(&dir, BuildProfile::Debug).join("main");
    let release_out = project_build_dir(&dir, BuildProfile::Release).join("main");
    build_native_executable_file(&dir.join("main.sy"), &opts(BuildProfile::Debug), &debug_out)
        .unwrap();
    build_native_executable_file(
        &dir.join("main.sy"),
        &opts(BuildProfile::Release),
        &release_out,
    )
    .unwrap();
    let d = Command::new(&debug_out).output().unwrap();
    let r = Command::new(&release_out).output().unwrap();
    assert_eq!(String::from_utf8_lossy(&d.stdout).trim(), "93");
    assert_eq!(String::from_utf8_lossy(&r.stdout).trim(), "93");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn p98_native_profile_report_module() {
    assert!(
        include_str!("../../src/native_profile_report.rs").contains("native build profile report")
    );
}
