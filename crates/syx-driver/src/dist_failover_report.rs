//! Phase 82: dist actor cross-continent failover report for `syx doctor --dist-failover-report`.

#[derive(Debug, Clone, Default)]
pub struct DistFailoverReport {
    pub cross_continent_auto_route_api: bool,
    pub failover_region_api: bool,
    pub failover_count: u64,
}

pub fn run_dist_failover_report() -> DistFailoverReport {
    let src = include_str!("../../syx-async/src/dist_transport.rs");
    DistFailoverReport {
        cross_continent_auto_route_api: src.contains("cross_continent_auto_route"),
        failover_region_api: src.contains("cross_continent_failover_region"),
        failover_count: src.matches("FAILOVER_COUNT").count() as u64,
    }
}

pub fn print_dist_failover_report(report: &DistFailoverReport) {
    println!("SYX dist cross-continent failover report (Phase 82)");
    println!(
        "  auto route API: {}",
        if report.cross_continent_auto_route_api {
            "yes"
        } else {
            "no"
        }
    );
    println!(
        "  failover region API: {}",
        if report.failover_region_api {
            "yes"
        } else {
            "no"
        }
    );
    println!("  failover events: {}", report.failover_count);
}
