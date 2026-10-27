//! Dist multi-region TCP anycast health + failover (inproc not GA).

use std::process::Command;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    dist_actor_mp_clear, dist_actor_mp_doctor_mode, dist_actor_mp_peer_send_once,
    dist_inproc_enabled, dist_region_anycast_clear, dist_region_anycast_configure,
    dist_region_anycast_fingerprint, dist_region_anycast_route, dist_region_tcp_health,
    region_route_mp, region_route_mp_ga, spawn_dist_tcp_mailbox,
};

/// Serialize dist anycast / inproc env tests — shared process globals + env.
static P109_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_anycast_fingerprint() {
    assert_eq!(
        dist_region_anycast_fingerprint(),
        "dist-region-anycast-v1:tcp-health"
    );
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-region-anycast-v1:tcp-health"));
    assert!(src.contains("SYX_DIST_INPROC"));
    assert!(src.contains("dist_region_tcp_health"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-region-anycast-v1:tcp-health"));
}

#[test]
fn dist_anycast_inproc_gate_and_doctor() {
    let _guard = P109_DIST_LOCK.lock().unwrap();
    dist_actor_mp_clear();
    dist_region_anycast_clear();
    std::env::remove_var("SYX_DIST_INPROC");

    assert!(!dist_inproc_enabled());
    assert_eq!(dist_actor_mp_doctor_mode(), "no-transport");
    assert!(
        !region_route_mp("us-east", 1, 2, "silent-block"),
        "production region_route_mp must not silently succeed via inproc"
    );
    let err = region_route_mp_ga("us-east", 1, 2, "not-ga").expect_err("inproc not GA");
    assert!(
        err.contains("requires TCP") || err.contains("not GA") || err.contains("inproc"),
        "got {err}"
    );

    std::env::set_var("SYX_DIST_INPROC", "1");
    assert!(dist_inproc_enabled());
    assert_eq!(dist_actor_mp_doctor_mode(), "inproc-dev-fallback");
    assert!(region_route_mp("us-east", 3, 4, "dev-inproc"));
    let err2 = region_route_mp_ga("us-east", 3, 4, "still-not-ga").expect_err("inproc not GA");
    assert!(
        err2.contains("requires TCP") || err2.contains("not GA") || err2.contains("inproc"),
        "got {err2}"
    );
    std::env::remove_var("SYX_DIST_INPROC");
}

#[test]
fn dist_anycast_cross_process_ack() {
    if let Ok(url) = std::env::var("SYX_DIST_ANYCAST_CHILD_URL_P109") {
        let ack = dist_actor_mp_peer_send_once(&url, "us-east", 9, 10, "p109-child-ack")
            .expect("child peer send");
        assert_eq!(ack, "p109-child-ack");
        println!("SYX_DIST_ANYCAST_ACK={ack}");
        return;
    }

    let _guard = P109_DIST_LOCK.lock().unwrap();
    dist_region_anycast_clear();
    dist_actor_mp_clear();
    let (addr, handle, stop) = spawn_dist_tcp_mailbox(8);
    thread::sleep(Duration::from_millis(20));
    let url = format!("tcp://{addr}");
    dist_region_anycast_configure(&[("us-east", url.as_str()), ("eu-west", url.as_str())]);
    let exe = std::env::current_exe().expect("current_exe");
    let output = Command::new(&exe)
        .env("SYX_DIST_ANYCAST_CHILD_URL_P109", &url)
        .args([
            "dist_anycast::dist_anycast_cross_process_ack",
            "--exact",
            "--nocapture",
        ])
        .output()
        .expect("spawn child process");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "child failed status={:?}\nstdout={stdout}\nstderr={stderr}",
        output.status
    );
    assert!(
        stdout.contains("SYX_DIST_ANYCAST_ACK=p109-child-ack")
            || stderr.contains("SYX_DIST_ANYCAST_ACK=p109-child-ack"),
        "expected child ACK\nstdout={stdout}\nstderr={stderr}"
    );
    stop.stop();
    let _ = handle.join();
    dist_region_anycast_clear();
    dist_actor_mp_clear();
}

#[test]
fn dist_anycast_kill_region_failover() {
    let _guard = P109_DIST_LOCK.lock().unwrap();
    dist_region_anycast_clear();
    dist_actor_mp_clear();
    let (addr_a, handle_a, stop_a) = spawn_dist_tcp_mailbox(16);
    let (addr_b, handle_b, stop_b) = spawn_dist_tcp_mailbox(16);
    thread::sleep(Duration::from_millis(25));
    let url_a = format!("tcp://{addr_a}");
    let url_b = format!("tcp://{addr_b}");
    dist_region_anycast_configure(&[("us-east", url_a.as_str()), ("eu-west", url_b.as_str())]);

    assert!(dist_region_tcp_health("us-east"));
    assert!(dist_region_tcp_health("eu-west"));

    let (active, ack) = dist_region_anycast_route("us-east", 1, 2, "p109-primary")
        .expect("primary region route");
    assert_eq!(active, "us-east");
    assert_eq!(ack, "p109-primary");

    // Kill preferred region mailbox — failover must land on eu-west.
    stop_a.stop();
    let _ = handle_a.join();
    thread::sleep(Duration::from_millis(40));
    assert!(
        !dist_region_tcp_health("us-east"),
        "killed region must fail health probe"
    );
    assert!(
        dist_region_tcp_health("eu-west"),
        "secondary region must remain healthy"
    );

    let (active2, ack2) = dist_region_anycast_route("us-east", 1, 2, "p109-failover")
        .expect("failover after kill");
    assert_eq!(
        active2, "eu-west",
        "must failover to healthy secondary region"
    );
    assert_eq!(ack2, "p109-failover");

    stop_b.stop();
    let _ = handle_b.join();
    dist_region_anycast_clear();
    dist_actor_mp_clear();
}

#[test]
fn dist_anycast_source_gate() {
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist_region_anycast_route"));
    assert!(src.contains("dist_tcp_endpoint_alive"));
    assert!(src.contains("SYX_DIST_INPROC"));
    let mp = src
        .split("pub fn region_route_mp(")
        .nth(1)
        .expect("region_route_mp(")
        .split("pub fn dist_actor_mp_peer_send_once")
        .next()
        .expect("mp body");
    assert!(
        mp.contains("dist_inproc_enabled"),
        "region_route_mp must gate inproc on SYX_DIST_INPROC"
    );
}
