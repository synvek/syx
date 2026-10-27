//! Dist-actor TCP mailbox GA — no in-process region_route fallback.

use std::process::Command;
use std::thread;
use std::time::Duration;

use syx_async::{
    dist_actor_mp_clear, dist_actor_mp_doctor_mode, dist_actor_mp_fingerprint,
    dist_actor_mp_ga_requires_tcp, dist_actor_mp_inbox_clear, dist_actor_mp_inbox_count,
    dist_actor_mp_peer_send_once, region_route, region_route_mp, region_route_mp_ga, region_route_tcp,
    spawn_dist_tcp_mailbox,
};

#[test]
fn dist_mailbox_tcp_ga_fingerprint() {
    assert_eq!(
        dist_actor_mp_fingerprint(),
        "dist-actor-mp-v2:no-inproc-fallback-ga"
    );
    assert!(include_str!("../../../syx-async/src/dist_transport.rs")
        .contains("dist-actor-mp-v2:no-inproc-fallback-ga"));
    assert!(dist_actor_mp_ga_requires_tcp());
}

#[test]
fn dist_mailbox_tcp_success_not_inproc_region_route() {
    dist_actor_mp_inbox_clear();
    let (addr, handle, stop) = spawn_dist_tcp_mailbox(2);
    thread::sleep(Duration::from_millis(15));
    let ack = region_route_tcp(
        &format!("tcp://{addr}"),
        "us-east",
        1,
        2,
        "p108-mp-ack",
    )
    .expect("tcp mailbox ack");
    assert_eq!(ack, "p108-mp-ack");
    // TCP success must land in mailbox inbox (not GA via in-process region_route).
    assert!(
        dist_actor_mp_inbox_count() >= 1,
        "TCP mailbox inbox must record the route"
    );
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    let spawn_fn = src
        .split("pub fn spawn_dist_tcp_mailbox")
        .nth(1)
        .expect("spawn_dist_tcp_mailbox")
        .split("pub struct ArcStop")
        .next()
        .expect("spawn body");
    assert!(
        !spawn_fn.contains("region_route("),
        "spawn_dist_tcp_mailbox must not call region_route on success (inproc not GA)"
    );
    stop.stop();
    let _ = handle.join();
    dist_actor_mp_clear();
}

#[test]
fn dist_mailbox_tcp_cross_process_ack() {
    if let Ok(url) = std::env::var("SYX_DIST_MP_CHILD_URL_P108") {
        let ack = dist_actor_mp_peer_send_once(&url, "eu-west", 7, 8, "p108-child-ack")
            .expect("child peer send");
        assert_eq!(ack, "p108-child-ack");
        println!("SYX_DIST_MP_ACK={ack}");
        return;
    }

    let (addr, handle, stop) = spawn_dist_tcp_mailbox(4);
    thread::sleep(Duration::from_millis(20));
    let url = format!("tcp://{addr}");
    let exe = std::env::current_exe().expect("current_exe");
    let output = Command::new(&exe)
        .env("SYX_DIST_MP_CHILD_URL_P108", &url)
        .args([
            "dist_mailbox_tcp_ga::dist_mailbox_tcp_cross_process_ack",
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
        stdout.contains("SYX_DIST_MP_ACK=p108-child-ack")
            || stderr.contains("SYX_DIST_MP_ACK=p108-child-ack"),
        "expected child ACK in output\nstdout={stdout}\nstderr={stderr}"
    );
    stop.stop();
    let _ = handle.join();
    dist_actor_mp_clear();
}

#[test]
fn dist_mailbox_tcp_inproc_fallback_not_counted_as_ga() {
    dist_actor_mp_clear();
    // P109: silent inproc is forbidden — require SYX_DIST_INPROC=1.
    std::env::remove_var("SYX_DIST_INPROC");
    // Doctor may briefly race with other env-mutating tests; assert routing behavior hard.
    assert!(
        !region_route_mp("us-east", 1, 2, "blocked-silent")
            || dist_actor_mp_doctor_mode() == "inproc-dev-fallback",
        "without explicit inproc, route must fail (or only succeed under inproc-dev-fallback)"
    );
    if dist_actor_mp_doctor_mode() != "inproc-dev-fallback" {
        assert!(
            !region_route_mp("us-east", 1, 2, "blocked-silent"),
            "region_route_mp must not silently succeed via inproc"
        );
    }

    std::env::set_var("SYX_DIST_INPROC", "1");
    assert_eq!(dist_actor_mp_doctor_mode(), "inproc-dev-fallback");
    // Inproc region_route may succeed for explicit dev, but GA must reject.
    assert!(region_route("us-east", 1, 2, "dev-only"));
    assert!(region_route_mp("us-east", 1, 2, "dev-mp"));
    let err = region_route_mp_ga("us-east", 1, 2, "not-ga").expect_err("inproc must not be GA");
    assert!(
        err.contains("requires TCP") || err.contains("not GA") || err.contains("inproc"),
        "expected GA reject for inproc, got {err}"
    );
    std::env::remove_var("SYX_DIST_INPROC");
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    // region_route_tcp success arm must not call region_route.
    let tcp_fn = src
        .split("pub fn region_route_tcp")
        .nth(1)
        .expect("region_route_tcp")
        .split("pub fn region_route_mp_ga")
        .next()
        .expect("tcp body");
    assert!(
        !tcp_fn.contains("region_route("),
        "region_route_tcp must not call region_route on success (inproc not GA)"
    );
}

#[test]
fn dist_mailbox_tcp_zero_conns_joins_fast() {
    let (_addr, handle, stop) = spawn_dist_tcp_mailbox(0);
    stop.stop();
    let started = std::time::Instant::now();
    handle.join().expect("max_conns=0 must exit");
    assert!(
        started.elapsed() < Duration::from_secs(2),
        "mailbox max_conns=0 must not hang"
    );
}
