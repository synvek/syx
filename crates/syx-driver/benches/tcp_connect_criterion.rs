//! Tcp Criterion formal-benchmark first tier.
//! Times `examples/tcp_microbench` (TcpStream.connect refuse×10000) via `run_file_printed`
//! on a 16MB stack thread (depth-bounded recursion; no listener).

use std::hint::black_box;
use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, Criterion};
use syx_driver::{run_file_printed, DriverOptions};

const RUN_STACK: usize = 16 * 1024 * 1024;

fn example_main() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/tcp_microbench/main.sy")
}

fn run_on_large_stack(path: &PathBuf) -> (i32, Vec<String>) {
    let path = path.clone();
    std::thread::Builder::new()
        .name("tcp_connect_criterion".into())
        .stack_size(RUN_STACK)
        .spawn(move || run_file_printed(&path, &DriverOptions::default()).expect("run"))
        .expect("spawn criterion thread")
        .join()
        .expect("join criterion thread")
}

fn tcp_connect_refuse_x10000(c: &mut Criterion) {
    let path = example_main();
    c.bench_function("tcp_connect_refuse_x10000", |b| {
        b.iter(|| {
            let (code, printed) = run_on_large_stack(&path);
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

criterion_group!(benches, tcp_connect_refuse_x10000);
criterion_main!(benches);
