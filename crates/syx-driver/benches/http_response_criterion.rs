//! HTTP Criterion formal-benchmark first tier.
//! Times `examples/http_microbench` (Http.response×10000) via `run_file_printed`.

use std::hint::black_box;
use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, Criterion};
use syx_driver::{run_file_printed, DriverOptions};

fn example_main() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/http_microbench/main.sy")
}

fn http_response_x10000(c: &mut Criterion) {
    let path = example_main();
    c.bench_function("http_response_x10000", |b| {
        b.iter(|| {
            let (code, printed) =
                run_file_printed(&path, &DriverOptions::default()).expect("run");
            assert_eq!(code, 0, "exit code must be 0");
            let out = printed.join("\n");
            assert!(
                out.lines().any(|l| l.trim() == "ok"),
                "must print ok: {out}"
            );
            black_box((code, printed));
        });
    });
}

criterion_group!(benches, http_response_x10000);
criterion_main!(benches);
