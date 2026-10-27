//! Dist-actor multi-process TCP mailbox (cross-process region_route).

use std::process::Command;
use std::thread;
use std::time::Duration;

use syx_async::{
    dist_actor_mp_clear, dist_actor_mp_fingerprint, dist_actor_mp_peer_send_once, region_route_tcp,
    spawn_dist_tcp_mailbox,
};

#[test]
fn dist_mailbox_mp_fingerprint() {
    // P108 supersedes MP GA fingerprint (no inproc fallback as GA).
    assert_eq!(
        dist_actor_mp_fingerprint(),
        "dist-actor-mp-v2:no-inproc-fallback-ga"
    );
    assert!(include_str!("../../../syx-async/src/dist_transport.rs")
        .contains("dist-actor-mp-v2:no-inproc-fallback-ga"));
}

#[test]
fn dist_mailbox_inprocess_ack() {
    let (addr, handle, stop) = spawn_dist_tcp_mailbox(2);
    thread::sleep(Duration::from_millis(15));
    let ack = region_route_tcp(
        &format!("tcp://{addr}"),
        "us-east",
        1,
        2,
        "p107-mp-ack",
    )
    .expect("tcp mailbox ack");
    assert_eq!(ack, "p107-mp-ack");
    stop.stop();
    let _ = handle.join();
    dist_actor_mp_clear();
}

/// Two OS processes: parent hosts mailbox; child sends ROUTE and receives ACK.
#[test]
fn dist_mailbox_two_process_ack() {
    // Child role (re-entered via current_exe).
    if let Ok(url) = std::env::var("SYX_DIST_MP_CHILD_URL") {
        let ack = dist_actor_mp_peer_send_once(&url, "eu-west", 7, 8, "p107-child-ack")
            .expect("child peer send");
        assert_eq!(ack, "p107-child-ack");
        println!("SYX_DIST_MP_ACK={ack}");
        return;
    }

    let (addr, handle, stop) = spawn_dist_tcp_mailbox(4);
    thread::sleep(Duration::from_millis(20));
    let url = format!("tcp://{addr}");
    let exe = std::env::current_exe().expect("current_exe");
    let output = Command::new(&exe)
        .env("SYX_DIST_MP_CHILD_URL", &url)
        .args([
            "dist_mailbox_multiprocess::dist_mailbox_two_process_ack",
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
        stdout.contains("SYX_DIST_MP_ACK=p107-child-ack")
            || stderr.contains("SYX_DIST_MP_ACK=p107-child-ack"),
        "expected child ACK in output\nstdout={stdout}\nstderr={stderr}"
    );
    stop.stop();
    let _ = handle.join();
    dist_actor_mp_clear();
}

#[test]
fn dist_mailbox_zero_conns_joins_fast() {
    let (_addr, handle, stop) = spawn_dist_tcp_mailbox(0);
    stop.stop();
    let started = std::time::Instant::now();
    handle.join().expect("max_conns=0 must exit");
    assert!(
        started.elapsed() < Duration::from_secs(2),
        "mailbox max_conns=0 must not hang"
    );
}
