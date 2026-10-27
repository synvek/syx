//! Driver-level WASM AOT matrix gate (Phase 69).

use std::env;

use syx_driver::aot_matrix;

fn main() {
    syx_driver::ensure_host_rt_link_stubs();
    let args: Vec<String> = env::args().collect();
    let matrix = args.iter().any(|a| a == "--matrix");
    let full = args.iter().any(|a| a == "--full");
    let breakdown = args.iter().any(|a| a == "--breakdown");

    if matrix {
        let report = aot_matrix::run_aot_matrix(breakdown);
        if full {
            println!("{:.6}", report.full_main_ratio().unwrap_or(0.0));
        } else {
            println!("{:.6}", report.strict_min_ratio());
        }
        return;
    }

    let path = aot_matrix::repo_root().join("examples/hot_loop.sy");
    let (_, ratio) = aot_matrix::ratio_for_path(&path);
    println!("{ratio:.6}");
}
