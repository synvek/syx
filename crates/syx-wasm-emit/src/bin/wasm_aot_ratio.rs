//! Print WASM AOT direct-emit ratio for nightly gate scripts.

use std::env;
use std::fs;
use std::process;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: wasm_aot_ratio <file.wasm>");
        process::exit(2);
    });
    let wasm = fs::read(&path).unwrap_or_else(|e| {
        eprintln!("read {path}: {e}");
        process::exit(1);
    });
    let (direct, mir_exec, ratio) = syx_wasm_emit::measure_aot_ratio(&wasm);
    println!("{ratio:.6}");
    eprintln!("direct={direct} mir_exec={mir_exec}");
}
