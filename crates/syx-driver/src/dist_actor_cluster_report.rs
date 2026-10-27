//! Phase 80: dist actor cluster report for `syx doctor --dist-actor-cluster-report`.

#[derive(Debug, Clone, Default)]
pub struct DistActorClusterReport {
    pub regions: Vec<String>,
    pub health_us_east: bool,
    pub route_table_api: bool,
}

pub fn run_dist_actor_cluster_report() -> DistActorClusterReport {
    let regions: Vec<String> = syx_async::multi_region_route_table()
        .into_iter()
        .map(str::to_string)
        .collect();
    DistActorClusterReport {
        health_us_east: syx_async::dist_actor_cluster_health_probe("us-east"),
        route_table_api: include_str!("../../syx-async/src/dist_transport.rs")
            .contains("multi_region_route_table"),
        regions,
    }
}

pub fn print_dist_actor_cluster_report(report: &DistActorClusterReport) {
    println!("SYX dist actor cluster report (Phase 80)");
    println!("  regions: {}", report.regions.join(", "));
    println!(
        "  us-east health: {}",
        if report.health_us_east { "ok" } else { "fail" }
    );
    println!(
        "  multi-region route API: {}",
        if report.route_table_api { "yes" } else { "no" }
    );
}
