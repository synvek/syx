//! R-21 diagnostic: compare the two program-execution entry points.
//!
//! `compile_and_run_embedded` (used by 204 test files) and `run_file_printed` (used by 67)
//! compile the same source through different pipelines. This probe runs one source string
//! through both and prints the observable differences, so a divergence can be located from
//! evidence instead of by reading code.
//!
//! Usage:
//!   cargo run -p syx-driver --example path_parity_probe
//!   cargo run -p syx-driver --example path_parity_probe -- <file.sy>

use syx_driver::DriverOptions;

const DEFAULT_SRC: &str = r#"
struct Point {
    var x: Int
    var y: Int
}

func main() {
    let p = Point(x: 3, y: 4)
    let q = Point(x: 3, y: 4)
    let r = Point(x: 9, y: 9)
    print(p == q)
    print(p != q)
    print(p == r)
    print(p != r)
}
"#;

fn main() {
    let src = match std::env::args().nth(1) {
        Some(path) => std::fs::read_to_string(&path).expect("read source file"),
        None => DEFAULT_SRC.to_string(),
    };

    println!("=== source ===");
    println!("{src}");

    // Path A: embedded (FrontendPipeline -> compile_to_program).
    println!("=== A: compile_and_run_embedded ===");
    match syx_driver::compile_and_run_embedded(&src, &DriverOptions::default()) {
        Ok(r) => {
            println!("exit={}", r.exit_code);
            println!("printed ({} lines):", r.printed.len());
            for (i, line) in r.printed.iter().enumerate() {
                println!("  [{i}] {line}");
            }
            println!("A_out={}", r.printed.join("|"));
        }
        Err(e) => println!("ERROR: {e}"),
    }

    // Path B: file path (compile_linked_with_options, includes trim + cache).
    println!("=== B: run_file_printed ===");
    let dir = std::env::temp_dir().join(format!(
        "syx_parity_probe_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("temp dir");
    let file = dir.join("main.sy");
    std::fs::write(&file, &src).expect("write source");
    match syx_driver::run_file_printed(&file, &DriverOptions::default()) {
        Ok((code, printed)) => {
            println!("exit={code}");
            println!("printed ({} lines):", printed.len());
            for (i, line) in printed.iter().enumerate() {
                println!("  [{i}] {line}");
            }
            println!("B_out={}", printed.join("|"));
        }
        Err(e) => println!("ERROR: {e}"),
    }

    // Path C: same as B but without the entry-reachability trim, to isolate its effect.
    println!("=== C: run_file_printed WITHOUT trim (SYX_PARITY_NO_TRIM=1) ===");
    std::env::set_var("SYX_PARITY_NO_TRIM", "1");
    match syx_driver::run_file_printed(&file, &DriverOptions::default()) {
        Ok((code, printed)) => {
            println!("exit={code}");
            println!("C_out={}", printed.join("|"));
        }
        Err(e) => println!("ERROR: {e}"),
    }
    std::env::remove_var("SYX_PARITY_NO_TRIM");

    let _ = std::fs::remove_dir_all(&dir);
}
