//! Phase 77: build farm queue report for `syx doctor --farm-report`.

#[derive(Debug, Clone, Default)]
pub struct FarmReport {
    pub last_queue_hit: Option<String>,
    pub push_enabled: bool,
}

pub fn run_farm_report() -> FarmReport {
    FarmReport {
        last_queue_hit: crate::farm_queue_last_hit(),
        push_enabled: std::env::var("SYX_CACHE_REMOTE_PUSH")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(true),
    }
}

pub fn print_farm_report(report: &FarmReport) {
    println!("SYX farm report (Phase 77)");
    println!(
        "  cache push enabled: {}",
        if report.push_enabled { "yes" } else { "no" }
    );
    println!(
        "  last queue hit: {}",
        report.last_queue_hit.as_deref().unwrap_or("(none)")
    );
}
