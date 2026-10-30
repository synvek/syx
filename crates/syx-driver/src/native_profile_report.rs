//! Phase 93: native build profile report for `syx doctor --native-profile-report`.

use syx_rt::rt_archive_path_for_profile;

#[derive(Debug, Clone, Default)]
pub struct NativeProfileReport {
    pub debug_clang_args: Vec<String>,
    pub release_clang_args: Vec<String>,
    pub debug_rt_archive: Option<String>,
    pub release_rt_archive: Option<String>,
    pub debug_link_flags: Vec<String>,
    pub release_link_flags: Vec<String>,
}

pub fn run_native_profile_report() -> NativeProfileReport {
    let debug_rt = rt_archive_path_for_profile("debug")
        .ok()
        .map(|p| p.display().to_string());
    let release_rt = rt_archive_path_for_profile("release")
        .ok()
        .map(|p| p.display().to_string());
    NativeProfileReport {
        debug_clang_args: vec!["-O0".into(), "-g".into()],
        release_clang_args: vec!["-O2".into(), "-DNDEBUG".into()],
        debug_rt_archive: debug_rt,
        release_rt_archive: release_rt,
        debug_link_flags: Vec::new(),
        release_link_flags: profile_link_flags(true),
    }
}

fn profile_link_flags(release: bool) -> Vec<String> {
    if !release {
        return Vec::new();
    }
    #[cfg(target_os = "macos")]
    {
        vec!["-Wl,-dead_strip".into()]
    }
    #[cfg(target_os = "linux")]
    {
        vec!["-Wl,--gc-sections".into()]
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        let _ = LinkOptions::default();
        Vec::new()
    }
}

pub fn print_native_profile_report(report: &NativeProfileReport) {
    println!("SYX native build profile report (Phase 93)");
    println!("  debug clang: {:?}", report.debug_clang_args);
    println!("  release clang: {:?}", report.release_clang_args);
    println!(
        "  debug syx-rt: {}",
        report.debug_rt_archive.as_deref().unwrap_or("missing")
    );
    println!(
        "  release syx-rt: {}",
        report.release_rt_archive.as_deref().unwrap_or("missing")
    );
    println!("  release link flags: {:?}", report.release_link_flags);
}
