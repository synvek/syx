//! Phase 80: farm billing meter report for `syx doctor --farm-billing-report`.

use syx_pkg::SyxLockfile;

#[derive(Debug, Clone, Default)]
pub struct FarmBillingReport {
    pub billing_meter_id: Option<String>,
    pub compile_minutes: u32,
}

pub fn run_farm_billing_report() -> FarmBillingReport {
    let lock = SyxLockfile::default();
    let compile_minutes = std::env::var("SYX_FARM_COMPILE_MINUTES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    FarmBillingReport {
        billing_meter_id: lock.billing_meter_id,
        compile_minutes,
    }
}

pub fn print_farm_billing_report(report: &FarmBillingReport) {
    println!("SYX farm billing report (Phase 80)");
    println!(
        "  billing meter: {}",
        report.billing_meter_id.as_deref().unwrap_or("(none)")
    );
    println!("  compile minutes (env): {}", report.compile_minutes);
}
