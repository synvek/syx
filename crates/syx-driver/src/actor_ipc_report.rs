//! Phase 79: actor IPC + heap root smoke report.

#[derive(Debug, Clone, Default)]
pub struct ActorIpcReport {
    pub route_count: usize,
    pub ipc_send_ok: bool,
    pub heap_root_api: bool,
}

pub fn run_actor_ipc_report() -> ActorIpcReport {
    syx_async::dist_actor_clear_routes();
    let ipc_send_ok = syx_async::dist_actor_send_process(1, 2, "doctor-smoke");
    ActorIpcReport {
        route_count: syx_async::dist_actor_route_count(),
        ipc_send_ok,
        heap_root_api: include_str!("../../syx-runtime-core/src/value.rs").contains("actor_id"),
    }
}

pub fn print_actor_ipc_report(report: &ActorIpcReport) {
    println!("SYX actor IPC report (Phase 79)");
    println!("  route count: {}", report.route_count);
    println!(
        "  ipc send: {}",
        if report.ipc_send_ok { "yes" } else { "no" }
    );
    println!(
        "  heap root api: {}",
        if report.heap_root_api { "yes" } else { "no" }
    );
}
