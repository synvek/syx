//! ADR-191 / M2 W47: EC-M2-4 Tcp Instant micro-benchmark first tier (TcpStream.connect refuse×N Instant).
// was: ec_m2_4_tcp_microbench

use std::path::PathBuf;
use std::time::Instant;

use syx_driver::{run_file_printed, DriverOptions};

const N: u32 = 10_000;
/// VM recursion for refuse×N needs more than the default test-thread stack.
const RUN_STACK: usize = 16 * 1024 * 1024;

fn example_main() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/tcp_microbench/main.sy")
}

#[test]
fn ec_m2_4_tcp_microbench_refuse_instant() {
    let handle = std::thread::Builder::new()
        .name("ec_m2_4_tcp_microbench".into())
        .stack_size(RUN_STACK)
        .spawn(|| {
            let t0 = Instant::now();
            let result = run_file_printed(&example_main(), &DriverOptions::default());
            let elapsed = t0.elapsed();
            (result, elapsed)
        })
        .expect("spawn microbench thread");
    let (result, elapsed) = handle.join().expect("join microbench thread");
    let (code, printed) = result.expect("run");

    assert_eq!(code, 0, "exit code must be 0, printed={printed:?}");
    let out = printed.join("\n");
    assert!(
        out.lines().any(|l| l.trim() == "ok"),
        "must print ok: {out}"
    );
    assert!(
        !out.contains("UNEXPECTED"),
        "must not print UNEXPECTED: {out}"
    );
    assert!(
        elapsed.as_nanos() > 0,
        "elapsed must be > 0 (got {elapsed:?})"
    );

    let per_op = elapsed / N;
    eprintln!(
        "ec_m2_4_tcp_microbench: TcpStream.connect refuse×{N} compile+run in {elapsed:?} (~{per_op:?}/op)"
    );
}

