//! Core Math Float32 instance methods smoke (includes F64 classify close parity slice).
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
        "syx_math_f32_{}_{}_{}",
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

// --- from former math_phase16 ---
const MATH_F64_CLASSIFY_F32_SLICE1_SMOKE: &str = r#"
func main() {
    var inf: Float64 = Math.log(0.0)
    var one64: Float64 = 1.0
    var four: Float32 = 4.0f32
    var zero: Float32 = 0.0f32
    var one32: Float32 = 1.0f32
    print(inf.isInfinite())
    print(one64.isFinite())
    print(four.sqrt())
    print(zero.sin())
    print(zero.cos())
    print(zero.tan())
    print(zero.exp())
    print(one32.log())
}
"#;

const MATH_F64_CLASSIFY_F32_SLICE1_SMOKE_EXPECTED: &str = "true\ntrue\n2\n0\n1\n0\n1\n0";

#[test]
fn math_f64_classify_f32_methods_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_F64_CLASSIFY_F32_SLICE1_SMOKE);
    assert_eq!(
        out, MATH_F64_CLASSIFY_F32_SLICE1_SMOKE_EXPECTED,
        "native mir Math F64 classify + F32 slice1: {out}"
    );
}

#[test]
fn math_f64_classify_f32_methods_smoke_wasm() {
    let out = run_wasm("smoke", MATH_F64_CLASSIFY_F32_SLICE1_SMOKE);
    assert_eq!(
        out, MATH_F64_CLASSIFY_F32_SLICE1_SMOKE_EXPECTED,
        "wasm Math F64 classify + F32 slice1: {out}"
    );
}

#[test]
fn math_f64_classify_f32_methods_parity_with_math_static_native_mir() {
    let src = r#"
func main() {
    var inf: Float64 = Math.log(0.0)
    print(inf.isInfinite())
    print(Math.isInfinite(inf))
    var one: Float64 = 1.0
    print(one.isFinite())
    print(Math.isFinite(one))
    var four: Float32 = 4.0f32
    print(four.sqrt())
    print(Math.sqrtf(four))
    var zero: Float32 = 0.0f32
    print(zero.sin())
    print(Math.sinf(zero))
}
"#;
    let out = run_native_mir("parity", src);
    assert_eq!(
        out, "true\ntrue\ntrue\ntrue\n2\n2\n0\n0",
        "instance vs Math.* parity: {out}"
    );
}

// --- from former math_phase17 ---
const MATH_F32_METHODS_SLICE2_SMOKE: &str = r#"
func main() {
    var x: Float32 = 1.9f32
    var pos: Float32 = 2.5f32
    var trunc_x: Float32 = 1.9f32
    var round_x: Float32 = 1.5f32
    var z: Float32 = 0.0f32
    var one: Float32 = 1.0f32
    print(x.floor())
    print(x.ceil())
    print(pos.abs())
    print(trunc_x.trunc())
    print(round_x.round())
    print(z.asin())
    print(one.acos())
    print(z.atan())
}
"#;

const MATH_F32_METHODS_SLICE2_SMOKE_EXPECTED: &str = "1\n2\n2.5\n1\n2\n0\n0\n0";

#[test]
fn math_f32_methods_slice2_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_F32_METHODS_SLICE2_SMOKE);
    assert_eq!(
        out, MATH_F32_METHODS_SLICE2_SMOKE_EXPECTED,
        "native mir Math F32 methods slice2: {out}"
    );
}

#[test]
fn math_f32_methods_slice2_smoke_wasm() {
    let out = run_wasm("smoke", MATH_F32_METHODS_SLICE2_SMOKE);
    assert_eq!(
        out, MATH_F32_METHODS_SLICE2_SMOKE_EXPECTED,
        "wasm Math F32 methods slice2: {out}"
    );
}

#[test]
fn math_f32_methods_slice2_parity_with_math_static_native_mir() {
    let src = r#"
func main() {
    var pos: Float32 = 2.5f32
    print(pos.abs())
    print(Math.fabsf(pos))
    var x: Float32 = 1.9f32
    print(x.floor())
    print(Math.floorf(x))
    var z: Float32 = 0.0f32
    print(z.asin())
    print(Math.asinf(z))
}
"#;
    let out = run_native_mir("parity", src);
    assert_eq!(
        out, "2.5\n2.5\n1\n1\n0\n0",
        "instance vs Math.*f parity: {out}"
    );
}

// --- from former math_phase18 ---
const MATH_F32_METHODS_SLICE3_SMOKE: &str = r#"
func main() {
    var eight: Float32 = 8.0f32
    var hundred: Float32 = 100.0f32
    var two: Float32 = 2.0f32
    var three: Float32 = 3.0f32
    var four: Float32 = 4.0f32
    var zero: Float32 = 0.0f32
    var one: Float32 = 1.0f32
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

const MATH_F32_METHODS_SLICE3_SMOKE_EXPECTED: &str = "3\n2\n8\n0\n5\n0\n1\n0";

#[test]
fn math_f32_methods_slice3_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_F32_METHODS_SLICE3_SMOKE);
    assert_eq!(
        out, MATH_F32_METHODS_SLICE3_SMOKE_EXPECTED,
        "native mir Math F32 methods slice3: {out}"
    );
}

#[test]
fn math_f32_methods_slice3_smoke_wasm() {
    let out = run_wasm("smoke", MATH_F32_METHODS_SLICE3_SMOKE);
    assert_eq!(
        out, MATH_F32_METHODS_SLICE3_SMOKE_EXPECTED,
        "wasm Math F32 methods slice3: {out}"
    );
}

#[test]
fn math_f32_methods_slice3_parity_with_math_static_native_mir() {
    let src = r#"
func main() {
    var x: Float32 = 8.0f32
    print(x.log2())
    print(Math.log2f(x))
    var base: Float32 = 2.0f32
    var exp: Float32 = 3.0f32
    print(base.pow(exp))
    print(Math.powf(base, exp))
    var a: Float32 = 3.0f32
    var b: Float32 = 4.0f32
    print(a.hypot(b))
    print(Math.hypotf(a, b))
    var z: Float32 = 0.0f32
    print(z.sinh())
    print(Math.sinhf(z))
}
"#;
    let out = run_native_mir("parity", src);
    assert_eq!(
        out, "3\n3\n8\n8\n5\n5\n0\n0",
        "instance vs Math.*f parity: {out}"
    );
}

// --- from former math_phase19 ---
const MATH_F32_METHODS_SLICE4_SMOKE: &str = r#"
func main() {
    var zero: Float32 = 0.0f32
    var one: Float32 = 1.0f32
    var five: Float32 = 5.0f32
    var three: Float32 = 3.0f32
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

// nextUp/Down: pin printed f32 ulp strings (same as math Soft smoke).
const MATH_F32_METHODS_SLICE4_SMOKE_EXPECTED: &str =
    "0\n0\n0\n0\n0\n-1\n1.0000001\n0.99999994";

#[test]
fn math_f32_methods_soft_remainder_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_F32_METHODS_SLICE4_SMOKE);
    assert_eq!(
        out, MATH_F32_METHODS_SLICE4_SMOKE_EXPECTED,
        "native mir Math F32 Soft remainder: {out}"
    );
}

#[test]
fn math_f32_methods_soft_remainder_smoke_wasm() {
    let out = run_wasm("smoke", MATH_F32_METHODS_SLICE4_SMOKE);
    assert_eq!(
        out, MATH_F32_METHODS_SLICE4_SMOKE_EXPECTED,
        "wasm Math F32 Soft remainder: {out}"
    );
}

#[test]
fn math_f32_methods_soft_remainder_parity_with_math_static_native_mir() {
    let src = r#"
func main() {
    var z: Float32 = 0.0f32
    print(z.expm1())
    print(Math.expm1f(z))
    print(z.asinh())
    print(Math.asinhf(z))
    var five: Float32 = 5.0f32
    var three: Float32 = 3.0f32
    print(five.remainder(three))
    print(Math.remainderf(five, three))
    var one: Float32 = 1.0f32
    print(one.nextUp())
    print(Math.nextUpf(one))
}
"#;
    let out = run_native_mir("parity", src);
    assert_eq!(
        out, "0\n0\n0\n0\n-1\n-1\n1.0000001\n1.0000001",
        "instance vs Math.*f parity: {out}"
    );
}

// --- from former math_phase20 ---
const MATH_F32_METHODS_SLICE5_SMOKE: &str = r#"
func main() {
    var five: Float32 = 5.0f32
    var two: Float32 = 2.0f32
    var mag: Float32 = 1.5f32
    var a: Float32 = 2.0f32
    var b: Float32 = 3.0f32
    var c: Float32 = 4.0f32
    var zero: Float32 = 0.0f32
    var one: Float32 = 1.0f32
    var nanv: Float32 = Math.sqrtf(-1.0f32)
    print(five.fmod(two))
    print(mag.copysign(-2.0f32))
    print(a.mulAdd(b, c))
    print(zero.erf())
    print(zero.erfc())
    print(one.tgamma())
    print(one.lgamma())
    print(nanv.isNaN())
}
"#;

const MATH_F32_METHODS_SLICE5_SMOKE_EXPECTED: &str = "1\n-1.5\n10\n0\n1\n1\n0\ntrue";

#[test]
fn math_f32_methods_soft_close_special_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_F32_METHODS_SLICE5_SMOKE);
    assert_eq!(
        out, MATH_F32_METHODS_SLICE5_SMOKE_EXPECTED,
        "native mir Math F32 Soft close+special: {out}"
    );
}

#[test]
fn math_f32_methods_soft_close_special_smoke_wasm() {
    let out = run_wasm("smoke", MATH_F32_METHODS_SLICE5_SMOKE);
    assert_eq!(
        out, MATH_F32_METHODS_SLICE5_SMOKE_EXPECTED,
        "wasm Math F32 Soft close+special: {out}"
    );
}

#[test]
fn math_f32_methods_soft_close_special_parity_with_math_static_native_mir() {
    let src = r#"
func main() {
    var five: Float32 = 5.0f32
    var two: Float32 = 2.0f32
    print(five.fmod(two))
    print(Math.fmodf(five, two))
    var a: Float32 = 2.0f32
    print(a.mulAdd(3.0f32, 4.0f32))
    print(Math.mulAddf(2.0f32, 3.0f32, 4.0f32))
    var zero: Float32 = 0.0f32
    print(zero.erf())
    print(Math.erff(zero))
    var nanv: Float32 = Math.sqrtf(-1.0f32)
    print(nanv.isNaN())
    print(Math.isNaNf(nanv))
}
"#;
    let out = run_native_mir("parity", src);
    assert_eq!(
        out, "1\n1\n10\n10\n0\n0\ntrue\ntrue",
        "instance vs Math.*f parity: {out}"
    );
}

// --- from former math_phase21 ---
const MATH_F32_METHODS_CLOSE_SMOKE: &str = r#"
func main() {
    var three: Float32 = 3.0f32
    var eight: Float32 = 8.0f32
    var inf: Float32 = Math.logf(0.0f32)
    var one: Float32 = 1.0f32
    print(three.exp2())
    print(eight.cbrt())
    print(inf.isInfinite())
    print(one.isFinite())
}
"#;

const MATH_F32_METHODS_CLOSE_SMOKE_EXPECTED: &str = "8\n2\ntrue\ntrue";

#[test]
fn math_f32_methods_close_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_F32_METHODS_CLOSE_SMOKE);
    assert_eq!(
        out, MATH_F32_METHODS_CLOSE_SMOKE_EXPECTED,
        "native mir Math F32 close: {out}"
    );
}

#[test]
fn math_f32_methods_close_smoke_wasm() {
    let out = run_wasm("smoke", MATH_F32_METHODS_CLOSE_SMOKE);
    assert_eq!(
        out, MATH_F32_METHODS_CLOSE_SMOKE_EXPECTED,
        "wasm Math F32 close: {out}"
    );
}

#[test]
fn math_f32_methods_close_parity_with_math_static_native_mir() {
    let src = r#"
func main() {
    var three: Float32 = 3.0f32
    print(three.exp2())
    print(Math.exp2f(three))
    var eight: Float32 = 8.0f32
    print(eight.cbrt())
    print(Math.cbrtf(eight))
    var inf: Float32 = Math.logf(0.0f32)
    print(inf.isInfinite())
    print(Math.isInfinitef(inf))
    var one: Float32 = 1.0f32
    print(one.isFinite())
    print(Math.isFinitef(one))
}
"#;
    let out = run_native_mir("parity", src);
    assert_eq!(
        out, "8\n8\n2\n2\ntrue\ntrue\ntrue\ntrue",
        "instance vs Math.*f parity: {out}"
    );
}

