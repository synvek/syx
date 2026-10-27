//! Numeric Type.min/max bounds + byteWidth/clamp/sign/epsilon smoke.
use std::fs;

use syx_driver::{run_native_mir_file_printed, DriverOptions};

fn lines_joined(lines: &[String]) -> String {
    lines
        .iter()
        .map(|l| l.trim_end_matches('\r').trim_end_matches('\n'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn write_src(label: &str, src: &str) -> std::path::PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let dir = std::env::temp_dir().join(format!(
        "syx_numeric_bounds_{}_{}_{}",
        label,
        std::process::id(),
        SEQ.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let path = dir.join("main.sy");
    fs::write(&path, src).expect("write main");
    path
}

fn run_native_mir(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src(label, src);
    let (code, lines) =
        run_native_mir_file_printed(&path, &DriverOptions::default()).expect("native mir");
    assert_eq!(code, 0, "native mir exit {code}: {lines:?}");
    lines_joined(&lines)
}

const NUMERIC_BOUNDS_SMOKE: &str = r#"
func main() {
    print(Int8.min)
    print(Int8.max)
    print(Int8.byteWidth())
    print(Int8.clamp(x: 100i8, lo: 0i8, hi: 50i8))
    print(Int8.sign(x: -5i8))
    print(Int.min)
    print(Int.max)
    print(UInt8.max)
    print(UInt.max)
    print(Int128.min)
    print(Int128.max)
    print(UInt128.max)
    print(Float64.epsilon())
    print(Float64.abs(x: -3.5))
    print(Float32.byteWidth())
}
"#;

const NUMERIC_BOUNDS_EXPECTED: &str = "\
-128
127
1
50
-1
-9223372036854775808
9223372036854775807
255
18446744073709551615
-170141183460469231731687303715884105728
170141183460469231731687303715884105727
340282366920938463463374607431768211455
0.0000000000000002220446049250313
3.5
4";

#[test]
fn numeric_bounds_smoke_native_mir() {
    let out = run_native_mir("smoke", NUMERIC_BOUNDS_SMOKE);
    assert_eq!(
        out, NUMERIC_BOUNDS_EXPECTED,
        "native mir numeric bounds smoke: {out}"
    );
}
