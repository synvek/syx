#![allow(dead_code)]
#![allow(unused_imports)]
mod cancellation;
mod lower;
#[cfg(feature = "runtime")]
mod dist_transport;
#[cfg(feature = "runtime")]
mod hosts;
#[cfg(feature = "runtime")]
mod runtime;
#[cfg(feature = "runtime")]
mod task;

pub use cancellation::CancellationToken;
#[cfg(feature = "runtime")]
pub use dist_transport::{
    dist_actor_tls_rustls_chain_verify, dist_actor_tls_rustls_crate_chain_verify,
    dist_actor_tls_rustls_crate_e2e_enabled, dist_actor_tls_rustls_crate_e2e_verify,
    dist_actor_tls_rustls_crate_enabled, dist_actor_tls_rustls_crate_prod_default_on,
    dist_actor_tls_rustls_crate_prod_enabled,
    dist_actor_win_pipe_api_enabled, dist_actor_win_pipe_named_io_enabled,
    dist_actor_win_pipe_prod_default_on, dist_actor_win_pipe_winapi_enabled,
    dist_rustls_e2e_handshake_smoke, dist_rustls_e2e_prod_default_enabled,
    dist_winpipe_e2e_frame_smoke, dist_winpipe_e2e_prod_default_enabled,
    dist_actor_cluster_health_probe, cdn_route,
    multi_region_route_table, region_route,
    dist_actor_mp_clear, dist_actor_mp_configure, dist_actor_mp_doctor_mode,
    dist_actor_mp_fingerprint, dist_actor_mp_ga_requires_tcp, dist_actor_mp_inbox_clear,
    dist_actor_mp_inbox_count, dist_actor_mp_peer_send_once, region_route_mp, region_route_mp_ga,
    region_route_tcp, spawn_dist_tcp_mailbox, spawn_dist_tcp_mailbox_dual_stack, spawn_dist_tcp_mailbox_on,
    ArcStop, dist_inproc_enabled, dist_bind_addr_is_non_loopback_v4, dist_region_nonloopback_fingerprint,
    DistDualStackMailboxes,
    cdn_route_anycast, cdn_route_ga, cdn_doctor_mode, cdn_memory_dev_enabled,
    cdn_route_counts_as_ga, cdn_routes_memory_clear, cdn_routes_memory_count,
    cdn_bgp_clear, cdn_bgp_configure, cdn_bgp_endpoint_count, cdn_bgp_export_peer_view,
    cdn_bgp_fingerprint, cdn_bgp_import_peer_view, cdn_bgp_list_peer_ids, cdn_bgp_ops_snapshot,
    cdn_bgp_ops_visibility_ready, cdn_bgp_peer_exchange_done, cdn_bgp_peer_health_map,
    cdn_bgp_peer_view_count, cdn_bgp_clear_peer_views, CdnBgpOpsSnapshot,
    cdn_bgp_convergence_snapshot, cdn_bgp_failover_order_ids, cdn_bgp_route_convergence_ready,
    cdn_bgp_configure_mesh, cdn_bgp_import_peer_mesh, cdn_bgp_mesh_regions,
    cdn_bgp_mesh_region_count, cdn_bgp_mesh_peer_view_count, cdn_bgp_mesh_snapshot,
    cdn_bgp_multi_region_mesh_ready, cdn_bgp_cross_region_failover_ready,
    cdn_bgp_cross_region_failover_order_ids, cdn_bgp_multi_pop_ready,
    cdn_bgp_multi_pop_mvp_ready, cdn_bgp_mesh_pop_count, cdn_bgp_mesh_pop_ids,
    cdn_bgp_pop_weights, cdn_bgp_weights_differentiated,
    cdn_bgp_weighted_anycast_ready, cdn_bgp_weighted_anycast_mvp_ready,
    cdn_bgp_sticky_pop_ready, cdn_bgp_sticky_pop_mvp_ready, cdn_bgp_sticky_binding,
    cdn_bgp_sticky_binding_expires_at_ms, cdn_bgp_sticky_pop_binding_count,
    cdn_bgp_sticky_pop_clear, cdn_bgp_sticky_ttl_fingerprint, cdn_bgp_sticky_ttl_ms,
    cdn_bgp_sticky_ttl_ready, cdn_bgp_sticky_persist_fingerprint,
    cdn_bgp_sticky_persist_path, cdn_bgp_sticky_persist_ready,
    cdn_bgp_sticky_persist_restore_from_file, cdn_bgp_route_sticky,
    cdn_bgp_sticky_multi_snapshot_fingerprint, cdn_bgp_sticky_multi_snapshot_dir,
    cdn_bgp_sticky_multi_snapshot_ready, cdn_bgp_sticky_multi_snapshot_restore_by_key,
    cdn_bgp_sticky_multi_snapshot_path_for_key, cdn_bgp_sticky_shard_key,
    cdn_bgp_peer_announce_sticky_fingerprint, cdn_bgp_peer_announce_sticky_dir,
    cdn_bgp_peer_announce_sticky_encode, cdn_bgp_peer_announce_sticky,
    cdn_bgp_peer_announce_sticky_receive, cdn_bgp_peer_announce_sticky_receive_from_peer,
    cdn_bgp_peer_announce_sticky_receive_and_ack, cdn_bgp_peer_announce_write_ack,
    cdn_bgp_peer_announce_sticky_ready, cdn_bgp_peer_announce_sticky_path_for,
    cdn_bgp_peer_announce_ack_fingerprint, cdn_bgp_peer_announce_ack_ready,
    cdn_bgp_peer_announce_ack_mesh_ready, cdn_bgp_peer_announce_ack_path_for,
    cdn_bgp_peer_announce_fanout_fingerprint, cdn_bgp_peer_announce_fanout,
    cdn_bgp_peer_announce_fanout_ready, cdn_bgp_peer_announce_fanout_mesh_ready,
    cdn_bgp_peer_announce_conflict_epoch_fingerprint, cdn_bgp_peer_announce_fanout_with_epoch,
    cdn_bgp_peer_announce_receive_ack_with_epoch, cdn_bgp_peer_announce_winning_epoch,
    cdn_bgp_peer_announce_conflict_epoch_ready, cdn_bgp_peer_announce_conflict_epoch_mesh_ready,
    cdn_bgp_peer_announce_epoch_persist_fingerprint, cdn_bgp_peer_announce_epoch_persist_path,
    cdn_bgp_peer_announce_epoch_persist_ready, cdn_bgp_peer_announce_epoch_persist_restore_from_file,
    cdn_bgp_peer_announce_epoch_memory_clear,
    cdn_bgp_peer_announce_ttl_fingerprint, cdn_bgp_peer_announce_ttl_ms,
    cdn_bgp_peer_announce_ttl_ready, cdn_bgp_peer_announce_peer_wire_expired,
    cdn_bgp_peer_announce_purge_expired_peers,
    cdn_bgp_peer_announce_priority_fingerprint, cdn_bgp_peer_announce_default_priority,
    cdn_bgp_peer_announce_peer_priority, cdn_bgp_peer_announce_priority_ready,
    cdn_bgp_peer_announce_sort_peers_by_priority, cdn_bgp_peer_announce_priority_wins,
    cdn_bgp_peer_announce_hold_fingerprint, cdn_bgp_peer_announce_hold_ms,
    cdn_bgp_peer_announce_hold_active, cdn_bgp_peer_announce_hold_mark,
    cdn_bgp_peer_announce_filter_held_peers, cdn_bgp_peer_announce_hold_ready,
    cdn_bgp_peer_announce_hold_memory_clear, cdn_bgp_peer_announce_parse_hold_wire,
    CdnBgpMeshSnapshot,
    CdnBgpConvergenceSnapshot,
    cdn_bgp_ga_ready, cdn_bgp_has_endpoints, cdn_bgp_health, cdn_bgp_route,
    cdn_pop_clear, cdn_pop_configure, cdn_pop_endpoint_count, cdn_pop_fingerprint,
    cdn_pop_multi_hostkey_route_ready,
    cdn_pop_register_serve_pid, cdn_pop_serve_pid_count, cdn_pop_sibling_process_peer_registered,
    cdn_pop_sibling_process_ready,
    cdn_pop_has_endpoints, cdn_pop_health, cdn_pop_host_key_passes_ga, cdn_pop_dual_process_host_ready,
    cdn_pop_dual_process_peer_registered, cdn_pop_remote_hostkey_ready, cdn_pop_route,
    cdn_pop_connect_urls_non_loopback, cdn_pop_hostkeys_are_distinct_remote,
    dist_concrete_local_ipv4, dist_connect_url_is_loopback, dist_host_key_addr_is_unspecified,
    dist_host_key_concrete_ip_for_bind, dist_host_key_remote_for_bind, dist_host_key_remote_ip_part,
    dist_pop_note_spawned_child, dist_pop_os_child_try_serve, dist_pop_pids_are_siblings,
    dist_pop_register_dual_process_peer, dist_pop_register_sibling_process_peer,
    dist_tcp_url_for_bind_addr,
    cdn_delivery_ack_clear, cdn_delivery_ack_count,
    cdn_tcp_peer_clear, cdn_tcp_peer_configure, cdn_tcp_peer_endpoint_count,
    cdn_tcp_peer_fingerprint, cdn_tcp_peer_has_endpoints, cdn_tcp_peer_inbox_count,
    cdn_tcp_peer_route,
    dist_region_anycast_clear, dist_region_anycast_configure,
    dist_region_anycast_endpoint_count, dist_region_anycast_fingerprint,
    dist_region_anycast_has_endpoints, dist_region_anycast_route, dist_region_tcp_health,
    dist_region_default_host_key, dist_host_key_addr_is_documentation,
    dist_host_key_addr_is_loopback, dist_host_key_bind_eq_key,
    dist_host_key_has_live_listener, dist_host_key_register_live_listener_bind,
    dist_host_key_register_runtime_bind,
    dist_host_key_runtime_binds_clear,
    dist_region_multihost_clear, dist_region_multihost_configure,
    dist_region_multihost_configure_keys, dist_region_multihost_endpoint_count,
    dist_region_multihost_fingerprint, dist_region_multihost_has_nonloopback_host_key,
    dist_region_multihost_has_runtime_bound_host_key,
    dist_region_multihost_has_runtime_bound_host_key_v4,
    dist_region_multihost_has_runtime_bound_host_key_v5,
    dist_host_key_runtime_bind_addr_is_ga,
    dist_region_multihost_has_endpoints, dist_region_multihost_route,
    dist_region_multihost_tcp_health, dist_region_routing_has_tcp,
};
#[cfg(feature = "runtime")]
pub use hosts::{install_hosts, wrap_await_value};
pub use lower::{lower_async_module, validate_no_unlowered_loop_await};
#[cfg(feature = "runtime")]
pub use runtime::{
    cooperative_yield, current_runtime, current_processing_actor, enter_processing_actor,
    leave_processing_actor, actor_reentrancy_policy,
    dist_actor_clear_process_routes, dist_actor_clear_routes,
    dist_actor_last_route, dist_actor_process_count, dist_actor_process_enabled, dist_actor_recv_process,
    dist_actor_route, dist_actor_route_count, dist_actor_send_process, dist_actor_send_tls,
    dist_actor_tls_count, dist_actor_tls_enabled, dist_actor_tls_real_count,
    dist_actor_tls_real_enabled, dist_actor_tls_rustls_count, dist_actor_tls_rustls_enabled,
    dist_actor_recv_tls, dist_actor_uds_enabled, dist_actor_uds_prod_enabled,
    dist_actor_win_pipe_enabled, dist_actor_win_pipe_prod_enabled,
    dist_actor_tls_rustls_prod_enabled,
    distributed_actor_mvp_enabled, mailbox_capacity, max_actors, probe_bounded_queue_abort_mvp,
    probe_bounded_queue_caller_runs_mvp, probe_bounded_queue_discard_oldest_mvp,
    probe_core_max_keepalive_mvp, probe_actor_shared_pool_mvp,
    set_actor_shared_pool_enabled, set_actor_shared_pool_size,
    actor_shared_pool_enabled, configured_actor_shared_pool_size,
    set_configured_task_queue_capacity, set_configured_task_reject_policy,
    configured_task_reject_policy, TaskRejectPolicy,
    set_configured_core_pool_size, set_configured_max_pool_size, set_configured_keep_alive_ms,
    configured_core_pool_size, configured_max_pool_size, configured_keep_alive_ms,
    set_mailbox_capacity, set_max_actors,
    task_live_count,
    SyxRuntime,
};
#[cfg(feature = "runtime")]
pub use task::{
    apply_deadline_to_cancel, clear_current_task_cancel, current_task_cancel, group_id_from_value,
    set_current_task_cancel, set_deadline_ms, task_id_from_value, task_local_get, task_local_set,
    task_should_stop, TaskGroup, TaskHandle, TaskRegistry,
};

#[cfg(not(feature = "runtime"))]
pub fn cooperative_yield() {}

use syx_ast::{Decl, Expr, Module, Stmt};
use syx_diagnostics::{SyxDiagnostic, SyxResult};

pub struct Task<T> {
    pub id: u64,
    pub result: Option<T>,
    done: bool,
}

impl<T> Task<T> {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            result: None,
            done: false,
        }
    }

    pub fn value(&self) -> Option<&T> {
        self.result.as_ref()
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}

/// Legacy single-threaded future queue (tests).
pub struct Runtime {
    queue: std::collections::VecDeque<
        std::pin::Pin<Box<dyn std::future::Future<Output = SyxResult<()>> + Send>>,
    >,
    next_id: u64,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            queue: std::collections::VecDeque::new(),
            next_id: 1,
        }
    }
}

impl Runtime {
    pub fn spawn<F>(&mut self, fut: F) -> u64
    where
        F: std::future::Future<Output = SyxResult<()>> + Send + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;
        self.queue.push_back(Box::pin(fut));
        id
    }

    pub fn run_until_idle(&mut self) {
        while let Some(mut task) = self.queue.pop_front() {
            let waker = noop_waker();
            let mut cx = std::task::Context::from_waker(&waker);
            while let std::task::Poll::Pending = task.as_mut().poll(&mut cx) {}
        }
    }
}

fn noop_waker() -> std::task::Waker {
    use std::task::{RawWaker, RawWakerVTable, Waker};
    static VTABLE: RawWakerVTable = RawWakerVTable::new(
        |_| RawWaker::new(std::ptr::null(), &VTABLE),
        |_| {},
        |_| {},
        |_| {},
    );
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}

pub async fn blocking<F, T>(f: F) -> SyxResult<T>
where
    F: FnOnce() -> SyxResult<T>,
{
    f()
}

pub fn await_poll<F, T>(f: F) -> SyxResult<T>
where
    F: FnOnce() -> SyxResult<T>,
{
    cooperative_yield();
    f()
}

pub fn blocking_poll<F, T>(f: F) -> SyxResult<T>
where
    F: FnOnce() -> SyxResult<T>,
{
    await_poll(f)
}

pub fn count_await_in_stmts(stmts: &[Stmt]) -> u32 {
    stmts.iter().map(count_await_in_stmt).sum()
}

fn count_await_in_stmt(stmt: &Stmt) -> u32 {
    match stmt {
        Stmt::Return { value: Some(v), .. } => count_await_in_expr(v),
        Stmt::Expr { expr, .. } => count_await_in_expr(expr),
        Stmt::VarDecl { init: Some(v), .. } => count_await_in_expr(v),
        Stmt::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            count_await_in_expr(condition)
                + count_await_in_stmts(then_branch)
                + else_branch
                    .as_ref()
                    .map(|b| count_await_in_stmts(b))
                    .unwrap_or(0)
        }
        Stmt::While { condition, body, .. } => {
            count_await_in_expr(condition) + count_await_in_stmts(body)
        }
        Stmt::ForIn { iterable, body, .. } => {
            count_await_in_expr(iterable) + count_await_in_stmts(body)
        }
        Stmt::Block { stmts, .. } => count_await_in_stmts(stmts),
        Stmt::Try { body, catch_body, .. } => {
            count_await_in_stmts(body) + count_await_in_stmts(catch_body)
        }
        Stmt::GuardLet { body, else_branch, .. } => {
            count_await_in_stmts(body) + count_await_in_stmts(else_branch)
        }
        Stmt::Print { expr, .. } => count_await_in_expr(expr),
        Stmt::RepeatWhile { body, condition, .. } => {
            count_await_in_stmts(body) + count_await_in_expr(condition)
        }
        Stmt::Switch { subject, cases, default, .. } => {
            count_await_in_expr(subject)
                + cases
                    .iter()
                    .map(|c| {
                        c.guard.as_ref().map(count_await_in_expr).unwrap_or(0)
                            + count_await_in_stmts(&c.body)
                    })
                    .sum::<u32>()
                + default
                    .as_ref()
                    .map(|b| count_await_in_stmts(b))
                    .unwrap_or(0)
        }
        Stmt::IfCase {
            subject,
            body,
            else_branch,
            ..
        } => {
            count_await_in_expr(subject)
                + count_await_in_stmts(body)
                + else_branch
                    .as_ref()
                    .map(|b| count_await_in_stmts(b))
                    .unwrap_or(0)
        }
        Stmt::Throw { expr, .. } => count_await_in_expr(expr),
        _ => 0,
    }
}

pub fn count_await_in_expr(expr: &Expr) -> u32 {
    match expr {
        Expr::Await { .. } => 1,
        Expr::Binary { left, right, .. } => {
            count_await_in_expr(left) + count_await_in_expr(right)
        }
        Expr::Unary { expr, .. } => count_await_in_expr(expr),
        Expr::Call { callee, args, .. } => {
            count_await_in_expr(callee) + args.iter().map(|a| count_await_in_expr(&a.value)).sum::<u32>()
        }
        Expr::Member { object, .. } => count_await_in_expr(object),
        Expr::Index { object, index, .. } => {
            count_await_in_expr(object) + count_await_in_expr(index)
        }
        Expr::Assign { value, .. } => count_await_in_expr(value),
        Expr::Group { expr, .. } => count_await_in_expr(expr),
        Expr::Array { elements, .. } => elements.iter().map(count_await_in_expr).sum(),
        Expr::OptionalChain { object, .. } => count_await_in_expr(object),
        Expr::ForceUnwrap { expr, .. } => count_await_in_expr(expr),
        Expr::Literal { value, .. } => match value {
            syx_ast::Literal::Interpolated(parts) => parts
                .iter()
                .map(|p| match p {
                    syx_ast::StringPart::Expr(e) => count_await_in_expr(e),
                    _ => 0,
                })
                .sum(),
            _ => 0,
        },
        _ => 0,
    }
}

pub fn validate_async_functions(module: &Module) -> SyxResult<()> {
    for decl in &module.decls {
        if let Decl::Function {
            name,
            body,
            is_async,
            ..
        } = decl
        {
            if *is_async {
                let _await_count = count_await_in_stmts(body);
            } else if count_await_in_stmts(body) > 0 {
                return Err(SyxDiagnostic::error(format!(
                    "Function '{name}' uses await but is not async"
                ))
                .with_code("A002"));
            }
        }
    }
    Ok(())
}

pub fn prepare_async_module(module: &mut Module) -> SyxResult<()> {
    validate_async_functions(module)?;
    validate_no_unlowered_loop_await(module)?;
    lower_async_module(module)
}

#[cfg(test)]
mod tests {
    use super::*;
    use syx_diagnostics::SourceFile;
    use syx_parser::Parser;

    #[test]
    fn runtime_executes_spawned_task() {
        let mut rt = Runtime::default();
        rt.spawn(async { Ok(()) });
        rt.run_until_idle();
    }

    #[test]
    fn blocking_poll_runs_closure() {
        let v = blocking_poll(|| Ok(42)).unwrap();
        assert_eq!(v, 42);
    }

    #[test]
    fn validates_single_await() {
        let source = SourceFile::new("t.sy", "async func f() { let x = await g() }");
        let module = Parser::parse(source).unwrap();
        validate_async_functions(&module).unwrap();
    }
}
