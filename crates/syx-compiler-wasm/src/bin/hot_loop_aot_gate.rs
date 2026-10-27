//! Nightly gate: compile fixture(s) and print AOT direct ratio.

use std::env;
use std::fs;
use std::path::PathBuf;

fn fixture_sources() -> Vec<(&'static str, String)> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    vec![
        (
            "hot_loop.sy",
            fs::read_to_string(root.join("examples/hot_loop.sy")).expect("hot_loop"),
        ),
        (
            "hello.sy",
            fs::read_to_string(root.join("examples/hello.sy")).expect("hello"),
        ),
        (
            "comparable_sort.sy",
            include_str!("../../../syx-driver/tests/fixtures/parity/comparable_sort.sy").into(),
        ),
        (
            "inheritance_dispatch.sy",
            include_str!("../../../syx-driver/tests/fixtures/parity/inheritance_aot.sy").into(),
        ),
        (
            "protocol_iterator.sy",
            include_str!("../../../syx-driver/tests/fixtures/parity/protocol_iterator.sy").into(),
        ),
    ]
}

fn ratio_for(name: &str, src: &str) -> f64 {
    let bytes =
        syx_compiler_wasm::compile_source_to_wasm_bytes(name, src).expect("compile");
    let (_, _, ratio) = syx_wasm_emit::measure_aot_ratio(&bytes);
    ratio
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let matrix = args.iter().any(|a| a == "--matrix");
    if matrix {
        let mut min = 1.0_f64;
        for (name, src) in fixture_sources() {
            let ratio = ratio_for(name, &src);
            eprintln!("{name}: {ratio:.6}");
            min = min.min(ratio);
        }
        println!("{min:.6}");
        return;
    }
    let src = include_str!("../../../../examples/hot_loop.sy");
    let ratio = ratio_for("hot_loop.sy", src);
    println!("{ratio:.6}");
}
