//! Doctor report for Go-like single-file native SDK layout (`syx doctor --native-sdk`).

pub use syx_rt::NativeSdkLayoutReport;

pub fn run_native_sdk_report() -> NativeSdkLayoutReport {
    syx_rt::native_sdk_layout_report()
}

pub fn print_native_sdk_report(report: &NativeSdkLayoutReport) {
    println!("syx doctor --native-sdk");
    println!("  triple:     {}", report.triple);
    println!(
        "  rt:         {}",
        report
            .rt_archive
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "(missing)".into())
    );
    println!(
        "  sysroot:    {}",
        report
            .sysroot
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "(missing)".into())
    );
    println!("  sysroot_ok: {}", report.sysroot_ok);
    println!("  ok:         {}", report.ok);
    for n in &report.notes {
        println!("  note: {n}");
    }
}
