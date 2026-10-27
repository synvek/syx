//! Phase 98: release thin-LTO opt-in smoke gate.

use syx_driver::BuildProfile;

#[test]
fn p98_native_lto_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("native-lto-v1"));
}

#[test]
fn p98_native_lto_default_off() {
    std::env::remove_var("SYX_NATIVE_LTO");
    assert!(!BuildProfile::Release.lto_enabled());
}

#[test]
fn p98_native_lto_opt_in_clang_args() {
    std::env::set_var("SYX_NATIVE_LTO", "1");
    let args = BuildProfile::Release.clang_ir_args();
    assert!(args.iter().any(|a| a.contains("flto")), "{args:?}");
    let link = BuildProfile::Release.link_profile_args();
    assert!(link.iter().any(|a| *a == "-flto"), "{link:?}");
    std::env::remove_var("SYX_NATIVE_LTO");
}
