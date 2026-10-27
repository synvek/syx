//! Phase 134 Track E3: bounded Task queue Abort MVP.
//! Fingerprint: `runtime-concurrency-v2:bounded-queue-abort-mvp`

#[test]
fn bounded_queue_abort_emits_c008() {
    let err = syx_async::probe_bounded_queue_abort_mvp().expect_err("third spawn should abort");
    let msg = err.to_string();
    assert!(
        msg.contains("C008") || msg.contains("Task queue full") || msg.contains("Abort"),
        "expected C008 / Abort, got: {msg}"
    );
}
