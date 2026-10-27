//! Core Math Float64 instance methods smoke.
use std::fs;

use syx_driver::{run_native_mir_file_printed, run_wasm_source_printed, DriverOptions};

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
        "syx_math_f64_{}_{}_{}",
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

fn run_wasm(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let path = write_src(label, src);
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (code, lines) =
        run_wasm_source_printed(&path, "main", &opts).expect("wasm");
    assert_eq!(code, 0, "wasm exit {code}: {lines:?}");
    lines_joined(&lines)
}

// --- from former math_phase11 ---
const MATH_F64_METHODS_SMOKE: &str = r#"
func main() {
    var z: Float64 = 0.0
    var four: Float64 = 4.0
    var e: Float64 = Math.e()
    var neg: Float64 = -1.5
    print(z.sin())
    print(z.cos())
    print(z.tan())
    print(four.sqrt())
    print(z.exp())
    print(e.log())
    print(neg.floor())
    print(neg.ceil())
}
"#;

const MATH_F64_METHODS_SMOKE_EXPECTED: &str = "0\n1\n0\n2\n1\n1\n-2\n-1";

#[test]
fn math_f64_methods_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_F64_METHODS_SMOKE);
    assert_eq!(
        out, MATH_F64_METHODS_SMOKE_EXPECTED,
        "native mir Math F64 methods: {out}"
    );
}

#[test]
fn math_f64_methods_smoke_wasm() {
    let out = run_wasm("smoke", MATH_F64_METHODS_SMOKE);
    assert_eq!(
        out, MATH_F64_METHODS_SMOKE_EXPECTED,
        "wasm Math F64 methods: {out}"
    );
}

#[test]
fn math_f64_methods_parity_with_math_static_native_mir() {
    let src = r#"
func main() {
    var x: Float64 = 0.0
    print(x.sin())
    print(Math.sin(x))
    print(x.cos())
    print(Math.cos(x))
}
"#;
    let out = run_native_mir("parity", src);
    assert_eq!(out, "0\n0\n1\n1", "instance vs Math.* parity: {out}");
}

// --- from former math_phase12 ---
const MATH_F64_METHODS_SLICE2_SMOKE: &str = r#"
func main() {
    var neg: Float64 = -2.5
    var trunc_x: Float64 = -1.9
    var round_x: Float64 = 1.5
    var z: Float64 = 0.0
    var one: Float64 = 1.0
    var three: Float64 = 3.0
    var eight: Float64 = 8.0
    print(neg.abs())
    print(trunc_x.trunc())
    print(round_x.round())
    print(z.asin())
    print(one.acos())
    print(z.atan())
    print(three.exp2())
    print(eight.cbrt())
}
"#;

const MATH_F64_METHODS_SLICE2_SMOKE_EXPECTED: &str = "2.5\n-1\n2\n0\n0\n0\n8\n2";

#[test]
fn math_f64_methods_slice2_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_F64_METHODS_SLICE2_SMOKE);
    assert_eq!(
        out, MATH_F64_METHODS_SLICE2_SMOKE_EXPECTED,
        "native mir Math F64 methods slice2: {out}"
    );
}

#[test]
fn math_f64_methods_slice2_smoke_wasm() {
    let out = run_wasm("smoke", MATH_F64_METHODS_SLICE2_SMOKE);
    assert_eq!(
        out, MATH_F64_METHODS_SLICE2_SMOKE_EXPECTED,
        "wasm Math F64 methods slice2: {out}"
    );
}

#[test]
fn math_f64_methods_slice2_parity_with_math_static_native_mir() {
    let src = r#"
func main() {
    var x: Float64 = -2.5
    print(x.abs())
    print(Math.fabs(x))
    var y: Float64 = 8.0
    print(y.cbrt())
    print(Math.cbrt(y))
}
"#;
    let out = run_native_mir("parity", src);
    assert_eq!(out, "2.5\n2.5\n2\n2", "instance vs Math.* parity: {out}");
}

// --- from former math_phase13 ---
const MATH_F64_METHODS_SLICE3_SMOKE: &str = r#"
func main() {
    var eight: Float64 = 8.0
    var hundred: Float64 = 100.0
    var two: Float64 = 2.0
    var three: Float64 = 3.0
    var four: Float64 = 4.0
    var zero: Float64 = 0.0
    var one: Float64 = 1.0
    print(eight.log2())
    print(hundred.log10())
    print(two.pow(three))
    print(zero.atan2(one))
    print(three.hypot(four))
    print(zero.sinh())
    print(zero.cosh())
    print(zero.tanh())
}
"#;

const MATH_F64_METHODS_SLICE3_SMOKE_EXPECTED: &str = "3\n2\n8\n0\n5\n0\n1\n0";

#[test]
fn math_f64_methods_slice3_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_F64_METHODS_SLICE3_SMOKE);
    assert_eq!(
        out, MATH_F64_METHODS_SLICE3_SMOKE_EXPECTED,
        "native mir Math F64 methods slice3: {out}"
    );
}

#[test]
fn math_f64_methods_slice3_smoke_wasm() {
    let out = run_wasm("smoke", MATH_F64_METHODS_SLICE3_SMOKE);
    assert_eq!(
        out, MATH_F64_METHODS_SLICE3_SMOKE_EXPECTED,
        "wasm Math F64 methods slice3: {out}"
    );
}

#[test]
fn math_f64_methods_slice3_parity_with_math_static_native_mir() {
    let src = r#"
func main() {
    var x: Float64 = 8.0
    print(x.log2())
    print(Math.log2(x))
    var base: Float64 = 2.0
    var exp: Float64 = 3.0
    print(base.pow(exp))
    print(Math.pow(base, exp))
    var a: Float64 = 3.0
    var b: Float64 = 4.0
    print(a.hypot(b))
    print(Math.hypot(a, b))
    var z: Float64 = 0.0
    print(z.sinh())
    print(Math.sinh(z))
}
"#;
    let out = run_native_mir("parity", src);
    assert_eq!(
        out, "3\n3\n8\n8\n5\n5\n0\n0",
        "instance vs Math.* parity: {out}"
    );
}

// --- from former math_phase14 ---
const MATH_F64_METHODS_SLICE4_SMOKE: &str = r#"
func main() {
    var zero: Float64 = 0.0
    var one: Float64 = 1.0
    var five: Float64 = 5.0
    var three: Float64 = 3.0
    print(zero.expm1())
    print(zero.log1p())
    print(zero.asinh())
    print(one.acosh())
    print(zero.atanh())
    print(five.remainder(three))
    print(one.nextUp())
    print(one.nextDown())
}
"#;

// nextUp/Down: pin printed ulp strings (same as math Soft smoke).
const MATH_F64_METHODS_SLICE4_SMOKE_EXPECTED: &str =
    "0\n0\n0\n0\n0\n-1\n1.0000000000000002\n0.9999999999999999";

#[test]
fn math_f64_methods_soft_remainder_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_F64_METHODS_SLICE4_SMOKE);
    assert_eq!(
        out, MATH_F64_METHODS_SLICE4_SMOKE_EXPECTED,
        "native mir Math F64 methods slice4: {out}"
    );
}

#[test]
fn math_f64_methods_soft_remainder_smoke_wasm() {
    let out = run_wasm("smoke", MATH_F64_METHODS_SLICE4_SMOKE);
    assert_eq!(
        out, MATH_F64_METHODS_SLICE4_SMOKE_EXPECTED,
        "wasm Math F64 methods slice4: {out}"
    );
}

#[test]
fn math_f64_methods_soft_remainder_parity_with_math_static_native_mir() {
    let src = r#"
func main() {
    var z: Float64 = 0.0
    print(z.expm1())
    print(Math.expm1(z))
    print(z.asinh())
    print(Math.asinh(z))
    var five: Float64 = 5.0
    var three: Float64 = 3.0
    print(five.remainder(three))
    print(Math.remainder(five, three))
    var one: Float64 = 1.0
    print(one.nextUp())
    print(Math.nextUp(one))
}
"#;
    let out = run_native_mir("parity", src);
    assert_eq!(
        out, "0\n0\n0\n0\n-1\n-1\n1.0000000000000002\n1.0000000000000002",
        "instance vs Math.* parity: {out}"
    );
}

// --- from former math_phase15 ---
const MATH_F64_METHODS_SLICE5_SMOKE: &str = r#"
func main() {
    var five: Float64 = 5.0
    var two: Float64 = 2.0
    var mag: Float64 = 1.5
    var neg: Float64 = -2.0
    var a: Float64 = 2.0
    var b: Float64 = 3.0
    var c: Float64 = 4.0
    var zero: Float64 = 0.0
    var one: Float64 = 1.0
    var nanv: Float64 = Math.sqrt(-1.0)
    print(five.fmod(two))
    print(mag.copysign(neg))
    print(a.mulAdd(b, c))
    print(zero.erf())
    print(zero.erfc())
    print(one.tgamma())
    print(one.lgamma())
    print(nanv.isNaN())
}
"#;

const MATH_F64_METHODS_SLICE5_SMOKE_EXPECTED: &str = "1\n-1.5\n10\n0\n1\n1\n0\ntrue";

#[test]
fn math_f64_methods_soft_close_special_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_F64_METHODS_SLICE5_SMOKE);
    assert_eq!(
        out, MATH_F64_METHODS_SLICE5_SMOKE_EXPECTED,
        "native mir Math F64 methods slice5: {out}"
    );
}

#[test]
fn math_f64_methods_soft_close_special_smoke_wasm() {
    let out = run_wasm("smoke", MATH_F64_METHODS_SLICE5_SMOKE);
    assert_eq!(
        out, MATH_F64_METHODS_SLICE5_SMOKE_EXPECTED,
        "wasm Math F64 methods slice5: {out}"
    );
}

#[test]
fn math_f64_methods_soft_close_special_parity_with_math_static_native_mir() {
    let src = r#"
func main() {
    var five: Float64 = 5.0
    var two: Float64 = 2.0
    print(five.fmod(two))
    print(Math.fmod(five, two))
    var a: Float64 = 2.0
    print(a.mulAdd(3.0, 4.0))
    print(Math.mulAdd(2.0, 3.0, 4.0))
    var zero: Float64 = 0.0
    print(zero.erf())
    print(Math.erf(zero))
    var nanv: Float64 = Math.sqrt(-1.0)
    print(nanv.isNaN())
    print(Math.isNaN(nanv))
}
"#;
    let out = run_native_mir("parity", src);
    assert_eq!(
        out, "1\n1\n10\n10\n0\n0\ntrue\ntrue",
        "instance vs Math.* parity: {out}"
    );
}

