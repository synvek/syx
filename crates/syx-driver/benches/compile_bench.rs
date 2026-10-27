use std::hint::black_box;
use std::path::Path;
use std::time::Instant;

use syx_driver::compile_file;
use syx_diagnostics::SourceFile;
use syx_parser::Parser;

fn main() {
    let dir = std::env::temp_dir().join("syx_compile_bench");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("bench.sy");
    let source = r#"func main() {
    let x = 1 + 2
    if x > 0 {
        print("bench")
    }
    return x
}"#;
    std::fs::write(&path, source).expect("write bench source");

    let iterations = 50usize;
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(compile_file(Path::new(&path)).expect("compile"));
    }
    let elapsed = start.elapsed();
    println!(
        "compiled {iterations} times in {:?} ({:?} per compile)",
        elapsed,
        elapsed / iterations as u32
    );

    let file = SourceFile::new("inline.sy", source);
    let parse_start = Instant::now();
    for _ in 0..iterations {
        black_box(Parser::parse(file.clone()).expect("parse"));
    }
    let parse_elapsed = parse_start.elapsed();
    println!(
        "parsed {iterations} times in {:?} ({:?} per parse)",
        parse_elapsed,
        parse_elapsed / iterations as u32
    );

    // Phase 11 AW1: MIR is default; SYX_USE_MIR=0 opts out to AST codegen.
    std::env::remove_var("SYX_USE_MIR");
    let default_start = Instant::now();
    for _ in 0..iterations {
        black_box(compile_file(Path::new(&path)).expect("default MIR compile"));
    }
    let default_elapsed = default_start.elapsed();
    println!(
        "default MIR compile {iterations} times in {:?} ({:?} per compile)",
        default_elapsed,
        default_elapsed / iterations as u32
    );

    std::env::set_var("SYX_USE_MIR", "1");
    let mir_start = Instant::now();
    for _ in 0..iterations {
        black_box(compile_file(Path::new(&path)).expect("mir compile"));
    }
    let mir_elapsed = mir_start.elapsed();
    println!(
        "MIR compile {iterations} times in {:?} ({:?} per compile)",
        mir_elapsed,
        mir_elapsed / iterations as u32
    );

    let native_source = dir.join("native_loop.sy");
    std::fs::write(
        &native_source,
        r#"func main() {
    var sum = 0
    for i in 0..10000 {
        sum = sum + i
    }
    return sum
}"#,
    )
    .expect("write native bench source");
    std::fs::write(
        dir.join("syx.toml"),
        "name = \"bench\"\nmain = \"native_loop.sy\"\n",
    )
    .expect("write bench manifest");
    let native_start = Instant::now();
    let exe = dir.join("bench_app");
    syx_driver::build_native_executable(&dir, &syx_driver::DriverOptions::default(), &exe)
        .expect("native build");
    let build_elapsed = native_start.elapsed();
    let run_start = Instant::now();
    for _ in 0..10 {
        let status = std::process::Command::new(&exe)
            .status()
            .expect("native run");
        assert!(status.success());
    }
    let run_elapsed = run_start.elapsed();
    println!(
        "native loop build {:?}, 10 runs {:?} ({:?} per run)",
        build_elapsed,
        run_elapsed,
        run_elapsed / 10
    );

    // Phase 11 AW1: ±20% warn / ±30% fail vs baseline (MIR-default path).
    // Set SYX_COMPILE_BENCH_BASELINE_MS to the median compile ms from a green nightly run.
    if let Ok(base) = std::env::var("SYX_COMPILE_BENCH_BASELINE_MS") {
        if let Ok(base_ms) = base.parse::<u128>() {
            let per_compile_ms = default_elapsed.as_millis() / iterations as u128;
            let warn_threshold = base_ms * 120 / 100;
            let fail_threshold = base_ms * 130 / 100;
            if per_compile_ms > warn_threshold {
                eprintln!(
                    "compile_bench WARN: {per_compile_ms}ms per compile (>120% of baseline {base_ms}ms)"
                );
            }
            if per_compile_ms > fail_threshold {
                panic!(
                    "compile_bench regression: {per_compile_ms}ms per compile exceeds 130% of baseline {base_ms}ms"
                );
            }
        }
    }

    // Phase 18 CD3: ADR-007 LICM v7 fixture compile timing baseline.
    let licm = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/p17_mir_licm_print_order.sy");
    if licm.exists() {
        let licm_start = Instant::now();
        for _ in 0..iterations {
            black_box(compile_file(&licm).expect("licm fixture compile"));
        }
        let licm_elapsed = licm_start.elapsed();
        println!(
            "LICM v7 fixture compile {iterations} times in {:?} ({:?} per compile)",
            licm_elapsed,
            licm_elapsed / iterations as u32
        );
    }
}
