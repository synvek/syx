//! Phase 81+/117: dist actor CDN route report for `syx doctor --dist-cdn-route-report`.

#[derive(Debug, Clone, Default)]
pub struct DistCdnRouteReport {
    pub cdn_doctor_mode: String,
    pub pop_endpoint_count: usize,
    pub pop_ga_ready: bool,
    pub bgp_ga_ready: bool,
    pub pop_fingerprint: String,
    pub bgp_fingerprint: String,
    pub failover_api: bool,
    pub cdn_route_api: bool,
}

pub fn run_dist_cdn_route_report() -> DistCdnRouteReport {
    DistCdnRouteReport {
        cdn_doctor_mode: syx_async::cdn_doctor_mode().to_string(),
        pop_endpoint_count: syx_async::cdn_pop_endpoint_count(),
        pop_ga_ready: syx_async::cdn_pop_sibling_process_ready(),
        bgp_ga_ready: syx_async::cdn_bgp_ga_ready(),
        pop_fingerprint: syx_async::cdn_pop_fingerprint().to_string(),
        bgp_fingerprint: syx_async::cdn_bgp_fingerprint().to_string(),
        failover_api: include_str!("../../syx-async/src/dist_transport.rs")
            .contains("cdn_failover_region"),
        cdn_route_api: include_str!("../../syx-async/src/dist_transport.rs").contains("cdn_route"),
    }
}

pub fn print_dist_cdn_route_report(report: &DistCdnRouteReport) {
    println!("SYX dist CDN route report (Phase 118)");
    println!("  CDN doctor mode: {}", report.cdn_doctor_mode);
    println!("  PoP fingerprint: {}", report.pop_fingerprint);
    println!("  BGP fingerprint: {}", report.bgp_fingerprint);
    println!("  PoP endpoint count: {}", report.pop_endpoint_count);
    println!(
        "  PoP sibling-process GA: {}",
        if report.pop_ga_ready {
            "ready"
        } else {
            "not-ready"
        }
    );
    println!(
        "  BGP peer-exchange GA: {}",
        if report.bgp_ga_ready {
            "ready"
        } else {
            "not-ready"
        }
    );
    println!(
        "  failover API: {}",
        if report.failover_api { "yes" } else { "no" }
    );
    println!(
        "  cdn_route API: {}",
        if report.cdn_route_api { "yes" } else { "no" }
    );
}
