//! Dist-actor TCP mailbox GA — no in-process region_route fallback.

use std::process::Command;
use std::thread;
use std::time::Duration;

use syx_async::{
    dist_actor_mp_clear, dist_actor_mp_peer_send_once, spawn_dist_tcp_mailbox,
};

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
