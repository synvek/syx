//! S3-I68 claim-prep: fib criterion scaffolding.
//! Times `examples/fib20.sy` (fib(20)) via `run_file_printed`.
//! ≠ fib(30) in criterion (too slow for CI); wall target stays examples/fib.sy.

use std::hint::black_box;
use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, Criterion};
use syx_driver::{run_file_printed, DriverOptions};

fn example_fib20() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/fib20.sy")
}

fn fib20_vm(c: &mut Criterion) {
    let path = example_fib20();
    c.bench_function("fib20_vm", |b| {
        b.iter(|| {
            let (code, printed) = run_file_printed(&path, &DriverOptions::default()).expect("run");
            assert_eq!(code, 0, "exit code must be 0");
            let out = printed.join("\n");
            assert!(
                out.lines().any(|l| l.trim() == "6765"),
                "fib(20) must print 6765: {out}"
            );
            black_box((code, printed));
        });
    });
}

criterion_group!(benches, fib20_vm);
criterion_main!(benches);
