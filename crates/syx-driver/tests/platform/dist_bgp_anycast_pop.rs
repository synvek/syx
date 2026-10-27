//! Dist BGP anycast MVP + remote-hostkey PoP.

use std::fs;
use std::process::Command;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_bgp_clear, cdn_bgp_configure_mesh, cdn_bgp_fingerprint, cdn_bgp_ga_ready, cdn_bgp_health,
    cdn_bgp_import_peer_mesh, cdn_bgp_route, cdn_delivery_ack_clear, cdn_delivery_ack_count,
    cdn_doctor_mode, cdn_pop_clear,
    cdn_pop_configure, cdn_pop_dual_process_host_ready,
    cdn_pop_fingerprint, cdn_pop_has_endpoints, cdn_pop_host_key_passes_ga,
    cdn_pop_hostkeys_are_distinct_remote, cdn_pop_register_serve_pid, cdn_pop_remote_hostkey_ready, cdn_pop_route, cdn_route,
    cdn_route_counts_as_ga, cdn_route_ga, cdn_routes_memory_clear, dist_connect_url_is_loopback,
    dist_host_key_addr_is_unspecified, dist_host_key_bind_eq_key, dist_host_key_concrete_ip_for_bind,
    dist_host_key_has_live_listener, dist_host_key_register_runtime_bind,
    dist_host_key_remote_for_bind, dist_host_key_remote_ip_part,
    dist_pop_os_child_try_serve, dist_pop_register_dual_process_peer, dist_region_multihost_clear,
    dist_region_multihost_configure_keys, dist_region_multihost_fingerprint,
    dist_region_multihost_has_runtime_bound_host_key_v5, dist_region_multihost_route,
    dist_region_multihost_tcp_health, dist_region_nonloopback_fingerprint,
    dist_tcp_url_for_bind_addr, spawn_dist_tcp_mailbox_on,
};

static P117_DIST_LOCK: Mutex<()> = Mutex::new(());

fn spawn_remote_pop(max_conns: usize) -> (String, String, syx_async::ArcStop) {
    let (bind_addr, _handle, stop) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", max_conns);
    thread::sleep(Duration::from_millis(25));
    let host_key = dist_host_key_remote_for_bind(&bind_addr);
    let connect_url = dist_tcp_url_for_bind_addr(&bind_addr);
    assert!(
        !connect_url.contains("127.0.0.1"),
        "connect URL must not rewrite to 127.0.0.1: {connect_url}"
    );
    assert!(!dist_connect_url_is_loopback(&connect_url));
    assert!(
        !dist_host_key_addr_is_unspecified(&dist_host_key_remote_ip_part(&host_key)),
        "hostkey must be concrete IP, got {host_key}"
    );
    (host_key, connect_url, stop)
}

#[test]
fn dist_bgp_anycast_os_child_entry() {
    if dist_pop_os_child_try_serve() {
        return;
    }
}

#[test]
fn dist_bgp_anycast_cdn_bgp_pop_fingerprints() {
    assert_eq!(cdn_bgp_fingerprint(), "dist-cdn-bgp-v2:peer-exchange");
    assert_eq!(cdn_pop_fingerprint(), "dist-cdn-pop-v6:multi-hostkey-route");
    assert_eq!(
        dist_region_nonloopback_fingerprint(),
        "dist-region-hostkey-v6:concrete-ip"
    );
    assert_eq!(
        dist_region_multihost_fingerprint(),
        "dist-region-hostkey-v6:concrete-ip"
    );
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-bgp-v2:peer-exchange"));
    assert!(src.contains("dist-cdn-pop-v6:multi-hostkey-route"));
    assert!(src.contains("peer-exchange") || src.contains("peer exchange"));
    assert!(src.contains("dist_host_key_remote_for_bind"));
    assert!(!src.contains("global_cdn_route_table"));
    assert!(!src.contains("global_cdn_health_probe"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-cdn-bgp-v1:anycast-mvp"));
    assert!(lib.contains("dist-cdn-pop-v4:remote-hostkey"));
    assert!(lib.contains("dist-cdn-bgp-v2:peer-exchange"));
    assert!(lib.contains("dist-cdn-pop-v6:multi-hostkey-route"));
    assert!(lib.contains("select-deadline-strict-v1:no-env"));
    let report = include_str!("../../src/dist_cdn_route_report.rs");
    assert!(report.contains("sibling-process") || report.contains("peer-exchange"));
    let pkg = include_str!("../../../syx-pkg/src/lock_enforce.rs");
    assert!(!pkg.contains("enforce_global_cdn_sla"));
    assert!(!pkg.contains("global_cdn_sla"));
}

#[test]
fn dist_bgp_anycast_hostkey_unspecified_not_ga() {
    let _guard = P117_DIST_LOCK.lock().unwrap();
    dist_host_key_register_runtime_bind(
        &"0.0.0.0:0"
            .parse()
            .expect("unspecified bind"),
    );
    assert!(dist_host_key_addr_is_unspecified("0.0.0.0"));
    assert!(!cdn_pop_host_key_passes_ga("0.0.0.0"));
    syx_async::dist_host_key_runtime_binds_clear();
}

#[test]
fn dist_bgp_anycast_cdn_pop_loopback_dual_port_not_ga() {
    let _guard = P117_DIST_LOCK.lock().unwrap();
    cdn_pop_clear();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();

    let (addr_a, _ha, stop_a) = spawn_dist_tcp_mailbox_on("127.0.0.1:0", 8);
    let (addr_b, _hb, stop_b) = spawn_dist_tcp_mailbox_on("127.0.0.1:0", 8);
    thread::sleep(Duration::from_millis(20));
    let url_a = dist_tcp_url_for_bind_addr(&addr_a);
    let url_b = dist_tcp_url_for_bind_addr(&addr_b);
    let key_a = dist_host_key_remote_for_bind(&addr_a);
    let key_b = dist_host_key_remote_for_bind(&addr_b);
    cdn_pop_configure(&[
        ("pop-us", key_a.as_str(), url_a.as_str()),
        ("pop-eu", key_b.as_str(), url_b.as_str()),
    ]);
    dist_pop_register_dual_process_peer();

    assert!(!cdn_pop_remote_hostkey_ready());
    assert!(!cdn_pop_dual_process_host_ready());
    assert!(!cdn_pop_has_endpoints());
    let err = cdn_pop_route("pop-us", 1, 2, "loopback-dual")
        .expect_err("same-machine 127-only must not pass pop-v4 GA");
    assert!(
        err.contains("remote-hostkey")
            || err.contains("dual-process")
            || err.contains("non-loopback")
            || err.contains("127"),
        "unexpected err: {err}"
    );

    stop_a.stop();
    stop_b.stop();
    cdn_pop_clear();
}

#[test]
fn dist_bgp_anycast_cdn_pop_same_hostkey_not_distinct() {
    let _guard = P117_DIST_LOCK.lock().unwrap();
    cdn_pop_clear();
    let (key_a, url_a, stop_a) = spawn_remote_pop(8);
    let ip_only = dist_host_key_remote_ip_part(&key_a);
    cdn_pop_configure(&[
        ("pop-us", ip_only.as_str(), url_a.as_str()),
        ("pop-eu", ip_only.as_str(), url_a.as_str()),
    ]);
    dist_pop_register_dual_process_peer();
    assert!(!cdn_pop_hostkeys_are_distinct_remote());
    assert!(!cdn_pop_has_endpoints());
    assert!(!cdn_pop_remote_hostkey_ready());
    stop_a.stop();
    cdn_pop_clear();
}

#[test]
fn dist_bgp_anycast_cdn_bgp_anycast_mvp() {
    let _guard = P117_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_pop_clear();
    cdn_routes_memory_clear();
    cdn_delivery_ack_clear();
    std::env::remove_var("SYX_DIST_CDN_MEMORY");

    let (addr_a, _ha, stop_a) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    let (addr_b, _hb, stop_b) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    thread::sleep(Duration::from_millis(25));
    let url_a = dist_tcp_url_for_bind_addr(&addr_a);
    let url_b = dist_tcp_url_for_bind_addr(&addr_b);
    assert!(!url_a.contains("127.0.0.1"));
    assert!(!url_b.contains("127.0.0.1"));

    cdn_bgp_configure_mesh("us-east", &[
        ("203.0.113.0/24", 100, "bgp-us", url_a.as_str()),
    ]);
    cdn_bgp_import_peer_mesh(
        "peer-eu",
        "eu-west",
        &[("198.51.100.0/24", 250, "bgp-eu", url_b.as_str())],
    );
    assert!(cdn_bgp_ga_ready());
    assert!(cdn_route_counts_as_ga());
    assert_eq!(cdn_doctor_mode(), "cdn-bgp-multi-region-mesh");
    assert!(cdn_bgp_health("bgp-us"));
    assert!(cdn_bgp_health("bgp-eu"));

    let ack_before = cdn_delivery_ack_count();
    let (active, ack) =
        cdn_bgp_route("bgp-us", 1, 2, "p117-bgp-primary").expect("bgp anycast MVP primary");
    assert_eq!(active, "bgp-eu");
    assert_eq!(ack, "p117-bgp-primary");
    assert!(cdn_delivery_ack_count() > ack_before);

    stop_b.stop();
    thread::sleep(Duration::from_millis(40));
    assert!(!cdn_bgp_health("bgp-eu"));
    assert!(cdn_bgp_health("bgp-us"));
    let (active2, ack2) = cdn_bgp_route("bgp-eu", 3, 4, "p117-bgp-failover")
        .expect("failover to lower-priority healthy pop");
    assert_eq!(active2, "bgp-us");
    assert_eq!(ack2, "p117-bgp-failover");

    assert!(cdn_route("bgp-us", 5, 6, "p117-cdn-bgp-route"));
    let ga = cdn_route_ga("bgp-us", 7, 8, "p117-cdn-bgp-ga").expect("BGP CDN GA");
    assert!(ga.0.contains("bgp"));
    assert_eq!(ga.1, "p117-cdn-bgp-ga");

    stop_a.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}

#[test]
fn dist_bgp_anycast_cdn_pop_parent_child_not_sibling_ga() {
    if let Ok(url) = std::env::var("SYX_DIST_POP_CHILD_READY_FILE") {
        let _ = dist_pop_os_child_try_serve();
        let _ = url;
        return;
    }

    let _guard = P117_DIST_LOCK.lock().unwrap();
    cdn_pop_clear();
    cdn_bgp_clear();
    cdn_routes_memory_clear();
    cdn_delivery_ack_clear();
    std::env::remove_var("SYX_DIST_CDN_MEMORY");
    std::env::remove_var("SYX_DIST_POP_CHILD_STOP");

    let ready_path = std::env::temp_dir().join(format!(
        "syx-dist-pop-p117-ready-{}",
        std::process::id()
    ));
    let _ = fs::remove_file(&ready_path);

    let exe = std::env::current_exe().expect("current_exe");
    let child = Command::new(&exe)
        .env("SYX_DIST_POP_CHILD_BIND", "0.0.0.0:0")
        .env("SYX_DIST_POP_CHILD_READY_FILE", &ready_path)
        .env("SYX_DIST_POP_CHILD_MAX_CONNS", "32")
        .env("SYX_DIST_POP_PPID", std::process::id().to_string())
        .args([
            "dist_bgp_anycast_pop::dist_bgp_anycast_os_child_entry",
            "--exact",
            "--nocapture",
        ])
        .spawn()
        .expect("spawn child PoP process");
    syx_async::dist_pop_note_spawned_child(child.id());

    let mut child_host_key = String::new();
    let mut child_url = String::new();
    for _ in 0..100 {
        if let Ok(body) = fs::read_to_string(&ready_path) {
            let mut lines = body.lines();
            child_host_key = lines.next().unwrap_or("").to_string();
            child_url = lines.next().unwrap_or("").to_string();
            if !child_host_key.is_empty() && !child_url.is_empty() {
                break;
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
    assert!(
        !child_host_key.is_empty() && !child_url.is_empty(),
        "child ready file missing host_key/url at {}",
        ready_path.display()
    );

    let (key_a, url_a, stop_a) = spawn_remote_pop(16);
    cdn_pop_configure(&[
        ("pop-us", key_a.as_str(), url_a.as_str()),
        ("pop-eu", child_host_key.as_str(), child_url.as_str()),
    ]);
    cdn_pop_register_serve_pid("pop-us", std::process::id(), 1);
    cdn_pop_register_serve_pid("pop-eu", child.id(), std::process::id());
    assert!(
        !cdn_pop_remote_hostkey_ready(),
        "parent/child spawn must not pass pop-v5 sibling GA"
    );
    assert!(!cdn_pop_dual_process_host_ready());
    let err = cdn_pop_route("pop-us", 1, 2, "p117-parent-child")
        .expect_err("parent/child PoP must not route");
    assert!(
        err.contains("sibling") || err.contains("serve pid"),
        "unexpected err: {err}"
    );

    std::env::set_var("SYX_DIST_POP_CHILD_STOP", "1");
    let _ = child.wait_with_output();
    std::env::remove_var("SYX_DIST_POP_CHILD_STOP");
    stop_a.stop();
    let _ = fs::remove_file(&ready_path);
    cdn_pop_clear();
    cdn_delivery_ack_clear();
}

#[test]
fn dist_bgp_anycast_hostkey_v6_concrete_bind_eq_key_route() {
    let _guard = P117_DIST_LOCK.lock().unwrap();
    dist_region_multihost_clear();
    let (bind_addr, _handle, stop) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    thread::sleep(Duration::from_millis(25));
    let host_key = dist_host_key_concrete_ip_for_bind(&bind_addr);
    let connect_url = dist_tcp_url_for_bind_addr(&bind_addr);
    assert!(!dist_host_key_addr_is_unspecified(&host_key));
    assert!(
        !connect_url.contains("127.0.0.1"),
        "connect URL must not rewrite to 127.0.0.1: {connect_url}"
    );
    assert!(dist_host_key_bind_eq_key(&host_key, &host_key));
    assert!(dist_host_key_has_live_listener(&host_key));
    dist_region_multihost_configure_keys(&[(
        "us-east",
        "edge-rt",
        host_key.as_str(),
        connect_url.as_str(),
    )]);
    assert!(dist_region_multihost_has_runtime_bound_host_key_v5());
    assert!(dist_region_multihost_tcp_health("us-east", "edge-rt"));
    let (region, host, ack) =
        dist_region_multihost_route("us-east", "edge-rt", 1, 2, "p117-concrete-ip")
            .expect("route via concrete-ip live listener");
    assert_eq!(region, "us-east");
    assert_eq!(host, "edge-rt");
    assert_eq!(ack, "p117-concrete-ip");
    stop.stop();
    dist_region_multihost_clear();
}

#[test]
fn dist_bgp_anycast_global_cdn_sla_purged_from_ga() {
    let lock = include_str!("../../../syx-pkg/src/lockfile.rs");
    assert!(!lock.contains("global_cdn_sla_tier"));
    let enforce = include_str!("../../../syx-pkg/src/lock_enforce.rs");
    assert!(!enforce.contains("enforce_global_cdn_sla"));
    let resolver = include_str!("../../../syx-pkg/src/resolver.rs");
    assert!(!resolver.contains("global_cdn_sla"));
    let installer = include_str!("../../../syx-pkg/src/installer.rs");
    assert!(!installer.contains("global_cdn_sla"));
    let billing = include_str!("../../src/farm_billing_report.rs");
    assert!(!billing.contains("global_cdn_sla"));
}
