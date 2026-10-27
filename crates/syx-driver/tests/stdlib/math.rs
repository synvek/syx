//! Core Math static API smoke (Float64 / Soft / special / Float32).
//! Phase-1 host smoke also covers native llvm AOT.
use std::fs;
use std::process::Command;

use syx_driver::{
    build_native_executable, run_native_mir_file_printed, run_wasm_source_printed, DriverOptions,
};

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
        "syx_math_{}_{}_{}",
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

/// W955 / ADR-1299 — Math Phase-1 host smoke via native llvm AOT (`build_native_executable`).
#[cfg(unix)]
fn run_native_llvm(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_math_llvm_{}_{}_{}",
        label,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native llvm Math build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native llvm Math");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let code = output.status.code().unwrap_or(1);
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        code, 0,
        "native llvm Math exit {code}: stdout={stdout} stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    lines_joined(
        &stdout
            .lines()
            .map(|l| l.trim_end_matches('\r').to_string())
            .collect::<Vec<_>>(),
    )
}

// --- from former math_phase1 ---
const MATH_SMOKE: &str = r#"
func main() {
    print(Math.gcd(12, 18))
    print(Math.lcm(4, 6))
    print(Math.clamp(5, 0, 3))
    print(Math.sign(-7))
    print(Math.fabs(-2.5))
    print(Math.trunc(-1.9))
    print(Math.exp(0.0))
    print(Math.log10(100.0))
    print(Math.log2(8.0))
    print(Math.hypot(3.0, 4.0))
    print(Math.cbrt(8.0))
    print(Math.atan2(0.0, -1.0))
}
"#;

const MATH_SMOKE_EXPECTED: &str = "6\n12\n3\n-1\n2.5\n-1\n1\n2\n3\n5\n2\n3.141592653589793";

// Wasm: host float path only (while/gcd not claimed for wasm this window).
const MATH_HOST_SMOKE: &str = r#"
func main() {
    print(Math.fabs(-2.5))
    print(Math.trunc(-1.9))
    print(Math.exp(0.0))
    print(Math.log10(100.0))
    print(Math.log2(8.0))
    print(Math.hypot(3.0, 4.0))
    print(Math.cbrt(8.0))
    print(Math.atan2(0.0, -1.0))
    print(Math.asin(0.0))
    print(Math.acos(1.0))
}
"#;

const MATH_HOST_SMOKE_EXPECTED: &str = "2.5\n-1\n1\n2\n3\n5\n2\n3.141592653589793\n0\n0";

const MATH_NAN: &str = r#"
func main() {
    print(__syx_f64_asin(2.0))
    print(__syx_f64_log2(-1.0))
    print(__syx_f64_sqrt(-1.0))
}
"#;

const MATH_NAN_EXPECTED: &str = "NaN\nNaN\nNaN";

#[test]
fn math_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_SMOKE);
    assert_eq!(out, MATH_SMOKE_EXPECTED, "native mir Math smoke: {out}");
}

/// W966 / ADR-1310 — MATH_HOST_SMOKE mir exact (QS1 Math pilot parity with wasm/llvm).
#[test]
fn math_host_smoke_native_mir() {
    let out = run_native_mir("host_smoke", MATH_HOST_SMOKE);
    assert_eq!(
        out, MATH_HOST_SMOKE_EXPECTED,
        "native mir Math host smoke: {out}"
    );
}

#[test]
fn math_host_smoke_wasm() {
    let out = run_wasm("host_smoke", MATH_HOST_SMOKE);
    assert_eq!(out, MATH_HOST_SMOKE_EXPECTED, "wasm Math host smoke: {out}");
}

/// W955 / ADR-1299 — Phase-1 MathFloat* llvm first-slice exact nail (≠ llvm Math GA).
#[cfg(unix)]
#[test]
fn math_host_smoke_native_llvm() {
    let out = run_native_llvm("host_smoke", MATH_HOST_SMOKE);
    assert_eq!(
        out, MATH_HOST_SMOKE_EXPECTED,
        "native llvm Math host smoke: {out}"
    );
}

/// W971 / ADR-1315 — Phase-1 gap HostFn not in MATH_HOST_SMOKE (QS1 Math P1gap).
const MATH_P1_GAP_SMOKE: &str = r#"
func main() {
    print(Math.sin(0.0))
    print(Math.cos(0.0))
    print(Math.tan(0.0))
    print(Math.sqrt(4.0))
    print(Math.pow(2.0, 3.0))
    print(Math.floor(-1.5))
    print(Math.ceil(-1.5))
    print(Math.round(1.5))
    print(Math.log(Math.e()))
    print(Math.atan(0.0))
    print(Math.exp2(3.0))
}
"#;

const MATH_P1_GAP_SMOKE_EXPECTED: &str = "0\n1\n0\n2\n8\n-2\n-1\n2\n1\n0\n8";

#[test]
fn math_p1_gap_smoke_native_mir() {
    let out = run_native_mir("p1_gap", MATH_P1_GAP_SMOKE);
    assert_eq!(
        out, MATH_P1_GAP_SMOKE_EXPECTED,
        "native mir Math Phase-1 gap: {out}"
    );
}

#[test]
fn math_p1_gap_smoke_wasm() {
    let out = run_wasm("p1_gap", MATH_P1_GAP_SMOKE);
    assert_eq!(
        out, MATH_P1_GAP_SMOKE_EXPECTED,
        "wasm Math Phase-1 gap: {out}"
    );
}

/// W971 / ADR-1315 — Phase-1 gap MathFloat*×11 llvm exact nail (≠ qs1_math_ga).
#[cfg(unix)]
#[test]
fn math_p1_gap_smoke_native_llvm() {
    let out = run_native_llvm("p1_gap", MATH_P1_GAP_SMOKE);
    assert_eq!(
        out, MATH_P1_GAP_SMOKE_EXPECTED,
        "native llvm Math Phase-1 gap: {out}"
    );
}

#[test]
fn math_nan_native_mir() {
    let out = run_native_mir("nan", MATH_NAN);
    assert_eq!(out, MATH_NAN_EXPECTED, "native mir Math NaN: {out}");
}

#[test]
fn math_nan_wasm() {
    let out = run_wasm("nan", MATH_NAN);
    assert_eq!(out, MATH_NAN_EXPECTED, "wasm Math NaN: {out}");
}

#[test]
fn math_constants_and_ln_alias_native_mir() {
    let src = r#"
func main() {
    print(Math.pi())
    print(Math.e())
    print(Math.tau())
    print(Math.ln(Math.e()))
    print(Math.log(Math.e()))
    print(Math.toDegrees(Math.pi()))
    print(Math.fracPi2())
    print(Math.sqrt2())
}
"#;
    let out = run_native_mir("consts", src);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 8, "expected 8 lines, got {out}");
    assert!(
        lines[0].starts_with("3.14159265358979"),
        "pi: {}",
        lines[0]
    );
    assert!(lines[1].starts_with("2.71828182845904"), "e: {}", lines[1]);
    assert!(
        lines[2].starts_with("6.28318530717958"),
        "tau: {}",
        lines[2]
    );
    assert_eq!(lines[3], "1", "ln(e)");
    assert_eq!(lines[4], "1", "log(e)");
    assert_eq!(lines[5], "180", "toDegrees(pi)");
    assert!(
        lines[6].starts_with("1.57079632679489"),
        "fracPi2: {}",
        lines[6]
    );
    assert!(
        lines[7].starts_with("1.41421356237309"),
        "sqrt2: {}",
        lines[7]
    );
}

// --- from former math_phase2 ---
const MATH_P2_SMOKE: &str = r#"
func main() {
    print(Math.sinh(0.0))
    print(Math.cosh(0.0))
    print(Math.tanh(0.0))
    print(Math.fmod(5.0, 2.0))
    print(Math.copysign(1.5, -2.0))
    print(Math.isNaN(Math.sqrt(-1.0)))
    print(Math.isInfinite(Math.log(0.0)))
    print(Math.isFinite(1.0))
    print(Math.expm1(0.0))
    print(Math.log1p(0.0))
}
"#;

const MATH_P2_SMOKE_EXPECTED: &str =
    "0\n1\n0\n1\n-1.5\ntrue\ntrue\ntrue\n0\n0";

#[test]
fn math_hyperbolic_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_P2_SMOKE);
    assert_eq!(out, MATH_P2_SMOKE_EXPECTED, "native mir Math Phase-2: {out}");
}

#[test]
fn math_hyperbolic_smoke_wasm() {
    let out = run_wasm("smoke", MATH_P2_SMOKE);
    assert_eq!(out, MATH_P2_SMOKE_EXPECTED, "wasm Math Phase-2: {out}");
}

/// W956 / ADR-1300 — Phase-2 MathFloat* llvm first-slice exact nail (≠ llvm Math GA).
#[cfg(unix)]
#[test]
fn math_hyperbolic_smoke_native_llvm() {
    let out = run_native_llvm("p2_smoke", MATH_P2_SMOKE);
    assert_eq!(
        out, MATH_P2_SMOKE_EXPECTED,
        "native llvm Math Phase-2: {out}"
    );
}

#[test]
fn math_hyperbolic_host_direct_nan_native_mir() {
    let src = r#"
func main() {
    print(__syx_f64_isnan(__syx_f64_sqrt(-1.0)))
    print(__syx_f64_isinf(__syx_f64_log(0.0)))
    print(__syx_f64_isfinite(0.0))
}
"#;
    let out = run_native_mir("host_nan", src);
    assert_eq!(out, "true\ntrue\ntrue", "host classify: {out}");
}

// --- from former math_phase3 ---
const MATH_SOFT_SMOKE: &str = r#"
func main() {
    print(Math.asinh(0.0))
    print(Math.acosh(1.0))
    print(Math.atanh(0.0))
    print(Math.remainder(5.0, 3.0))
    print(Math.mulAdd(2.0, 3.0, 4.0))
    print(Math.nextUp(1.0))
    print(Math.nextDown(1.0))
}
"#;

// nextUp/Down: avoid Float `>` on adjacent values (SYX cmp may truncate); pin printed ulp strings.
const MATH_SOFT_SMOKE_EXPECTED: &str =
    "0\n0\n0\n-1\n10\n1.0000000000000002\n0.9999999999999999";

#[test]
fn math_soft_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_SOFT_SMOKE);
    assert_eq!(out, MATH_SOFT_SMOKE_EXPECTED, "native mir Math Soft: {out}");
}

#[test]
fn math_soft_smoke_wasm() {
    let out = run_wasm("smoke", MATH_SOFT_SMOKE);
    assert_eq!(out, MATH_SOFT_SMOKE_EXPECTED, "wasm Math Soft: {out}");
}

/// W957 / ADR-1301 — Soft MathFloat* llvm first-slice exact nail (≠ llvm Math GA).
#[cfg(unix)]
#[test]
fn math_soft_smoke_native_llvm() {
    let out = run_native_llvm("soft_smoke", MATH_SOFT_SMOKE);
    assert_eq!(
        out, MATH_SOFT_SMOKE_EXPECTED,
        "native llvm Math Soft: {out}"
    );
}

#[test]
fn math_soft_host_direct_native_mir() {
    let src = r#"
func main() {
    print(__syx_f64_asinh(0.0))
    print(__syx_f64_mul_add(2.0, 3.0, 4.0))
    print(__syx_f64_remainder(5.0, 3.0))
}
"#;
    let out = run_native_mir("host_direct", src);
    assert_eq!(out, "0\n10\n-1", "host soft direct: {out}");
}

// --- from former math_phase4 ---
const MATH_SPECIAL_SMOKE: &str = r#"
func main() {
    print(Math.erf(0.0))
    print(Math.erfc(0.0))
    print(Math.tgamma(1.0))
    print(Math.tgamma(2.0))
    print(Math.lgamma(1.0))
}
"#;

const MATH_SPECIAL_SMOKE_EXPECTED: &str = "0\n1\n1\n1\n0";

#[test]
fn math_special_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_SPECIAL_SMOKE);
    assert_eq!(out, MATH_SPECIAL_SMOKE_EXPECTED, "native mir Math Special: {out}");
}

#[test]
fn math_special_smoke_wasm() {
    let out = run_wasm("smoke", MATH_SPECIAL_SMOKE);
    assert_eq!(out, MATH_SPECIAL_SMOKE_EXPECTED, "wasm Math Special: {out}");
}

/// W958 / ADR-1302 — Special MathFloat* llvm first-slice exact nail (≠ llvm Math GA).
#[cfg(unix)]
#[test]
fn math_special_smoke_native_llvm() {
    let out = run_native_llvm("special_smoke", MATH_SPECIAL_SMOKE);
    assert_eq!(
        out, MATH_SPECIAL_SMOKE_EXPECTED,
        "native llvm Math Special: {out}"
    );
}

#[test]
fn math_special_host_direct_native_mir() {
    let src = r#"
func main() {
    print(__syx_f64_erf(0.0))
    print(__syx_f64_erfc(0.0))
    print(__syx_f64_tgamma(1.0))
    print(__syx_f64_lgamma(1.0))
}
"#;
    let out = run_native_mir("host_direct", src);
    assert_eq!(out, "0\n1\n1\n0", "host special direct: {out}");
}

// --- from former math_phase5 ---
const MATH_F32_SMOKE: &str = r#"
func main() {
    print(Math.sqrtf(4.0f32))
    print(Math.sinf(0.0f32))
    print(Math.cosf(0.0f32))
    print(Math.tanf(0.0f32))
    print(Math.expf(0.0f32))
    print(Math.logf(1.0f32))
    print(Math.fabsf(-1.5f32))
    print(Math.floorf(1.9f32))
}
"#;

const MATH_F32_SMOKE_EXPECTED: &str = "2\n0\n1\n0\n1\n0\n1.5\n1";

#[test]
fn math_f32_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_F32_SMOKE);
    assert_eq!(out, MATH_F32_SMOKE_EXPECTED, "native mir Math F32: {out}");
}

#[test]
fn math_f32_smoke_wasm() {
    let out = run_wasm("smoke", MATH_F32_SMOKE);
    assert_eq!(out, MATH_F32_SMOKE_EXPECTED, "wasm Math F32: {out}");
}

/// W959 / ADR-1303 — F32 W906 MathFloat32* llvm first-slice exact nail (≠ full F32×42 / ≠ llvm Math GA).
#[cfg(unix)]
#[test]
fn math_f32_smoke_native_llvm() {
    let out = run_native_llvm("f32_smoke", MATH_F32_SMOKE);
    assert_eq!(
        out, MATH_F32_SMOKE_EXPECTED,
        "native llvm Math F32 W906: {out}"
    );
}

#[test]
fn math_f32_host_direct_native_mir() {
    let src = r#"
func main() {
    print(__syx_f32_sqrt(4.0f32))
    print(__syx_f32_abs(-2.0f32))
    print(__syx_f32_floor(3.7f32))
}
"#;
    let out = run_native_mir("host_direct", src);
    assert_eq!(out, "2\n2\n3", "host f32 direct: {out}");
}

// --- from former math_phase6 ---
const MATH_F32_SLICE2_SMOKE: &str = r#"
func main() {
    print(Math.ceilf(1.1f32))
    print(Math.roundf(1.5f32))
    print(Math.truncf(1.9f32))
    print(Math.powf(2.0f32, 3.0f32))
    print(Math.asinf(0.0f32))
    print(Math.acosf(1.0f32))
    print(Math.atanf(0.0f32))
    print(Math.hypotf(3.0f32, 4.0f32))
}
"#;

const MATH_F32_SLICE2_SMOKE_EXPECTED: &str = "2\n2\n1\n8\n0\n0\n0\n5";

#[test]
fn math_f32_slice2_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_F32_SLICE2_SMOKE);
    assert_eq!(out, MATH_F32_SLICE2_SMOKE_EXPECTED, "native mir Math F32 slice2: {out}");
}

#[test]
fn math_f32_slice2_smoke_wasm() {
    let out = run_wasm("smoke", MATH_F32_SLICE2_SMOKE);
    assert_eq!(out, MATH_F32_SLICE2_SMOKE_EXPECTED, "wasm Math F32 slice2: {out}");
}

/// W960 / ADR-1304 — F32 W907 MathFloat32* llvm slice2 exact nail (≠ full F32×42 / ≠ llvm Math GA).
#[cfg(unix)]
#[test]
fn math_f32_slice2_smoke_native_llvm() {
    let out = run_native_llvm("f32_slice2_smoke", MATH_F32_SLICE2_SMOKE);
    assert_eq!(
        out, MATH_F32_SLICE2_SMOKE_EXPECTED,
        "native llvm Math F32 W907 slice2: {out}"
    );
}

#[test]
fn math_f32_slice2_host_direct_native_mir() {
    let src = r#"
func main() {
    print(__syx_f32_ceil(1.1f32))
    print(__syx_f32_pow(2.0f32, 3.0f32))
    print(__syx_f32_hypot(3.0f32, 4.0f32))
}
"#;
    let out = run_native_mir("host_direct", src);
    assert_eq!(out, "2\n8\n5", "host f32 slice2 direct: {out}");
}

// --- from former math_phase7 ---
const MATH_F32_SLICE3_SMOKE: &str = r#"
func main() {
    print(Math.atan2f(0.0f32, 1.0f32))
    print(Math.log2f(8.0f32))
    print(Math.log10f(100.0f32))
    print(Math.exp2f(3.0f32))
    print(Math.cbrtf(8.0f32))
    print(Math.sinhf(0.0f32))
    print(Math.coshf(0.0f32))
    print(Math.tanhf(0.0f32))
}
"#;

const MATH_F32_SLICE3_SMOKE_EXPECTED: &str = "0\n3\n2\n8\n2\n0\n1\n0";

#[test]
fn math_f32_slice3_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_F32_SLICE3_SMOKE);
    assert_eq!(out, MATH_F32_SLICE3_SMOKE_EXPECTED, "native mir Math F32 slice3: {out}");
}

#[test]
fn math_f32_slice3_smoke_wasm() {
    let out = run_wasm("smoke", MATH_F32_SLICE3_SMOKE);
    assert_eq!(out, MATH_F32_SLICE3_SMOKE_EXPECTED, "wasm Math F32 slice3: {out}");
}

/// W961 / ADR-1305 — F32 W908 MathFloat32* llvm slice3 exact nail (≠ full F32×42 / ≠ llvm Math GA).
#[cfg(unix)]
#[test]
fn math_f32_slice3_smoke_native_llvm() {
    let out = run_native_llvm("f32_slice3_smoke", MATH_F32_SLICE3_SMOKE);
    assert_eq!(
        out, MATH_F32_SLICE3_SMOKE_EXPECTED,
        "native llvm Math F32 W908 slice3: {out}"
    );
}

#[test]
fn math_f32_slice3_host_direct_native_mir() {
    let src = r#"
func main() {
    print(__syx_f32_atan2(0.0f32, 1.0f32))
    print(__syx_f32_log2(8.0f32))
    print(__syx_f32_exp2(3.0f32))
}
"#;
    let out = run_native_mir("host_direct", src);
    assert_eq!(out, "0\n3\n8", "host f32 slice3 direct: {out}");
}

// --- from former math_phase8 ---
const MATH_F32_SLICE4_SMOKE: &str = r#"
func main() {
    print(Math.fmodf(5.0f32, 2.0f32))
    print(Math.copysignf(1.0f32, -1.0f32))
    print(Math.expm1f(0.0f32))
    print(Math.log1pf(0.0f32))
    print(Math.asinhf(0.0f32))
    print(Math.acoshf(1.0f32))
    print(Math.atanhf(0.0f32))
    print(Math.erff(0.0f32))
}
"#;

const MATH_F32_SLICE4_SMOKE_EXPECTED: &str = "1\n-1\n0\n0\n0\n0\n0\n0";

#[test]
fn math_f32_slice4_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_F32_SLICE4_SMOKE);
    assert_eq!(out, MATH_F32_SLICE4_SMOKE_EXPECTED, "native mir Math F32 slice4: {out}");
}

#[test]
fn math_f32_slice4_smoke_wasm() {
    let out = run_wasm("smoke", MATH_F32_SLICE4_SMOKE);
    assert_eq!(out, MATH_F32_SLICE4_SMOKE_EXPECTED, "wasm Math F32 slice4: {out}");
}

/// W962 / ADR-1306 — F32 W909 MathFloat32* llvm slice4 exact nail (≠ full F32×42 / ≠ llvm Math GA).
#[cfg(unix)]
#[test]
fn math_f32_slice4_smoke_native_llvm() {
    let out = run_native_llvm("f32_slice4_smoke", MATH_F32_SLICE4_SMOKE);
    assert_eq!(
        out, MATH_F32_SLICE4_SMOKE_EXPECTED,
        "native llvm Math F32 W909 slice4: {out}"
    );
}

#[test]
fn math_f32_slice4_host_direct_native_mir() {
    let src = r#"
func main() {
    print(__syx_f32_fmod(5.0f32, 2.0f32))
    print(__syx_f32_asinh(0.0f32))
    print(__syx_f32_erf(0.0f32))
}
"#;
    let out = run_native_mir("host_direct", src);
    assert_eq!(out, "1\n0\n0", "host f32 slice4 direct: {out}");
}

// --- from former math_phase9 ---
const MATH_F32_SLICE5_SMOKE: &str = r#"
func main() {
    print(Math.erfcf(0.0f32))
    print(Math.tgammaf(1.0f32))
    print(Math.lgammaf(1.0f32))
    print(Math.remainderf(5.0f32, 3.0f32))
    print(Math.isNaNf(0.0f32))
    print(Math.isInfinitef(0.0f32))
    print(Math.isFinitef(1.0f32))
    print(Math.nextUpf(1.0f32))
}
"#;

// nextUpf: pin printed ulp string (avoid Float cmp on adjacent values).
const MATH_F32_SLICE5_SMOKE_EXPECTED: &str = "1\n1\n0\n-1\nfalse\nfalse\ntrue\n1.0000001";

#[test]
fn math_f32_slice5_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_F32_SLICE5_SMOKE);
    assert_eq!(out, MATH_F32_SLICE5_SMOKE_EXPECTED, "native mir Math F32 slice5: {out}");
}

#[test]
fn math_f32_slice5_smoke_wasm() {
    let out = run_wasm("smoke", MATH_F32_SLICE5_SMOKE);
    assert_eq!(out, MATH_F32_SLICE5_SMOKE_EXPECTED, "wasm Math F32 slice5: {out}");
}

/// W963 / ADR-1307 — F32 W910 MathFloat32* llvm slice5 exact nail (≠ full F32×42 / ≠ llvm Math GA).
#[cfg(unix)]
#[test]
fn math_f32_slice5_smoke_native_llvm() {
    let out = run_native_llvm("f32_slice5_smoke", MATH_F32_SLICE5_SMOKE);
    assert_eq!(
        out, MATH_F32_SLICE5_SMOKE_EXPECTED,
        "native llvm Math F32 W910 slice5: {out}"
    );
}

#[test]
fn math_f32_slice5_host_direct_native_mir() {
    let src = r#"
func main() {
    print(__syx_f32_erfc(0.0f32))
    print(__syx_f32_remainder(5.0f32, 3.0f32))
    print(__syx_f32_isnan(0.0f32))
}
"#;
    let out = run_native_mir("host_direct", src);
    assert_eq!(out, "1\n-1\nfalse", "host f32 slice5 direct: {out}");
}

// --- from former math_phase10 ---
const MATH_F32_SOFT_CLOSE_SMOKE: &str = r#"
func main() {
    print(Math.mulAddf(2.0f32, 3.0f32, 4.0f32))
    print(Math.nextUpf(1.0f32))
    print(Math.nextDownf(1.0f32))
}
"#;

// nextUp/Down: pin printed ulp strings (avoid Float cmp on adjacent values).
const MATH_F32_SOFT_CLOSE_SMOKE_EXPECTED: &str = "10\n1.0000001\n0.99999994";

#[test]
fn math_f32_soft_close_smoke_native_mir() {
    let out = run_native_mir("smoke", MATH_F32_SOFT_CLOSE_SMOKE);
    assert_eq!(
        out, MATH_F32_SOFT_CLOSE_SMOKE_EXPECTED,
        "native mir Math F32 Soft close: {out}"
    );
}

#[test]
fn math_f32_soft_close_smoke_wasm() {
    let out = run_wasm("smoke", MATH_F32_SOFT_CLOSE_SMOKE);
    assert_eq!(
        out, MATH_F32_SOFT_CLOSE_SMOKE_EXPECTED,
        "wasm Math F32 Soft close: {out}"
    );
}

/// W964 / ADR-1308 — F32 W911 soft-close llvm exact nail (MathFloat* 84/84 ≠ llvm Math GA).
#[cfg(unix)]
#[test]
fn math_f32_soft_close_smoke_native_llvm() {
    let out = run_native_llvm("f32_soft_close_smoke", MATH_F32_SOFT_CLOSE_SMOKE);
    assert_eq!(
        out, MATH_F32_SOFT_CLOSE_SMOKE_EXPECTED,
        "native llvm Math F32 W911 soft-close: {out}"
    );
}

#[test]
fn math_f32_soft_close_host_direct_native_mir() {
    let src = r#"
func main() {
    print(__syx_f32_mul_add(2.0f32, 3.0f32, 4.0f32))
    print(__syx_f32_next_down(1.0f32))
}
"#;
    let out = run_native_mir("host_direct", src);
    assert_eq!(out, "10\n0.99999994", "host f32 Soft close direct: {out}");
}

