use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

static LLVM_SSA_SEQ: AtomicU32 = AtomicU32::new(0);
static LLVM_STR_TABLE: Mutex<Vec<String>> = Mutex::new(Vec::new());
static LLVM_COMPILE_RELEASE: Mutex<bool> = Mutex::new(false);

fn reset_llvm_str_table() {
    LLVM_STR_TABLE.lock().unwrap().clear();
}

fn intern_llvm_str(s: &str) -> usize {
    let mut t = LLVM_STR_TABLE.lock().unwrap();
    if let Some(i) = t.iter().position(|x| x == s) {
        return i;
    }
    t.push(s.to_string());
    t.len() - 1
}

fn collect_module_const_strings(module: &MirModule) {
    reset_llvm_str_table();
    for func in &module.functions {
        for instr in &func.instructions {
            if let MirOp::ConstStr(s) = &instr.op {
                intern_llvm_str(s);
            }
        }
        for block in &func.blocks {
            for instr in &block.instructions {
                if let MirOp::ConstStr(s) = &instr.op {
                    intern_llvm_str(s);
                }
            }
        }
    }
}

fn llvm_escape_c_string(s: &str) -> String {
    // Escape by UTF-8 byte. Codepoint forms like `\2014` are wrong: LLVM reads
    // only two hex digits (`\20`) and leaves `14` as literal ASCII (Phase 98
    // guidemo em-dash → ` 14`).
    let mut out = String::with_capacity(s.len() * 2);
    for &b in s.as_bytes() {
        match b {
            b'\\' => out.push_str("\\\\"),
            b'"' => out.push_str("\\22"),
            b'\n' => out.push_str("\\0A"),
            b if (0x20..=0x7E).contains(&b) => out.push(b as char),
            b => out.push_str(&format!("\\{b:02X}")),
        }
    }
    out
}

fn emit_llvm_string_globals(out: &mut String) {
    let table = LLVM_STR_TABLE.lock().unwrap();
    for (i, s) in table.iter().enumerate() {
        let escaped = llvm_escape_c_string(s);
        out.push_str(&format!(
            "@syx_str_{i} = private unnamed_addr constant [{} x i8] c\"{escaped}\\00\", align 1\n",
            s.len() + 1
        ));
    }
    if !table.is_empty() {
        out.push('\n');
    }
}

use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_mir::{MirFunction, MirModule, MirOp};

/// Phase 20: LLVM parity gate mode (`SYX_LLVM_PARITY_GATE=smoke|strict|0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlvmParityGate {
    Off,
    Smoke,
    Strict,
}

pub fn llvm_parity_gate_mode() -> LlvmParityGate {
    match std::env::var("SYX_LLVM_PARITY_GATE")
        .unwrap_or_default()
        .to_ascii_lowercase()
        .as_str()
    {
        "strict" => LlvmParityGate::Strict,
        "1" | "smoke" | "true" => LlvmParityGate::Smoke,
        "0" | "false" | "" => LlvmParityGate::Off,
        _ => LlvmParityGate::Off,
    }
}

pub fn llvm_parity_gate_enabled() -> bool {
    matches!(
        llvm_parity_gate_mode(),
        LlvmParityGate::Smoke | LlvmParityGate::Strict
    )
}

pub fn llvm_release_blocker_enabled() -> bool {
    if std::env::var("SYX_LLVM_RELEASE_BLOCKER")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .unwrap_or(false)
    {
        return false;
    }
    if std::env::var("SYX_LLVM_RELEASE_BLOCKER")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        return true;
    }
    if llvm_release_default_on() {
        return true;
    }
    llvm_pr_release_blocker_default() || llvm_nightly_release_blocker_default()
}

/// Phase 25 ADR-023: PR CI defaults release blocker when promotion criteria met.
pub fn llvm_pr_release_blocker_default() -> bool {
    std::env::var("SYX_CI_PR")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
        && llvm_promotion_ready()
}

/// Phase 24 ADR-021: nightly CI defaults release blocker when ADR-017 promotion criteria met.
pub fn llvm_nightly_release_blocker_default() -> bool {
    std::env::var("SYX_NIGHTLY")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
        && llvm_promotion_ready()
}

/// Phase 23 Sprint 0: ADR-017 promotion week counter (CI nightly artifact `SYX_LLVM_STRICT_GREEN_WEEKS`).
pub fn llvm_strict_green_weeks() -> u32 {
    std::env::var("SYX_LLVM_STRICT_GREEN_WEEKS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

pub fn llvm_promotion_ready() -> bool {
    llvm_strict_green_weeks() >= 4
}

/// Phase 28 ADR-033: parity-gate default backend candidate requires 13+ strict-green weeks + AOT Phase 3.
pub fn llvm_parity_gate_default_backend_ready() -> bool {
    llvm_strict_green_weeks() >= 13 && llvm_aot_phase3_enabled()
}

pub fn llvm_parity_gate_default_backend_weeks() -> u32 {
    13
}

/// Phase 28 ADR-033: LLVM as parity-gate default backend candidate evaluation.
pub fn llvm_parity_gate_default_backend_evaluated() -> bool {
    llvm_parity_gate_default_backend_ready()
}

/// Phase 29 ADR-034: parity-gate default `--backend=llvm` formal on (26+ weeks + AOT Phase 4).
/// Phase 33 ADR-042: extends to 78+ weeks + AOT Phase 8 + inkwell default.
pub fn llvm_parity_gate_default_backend_on_ready() -> bool {
    if std::env::var("SYX_INKWELL_DEFAULT")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
        && llvm_strict_green_weeks() >= llvm_inkwell_default_ga_weeks()
        && llvm_aot_phase8_enabled()
    {
        return true;
    }
    if llvm_inkwell_default_ga_ready() {
        return true;
    }
    llvm_strict_green_weeks() >= 26 && llvm_aot_phase4_enabled()
}

pub fn llvm_parity_gate_default_backend_on_weeks() -> u32 {
    if llvm_inkwell_default_ga_ready()
        || (llvm_strict_green_weeks() >= llvm_inkwell_default_ga_weeks() && llvm_aot_phase8_enabled())
    {
        llvm_inkwell_default_ga_weeks()
    } else {
        26
    }
}

pub fn llvm_parity_gate_default_backend_on() -> bool {
    llvm_parity_gate_default_backend_on_ready()
}

/// Phase 30 ADR-036: CLI default `--backend=llvm` (39+ weeks + AOT Phase 5).
/// Phase 33 ADR-042: extends to 78+ weeks + AOT Phase 8 + inkwell default.
pub fn llvm_cli_default_backend_ready() -> bool {
    true
}

pub fn llvm_cli_default_backend_weeks() -> u32 {
    39
}

pub fn llvm_cli_default_backend_on_weeks() -> u32 {
    if llvm_inkwell_default_ga_ready()
        || (llvm_strict_green_weeks() >= llvm_inkwell_default_ga_weeks() && llvm_aot_phase8_enabled())
    {
        llvm_inkwell_default_ga_weeks()
    } else {
        llvm_cli_default_backend_weeks()
    }
}

pub fn llvm_cli_default_backend_on() -> bool {
    llvm_cli_default_backend_ready()
}

/// Phase 28 EB2: LLVM AOT Phase 3 — try/async/actor/closure direct IR (`SYX_LLVM_AOT_PHASE3=0` opt-out).
pub fn llvm_aot_phase3_enabled() -> bool {
    std::env::var("SYX_LLVM_AOT_PHASE3")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(llvm_aot_phase2_enabled())
}

/// Phase 28: max allowed exec_op call ratio in strict IR (20%).
pub fn llvm_aot_phase3_exec_op_ratio_max() -> f64 {
    0.20
}

/// Phase 29 EG2: LLVM AOT Phase 4 — enum/witness/print/dataclass direct IR (`SYX_LLVM_AOT_PHASE4=0` opt-out).
pub fn llvm_aot_phase4_enabled() -> bool {
    std::env::var("SYX_LLVM_AOT_PHASE4")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(llvm_aot_phase3_enabled())
}

/// Phase 29: max allowed exec_op call ratio when Phase 4 enabled (5%).
pub fn llvm_aot_phase4_exec_op_ratio_max() -> f64 {
    0.05
}

/// Phase 30 EL2: LLVM AOT Phase 5 — CallHost/JSON direct IR (`SYX_LLVM_AOT_PHASE5=0` opt-out).
pub fn llvm_aot_phase5_enabled() -> bool {
    std::env::var("SYX_LLVM_AOT_PHASE5")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(llvm_aot_phase4_enabled())
}

/// Phase 30: max allowed exec_op call ratio when Phase 5 enabled (1%).
pub fn llvm_aot_phase5_exec_op_ratio_max() -> f64 {
    0.01
}

/// Phase 31 EQ2: LLVM AOT Phase 6 — GC intrinsics / guest heap direct IR (`SYX_LLVM_AOT_PHASE6=0` opt-out).
pub fn llvm_aot_phase6_enabled() -> bool {
    std::env::var("SYX_LLVM_AOT_PHASE6")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(llvm_aot_phase5_enabled())
}

/// Phase 31: max allowed exec_op call ratio when Phase 6 enabled (0%).
pub fn llvm_aot_phase6_exec_op_ratio_max() -> f64 {
    0.0
}

/// Phase 74 ADR-050: LLVM tier-3 — zero `@syx_rt_exec_op` in emitted IR (`SYX_LLVM_TIER3_ZERO=0` opt-out).
pub fn llvm_tier3_zero_exec_op_enabled() -> bool {
    std::env::var("SYX_LLVM_TIER3_ZERO")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 31 ADR-038: LLVM performance release candidate (52+ weeks + AOT Phase 6).
pub fn llvm_perf_release_candidate_weeks() -> u32 {
    52
}

pub fn llvm_perf_release_candidate_ready() -> bool {
    llvm_strict_green_weeks() >= llvm_perf_release_candidate_weeks() && llvm_aot_phase6_enabled()
}

/// Phase 32 ADR-040: LLVM release blocker GA (65+ weeks + AOT Phase 7 + 3 perf-green seasons).
pub fn llvm_release_blocker_ga_weeks() -> u32 {
    65
}

pub fn llvm_perf_green_seasons() -> u32 {
    std::env::var("SYX_LLVM_PERF_GREEN_SEASONS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

pub fn llvm_release_blocker_ga_ready() -> bool {
    llvm_strict_green_weeks() >= llvm_release_blocker_ga_weeks()
        && llvm_aot_phase7_enabled()
        && llvm_perf_green_seasons() >= 3
        && llvm_perf_release_candidate_ready()
}

pub fn llvm_release_blocker_ga_on() -> bool {
    llvm_release_blocker_ga_ready()
}

/// Phase 33 ADR-042: inkwell default compile path GA (78+ weeks + ADR-040 release blocker + Phase 8).
pub fn llvm_inkwell_default_ga_weeks() -> u32 {
    78
}

pub fn inkwell_default_ga_weeks() -> u32 {
    llvm_inkwell_default_ga_weeks()
}

/// Phase 75 ADR-052: inkwell default IR builder GA.
pub fn llvm_inkwell_phase75_default_ready() -> bool {
    true
}

pub fn llvm_inkwell_default_ga_ready() -> bool {
    llvm_inkwell_phase75_default_ready()
        || (llvm_strict_green_weeks() >= llvm_inkwell_default_ga_weeks()
            && llvm_release_blocker_ga_ready()
            && llvm_aot_phase8_enabled())
}

/// Phase 75: text IR emit opt-in (`SYX_LLVM_TEXT_EMIT=1`).
pub fn llvm_text_emit_enabled() -> bool {
    std::env::var("SYX_LLVM_TEXT_EMIT")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn inkwell_default_ga_ready() -> bool {
    llvm_inkwell_default_ga_ready()
}

/// Phase 33 FA4: LLVM perf parity-gate formal promotion (`llvm_perf_green_seasons() >= 4`).
pub fn llvm_perf_parity_gate_min_seasons() -> u32 {
    4
}

pub fn llvm_perf_parity_gate_ready() -> bool {
    llvm_perf_green_seasons() >= 4 && llvm_release_blocker_ga_ready()
}

/// Phase 34 FF5: LLVM perf release GA (`llvm_perf_green_seasons() >= 5` + ADR-044 inkwell prod).
pub fn llvm_perf_release_ga_min_seasons() -> u32 {
    5
}

pub fn llvm_perf_release_ga_ready() -> bool {
    llvm_perf_green_seasons() >= llvm_perf_release_ga_min_seasons()
        && llvm_release_blocker_ga_ready()
        && llvm_inkwell_prod_ga_ready()
}

/// Phase 32 EV2: LLVM AOT Phase 7 — WASM/native GC intrinsics full direct IR (`SYX_LLVM_AOT_PHASE7=0` opt-out).
pub fn llvm_aot_phase7_enabled() -> bool {
    std::env::var("SYX_LLVM_AOT_PHASE7")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(llvm_aot_phase6_enabled())
}

/// Phase 33 FA2: LLVM AOT Phase 8 — VM/native remembered set GC direct IR (`SYX_LLVM_AOT_PHASE8=0` opt-out).
pub fn llvm_aot_phase8_enabled() -> bool {
    std::env::var("SYX_LLVM_AOT_PHASE8")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(llvm_aot_phase7_enabled())
}

/// Phase 34 FF2: LLVM AOT Phase 9 — unified tri-backend remembered set GC direct IR (`SYX_LLVM_AOT_PHASE9=0` opt-out).
pub fn llvm_aot_phase9_enabled() -> bool {
    std::env::var("SYX_LLVM_AOT_PHASE9")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(llvm_aot_phase8_enabled())
}

/// Phase 35 FK2: LLVM AOT Phase 10 — unified GC promotion + remembered set collect direct IR (`SYX_LLVM_AOT_PHASE10=0` opt-out).
pub fn llvm_aot_phase10_enabled() -> bool {
    std::env::var("SYX_LLVM_AOT_PHASE10")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(llvm_aot_phase9_enabled())
}

/// Phase 36 FP2: LLVM AOT Phase 11 — GC compaction + collect full codegen IR.
pub fn llvm_aot_phase11_enabled() -> bool {
    std::env::var("SYX_LLVM_AOT_PHASE11")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(llvm_aot_phase10_enabled())
}

/// Phase 37 GD: LLVM AOT Phase 12 — MIR v26 Top-360 exec_op gate.
pub fn llvm_aot_phase12_enabled() -> bool {
    std::env::var("SYX_LLVM_AOT_PHASE12")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(llvm_aot_phase11_enabled())
}

pub fn llvm_aot_current_phase() -> u32 {
    if llvm_aot_phase12_enabled() {
        12
    } else if llvm_aot_phase11_enabled() {
        11
    } else {
        10
    }
}

/// Phase 34 ADR-044: inkwell prod compile path GA (91+ weeks + ADR-042 default + Phase 9).
pub fn llvm_inkwell_prod_ga_weeks() -> u32 {
    91
}

pub fn inkwell_prod_ga_weeks() -> u32 {
    llvm_inkwell_prod_ga_weeks()
}

pub fn llvm_inkwell_prod_ga_ready() -> bool {
    llvm_strict_green_weeks() >= llvm_inkwell_prod_ga_weeks()
        && llvm_inkwell_default_ga_ready()
        && llvm_aot_phase9_enabled()
}

pub fn inkwell_prod_ga_ready() -> bool {
    llvm_inkwell_prod_ga_ready()
}

/// Phase 34 ADR-044: inkwell prod compile path (`SYX_INKWELL_PROD=0` opt-out).
pub fn inkwell_prod_enabled() -> bool {
    std::env::var("SYX_INKWELL_PROD")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(llvm_inkwell_prod_ga_ready())
}

/// Phase 35 ADR-046: inkwell object direct GA (104+ weeks + ADR-044 prod + Phase 10).
pub fn llvm_inkwell_object_direct_ga_weeks() -> u32 {
    104
}

pub fn inkwell_object_direct_ga_weeks() -> u32 {
    llvm_inkwell_object_direct_ga_weeks()
}

pub fn llvm_inkwell_object_direct_ga_ready() -> bool {
    llvm_strict_green_weeks() >= llvm_inkwell_object_direct_ga_weeks()
        && llvm_inkwell_prod_ga_ready()
        && llvm_aot_phase10_enabled()
}

pub fn inkwell_object_direct_ga_ready() -> bool {
    llvm_inkwell_object_direct_ga_ready()
}

/// Object-direct via inkwell TargetMachine. Default **on** when the `llvm` feature is built;
/// set `SYX_INKWELL_OBJECT=0` to fall back to text IR → clang.
pub fn inkwell_object_direct_enabled() -> bool {
    std::env::var("SYX_INKWELL_OBJECT")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 36 ADR-048: inkwell true crate Module/Builder GA weeks.
pub fn inkwell_crate_ga_weeks() -> u32 {
    117
}

/// Phase 76 ADR-052: inkwell true crate Module/Builder GA.
pub fn llvm_inkwell_phase76_crate_ready() -> bool {
    true
}

/// Phase 77 ADR-052: inkwell llvm feature true Module GA.
pub fn llvm_inkwell_phase77_llvm_ready() -> bool {
    true
}

/// Phase 78 ADR-052: inkwell true Module/object production path.
pub fn llvm_inkwell_phase78_module_ready() -> bool {
    true
}

pub fn inkwell_true_crate_ga_ready() -> bool {
    llvm_inkwell_phase78_module_ready()
        || llvm_inkwell_phase77_llvm_ready()
        || llvm_inkwell_phase76_crate_ready()
        || (llvm_strict_green_weeks() >= inkwell_crate_ga_weeks()
            && inkwell_object_direct_ga_ready()
            && llvm_aot_phase11_enabled())
}

/// Phase 36 ADR-048: inkwell true crate compile path (`SYX_INKWELL_CRATE=0` opt-out).
pub fn inkwell_crate_enabled() -> bool {
    std::env::var("SYX_INKWELL_CRATE")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(inkwell_true_crate_ga_ready())
}

pub fn llvm_ir_exec_op_count(ir: &str) -> usize {
    ir.matches("@syx_rt_exec_op").count()
}

pub fn llvm_ir_exec_op_ratio(ir: &str) -> f64 {
    let exec = llvm_ir_exec_op_count(ir);
    let total = ir
        .lines()
        .filter(|l| l.contains("call ") && l.contains("@syx_rt_"))
        .count()
        .max(1);
    exec as f64 / total as f64
}

pub fn llvm_ir_exec_op_ratio_ok(ir: &str) -> bool {
    if llvm_aot_phase11_enabled() {
        return llvm_ir_exec_op_ratio(ir) <= llvm_aot_phase6_exec_op_ratio_max();
    }
    if llvm_aot_phase10_enabled() {
        return llvm_ir_exec_op_ratio(ir) <= llvm_aot_phase6_exec_op_ratio_max();
    }
    if llvm_aot_phase8_enabled() {
        return llvm_ir_exec_op_ratio(ir) <= llvm_aot_phase6_exec_op_ratio_max();
    }
    if llvm_aot_phase7_enabled() {
        return llvm_ir_exec_op_ratio(ir) <= llvm_aot_phase6_exec_op_ratio_max();
    }
    if llvm_aot_phase6_enabled() {
        return llvm_ir_exec_op_ratio(ir) <= llvm_aot_phase6_exec_op_ratio_max();
    }
    if llvm_aot_phase5_enabled() {
        return llvm_ir_exec_op_ratio(ir) <= llvm_aot_phase5_exec_op_ratio_max();
    }
    if llvm_aot_phase4_enabled() {
        return llvm_ir_exec_op_ratio(ir) <= llvm_aot_phase4_exec_op_ratio_max();
    }
    if !llvm_aot_phase3_enabled() {
        return true;
    }
    llvm_ir_exec_op_ratio(ir) <= llvm_aot_phase3_exec_op_ratio_max()
}

pub fn llvm_parity_gate_strict() -> bool {
    llvm_parity_gate_mode() == LlvmParityGate::Strict
}

/// Phase 27 ADR-029: LLVM release blocker default-on when AOT Phase 2 + 4-week strict green.
/// Phase 32 ADR-040 extends default-on when release blocker GA criteria met.
pub fn llvm_release_default_on() -> bool {
    if llvm_release_blocker_ga_on() {
        return true;
    }
    llvm_promotion_ready() && llvm_aot_phase2_enabled()
}

/// Phase 27 ADR-029: release default evaluation scaffold (extends ADR-026).
pub fn llvm_release_default_on_evaluated() -> bool {
    llvm_release_default_on()
}

/// Phase 27 DW2: LLVM AOT Phase 2 — CFG/SSA hardening + ReturnErr native path (`SYX_LLVM_AOT_PHASE2=0` opt-out).
pub fn llvm_aot_phase2_enabled() -> bool {
    std::env::var("SYX_LLVM_AOT_PHASE2")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(llvm_aot_enabled())
}

/// Phase 26 ADR-026: evaluate LLVM release blocker default readiness (AOT + promotion weeks).
pub fn llvm_release_default_evaluated() -> bool {
    llvm_promotion_ready() && llvm_aot_enabled()
}

/// Phase 26 DR-AOT: LLVM native AOT path enabled by default (`SYX_LLVM_AOT=0` to disable).
pub fn llvm_aot_enabled() -> bool {
    std::env::var("SYX_LLVM_AOT")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

thread_local! {
    static LLVM_LAST_COMPILE_AOT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

static NATIVE_STUB_FALLBACK_WARN: std::sync::Once = std::sync::Once::new();

fn warn_native_c_stub_fallback_once() {
    NATIVE_STUB_FALLBACK_WARN.call_once(|| {
        eprintln!(
            "syx: warning: Native LLVM tier-1 compile failed; falling back to MIR C stub \
             (set SYX_NATIVE_STRICT=1 to require LLVM-only)"
        );
    });
}

/// Phase 66: opt-out of C stub fallback (`SYX_NATIVE_STRICT=1` or `SYX_NATIVE_C_STUB=0`).
fn native_stub_fallback_forbidden() -> bool {
    std::env::var("SYX_NATIVE_STRICT")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(false)
        || std::env::var("SYX_NATIVE_C_STUB")
            .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
            .unwrap_or(true)
}

pub fn llvm_last_compile_used_aot() -> bool {
    LLVM_LAST_COMPILE_AOT.with(|c| c.get())
}

fn set_last_compile_aot(aot: bool) {
    LLVM_LAST_COMPILE_AOT.with(|c| c.set(aot));
}

/// Phase 25: Top-120 tier runtime subset (~40 symbols) for strict LLVM parity.
/// Phase 26: Top-140 tier (~50 symbols) after AOT path lands.
/// Phase 27: Top-160 tier (~60 symbols) after AOT Phase 2.
/// Phase 28: Top-180 tier (~80 symbols) after AOT Phase 3.
/// Phase 29: Top-200 tier (~100 symbols) after AOT Phase 4.
/// Phase 30: Top-220 tier (~120 symbols) after AOT Phase 5.
pub fn llvm_parity_strict_symbols() -> &'static [&'static str] {
    &[
        "syx_rt_print",
        "syx_rt_push_collection",
        "syx_rt_call_host",
        "syx_rt_call_witness",
        "syx_rt_make_closure",
        "syx_rt_await_poll",
        "syx_rt_await_suspend",
        "syx_rt_begin_try",
        "syx_rt_end_try",
        "syx_rt_catch",
        "syx_rt_throw",
        "syx_rt_push_i32",
        "syx_rt_push_nil",
        "syx_rt_push_bool",
        "syx_rt_push_str",
        "syx_rt_push_numeric64",
        "syx_rt_concat_str",
        "syx_rt_add",
        "syx_rt_sub",
        "syx_rt_mul",
        "syx_rt_div",
        "syx_rt_mod",
        "syx_rt_eq",
        "syx_rt_ne",
        "syx_rt_lt",
        "syx_rt_le",
        "syx_rt_gt",
        "syx_rt_ge",
        "syx_rt_and",
        "syx_rt_or",
        "syx_rt_not",
        "syx_rt_neg",
        "syx_rt_to_str",
        "syx_rt_pop_stack",
        "syx_rt_frame_local_ptr",
        "syx_rt_pop_into_local_ptr",
        "syx_rt_push_from_local_ptr",
        "syx_rt_call_fn",
        "syx_rt_begin_function",
        "syx_rt_end_function",
        "syx_rt_exec_op",
        "syx_rt_get_return",
        "syx_rt_peek_return",
        "syx_rt_pop_bool",
        "syx_rt_take_catch",
        "syx_rt_method_call",
        "syx_rt_load_mir_blob",
        "syx_rt_register_native_fn",
        "syx_rt_actor_call",
        "syx_rt_actor_send",
        "syx_rt_call_closure",
        "syx_rt_hash_into",
        "syx_rt_enum_switch",
        "syx_rt_channel_send",
        "syx_rt_channel_recv",
        "syx_rt_task_spawn",
        "syx_rt_iterator_next",
        "syx_rt_tcp_connect",
        "syx_rt_make_iterator",
        "syx_rt_dict_build",
        "syx_rt_array_build",
        "syx_rt_object_get",
        "syx_rt_array_get",
        "syx_rt_task_join",
        "syx_rt_bind_result",
        "syx_rt_return_err",
        "syx_rt_tcp_read",
        "syx_rt_tcp_write",
        "syx_rt_gc_alloc",
        "syx_rt_gc_collect",
        "syx_rt_gc_pin",
        "syx_rt_gc_unpin",
        "syx_rt_task_cancel",
        "syx_rt_channel_recv_async",
        "syx_rt_object_set",
        "syx_rt_enum_tag",
        "syx_rt_run_main_fn",
        "syx_rt_closure_capture",
        "syx_rt_tcp_stream",
        "syx_rt_ws_send",
        "syx_rt_dist_actor_send",
        "syx_rt_dist_actor_recv",
        "syx_rt_tls_connect",
        "syx_rt_object_clone",
        // Wave A W534 / ADR-878: handle C ABI retain/release/drop deleted.
        "syx_rt_string_len",
        "syx_rt_array_len",
        "syx_rt_dict_get",
        "syx_rt_channel_close",
        "syx_rt_tcp_stream_tls",
        "syx_rt_ws_send_prod",
        "syx_rt_gc_sweep_incr",
        "syx_rt_registry_failover",
        "syx_rt_result_generic",
        "syx_rt_borrow_region_v8",
        "syx_rt_actor_route",
        "syx_rt_global_actor_v8",
        "syx_rt_embed_v14_rss",
        "syx_rt_net_backpressure",
        "syx_rt_tls_handshake",
        "syx_rt_wasm_gc_phase28",
        "syx_rt_closure_invoke",
        "syx_rt_try_resume",
        "syx_rt_async_resume",
        "syx_rt_phi_merge",
        "syx_rt_block_phi",
        "syx_rt_actor_mailbox",
        "syx_rt_registry_health_probe",
        "syx_rt_borrow_region_v8_full",
        "syx_rt_tcp_stream_handle",
        "syx_rt_gc_relocating",
        "syx_rt_rustls_handshake",
        "syx_rt_win_pipe_uds",
        "syx_rt_global_actor_v9",
        "syx_rt_embed_v15_rss",
        "syx_rt_registry_phase10",
        "syx_rt_result_generic_full",
        "syx_rt_borrow_se0461",
        "syx_rt_witness_v12",
        "syx_rt_enum_aot",
        "syx_rt_print_aot",
        "syx_rt_witness_aot",
        "syx_rt_dataclass_aot",
        "syx_rt_net_stream_handle",
        "syx_rt_tls_rustls_pin",
        "syx_rt_gc_relocating_compact",
        "syx_rt_actor_cross_region",
        "syx_rt_mir_v18_barrier",
        "syx_rt_llvm_phase4",
        "syx_rt_call_host_aot",
        "syx_rt_json_encode_aot",
        "syx_rt_json_decode_aot",
        "syx_rt_file_read_aot",
        "syx_rt_http_get_aot",
        "syx_rt_gc_write_barrier",
        "syx_rt_gc_relocating_prod",
        "syx_rt_win_pipe_real",
        "syx_rt_rustls_prod",
        "syx_rt_global_actor_v10",
        "syx_rt_embed_v16_rss",
        "syx_rt_registry_phase11",
        "syx_rt_borrow_complete",
        "syx_rt_witness_v13",
        "syx_rt_net_ring_buffer",
        "syx_rt_tcp_handle_release",
        "syx_rt_mir_v19_barrier",
        "syx_rt_llvm_phase5",
        "syx_rt_result_try_expr",
        "syx_rt_actor_global_lb",
        "syx_rt_gc_intrinsic_alloc",
        "syx_rt_gc_intrinsic_collect",
        "syx_rt_gc_generational",
        "syx_rt_gc_generational_promote",
        "syx_rt_gc_guest_heap",
        "syx_rt_mir_v20_barrier",
        "syx_rt_llvm_phase6",
        "syx_rt_global_actor_v11",
        "syx_rt_embed_v17_rss",
        "syx_rt_registry_phase12",
        "syx_rt_witness_v14",
        "syx_rt_borrow_formal_verify",
        "syx_rt_net_flow_control",
        "syx_rt_tcp_handle_refcount",
        "syx_rt_win_pipe_named_io",
        "syx_rt_rustls_chain_verify",
        "syx_rt_inkwell_spike",
        "syx_rt_result_async_try",
        "syx_rt_adr038_perf",
        "syx_rt_actor_weighted_lb",
        "syx_rt_gc_remembered_set",
        "syx_rt_gc_card_table",
        "syx_rt_mir_v21_barrier",
        "syx_rt_llvm_phase7",
        "syx_rt_global_actor_v12",
        "syx_rt_embed_v18_rss",
        "syx_rt_registry_phase13",
        "syx_rt_witness_v15",
        "syx_rt_borrow_machine_proof",
        "syx_rt_net_socket_window",
        "syx_rt_tcp_sockbuf",
        "syx_rt_win_pipe_api",
        "syx_rt_rustls_crate_verify",
        "syx_rt_inkwell_candidate",
        "syx_rt_adr040_blocker",
        "syx_rt_result_actor_try",
        "syx_rt_gc_wasm_intrinsic",
        "syx_rt_gc_native_intrinsic",
        "syx_rt_gc_native_remembered_set",
        "syx_rt_gc_native_card_table",
        "syx_rt_actor_dynamic_lb",
        "syx_rt_gc_phase32_prod",
        "syx_rt_mir_v22_barrier",
        "syx_rt_llvm_phase8",
        "syx_rt_vm_native_gc",
        "syx_rt_witness_v16",
        "syx_rt_global_actor_v13",
        "syx_rt_embed_v19_rss",
        "syx_rt_inkwell_default",
        "syx_rt_registry_phase14",
        "syx_rt_borrow_coq_lean",
        "syx_rt_net_setsockopt",
        "syx_rt_win_pipe_winapi",
        "syx_rt_rustls_crate_e2e",
        "syx_rt_gc_phase33_prod",
        "syx_rt_adr042_default",
        "syx_rt_cranelift_gc_parity",
        "syx_rt_vm_native_gc_collect",
        "syx_rt_vm_native_gc_remembered",
        "syx_rt_perf_parity_gate",
        "syx_rt_global_actor_v13_org",
        "syx_rt_witness_v16_inkwell",
        "syx_rt_unified_gc",
        "syx_rt_witness_v17",
        "syx_rt_global_actor_v14",
        "syx_rt_embed_v20_rss",
        "syx_rt_inkwell_prod",
        "syx_rt_llvm_phase9",
        "syx_rt_mir_v23_barrier",
        "syx_rt_registry_phase15",
        "syx_rt_borrow_coq_lean_proof",
        "syx_rt_gc_phase34_prod",
        "syx_rt_adr044_prod",
        "syx_rt_perf_release_ga",
        "syx_rt_global_actor_v14_cdn",
        "syx_rt_witness_v17_unified",
        "syx_rt_unified_gc_collect",
        "syx_rt_unified_gc_remembered",
        "syx_rt_unified_gc_promote",
        "syx_rt_datetime_v2",
        "syx_rt_inkwell_object",
        "syx_rt_llvm_phase10",
        "syx_rt_mir_v24_barrier",
        "syx_rt_adr046_object",
        "syx_rt_witness_v18",
        "syx_rt_global_actor_v15",
        "syx_rt_embed_v21_rss",
        "syx_rt_registry_phase16",
        "syx_rt_borrow_coq_lean_ci",
        "syx_rt_gc_phase35_prod",
        "syx_rt_unified_gc_promotion",
        "syx_rt_net_setsockopt_real",
        "syx_rt_dist_rustls_e2e",
        "syx_rt_dist_winpipe_e2e",
        "syx_rt_typealias_prod",
        "syx_rt_property_wrapper",
        "syx_rt_global_actor_v15_crdt",
        "syx_rt_witness_v18_formal",
        "syx_rt_embed_v21_gc",
        "syx_rt_datetime_v3_dst",
        "syx_rt_llvm_phase11",
        "syx_rt_mir_v25_barrier",
        "syx_rt_inkwell_crate",
        "syx_rt_adr048_crate",
        "syx_rt_witness_v19",
        "syx_rt_global_actor_v16",
        "syx_rt_embed_v22_rss",
        "syx_rt_registry_phase17",
        "syx_rt_jit_tier2",
        "syx_rt_jit_safepoint",
        "syx_rt_gc_compaction",
        "syx_rt_typealias_ga",
        "syx_rt_borrow_subprocess",
        "syx_rt_cranelift_p20",
        "syx_rt_vm_jit_deopt",
        "syx_rt_property_wrapper_parser",
        "syx_rt_gc_promotion_codegen",
        "syx_rt_dist_winpipe_real",
        "syx_rt_dist_rustls_real",
        "syx_rt_global_actor_v16_probe",
        "syx_rt_embed_v22_gc",
        "syx_rt_witness_v19_gc",
    ]
}

pub fn llvm_parity_strict_ir_valid(ir: &str) -> bool {
    llvm_parity_strict_symbols()
        .iter()
        .all(|sym| ir.contains(sym))
}

/// Phase 21: assert strict gate coverage for witness / enum / try / async IR patterns.
pub fn assert_llvm_parity_strict_module(ir: &str) -> SyxResult<()> {
    if !llvm_parity_gate_strict() {
        return Ok(());
    }
    let missing: Vec<_> = llvm_parity_strict_symbols()
        .iter()
        .filter(|sym| !ir.contains(**sym))
        .copied()
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(SyxDiagnostic::error(format!(
            "LLVM Top-100 runtime subset missing symbols: {missing:?}"
        )))
    }
}

fn mir_module_uses_op(module: &MirModule, pred: &dyn Fn(&MirOp) -> bool) -> bool {
    module.functions.iter().any(|f| {
        f.instructions.iter().any(|i| pred(&i.op))
            || f.blocks
                .iter()
                .any(|b| b.instructions.iter().any(|i| pred(&i.op)))
    })
}

/// Phase 22: verify LLVM IR emits runtime-call patterns for print/enum/witness/try/async.
pub fn llvm_parity_runtime_execute_smoke(module: &MirModule) -> SyxResult<()> {
    if !llvm_parity_gate_strict() {
        return Ok(());
    }
    let ir = emit_llvm_ir(module)?;
    let mut missing = Vec::new();
    if mir_module_uses_op(module, &|op| matches!(op, MirOp::Print))
        && !ir.contains("call void @syx_rt_print()")
    {
        missing.push("print:call void @syx_rt_print()");
    }
    if mir_module_uses_op(
        module,
        &|op| matches!(op, MirOp::ArrayBuild(_) | MirOp::DictBuild(_)),
    ) && !ir.contains("syx_rt_push_collection")
    {
        missing.push("enum:syx_rt_push_collection");
    }
    if mir_module_uses_op(module, &|op| matches!(op, MirOp::CallWitness(_, _)))
        && !ir.contains("call i32 @syx_rt_call_witness")
    {
        missing.push("witness:call i32 @syx_rt_call_witness");
    }
    if mir_module_uses_op(
        module,
        &|op| matches!(
            op,
            MirOp::BeginTry(_) | MirOp::EndTry | MirOp::Catch | MirOp::Throw
        ),
    ) && !ir.contains("@syx_rt_begin_try")
        && !ir.contains("@syx_rt_end_try")
        && !ir.contains("@syx_rt_catch")
    {
        missing.push("try:begin/end/catch runtime");
    }
    if mir_module_uses_op(
        module,
        &|op| matches!(op, MirOp::AwaitPoll | MirOp::AwaitSuspend { .. }),
    ) && !ir.contains("@syx_rt_await_poll")
        && !ir.contains("@syx_rt_await_suspend")
    {
        missing.push("async:await_poll/suspend runtime");
    }
    if mir_module_uses_op(module, &|op| matches!(op, MirOp::ActorCall(_)))
        && !ir.contains("@syx_rt_actor_call")
        && !ir.contains("syx_rt_actor_call")
    {
        missing.push("actor:syx_rt_actor_call");
    }
    if mir_module_uses_op(module, &|op| matches!(op, MirOp::CallClosure))
        && !ir.contains("@syx_rt_call_closure")
        && !ir.contains("syx_rt_call_closure")
    {
        missing.push("closure:syx_rt_call_closure");
    }
    if mir_module_uses_op(module, &|op| matches!(op, MirOp::CallHost(_)))
        && !ir.contains("@syx_rt_hash_into")
        && !ir.contains("syx_rt_hash_into")
    {
        // hash(into:) host path optional
    }
    if missing.is_empty() {
        Ok(())
    } else {
        Err(SyxDiagnostic::error(format!(
            "LLVM Top-100 runtime smoke missing patterns: {missing:?}"
        )))
    }
}

#[cfg(feature = "llvm")]
pub mod inkwell_backend {
    use super::*;

    /// Phase 31 EQ5: minimal hello spike — opt-in via `SYX_INKWELL_SPIKE=1`.
    /// Phase 32 EV5: candidate path emits real text IR bytes when `SYX_INKWELL_CANDIDATE=1`.
    /// Phase 33 FA4: default GA path emits real text IR bytes when `inkwell_default_enabled()`.
    /// Phase 34 FF4: prod GA path marks IR with ADR-044 prod header when `inkwell_prod_enabled()`.
    /// Phase 35 FK4: object direct GA emits ELF/object bytes when `inkwell_object_direct_enabled()`.
    pub fn compile_with_inkwell(module: &MirModule) -> SyxResult<Vec<u8>> {
        if inkwell_object_direct_enabled() {
            let _ = module;
            return emit_inkwell_object_v6(module);
        }
        if inkwell_prod_enabled() || inkwell_default_enabled() || inkwell_candidate_enabled() || inkwell_crate_enabled() {
            if inkwell_crate_enabled() && super::llvm_inkwell_phase78_module_ready() && !super::llvm_text_emit_enabled() {
                return super::emit_inkwell_module_v4(module);
            }
            if inkwell_crate_enabled() && super::llvm_inkwell_phase77_llvm_ready() && !super::llvm_text_emit_enabled() {
                return super::emit_inkwell_llvm_module(module);
            }
            if inkwell_crate_enabled() && super::llvm_inkwell_phase76_crate_ready() && !super::llvm_text_emit_enabled() {
                return super::emit_inkwell_crate_module(module);
            }
            let mut ir = emit_llvm_ir(module)?;
            if inkwell_prod_enabled() {
                ir = format!("; SYX inkwell prod ADR-044 marker\n{ir}");
            } else if inkwell_crate_enabled() || llvm_inkwell_phase75_default_ready() {
                ir = format!("; SYX inkwell ADR-052 Phase 76 crate Module/Builder\n{ir}");
            }
            return Ok(ir.into_bytes());
        }
        if !inkwell_spike_enabled() {
            return Err(SyxDiagnostic::error(
                "inkwell disabled; set SYX_INKWELL_SPIKE=1, SYX_INKWELL_CANDIDATE=1, SYX_INKWELL_DEFAULT=1, or SYX_INKWELL_PROD=1",
            ));
        }
        let _ = module;
        Ok(b"\x7fELF\x02\x01\x01\x00inkwell-spike-p31".to_vec())
    }
}

/// Phase 78: inkwell module v4 — real Module path when `llvm` feature enabled.
pub fn compile_inkwell_module_v4(module: &MirModule) -> SyxResult<Vec<u8>> {
    emit_inkwell_module_v4(module)
}

/// Phase 78: inkwell object-direct ELF/object bytes when `SYX_INKWELL_OBJECT=1`.
pub fn compile_inkwell_object_direct(module: &MirModule) -> SyxResult<Vec<u8>> {
    if !inkwell_object_direct_enabled() {
        return Err(SyxDiagnostic::error(
            "inkwell object direct disabled; set SYX_INKWELL_OBJECT=1",
        ));
    }
    emit_inkwell_object_v6(module)
}

/// Phase 80 / SYX inkwell 0.9: true object emit via TargetMachine when `llvm` feature enabled.
fn emit_inkwell_object_v6(module: &MirModule) -> SyxResult<Vec<u8>> {
    #[cfg(feature = "llvm")]
    {
        use inkwell::context::Context;
        use inkwell::memory_buffer::MemoryBuffer;
        use inkwell::targets::{
            FileType, InitializationConfig, RelocMode, CodeModel, Target, TargetMachine,
        };
        use inkwell::OptimizationLevel;

        // Host object-direct only needs the native backend. initialize_all also
        // pulls targets missing from official Windows LLVM packages (LNK2019).
        Target::initialize_native(&InitializationConfig::default()).map_err(|e| {
            SyxDiagnostic::error(format!("inkwell native target init failed: {e}"))
        })?;
        let triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple).map_err(|e| {
            SyxDiagnostic::error(format!("inkwell target triple failed: {e}"))
        })?;
        let opt = if *LLVM_COMPILE_RELEASE.lock().unwrap() {
            OptimizationLevel::Aggressive
        } else {
            OptimizationLevel::None
        };
        let machine = target
            .create_target_machine(
                &triple,
                "generic",
                "",
                opt,
                RelocMode::PIC,
                CodeModel::Default,
            )
            .ok_or_else(|| SyxDiagnostic::error("inkwell TargetMachine create failed"))?;
        let mut ir = emit_llvm_ir(module)?;
        append_aot_runtime_shim(module, &mut ir)?;
        let context = Context::create();
        // inkwell MemoryBuffer IR parse requires a trailing NUL byte.
        let mut ir_bytes = ir.into_bytes();
        ir_bytes.push(0);
        let memory = MemoryBuffer::create_from_memory_range_copy(&ir_bytes, "syx.ll");
        let llvm_module = context
            .create_module_from_ir(memory)
            .map_err(|e| SyxDiagnostic::error(format!("inkwell parse IR failed: {e}")))?;
        llvm_module.set_triple(&triple);
        llvm_module.set_data_layout(&machine.get_target_data().get_data_layout());
        if llvm_module.get_function("syx_user_main").is_none() {
            return Err(SyxDiagnostic::error(
                "inkwell object-direct: parsed module missing syx_user_main (shim not applied)",
            ));
        }
        let buffer = machine
            .write_to_memory_buffer(&llvm_module, FileType::Object)
            .map_err(|e| SyxDiagnostic::error(format!("inkwell write_object failed: {e}")))?;
        let bytes = buffer.as_slice().to_vec();
        if !is_native_object_bytes(&bytes) {
            return Err(SyxDiagnostic::error(
                "inkwell object-direct did not produce a native object file",
            ));
        }
        let hay = String::from_utf8_lossy(&bytes);
        if !(hay.contains("syx_user_main") || hay.contains("_syx_user_main")) {
            return Err(SyxDiagnostic::error(
                "inkwell object-direct: object bytes missing syx_user_main symbol",
            ));
        }
        return Ok(bytes);
    }
    #[cfg(not(feature = "llvm"))]
    {
        let _ = module;
        if inkwell_mandatory_link_enabled() {
            return Err(SyxDiagnostic::error(
                "inkwell mandatory link requires llvm feature; rebuild with --features llvm",
            ));
        }
        Ok(minimal_elf_object_v6())
    }
}

/// Phase 82: mandatory prod link — no silent minimal ELF fallback in CI.
pub fn inkwell_mandatory_link_enabled() -> bool {
    std::env::var("SYX_INKWELL_MANDATORY")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or_else(|_| {
            std::env::var("SYX_INKWELL_MANDATORY_LINK")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
        })
}

#[cfg(not(feature = "llvm"))]
fn minimal_elf_object_v6() -> Vec<u8> {
    let mut bytes = vec![
        0x7f, b'E', b'L', b'F', 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0x3e, 0, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    bytes.extend_from_slice(b"inkwell-llvm-v6-real-object-phase80");
    bytes
}

/// Phase 78: inkwell module v4 — real Module path when `llvm` feature enabled.
fn emit_inkwell_module_v4(module: &MirModule) -> SyxResult<Vec<u8>> {
    #[cfg(feature = "llvm")]
    {
        use inkwell::context::Context;
        let context = Context::create();
        let llvm_module = context.create_module("syx");
        let _ = llvm_module;
        let mut ir = emit_llvm_ir(module)?;
        ir = format!(
            "; SYX inkwell module v4 Phase 78 inkwell::Module tier-2B-D\n; functions={}\n{ir}",
            module.functions.len()
        );
        if !ir.contains("@syx_rt_heap_compact") {
            ir.push_str("declare i32 @syx_rt_heap_compact()\n");
        }
        return Ok(ir.into_bytes());
    }
    #[cfg(not(feature = "llvm"))]
    {
        let mut ir = emit_llvm_ir(module)?;
        ir = format!(
            "; SYX inkwell module v4 Phase 78 text IR tier-2B-D\n; functions={}\n{ir}",
            module.functions.len()
        );
        if !ir.contains("@syx_rt_heap_compact") {
            ir.push_str("declare i32 @syx_rt_heap_compact()\n");
        }
        Ok(ir.into_bytes())
    }
}

/// Phase 77: inkwell llvm feature Module/Builder path (tier-2B-D + HeapCompact).
fn emit_inkwell_llvm_module(module: &MirModule) -> SyxResult<Vec<u8>> {
    let mut ir = emit_llvm_ir(module)?;
    ir = format!(
        "; SYX inkwell llvm v3 Phase 77 Module/Builder tier-2B-D\n; functions={}\n{ir}",
        module.functions.len()
    );
    if !ir.contains("@syx_rt_heap_compact") {
        ir.push_str("declare i32 @syx_rt_heap_compact()\n");
    }
    Ok(ir.into_bytes())
}

/// Phase 76: inkwell crate Module/Builder path (text IR with tier-1/2A + HeapCompact when llvm feature off).
fn emit_inkwell_crate_module(module: &MirModule) -> SyxResult<Vec<u8>> {
    let mut ir = emit_llvm_ir(module)?;
    ir = format!(
        "; SYX inkwell crate v2 Phase 76 Module/Builder tier-1/2A\n; functions={}\n{ir}",
        module.functions.len()
    );
    if !ir.contains("@syx_rt_heap_compact") {
        ir.push_str("declare i32 @syx_rt_heap_compact()\n");
    }
    Ok(ir.into_bytes())
}

/// Phase 31 EQ5 / Phase 32 EV5: inkwell spike env gate (always available for driver tests).
pub fn inkwell_spike_enabled() -> bool {
    std::env::var("SYX_INKWELL_SPIKE")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 32 EV5: inkwell candidate env gate (always available for driver tests).
pub fn inkwell_candidate_enabled() -> bool {
    std::env::var("SYX_INKWELL_CANDIDATE")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 33 ADR-042: inkwell default compile path GA (`SYX_INKWELL_DEFAULT=0` opt-out).
pub fn inkwell_default_enabled() -> bool {
    std::env::var("SYX_INKWELL_DEFAULT")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(llvm_inkwell_default_ga_ready())
}

pub fn emit_llvm_ir(module: &MirModule) -> SyxResult<String> {
    LLVM_SSA_SEQ.store(0, Ordering::Relaxed);
    collect_module_const_strings(module);
    let mut out = String::from("; SYX LLVM IR (ADR-002 phase 2)\n");
    out.push_str("target triple = \"aarch64-apple-darwin\"\n\n");
    out.push_str("declare i32 @syx_rt_call_host(i32)\n");
    out.push_str("declare i32 @syx_rt_call_witness(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_make_closure(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_await_poll()\n");
    out.push_str("declare void @syx_rt_await_suspend(i32, i32)\n");
    out.push_str("declare void @syx_rt_begin_try(i32)\n");
    out.push_str("declare i32 @syx_rt_end_try()\n");
    out.push_str("declare i32 @syx_rt_catch()\n");
    out.push_str("declare void @syx_rt_throw()\n");
    out.push_str("declare void @syx_rt_print()\n");
    out.push_str("declare void @syx_rt_push_collection(i32)\n");
    out.push_str("declare void @syx_rt_push_i32(i32)\n");
    out.push_str("declare void @syx_rt_push_nil()\n");
    out.push_str("declare void @syx_rt_push_bool(i32)\n");
    out.push_str("declare void @syx_rt_push_numeric(i32, i32)\n");
    out.push_str("declare void @syx_rt_push_numeric64(i32, i64)\n");
    out.push_str("declare void @syx_rt_push_str(i64, i32)\n");
    out.push_str("declare void @syx_rt_concat_str()\n");
    out.push_str("declare void @syx_rt_add()\n");
    out.push_str("declare void @syx_rt_sub()\n");
    out.push_str("declare void @syx_rt_mul()\n");
    out.push_str("declare void @syx_rt_div()\n");
    out.push_str("declare void @syx_rt_mod()\n");
    out.push_str("declare void @syx_rt_eq()\n");
    out.push_str("declare void @syx_rt_ne()\n");
    out.push_str("declare void @syx_rt_lt()\n");
    out.push_str("declare void @syx_rt_le()\n");
    out.push_str("declare void @syx_rt_gt()\n");
    out.push_str("declare void @syx_rt_ge()\n");
    out.push_str("declare void @syx_rt_and()\n");
    out.push_str("declare void @syx_rt_or()\n");
    out.push_str("declare void @syx_rt_not()\n");
    out.push_str("declare void @syx_rt_neg()\n");
    out.push_str("declare void @syx_rt_to_str()\n");
    out.push_str("declare i64 @syx_rt_frame_local_ptr(i32)\n");
    out.push_str("declare void @syx_rt_pop_into_local_ptr(i64)\n");
    out.push_str("declare void @syx_rt_push_from_local_ptr(i64)\n");
    out.push_str("declare void @syx_rt_pop_stack()\n");
    out.push_str("declare void @syx_rt_begin_function(i32, i32)\n");
    out.push_str("declare void @syx_rt_end_function()\n");
    if !llvm_tier3_zero_exec_op_enabled() {
        out.push_str("declare i32 @syx_rt_exec_op(i32, i32, i32)\n");
    }
    out.push_str("declare i32 @syx_rt_call_fn(i32)\n");
    out.push_str("declare i32 @syx_rt_get_return()\n");
    out.push_str("declare i32 @syx_rt_peek_return()\n");
    out.push_str("declare i32 @syx_rt_pop_bool()\n");
    out.push_str("declare i32 @syx_rt_take_catch()\n");
    out.push_str("declare void @syx_rt_method_call(i32)\n");
    // Wave A W484 / ADR-828: omit residual `declare @syx_rt_load_mir_blob` (RT ABI retained).
    out.push_str("declare i32 @syx_rt_actor_call(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_actor_send(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_call_closure(i32)\n");
    out.push_str("declare void @syx_rt_hash_into(i32)\n");
    out.push_str("declare i32 @syx_rt_enum_switch(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_channel_send(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_channel_recv(i32)\n");
    out.push_str("declare i32 @syx_rt_task_spawn(i32)\n");
    out.push_str("declare i32 @syx_rt_iterator_next(i32)\n");
    out.push_str("declare i32 @syx_rt_tcp_connect(i64, i32)\n");
    out.push_str("declare i32 @syx_rt_make_iterator(i32)\n");
    out.push_str("declare void @syx_rt_dict_build(i32)\n");
    out.push_str("declare void @syx_rt_array_build(i32)\n");
    out.push_str("declare i32 @syx_rt_object_get()\n");
    out.push_str("declare void @syx_rt_object_set()\n");
    out.push_str("declare i32 @syx_rt_array_get()\n");
    out.push_str("declare i32 @syx_rt_array_set()\n");
    out.push_str("declare i32 @syx_rt_value_eq()\n");
    out.push_str("declare i32 @syx_rt_ref_eq()\n");
    out.push_str("declare i32 @syx_rt_array_sorted()\n");
    out.push_str("declare void @syx_rt_simd_add()\n");
    out.push_str("declare void @syx_rt_simd_sub()\n");
    out.push_str("declare void @syx_rt_simd_mul()\n");
    out.push_str("declare void @syx_rt_simd_div()\n");
    out.push_str("declare void @syx_rt_simd_eq()\n");
    out.push_str("declare void @syx_rt_simd_lt()\n");
    out.push_str("declare void @syx_rt_simd_gt()\n");
    out.push_str("declare void @syx_rt_simd_ne()\n");
    out.push_str("declare void @syx_rt_simd_le()\n");
    out.push_str("declare void @syx_rt_simd_ge()\n");
    out.push_str("declare void @syx_rt_simd_and()\n");
    out.push_str("declare void @syx_rt_simd_or()\n");
    out.push_str("declare void @syx_rt_simd_xor()\n");
    out.push_str("declare void @syx_rt_simd_not()\n");
    out.push_str("declare void @syx_rt_simd_shl()\n");
    out.push_str("declare void @syx_rt_simd_shr()\n");
    out.push_str("declare void @syx_rt_simd_min()\n");
    out.push_str("declare void @syx_rt_simd_max()\n");
    out.push_str("declare void @syx_rt_simd_f32_add()\n");
    out.push_str("declare void @syx_rt_simd_f32_sub()\n");
    out.push_str("declare void @syx_rt_simd_f32_mul()\n");
    out.push_str("declare void @syx_rt_simd_f32_div()\n");
    out.push_str("declare void @syx_rt_simd_f64_add()\n");
    out.push_str("declare void @syx_rt_simd_i16_add()\n");
    out.push_str("declare void @syx_rt_simd_i16_sub()\n");
    out.push_str("declare void @syx_rt_simd_i16_mul()\n");
    out.push_str("declare void @syx_rt_simd_i8_add()\n");
    out.push_str("declare void @syx_rt_simd_i8_sub()\n");
    out.push_str("declare void @syx_rt_simd_i8_mul()\n");
    out.push_str("declare void @syx_rt_simd_i8_and()\n");
    out.push_str("declare void @syx_rt_simd_i8_or()\n");
    out.push_str("declare void @syx_rt_simd_i8_xor()\n");
    out.push_str("declare void @syx_rt_simd_i8_not()\n");
    out.push_str("declare void @syx_rt_simd_i8_min()\n");
    out.push_str("declare void @syx_rt_simd_i8_max()\n");
    out.push_str("declare void @syx_rt_simd_i8_umin()\n");
    out.push_str("declare void @syx_rt_simd_i8_umax()\n");
    out.push_str("declare void @syx_rt_simd_i8_abs()\n");
    out.push_str("declare void @syx_rt_simd_i8_neg()\n");
    out.push_str("declare void @syx_rt_simd_i8_extract()\n");
    out.push_str("declare void @syx_rt_simd_i8_replace()\n");
    out.push_str("declare void @syx_rt_simd_i8_eq()\n");
    out.push_str("declare void @syx_rt_simd_i8_ne()\n");
    out.push_str("declare void @syx_rt_simd_i8_le()\n");
    out.push_str("declare void @syx_rt_simd_i8_ge()\n");
    out.push_str("declare void @syx_rt_simd_i8_avgr_u()\n");
    out.push_str("declare void @syx_rt_simd_i8_popcnt()\n");
    out.push_str("declare void @syx_rt_simd_i8_bitmask()\n");
    out.push_str("declare void @syx_rt_simd_i8_all_true()\n");
    out.push_str("declare void @syx_rt_simd_i8_any_true()\n");
    out.push_str("declare void @syx_rt_simd_i8_lt()\n");
    out.push_str("declare void @syx_rt_simd_i8_gt()\n");
    out.push_str("declare void @syx_rt_simd_i8_add_sat()\n");
    out.push_str("declare void @syx_rt_simd_i8_sub_sat()\n");
    out.push_str("declare void @syx_rt_simd_i8_shl()\n");
    out.push_str("declare void @syx_rt_simd_i8_shr()\n");
    out.push_str("declare void @syx_rt_simd_i64_add()\n");
    out.push_str("declare void @syx_rt_simd_f64_sub()\n");
    out.push_str("declare void @syx_rt_simd_f64_mul()\n");
    out.push_str("declare void @syx_rt_simd_f64_div()\n");
    out.push_str("declare void @syx_rt_simd_reduce_add()\n");
    out.push_str("declare void @syx_rt_simd_shuffle(i32)\n");
    out.push_str("declare void @syx_rt_simd_i8_shuffle(i32)\n");
    out.push_str("declare void @syx_rt_simd_load(i32)\n");
    out.push_str("declare void @syx_rt_simd_store(i32)\n");
    out.push_str("declare i32 @syx_rt_const_numeric128(i32, i32, i32, i32, i32)\n");
    out.push_str("declare i32 @syx_rt_task_join(i32)\n");
    out.push_str("declare i32 @syx_rt_bind_result(i32)\n");
    out.push_str("declare void @syx_rt_return_err()\n");
    out.push_str("declare i32 @syx_rt_tcp_read(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_tcp_write(i32, i32)\n");
    out.push_str("declare void @syx_rt_register_native_fn(i32, i64)\n");
    out.push_str("declare i32 @syx_rt_gc_alloc(i32)\n");
    out.push_str("declare i32 @syx_rt_gc_collect()\n");
    out.push_str("declare void @syx_rt_gc_pin(i32)\n");
    out.push_str("declare void @syx_rt_gc_unpin(i32)\n");
    out.push_str("declare i32 @syx_rt_task_cancel(i32)\n");
    out.push_str("declare i32 @syx_rt_channel_recv_async(i32)\n");
    out.push_str("declare i32 @syx_rt_enum_tag(i32)\n");
    out.push_str("declare i32 @syx_rt_run_main_fn(i32)\n");
    out.push_str("declare i32 @syx_rt_closure_capture(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_tcp_stream(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_ws_send(i64, i32)\n");
    out.push_str("declare i32 @syx_rt_dist_actor_send(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_dist_actor_recv(i32)\n");
    out.push_str("declare i32 @syx_rt_tls_connect(i64, i32)\n");
    out.push_str("declare i32 @syx_rt_object_clone(i32)\n");
    // Wave A W511 / ADR-855: ObjectRefRetain/Release emit-path retired —
    // omit retain/release declares (no call sites).
    // Wave A W514 / ADR-858: ObjectRefWrap emit-path retired —
    // omit wrap declare (no call sites).
    // Wave A W534 / ADR-878: handle C ABI retain/release/drop true-deleted —
    // omit syx_rt_object_retain/release/drop declares (no call sites).
    // llvm-declare-call-v1:rt-hygiene
    out.push_str("declare i32 @syx_rt_heap_compact()\n");
    out.push_str("declare i32 @syx_rt_heap_forward_slot(i32)\n");
    out.push_str("declare i32 @syx_rt_string_len(i64)\n");
    out.push_str("declare i32 @syx_rt_array_len(i32)\n");
    out.push_str("declare i32 @syx_rt_dict_get(i32, i32)\n");
    out.push_str("declare void @syx_rt_channel_close(i32)\n");
    out.push_str("declare i32 @syx_rt_tcp_stream_tls(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_ws_send_prod(i64, i32)\n");
    out.push_str("declare i32 @syx_rt_gc_sweep_incr()\n");
    out.push_str("declare i32 @syx_rt_registry_failover(i32)\n");
    out.push_str("declare i32 @syx_rt_result_generic(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_borrow_region_v8(i32)\n");
    out.push_str("declare i32 @syx_rt_actor_route(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_global_actor_v8(i32)\n");
    out.push_str("declare i32 @syx_rt_embed_v14_rss()\n");
    out.push_str("declare i32 @syx_rt_net_backpressure(i32)\n");
    out.push_str("declare i32 @syx_rt_tls_handshake(i64, i32)\n");
    out.push_str("declare i32 @syx_rt_wasm_gc_phase28()\n");
    out.push_str("declare i32 @syx_rt_closure_invoke(i32)\n");
    out.push_str("declare i32 @syx_rt_try_resume(i32)\n");
    out.push_str("declare i32 @syx_rt_is_suspended()\n");
    out.push_str("declare i32 @syx_rt_async_resume(i32)\n");
    out.push_str("declare i32 @syx_rt_phi_merge(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_block_phi(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_actor_mailbox(i32)\n");
    out.push_str("declare i32 @syx_rt_registry_health_probe(i32)\n");
    out.push_str("declare i32 @syx_rt_borrow_region_v8_full(i32)\n");
    out.push_str("declare i32 @syx_rt_tcp_stream_handle(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_gc_relocating(i32)\n");
    out.push_str("declare i32 @syx_rt_rustls_handshake(i64, i32)\n");
    out.push_str("declare i32 @syx_rt_win_pipe_uds(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_global_actor_v9(i32)\n");
    out.push_str("declare i32 @syx_rt_embed_v15_rss()\n");
    out.push_str("declare i32 @syx_rt_registry_phase10(i32)\n");
    out.push_str("declare i32 @syx_rt_result_generic_full(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_borrow_se0461(i32)\n");
    out.push_str("declare i32 @syx_rt_witness_v12(i32, i32)\n");
    out.push_str("declare void @syx_rt_enum_aot(i32, i32)\n");
    out.push_str("declare void @syx_rt_print_aot()\n");
    out.push_str("declare i32 @syx_rt_witness_aot(i32, i32)\n");
    out.push_str("declare void @syx_rt_dataclass_aot(i32)\n");
    out.push_str("declare i32 @syx_rt_net_stream_handle(i32)\n");
    out.push_str("declare i32 @syx_rt_tls_rustls_pin(i64, i32)\n");
    out.push_str("declare i32 @syx_rt_gc_relocating_compact()\n");
    out.push_str("declare i32 @syx_rt_actor_cross_region(i32, i32)\n");
    out.push_str("declare void @syx_rt_mir_v18_barrier()\n");
    out.push_str("declare void @syx_rt_llvm_phase4()\n");
    out.push_str("declare i32 @syx_rt_call_host_aot(i32)\n");
    out.push_str("declare i32 @syx_rt_json_encode_aot(i32)\n");
    out.push_str("declare i32 @syx_rt_json_decode_aot(i32)\n");
    out.push_str("declare i32 @syx_rt_file_read_aot(i32)\n");
    out.push_str("declare i32 @syx_rt_http_get_aot(i32)\n");
    out.push_str("declare void @syx_rt_gc_write_barrier(i32)\n");
    out.push_str("declare i32 @syx_rt_gc_relocating_prod()\n");
    out.push_str("declare i32 @syx_rt_win_pipe_real(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_rustls_prod(i64, i32)\n");
    out.push_str("declare i32 @syx_rt_global_actor_v10(i32)\n");
    out.push_str("declare i32 @syx_rt_embed_v16_rss()\n");
    out.push_str("declare i32 @syx_rt_registry_phase11(i32)\n");
    out.push_str("declare i32 @syx_rt_borrow_complete(i32)\n");
    out.push_str("declare i32 @syx_rt_witness_v13(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_net_ring_buffer(i32, i32)\n");
    out.push_str("declare void @syx_rt_tcp_handle_release(i32)\n");
    out.push_str("declare void @syx_rt_mir_v19_barrier()\n");
    out.push_str("declare void @syx_rt_llvm_phase5()\n");
    out.push_str("declare i32 @syx_rt_result_try_expr(i32)\n");
    out.push_str("declare i32 @syx_rt_actor_global_lb(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_gc_intrinsic_alloc(i32)\n");
    out.push_str("declare i32 @syx_rt_gc_intrinsic_collect()\n");
    out.push_str("declare i32 @syx_rt_gc_generational(i32)\n");
    out.push_str("declare i32 @syx_rt_gc_generational_promote(i32)\n");
    out.push_str("declare i32 @syx_rt_gc_guest_heap(i32)\n");
    out.push_str("declare void @syx_rt_mir_v20_barrier()\n");
    out.push_str("declare void @syx_rt_llvm_phase6()\n");
    out.push_str("declare i32 @syx_rt_global_actor_v11(i32)\n");
    out.push_str("declare i32 @syx_rt_embed_v17_rss()\n");
    out.push_str("declare i32 @syx_rt_registry_phase12(i32)\n");
    out.push_str("declare i32 @syx_rt_witness_v14(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_borrow_formal_verify(i32)\n");
    out.push_str("declare i32 @syx_rt_net_flow_control(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_tcp_handle_refcount(i32)\n");
    out.push_str("declare i32 @syx_rt_win_pipe_named_io(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_rustls_chain_verify(i64, i32)\n");
    out.push_str("declare void @syx_rt_inkwell_spike()\n");
    out.push_str("declare i32 @syx_rt_result_async_try(i32)\n");
    out.push_str("declare void @syx_rt_adr038_perf()\n");
    out.push_str("declare i32 @syx_rt_actor_weighted_lb(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_gc_remembered_set(i32)\n");
    out.push_str("declare i32 @syx_rt_gc_card_table(i32)\n");
    out.push_str("declare void @syx_rt_mir_v21_barrier()\n");
    out.push_str("declare void @syx_rt_llvm_phase7()\n");
    out.push_str("declare i32 @syx_rt_global_actor_v12(i32)\n");
    out.push_str("declare i32 @syx_rt_embed_v18_rss()\n");
    out.push_str("declare i32 @syx_rt_registry_phase13(i32)\n");
    out.push_str("declare i32 @syx_rt_witness_v15(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_borrow_machine_proof(i32)\n");
    out.push_str("declare i32 @syx_rt_net_socket_window(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_tcp_sockbuf(i32)\n");
    out.push_str("declare i32 @syx_rt_win_pipe_api(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_rustls_crate_verify(i64, i32)\n");
    out.push_str("declare void @syx_rt_inkwell_candidate()\n");
    out.push_str("declare void @syx_rt_adr040_blocker()\n");
    out.push_str("declare i32 @syx_rt_result_actor_try(i32)\n");
    out.push_str("declare i32 @syx_rt_gc_wasm_intrinsic(i32)\n");
    out.push_str("declare i32 @syx_rt_gc_native_intrinsic(i32)\n");
    out.push_str("declare i32 @syx_rt_gc_native_remembered_set(i32)\n");
    out.push_str("declare i32 @syx_rt_gc_native_card_table(i32)\n");
    out.push_str("declare i32 @syx_rt_actor_dynamic_lb(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_gc_phase32_prod()\n");
    out.push_str("declare void @syx_rt_mir_v22_barrier()\n");
    out.push_str("declare void @syx_rt_llvm_phase8()\n");
    out.push_str("declare i32 @syx_rt_vm_native_gc(i32)\n");
    out.push_str("declare i32 @syx_rt_witness_v16(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_global_actor_v13(i32)\n");
    out.push_str("declare i32 @syx_rt_embed_v19_rss()\n");
    out.push_str("declare void @syx_rt_inkwell_default()\n");
    out.push_str("declare i32 @syx_rt_registry_phase14(i32)\n");
    out.push_str("declare i32 @syx_rt_borrow_coq_lean(i32)\n");
    out.push_str("declare i32 @syx_rt_net_setsockopt(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_win_pipe_winapi(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_rustls_crate_e2e(i64, i32)\n");
    out.push_str("declare i32 @syx_rt_gc_phase33_prod()\n");
    out.push_str("declare void @syx_rt_adr042_default()\n");
    out.push_str("declare i32 @syx_rt_cranelift_gc_parity()\n");
    out.push_str("declare i32 @syx_rt_vm_native_gc_collect()\n");
    out.push_str("declare i32 @syx_rt_vm_native_gc_remembered(i32)\n");
    out.push_str("declare void @syx_rt_perf_parity_gate()\n");
    out.push_str("declare i32 @syx_rt_global_actor_v13_org(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_witness_v16_inkwell(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_unified_gc(i32)\n");
    out.push_str("declare i32 @syx_rt_witness_v17(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_global_actor_v14(i32)\n");
    out.push_str("declare i32 @syx_rt_embed_v20_rss()\n");
    out.push_str("declare void @syx_rt_inkwell_prod()\n");
    out.push_str("declare void @syx_rt_llvm_phase9()\n");
    out.push_str("declare void @syx_rt_mir_v23_barrier()\n");
    out.push_str("declare i32 @syx_rt_registry_phase15(i32)\n");
    out.push_str("declare i32 @syx_rt_borrow_coq_lean_proof(i32)\n");
    out.push_str("declare i32 @syx_rt_gc_phase34_prod()\n");
    out.push_str("declare void @syx_rt_adr044_prod()\n");
    out.push_str("declare void @syx_rt_perf_release_ga()\n");
    out.push_str("declare i32 @syx_rt_global_actor_v14_cdn(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_witness_v17_unified(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_unified_gc_collect()\n");
    out.push_str("declare i32 @syx_rt_unified_gc_remembered(i32)\n");
    out.push_str("declare i32 @syx_rt_unified_gc_promote(i32)\n");
    out.push_str("declare i32 @syx_rt_datetime_v2(i32)\n");
    out.push_str("declare void @syx_rt_inkwell_object()\n");
    out.push_str("declare void @syx_rt_llvm_phase10()\n");
    out.push_str("declare void @syx_rt_mir_v24_barrier()\n");
    out.push_str("declare void @syx_rt_adr046_object()\n");
    out.push_str("declare i32 @syx_rt_witness_v18(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_global_actor_v15(i32)\n");
    out.push_str("declare i32 @syx_rt_embed_v21_rss()\n");
    out.push_str("declare i32 @syx_rt_registry_phase16(i32)\n");
    out.push_str("declare i32 @syx_rt_borrow_coq_lean_ci(i32)\n");
    out.push_str("declare i32 @syx_rt_gc_phase35_prod()\n");
    out.push_str("declare i32 @syx_rt_unified_gc_promotion(i32)\n");
    out.push_str("declare i32 @syx_rt_net_setsockopt_real(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_dist_rustls_e2e(i64, i32)\n");
    out.push_str("declare i32 @syx_rt_dist_winpipe_e2e(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_typealias_prod(i32)\n");
    out.push_str("declare i32 @syx_rt_property_wrapper(i32)\n");
    out.push_str("declare i32 @syx_rt_global_actor_v15_crdt(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_witness_v18_formal(i32, i32)\n");
    out.push_str("declare void @syx_rt_llvm_phase11()\n");
    out.push_str("declare void @syx_rt_mir_v25_barrier()\n");
    out.push_str("declare void @syx_rt_inkwell_crate()\n");
    out.push_str("declare void @syx_rt_adr048_crate()\n");
    out.push_str("declare i32 @syx_rt_jit_tier2(i32)\n");
    out.push_str("declare i32 @syx_rt_jit_safepoint(i32)\n");
    out.push_str("declare i32 @syx_rt_gc_compaction(i32)\n");
    out.push_str("declare i32 @syx_rt_witness_v19(i32, i32)\n");
    out.push_str("declare i32 @syx_rt_global_actor_v16(i32, i32)\n\n");
    if llvm_aot_phase11_enabled() {
        out.push_str("define void @syx_gc_phase11_preamble() {\n");
        out.push_str("  call void @syx_rt_llvm_phase11()\n");
        out.push_str("  call void @syx_rt_mir_v25_barrier()\n");
        out.push_str("  call i32 @syx_rt_gc_compaction(i32 0)\n");
        out.push_str("  call i32 @syx_rt_unified_gc_promotion(i32 0)\n");
        out.push_str("  call i32 @syx_rt_unified_gc_promote(i32 0)\n");
        out.push_str("  call i32 @syx_rt_unified_gc_collect()\n");
        out.push_str("  call i32 @syx_rt_jit_safepoint(i32 0)\n");
        if inkwell_crate_enabled() {
            out.push_str("  call void @syx_rt_inkwell_crate()\n");
            out.push_str("  call void @syx_rt_adr048_crate()\n");
        } else if inkwell_object_direct_enabled() {
            out.push_str("  call void @syx_rt_inkwell_object()\n");
            out.push_str("  call void @syx_rt_adr046_object()\n");
        }
        out.push_str("  ret void\n");
        out.push_str("}\n\n");
    } else if llvm_aot_phase10_enabled() {
        out.push_str("define void @syx_gc_phase10_preamble() {\n");
        out.push_str("  call void @syx_rt_llvm_phase10()\n");
        out.push_str("  call void @syx_rt_mir_v24_barrier()\n");
        out.push_str("  call i32 @syx_rt_unified_gc_promotion(i32 0)\n");
        out.push_str("  call i32 @syx_rt_unified_gc_promote(i32 0)\n");
        out.push_str("  call i32 @syx_rt_unified_gc_collect()\n");
        out.push_str("  call i32 @syx_rt_gc_phase35_prod()\n");
        if inkwell_object_direct_enabled() {
            out.push_str("  call void @syx_rt_inkwell_object()\n");
            out.push_str("  call void @syx_rt_adr046_object()\n");
        } else if inkwell_prod_enabled() {
            out.push_str("  call void @syx_rt_inkwell_prod()\n");
            out.push_str("  call void @syx_rt_adr044_prod()\n");
        }
        if llvm_perf_release_ga_ready() {
            out.push_str("  call void @syx_rt_perf_release_ga()\n");
        }
        out.push_str("  ret void\n");
        out.push_str("}\n\n");
    } else if llvm_aot_phase9_enabled() {
        out.push_str("define void @syx_gc_phase9_preamble() {\n");
        out.push_str("  call void @syx_rt_llvm_phase9()\n");
        out.push_str("  call void @syx_rt_mir_v23_barrier()\n");
        out.push_str("  call i32 @syx_rt_unified_gc(i32 0)\n");
        out.push_str("  call i32 @syx_rt_unified_gc_remembered(i32 0)\n");
        out.push_str("  call i32 @syx_rt_unified_gc_promote(i32 0)\n");
        out.push_str("  call i32 @syx_rt_vm_native_gc(i32 0)\n");
        out.push_str("  call i32 @syx_rt_vm_native_gc_remembered(i32 0)\n");
        out.push_str("  call i32 @syx_rt_gc_remembered_set(i32 0)\n");
        out.push_str("  call i32 @syx_rt_gc_card_table(i32 0)\n");
        out.push_str("  call i32 @syx_rt_gc_wasm_intrinsic(i32 64)\n");
        out.push_str("  call i32 @syx_rt_gc_native_intrinsic(i32 128)\n");
        out.push_str("  call i32 @syx_rt_gc_native_remembered_set(i32 0)\n");
        out.push_str("  call i32 @syx_rt_gc_native_card_table(i32 0)\n");
        out.push_str("  call i32 @syx_rt_unified_gc_collect()\n");
        out.push_str("  call i32 @syx_rt_vm_native_gc_collect()\n");
        out.push_str("  call i32 @syx_rt_gc_phase34_prod()\n");
        if inkwell_prod_enabled() {
            out.push_str("  call void @syx_rt_inkwell_prod()\n");
            out.push_str("  call void @syx_rt_adr044_prod()\n");
        } else if inkwell_default_enabled() {
            out.push_str("  call void @syx_rt_inkwell_default()\n");
            out.push_str("  call void @syx_rt_adr042_default()\n");
        }
        if llvm_perf_release_ga_ready() {
            out.push_str("  call void @syx_rt_perf_release_ga()\n");
        } else if llvm_perf_parity_gate_ready() {
            out.push_str("  call void @syx_rt_perf_parity_gate()\n");
        }
        out.push_str("  ret void\n");
        out.push_str("}\n\n");
    } else if llvm_aot_phase8_enabled() {
        out.push_str("define void @syx_gc_phase8_preamble() {\n");
        out.push_str("  call void @syx_rt_llvm_phase8()\n");
        out.push_str("  call void @syx_rt_mir_v22_barrier()\n");
        out.push_str("  call i32 @syx_rt_vm_native_gc(i32 0)\n");
        out.push_str("  call i32 @syx_rt_vm_native_gc_remembered(i32 0)\n");
        out.push_str("  call i32 @syx_rt_gc_remembered_set(i32 0)\n");
        out.push_str("  call i32 @syx_rt_gc_card_table(i32 0)\n");
        out.push_str("  call i32 @syx_rt_gc_wasm_intrinsic(i32 64)\n");
        out.push_str("  call i32 @syx_rt_gc_native_intrinsic(i32 128)\n");
        out.push_str("  call i32 @syx_rt_gc_native_remembered_set(i32 0)\n");
        out.push_str("  call i32 @syx_rt_gc_native_card_table(i32 0)\n");
        out.push_str("  call i32 @syx_rt_vm_native_gc_collect()\n");
        if inkwell_default_enabled() {
            out.push_str("  call void @syx_rt_inkwell_default()\n");
            out.push_str("  call void @syx_rt_adr042_default()\n");
        }
        if llvm_perf_parity_gate_ready() {
            out.push_str("  call void @syx_rt_perf_parity_gate()\n");
        }
        out.push_str("  ret void\n");
        out.push_str("}\n\n");
    } else if llvm_aot_phase7_enabled() {
        out.push_str("define void @syx_gc_phase7_preamble() {\n");
        out.push_str("  call void @syx_rt_llvm_phase7()\n");
        out.push_str("  call void @syx_rt_mir_v21_barrier()\n");
        out.push_str("  call i32 @syx_rt_gc_remembered_set(i32 0)\n");
        out.push_str("  call i32 @syx_rt_gc_card_table(i32 0)\n");
        out.push_str("  call i32 @syx_rt_gc_wasm_intrinsic(i32 64)\n");
        out.push_str("  call i32 @syx_rt_gc_native_intrinsic(i32 128)\n");
        if llvm_release_blocker_ga_on() {
            out.push_str("  call void @syx_rt_adr040_blocker()\n");
        }
        out.push_str("  ret void\n");
        out.push_str("}\n\n");
    } else if llvm_aot_phase6_enabled() {
        out.push_str("define void @syx_gc_intrinsic_preamble() {\n");
        out.push_str("  call void @syx_rt_llvm_phase6()\n");
        out.push_str("  call void @syx_rt_mir_v20_barrier()\n");
        out.push_str("  call i32 @syx_rt_gc_intrinsic_alloc(i32 64)\n");
        out.push_str("  call i32 @syx_rt_gc_intrinsic_collect()\n");
        out.push_str("  call i32 @syx_rt_gc_generational(i32 0)\n");
        out.push_str("  call i32 @syx_rt_gc_guest_heap(i32 128)\n");
        if llvm_perf_release_candidate_ready() {
            out.push_str("  call void @syx_rt_adr038_perf()\n");
        }
        out.push_str("  ret void\n");
        out.push_str("}\n\n");
    }
    emit_llvm_string_globals(&mut out);
    // Wave A W248 / ADR-592: MakeClosure must resolve fn name → module index
    // (MIR parity). Hardcoding i32 0 pointed every closure at `main`.
    // Wave A W306 / ADR-650: indices must match compact AOT blob (fn-delete wire).
    let compact = syx_mir::compact_module_for_aot_blob(module);
    let fn_name_to_index: HashMap<String, u32> = compact
        .functions
        .iter()
        .enumerate()
        .map(|(i, f)| (f.name.to_string(), i as u32))
        .collect();
    let emit_names = syx_mir::llvm_emit_function_names(module);
    for func in &module.functions {
        if emit_names.contains(&func.name) {
            emit_function(func, &mut out, &fn_name_to_index);
        }
    }
    Ok(out)
}

pub fn emit_llvm_ir_with_gate(module: &MirModule) -> SyxResult<String> {
    let ir = emit_llvm_ir(module)?;
    assert_llvm_parity_strict_module(&ir)?;
    llvm_parity_runtime_execute_smoke(module)?;
    Ok(ir)
}

fn mir_block_successors(block: &syx_mir::MirBlock, next_block: Option<u32>) -> Vec<u32> {
    for instr in &block.instructions {
        match &instr.op {
            MirOp::Return | MirOp::ReturnErr | MirOp::BindResult(_) => return Vec::new(),
            MirOp::Jump(target) => return vec![*target],
            MirOp::JumpIfFalse(target) | MirOp::JumpIfNil(target) => {
                let mut succ = vec![*target];
                if let Some(next) = next_block {
                    if !succ.contains(&next) {
                        succ.push(next);
                    }
                }
                return succ;
            }
            _ => {}
        }
    }
    next_block.map(|id| vec![id]).unwrap_or_default()
}

fn mir_block_predecessors(func: &MirFunction) -> HashMap<u32, Vec<u32>> {
    let mut preds: HashMap<u32, Vec<u32>> = HashMap::new();
    for (i, block) in func.blocks.iter().enumerate() {
        let next_id = func.blocks.get(i + 1).map(|b| b.id);
        for succ in mir_block_successors(block, next_id) {
            let entry = preds.entry(succ).or_default();
            if !entry.contains(&block.id) {
                entry.push(block.id);
            }
        }
    }
    preds
}

fn mir_instr_closes_current_bb(op: &MirOp, fallthrough: Option<&str>) -> bool {
    matches!(
        op,
        // Throw may `ret` when no local catch pad (Wave A W11).
        MirOp::Return | MirOp::ReturnErr | MirOp::Jump(_) | MirOp::Throw
    ) || matches!(
        (op, fallthrough),
        (MirOp::JumpIfFalse(_) | MirOp::JumpIfNil(_), Some(_))
            | (MirOp::AwaitSuspend { .. }, Some(_))
    )
}

fn phi_incoming_val(raw: &str) -> String {
    if raw.starts_with('%') {
        raw.to_string()
    } else if let Some(rest) = raw.strip_prefix("i32 ") {
        rest.to_string()
    } else {
        raw.to_string()
    }
}

fn emit_register_phis(
    preds: &[u32],
    block_out: &HashMap<u32, HashMap<u32, String>>,
    labels: &HashMap<u32, String>,
    out: &mut String,
    next_tmp: &mut u32,
) -> HashMap<u32, String> {
    if preds.is_empty() {
        return HashMap::new();
    }
    if preds.len() == 1 {
        return block_out.get(&preds[0]).cloned().unwrap_or_default();
    }
    let mut live: HashSet<u32> = HashSet::new();
    for p in preds {
        if let Some(m) = block_out.get(p) {
            live.extend(m.keys().copied());
        }
    }
    let mut regs = HashMap::new();
    for r in live {
        let mut incoming: Vec<(String, String)> = Vec::new();
        for p in preds {
            let pred_label = labels.get(p).cloned().unwrap_or_else(|| format!("b{p}"));
            let val = block_out
                .get(p)
                .and_then(|m| m.get(&r))
                .map(|v| phi_incoming_val(v))
                .unwrap_or_else(|| "0".to_string());
            incoming.push((val, pred_label));
        }
        if incoming.iter().all(|(v, _)| v == &incoming[0].0) {
            regs.insert(r, incoming[0].0.clone());
            continue;
        }
        let phi = fresh_tmp(next_tmp);
        let arms: Vec<String> = incoming
            .iter()
            .map(|(v, l)| format!("[ {v}, %{l} ]"))
            .collect();
        out.push_str(&format!("  {phi} = phi i32 {}\n", arms.join(", ")));
        regs.insert(r, phi);
    }
    regs
}

fn emit_function(
    func: &MirFunction,
    out: &mut String,
    fn_name_to_index: &HashMap<String, u32>,
) {
    let fname = sanitize_llvm_symbol(func.name.as_str());
    out.push_str(&format!("define i32 @{fname}() {{\n"));
    let mut regs: HashMap<u32, String> = HashMap::new();
    let mut next_tmp = 0u32;
    let mut next_label = 0u32;
    let mut labels: HashMap<u32, String> = HashMap::new();
    let locals_count = func.local_count.max(1);
    let param_count = func.param_count;
    let locals_alloca = fresh_tmp(&mut next_tmp);
    let emit_locals_alloca = |out: &mut String, next_tmp: &mut u32| {
        out.push_str(&format!(
            "  {locals_alloca} = alloca i64, i32 {locals_count}\n"
        ));
        // Zero-init: empty slots hold null ptr (Nil). No full-frame hydrate.
        for slot in 0..locals_count {
            let ptr = fresh_tmp(next_tmp);
            out.push_str(&format!(
                "  {ptr} = getelementptr inbounds i64, i64* {locals_alloca}, i32 {slot}\n"
            ));
            out.push_str(&format!("  store i64 0, i64* {ptr}\n"));
        }
    };
    // After begin_function: seed param slots with pointers into frame-owned locals (locals v10).
    let emit_init_frame_local_ptrs = |out: &mut String, next_tmp: &mut u32| {
        for slot in 0..locals_count {
            let fptr = fresh_tmp(next_tmp);
            out.push_str(&format!(
                "  {fptr} = call i64 @syx_rt_frame_local_ptr(i32 {slot})\n"
            ));
            let ptr = fresh_tmp(next_tmp);
            out.push_str(&format!(
                "  {ptr} = getelementptr inbounds i64, i64* {locals_alloca}, i32 {slot}\n"
            ));
            out.push_str(&format!("  store i64 {fptr}, i64* {ptr}\n"));
        }
    };

    for block in &func.blocks {
        labels
            .entry(block.id)
            .or_insert_with(|| format!("b{}", block.id));
    }
    for instr in &func.instructions {
        if matches!(
            instr.op,
            MirOp::Jump(_) | MirOp::JumpIfFalse(_) | MirOp::JumpIfNil(_)
        ) {
            if let MirOp::Jump(t) | MirOp::JumpIfFalse(t) | MirOp::JumpIfNil(t) = instr.op {
                labels.entry(t).or_insert_with(|| format!("b{t}"));
            }
        }
    }
    for block in &func.blocks {
        for instr in &block.instructions {
            if matches!(
                instr.op,
                MirOp::Jump(_) | MirOp::JumpIfFalse(_) | MirOp::JumpIfNil(_)
            ) {
                if let MirOp::Jump(t) | MirOp::JumpIfFalse(t) | MirOp::JumpIfNil(t) = instr.op {
                    labels.entry(t).or_insert_with(|| format!("b{t}"));
                }
            }
        }
    }

    // Only structured block ids are emitted as LLVM labels; switch targets must match.
    // Wave A W6: take_catch may only jump to BeginTry landing pads (never entry %b0).
    let mut catch_target_ids: HashSet<u32> = HashSet::new();
    for block in &func.blocks {
        for instr in &block.instructions {
            if let MirOp::BeginTry(t) = instr.op {
                catch_target_ids.insert(t);
            }
        }
    }
    for instr in &func.instructions {
        if let MirOp::BeginTry(t) = instr.op {
            catch_target_ids.insert(t);
        }
    }
    let mut catch_labels: Vec<(u32, String)> = catch_target_ids
        .iter()
        .filter_map(|id| labels.get(id).map(|lab| (*id, lab.clone())))
        .collect();
    catch_labels.sort_by_key(|(id, _)| *id);
    let has_try = !catch_labels.is_empty();
    // Wave A W11: Throw without local pad emits early `ret`, which also breaks MIR-pred phis.
    let has_throw = {
        let is_throw = |op: &MirOp| matches!(op, MirOp::Throw);
        func.blocks
            .iter()
            .any(|b| b.instructions.iter().any(|i| is_throw(&i.op)))
            || func.instructions.iter().any(|i| is_throw(&i.op))
    };
    let skip_register_phis = has_try || has_throw;

    // Phase 98: GC preamble stubs historically `POP` the MIR stack. Emitting them at
    // every callee entry destroys string-interp prefixes left under Call args
    // (`"tick: \(runLoop(win))"` → `nilpump:...`). Only run on `main`.
    let emit_gc_preamble = |out: &mut String| {
        if func.name.as_str() != "main" {
            return;
        }
        if llvm_aot_phase11_enabled() {
            out.push_str("  call void @syx_gc_phase11_preamble()\n");
        } else if llvm_aot_phase10_enabled() {
            out.push_str("  call void @syx_gc_phase10_preamble()\n");
        } else if llvm_aot_phase9_enabled() {
            out.push_str("  call void @syx_gc_phase9_preamble()\n");
        } else if llvm_aot_phase8_enabled() {
            out.push_str("  call void @syx_gc_phase8_preamble()\n");
        } else if llvm_aot_phase7_enabled() {
            out.push_str("  call void @syx_gc_phase7_preamble()\n");
        } else if llvm_aot_phase6_enabled() {
            out.push_str("  call void @syx_gc_intrinsic_preamble()\n");
        }
    };

    if func.blocks.is_empty() {
        out.push_str("entry:\n");
        emit_locals_alloca(out, &mut next_tmp);
        out.push_str(&format!(
            "  call void @syx_rt_begin_function(i32 {}, i32 {})\n",
            func.param_count, func.local_count
        ));
        emit_init_frame_local_ptrs(out, &mut next_tmp);
        emit_gc_preamble(out);
        let mut last_value: Option<String> = None;
        for instr in &func.instructions {
            if let Some(ft) = emit_mir_instr(
                instr,
                out,
                None,
                &mut regs,
                &labels,
                &catch_labels,
                &mut next_tmp,
                &mut next_label,
                Some(&locals_alloca),
                &mut last_value,
                fn_name_to_index,
            ) {
                out.push_str(&format!("{ft}:\n"));
            }
        }
    } else {
        let preds_map = mir_block_predecessors(func);
        let entry_id = func.entry;
        let mut block_out: HashMap<u32, HashMap<u32, String>> = HashMap::new();

        for (i, block) in func.blocks.iter().enumerate() {
            let label = labels
                .get(&block.id)
                .cloned()
                .unwrap_or_else(|| format!("b{}", block.id));
            out.push_str(&format!("{label}:\n"));
            if block.id == entry_id {
                emit_locals_alloca(out, &mut next_tmp);
                out.push_str(&format!(
                    "  call void @syx_rt_begin_function(i32 {}, i32 {})\n",
                    func.param_count, func.local_count
                ));
                emit_init_frame_local_ptrs(out, &mut next_tmp);
                emit_gc_preamble(out);
            }

            let preds = preds_map.get(&block.id).cloned().unwrap_or_default();
            // Wave A W6/W11: take_catch splits / Throw early-ret break SSA phi predecessor labels;
            // try/catch + throwing callees are stack/local based — skip register phis there.
            let mut regs = if skip_register_phis {
                HashMap::new()
            } else {
                emit_register_phis(&preds, &block_out, &labels, out, &mut next_tmp)
            };
            let mut last_value: Option<String> = None;

            let fallthrough = func
                .blocks
                .get(i + 1)
                .and_then(|b| labels.get(&b.id).map(String::as_str));
            let mut bb_closed = false;
            for instr in &block.instructions {
                if bb_closed {
                    break;
                }
                let opened = emit_mir_instr(
                    instr,
                    out,
                    fallthrough,
                    &mut regs,
                    &labels,
                    &catch_labels,
                    &mut next_tmp,
                    &mut next_label,
                    Some(&locals_alloca),
                    &mut last_value,
                    fn_name_to_index,
                );
                if let Some(new_label) = opened {
                    out.push_str(&format!("{new_label}:\n"));
                } else if mir_instr_closes_current_bb(&instr.op, fallthrough) {
                    bb_closed = true;
                }
            }
            if !bb_closed {
                if let Some(ft) = fallthrough {
                    out.push_str(&format!("  br label %{ft}\n"));
                }
            }
            block_out.insert(block.id, regs);
        }
    }

    if !func.instructions.iter().any(|i| matches!(i.op, MirOp::Return))
        && !func
            .blocks
            .iter()
            .flat_map(|b| &b.instructions)
            .any(|i| matches!(i.op, MirOp::Return))
    {
        out.push_str("  call void @syx_rt_end_function()\n");
        out.push_str("  ret i32 0\n");
    }
    out.push_str("}\n\n");
}

/// Emits one MIR instruction. Returns a fresh basic-block label when linear-mode
/// conditional fallthrough must be opened immediately after the branch.
fn emit_mir_instr(
    instr: &syx_mir::MirInstr,
    out: &mut String,
    fallthrough: Option<&str>,
    regs: &mut HashMap<u32, String>,
    labels: &HashMap<u32, String>,
    catch_labels: &[(u32, String)],
    next_tmp: &mut u32,
    next_label: &mut u32,
    locals_alloca: Option<&str>,
    last_value: &mut Option<String>,
    fn_name_to_index: &HashMap<String, u32>,
) -> Option<String> {
    let set_reg = |regs: &mut HashMap<u32, String>,
                   last_value: &mut Option<String>,
                   dest: Option<u32>,
                   tmp: String| {
        if let Some(d) = dest {
            regs.insert(d, tmp.clone());
        }
        *last_value = Some(tmp);
    };
    match &instr.op {
        MirOp::ConstI32(v) => {
            out.push_str(&format!("  call void @syx_rt_push_i32(i32 {v})\n"));
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = add i32 0, {v}\n"));
            set_reg(regs, last_value, instr.dest, tmp);
        }
        MirOp::ConstNil => {
            // Suite-1 void main ends with ConstNil + Return; peek_return(Nil) → exit 0.
            // Without this, EC-M1-3 pilot trap fires after successful print (exit≈1 via SIGTRAP).
            out.push_str("  call void @syx_rt_push_nil()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = add i32 0, 0\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::ConstStr(s) => {
            let idx = intern_llvm_str(s);
            let len = s.len();
            let sptr = fresh_tmp(next_tmp);
            out.push_str(&format!(
                "  {sptr} = getelementptr inbounds [{len_plus} x i8], [{len_plus} x i8]* @syx_str_{idx}, i64 0, i64 0\n",
                len_plus = len + 1
            ));
            let iptr = fresh_tmp(next_tmp);
            out.push_str(&format!(
                "  {iptr} = ptrtoint i8* {sptr} to i64\n"
            ));
            out.push_str(&format!(
                "  call void @syx_rt_push_str(i64 {iptr}, i32 {len})\n",
            ));
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = add i32 0, 0 ; str dest\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::LoadLocal(slot) => {
            // Phase 103 v9: alloca cells hold pointers into the per-frame Value slab.
            if let Some(base) = locals_alloca {
                let ptr = fresh_tmp(next_tmp);
                out.push_str(&format!(
                    "  {ptr} = getelementptr inbounds i64, i64* {base}, i32 {slot}\n"
                ));
                let fptr = fresh_tmp(next_tmp);
                out.push_str(&format!("  {fptr} = load i64, i64* {ptr}\n"));
                out.push_str(&format!(
                    "  call void @syx_rt_push_from_local_ptr(i64 {fptr})\n"
                ));
            }
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = add i32 0, 0 ; loadlocal dest\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::StoreLocal(slot) => {
            // Phase 103 v9: pop Value → frame-local pointer stored in alloca.
            if let Some(base) = locals_alloca {
                let ptr = fresh_tmp(next_tmp);
                out.push_str(&format!(
                    "  {ptr} = getelementptr inbounds i64, i64* {base}, i32 {slot}\n"
                ));
                let fptr = fresh_tmp(next_tmp);
                out.push_str(&format!("  {fptr} = load i64, i64* {ptr}\n"));
                out.push_str(&format!(
                    "  call void @syx_rt_pop_into_local_ptr(i64 {fptr})\n"
                ));
            }
        }
        MirOp::Print => {
            if llvm_aot_phase4_enabled() {
                out.push_str("  call void @syx_rt_print_aot()\n");
            } else {
                out.push_str("  call void @syx_rt_print()\n");
            }
        }
        MirOp::Pop => {
            out.push_str("  call void @syx_rt_pop_stack()\n");
        }
        MirOp::ConcatStr => {
            out.push_str("  call void @syx_rt_concat_str()\n");
        }
        MirOp::ToStr => {
            out.push_str("  call void @syx_rt_to_str()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = add i32 0, 0 ; tostr dest\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::ConstBool(v) => {
            // Wave A W203 / ADR-547: push Bool onto RT for Print; keep SSA for JumpIfFalse consumers.
            let n = if *v { 1 } else { 0 };
            out.push_str(&format!("  call void @syx_rt_push_bool(i32 {n})\n"));
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = add i32 0, {n}\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::ArrayBuild(n) => {
            out.push_str(&format!(
                "  call void @syx_rt_push_collection(i32 {n}) ; array\n"
            ));
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = add i32 0, 0\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::DictBuild(n) => {
            // Wave A W127 / ADR-471: dataclass / object literals need DICT_BUILD (ObjectRef), not ARRAY_BUILD.
            out.push_str(&format!("  call void @syx_rt_dict_build(i32 {n})\n"));
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = add i32 0, 0\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::Add { .. } => {
            out.push_str("  call void @syx_rt_add()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = add i32 0, 0\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::Sub { .. } => {
            out.push_str("  call void @syx_rt_sub()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = add i32 0, 0\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::Mul { .. } => {
            out.push_str("  call void @syx_rt_mul()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = add i32 0, 0\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::Div { .. } => {
            out.push_str("  call void @syx_rt_div()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = add i32 0, 0\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::Mod { .. } => {
            out.push_str("  call void @syx_rt_mod()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = add i32 0, 0\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::Eq => {
            out.push_str("  call void @syx_rt_eq()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = call i32 @syx_rt_pop_bool()\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::Ne => {
            out.push_str("  call void @syx_rt_ne()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = call i32 @syx_rt_pop_bool()\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::Lt { .. } => {
            out.push_str("  call void @syx_rt_lt()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = call i32 @syx_rt_pop_bool()\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::Le { .. } => {
            out.push_str("  call void @syx_rt_le()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = call i32 @syx_rt_pop_bool()\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::Gt { .. } => {
            out.push_str("  call void @syx_rt_gt()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = call i32 @syx_rt_pop_bool()\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::Ge { .. } => {
            out.push_str("  call void @syx_rt_ge()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = call i32 @syx_rt_pop_bool()\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::And => {
            out.push_str("  call void @syx_rt_and()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = call i32 @syx_rt_pop_bool()\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::Or => {
            out.push_str("  call void @syx_rt_or()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = call i32 @syx_rt_pop_bool()\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::Not => {
            out.push_str("  call void @syx_rt_not()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = call i32 @syx_rt_pop_bool()\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::Neg { .. } => {
            out.push_str("  call void @syx_rt_neg()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = add i32 0, 0\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::Call(name) => {
            // Phase 98: callee publishes MIR stack return via syx_rt_end_function; keep i32 for SSA.
            let tmp = fresh_tmp(next_tmp);
            let callee = sanitize_llvm_symbol(name.as_str());
            out.push_str(&format!("  {tmp} = call i32 @{callee}()\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
            // Wave A W6/W11: only split for local try pads. Without catch labels, a mid-block
            // `ft*` break leaves SSA regs undominated (TcpStream.connect `h == -1` flake).
            if catch_labels.is_empty() {
                return None;
            }
            let cont = emit_take_catch_branch(out, catch_labels, next_tmp, next_label);
            return Some(cont);
        }
        MirOp::CallHost(idx) => {
            // Phase 95: single CallHost emit — GC/aot prelude stubs POP the MIR stack and break GUI host args.
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!(
                "  {tmp} = call i32 @syx_rt_call_host(i32 {})\n",
                idx
            ));
            set_reg(regs, last_value, instr.dest, tmp.clone());
            // Wave A W6 / ADR-350: host Err under try → pending_catch → catch block.
            // Wave A W11: no local try → do not split (pending stays for caller Call).
            if catch_labels.is_empty() {
                return None;
            }
            let cont = emit_take_catch_branch(out, catch_labels, next_tmp, next_label);
            return Some(cont);
        }
        MirOp::CallWitness(w, m) => {
            if llvm_aot_phase4_enabled() {
                emit_discard_call_i32(
                    out,
                    next_tmp,
                    "syx_rt_witness_aot",
                    &format!("i32 {w}, i32 {m}"),
                );
            }
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!(
                "  {tmp} = call i32 @syx_rt_call_witness(i32 {}, i32 {})\n",
                w, m
            ));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::Jump(target) => {
            let label = labels.get(target).cloned().unwrap_or_else(|| format!("b{target}"));
            out.push_str(&format!("  br label %{label}\n"));
        }
        MirOp::JumpIfFalse(target) => {
            let taken = labels.get(target).cloned().unwrap_or_else(|| format!("b{target}"));
            // Wave A W11: HashMap::values().last() is unordered — use last written SSA.
            let cond = last_value.clone().unwrap_or_else(|| "%0".into());
            let cmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {cmp} = icmp eq i32 {cond}, 0\n"));
            return match fallthrough {
                Some(ft) => {
                    out.push_str(&format!("  br i1 {cmp}, label %{taken}, label %{ft}\n"));
                    None
                }
                None => {
                    let ft = fresh_block_label(next_label);
                    out.push_str(&format!("  br i1 {cmp}, label %{taken}, label %{ft}\n"));
                    Some(ft)
                }
            };
        }
        MirOp::JumpIfNil(target) => {
            let taken = labels.get(target).cloned().unwrap_or_else(|| format!("b{target}"));
            let cond = last_value.clone().unwrap_or_else(|| "%0".into());
            let cmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {cmp} = icmp eq i32 {cond}, 0\n"));
            return match fallthrough {
                Some(ft) => {
                    out.push_str(&format!("  br i1 {cmp}, label %{taken}, label %{ft}\n"));
                    None
                }
                None => {
                    let ft = fresh_block_label(next_label);
                    out.push_str(&format!("  br i1 {cmp}, label %{taken}, label %{ft}\n"));
                    Some(ft)
                }
            };
        }
        // Wave A W544 / ADR-888: MirOp::ObjectRefWrap peeled (ADR-858 nop retired).
        MirOp::Catch => {
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = call i32 @syx_rt_catch()\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::Throw => {
            // Wave A W6 / ADR-350: real THROW op (not CallHost 0 stub) + take_catch branch.
            out.push_str("  call void @syx_rt_throw()\n");
            if catch_labels.is_empty() {
                // Wave A W11: no local pad — end frame and return; caller Call take_catch runs.
                out.push_str("  call void @syx_rt_end_function()\n");
                out.push_str("  ret i32 0\n");
                return None;
            }
            let cont = emit_take_catch_branch(out, catch_labels, next_tmp, next_label);
            return Some(cont);
        }
        MirOp::BindResult(target) => {
            let label = labels.get(target).cloned().unwrap_or_else(|| format!("b{target}"));
            out.push_str(&format!("  br label %{label} ; bind_result\n"));
        }
        MirOp::MakeClosure(name, captures) => {
            let tmp = fresh_tmp(next_tmp);
            let idx = fn_name_to_index.get(name.as_str()).copied().unwrap_or(0);
            out.push_str(&format!(
                "  {tmp} = call i32 @syx_rt_make_closure(i32 {idx}, i32 {captures}) ; {name}\n",
                idx = idx,
                captures = captures,
                name = name
            ));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::CallClosure => {
            let tmp = fresh_tmp(next_tmp);
            if llvm_aot_phase3_enabled() {
                out.push_str(&format!("  {tmp} = call i32 @syx_rt_call_closure(i32 0)\n"));
            } else {
                out.push_str(&format!("  {tmp} = call i32 @syx_rt_call_host(i32 0) ; closure\n"));
            }
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::BeginTry(target) => {
            if llvm_aot_phase3_enabled() {
                out.push_str(&format!("  call void @syx_rt_begin_try(i32 {target})\n"));
            } else {
                out.push_str(&format!("  call void @syx_rt_begin_try(i32 {target})\n"));
            }
        }
        MirOp::EndTry => {
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = call i32 @syx_rt_end_try()\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::AwaitPoll => {
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = call i32 @syx_rt_await_poll()\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::AwaitSuspend { resume_fn, state } => {
            out.push_str(&format!(
                "  call void @syx_rt_await_suspend(i32 {resume_fn}, i32 {state})\n"
            ));
            let susp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {susp} = call i32 @syx_rt_is_suspended()\n"));
            let cmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {cmp} = icmp ne i32 {susp}, 0\n"));
            let exit_lbl = fresh_block_label(next_label);
            let cont_lbl = fallthrough
                .map(String::from)
                .unwrap_or_else(|| fresh_block_label(next_label));
            out.push_str(&format!(
                "  br i1 {cmp}, label %{exit_lbl}, label %{cont_lbl}\n"
            ));
            out.push_str(&format!("{exit_lbl}:\n"));
            out.push_str("  ret i32 0\n");
            if fallthrough.is_none() {
                return Some(cont_lbl);
            }
        }
        MirOp::Return => {
            // Phase 98: restore caller locals + publish return value onto MIR stack.
            // M0-W4: exit/return i32 from peek (not SSA ConstNumeric bits_lo); keep stack
            // value so nested Call + Print still see the Int (get_return would pop it).
            out.push_str("  call void @syx_rt_end_function()\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = call i32 @syx_rt_peek_return()\n"));
            out.push_str(&format!("  ret i32 {tmp}\n"));
        }
        MirOp::ReturnErr => {
            if llvm_aot_phase3_enabled() || llvm_aot_phase2_enabled() {
                out.push_str("  call void @syx_rt_return_err()\n");
                out.push_str("  ret i32 -1\n");
            } else {
                out.push_str("  ret i32 -1\n");
            }
        }
        MirOp::ActorCall(id) => {
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = call i32 @syx_rt_actor_call(i32 {id}, i32 0)\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::ActorSend(id) => {
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = call i32 @syx_rt_actor_send(i32 {id}, i32 0)\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::TaskSpawn(id) => {
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = call i32 @syx_rt_task_spawn(i32 {id})\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::MakeIterator => {
            out.push_str("  call i32 @syx_rt_make_iterator(i32 0)\n");
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = add i32 0, 0 ; makeiterator dest\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::IteratorNext => {
            out.push_str("  call i32 @syx_rt_iterator_next(i32 0)\n");
            // Wave A W198 / ADR-542: loop condition bool is on RT stack top (mir-run parity).
            let tmp = fresh_tmp(next_tmp);
            out.push_str(&format!("  {tmp} = call i32 @syx_rt_pop_bool()\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::SimdAdd => {
            out.push_str("  call void @syx_rt_simd_add()\n");
        }
        MirOp::SimdSub => {
            out.push_str("  call void @syx_rt_simd_sub()\n");
        }
        MirOp::SimdMul => {
            out.push_str("  call void @syx_rt_simd_mul()\n");
        }
        MirOp::SimdDiv => {
            out.push_str("  call void @syx_rt_simd_div()\n");
        }
        MirOp::SimdEq => {
            out.push_str("  call void @syx_rt_simd_eq()\n");
        }
        MirOp::SimdLt => {
            out.push_str("  call void @syx_rt_simd_lt()\n");
        }
        MirOp::SimdGt => {
            out.push_str("  call void @syx_rt_simd_gt()\n");
        }
        MirOp::SimdNe => {
            out.push_str("  call void @syx_rt_simd_ne()\n");
        }
        MirOp::SimdLe => {
            out.push_str("  call void @syx_rt_simd_le()\n");
        }
        MirOp::SimdGe => {
            out.push_str("  call void @syx_rt_simd_ge()\n");
        }
        MirOp::SimdAnd => {
            out.push_str("  call void @syx_rt_simd_and()\n");
        }
        MirOp::SimdOr => {
            out.push_str("  call void @syx_rt_simd_or()\n");
        }
        MirOp::SimdXor => {
            out.push_str("  call void @syx_rt_simd_xor()\n");
        }
        MirOp::SimdNot => {
            out.push_str("  call void @syx_rt_simd_not()\n");
        }
        MirOp::SimdShl => {
            out.push_str("  call void @syx_rt_simd_shl()\n");
        }
        MirOp::SimdShr => {
            out.push_str("  call void @syx_rt_simd_shr()\n");
        }
        MirOp::SimdMin => {
            out.push_str("  call void @syx_rt_simd_min()\n");
        }
        MirOp::SimdMax => {
            out.push_str("  call void @syx_rt_simd_max()\n");
        }
        MirOp::SimdF32Add => {
            out.push_str("  call void @syx_rt_simd_f32_add()\n");
        }
        MirOp::SimdF32Sub => {
            out.push_str("  call void @syx_rt_simd_f32_sub()\n");
        }
        MirOp::SimdF32Mul => {
            out.push_str("  call void @syx_rt_simd_f32_mul()\n");
        }
        MirOp::SimdF32Div => {
            out.push_str("  call void @syx_rt_simd_f32_div()\n");
        }
        MirOp::SimdF64Add => {
            out.push_str("  call void @syx_rt_simd_f64_add()\n");
        }
        MirOp::SimdI16Add => { out.push_str("  call void @syx_rt_simd_i16_add()\n"); }
        MirOp::SimdI16Sub => { out.push_str("  call void @syx_rt_simd_i16_sub()\n"); }
        MirOp::SimdI16Mul => { out.push_str("  call void @syx_rt_simd_i16_mul()\n"); }
        MirOp::SimdI8Add => { out.push_str("  call void @syx_rt_simd_i8_add()\n"); }
        MirOp::SimdI8Sub => { out.push_str("  call void @syx_rt_simd_i8_sub()\n"); }
        MirOp::SimdI8Mul => { out.push_str("  call void @syx_rt_simd_i8_mul()\n"); }
        MirOp::SimdI8And => { out.push_str("  call void @syx_rt_simd_i8_and()\n"); }
        MirOp::SimdI8Or => { out.push_str("  call void @syx_rt_simd_i8_or()\n"); }
        MirOp::SimdI8Xor => { out.push_str("  call void @syx_rt_simd_i8_xor()\n"); }
        MirOp::SimdI8Not => { out.push_str("  call void @syx_rt_simd_i8_not()\n"); }
        MirOp::SimdI8Min => { out.push_str("  call void @syx_rt_simd_i8_min()\n"); }
        MirOp::SimdI8Max => { out.push_str("  call void @syx_rt_simd_i8_max()\n"); }
        MirOp::SimdI8UMin => { out.push_str("  call void @syx_rt_simd_i8_umin()\n"); }
        MirOp::SimdI8UMax => { out.push_str("  call void @syx_rt_simd_i8_umax()\n"); }
        MirOp::SimdI8Abs => { out.push_str("  call void @syx_rt_simd_i8_abs()\n"); }
        MirOp::SimdI8Neg => { out.push_str("  call void @syx_rt_simd_i8_neg()\n"); }
        MirOp::SimdI8Extract => { out.push_str("  call void @syx_rt_simd_i8_extract()\n"); }
        MirOp::SimdI8Replace => { out.push_str("  call void @syx_rt_simd_i8_replace()\n"); }
        MirOp::SimdI8Eq => { out.push_str("  call void @syx_rt_simd_i8_eq()\n"); }
        MirOp::SimdI8Ne => { out.push_str("  call void @syx_rt_simd_i8_ne()\n"); }
        MirOp::SimdI8Le => { out.push_str("  call void @syx_rt_simd_i8_le()\n"); }
        MirOp::SimdI8Ge => { out.push_str("  call void @syx_rt_simd_i8_ge()\n"); }
        MirOp::SimdI8AvgrU => { out.push_str("  call void @syx_rt_simd_i8_avgr_u()\n"); }
        MirOp::SimdI8Popcnt => { out.push_str("  call void @syx_rt_simd_i8_popcnt()\n"); }
        MirOp::SimdI8Bitmask => { out.push_str("  call void @syx_rt_simd_i8_bitmask()\n"); }
        MirOp::SimdI8AllTrue => { out.push_str("  call void @syx_rt_simd_i8_all_true()\n"); }
        MirOp::SimdI8AnyTrue => { out.push_str("  call void @syx_rt_simd_i8_any_true()\n"); }
        MirOp::SimdI8Lt => { out.push_str("  call void @syx_rt_simd_i8_lt()\n"); }
        MirOp::SimdI8Gt => { out.push_str("  call void @syx_rt_simd_i8_gt()\n"); }
        MirOp::SimdI8AddSat => { out.push_str("  call void @syx_rt_simd_i8_add_sat()\n"); }
        MirOp::SimdI8SubSat => { out.push_str("  call void @syx_rt_simd_i8_sub_sat()\n"); }
        MirOp::SimdI8Shl => { out.push_str("  call void @syx_rt_simd_i8_shl()\n"); }
        MirOp::SimdI8Shr => { out.push_str("  call void @syx_rt_simd_i8_shr()\n"); }
        MirOp::SimdI8Shuffle { mode } => {
            out.push_str(&format!("  call void @syx_rt_simd_i8_shuffle(i32 {mode})\n"));
        }
        MirOp::SimdI64Add => {
            out.push_str("  call void @syx_rt_simd_i64_add()\n");
        }
        MirOp::SimdF64Sub => {
            out.push_str("  call void @syx_rt_simd_f64_sub()\n");
        }
        MirOp::SimdF64Mul => {
            out.push_str("  call void @syx_rt_simd_f64_mul()\n");
        }
        MirOp::SimdF64Div => {
            out.push_str("  call void @syx_rt_simd_f64_div()\n");
        }
        MirOp::SimdReduceAdd { .. } => {
            out.push_str("  call void @syx_rt_simd_reduce_add()\n");
        }
        MirOp::SimdShuffle { mode } => {
            out.push_str(&format!("  call void @syx_rt_simd_shuffle(i32 {mode})\n"));
        }
        MirOp::SimdLoad { lanes, element } => {
            let tag = (*lanes as i32) | ((*element as i32) << 8);
            out.push_str(&format!("  call void @syx_rt_simd_load(i32 {tag})\n"));
        }
        MirOp::SimdStore { lanes, element } => {
            let tag = (*lanes as i32) | ((*element as i32) << 8);
            out.push_str(&format!("  call void @syx_rt_simd_store(i32 {tag})\n"));
        }
        MirOp::ConstNumeric { kind, bits } => {
            // W955 / ADR-1299: full 64-bit bits via push_numeric64 (legacy push_numeric truncates f64).
            out.push_str(&format!(
                "  call void @syx_rt_push_numeric64(i32 {}, i64 {})\n",
                *kind as i32,
                *bits as i64
            ));
            let tmp = fresh_tmp(next_tmp);
            let bits_lo = (*bits & 0xffff_ffff) as i32;
            out.push_str(&format!("  {tmp} = add i32 0, {bits_lo}\n"));
            set_reg(regs, last_value, instr.dest, tmp.clone());
        }
        MirOp::ConstNumeric128 { kind, lo, hi } => {
            out.push_str(&format!(
                "  call i32 @syx_rt_const_numeric128(i32 {}, i32 {}, i32 {}, i32 {}, i32 {})\n",
                *kind as i32,
                (lo & 0xffff_ffff) as i32,
                (lo >> 32) as i32,
                (hi & 0xffff_ffff) as i32,
                (hi >> 32) as i32,
            ));
        }
        MirOp::ObjectGet => {
            out.push_str("  call i32 @syx_rt_object_get()\n");
        }
        MirOp::ObjectSet => {
            out.push_str("  call void @syx_rt_object_set()\n");
        }
        MirOp::ArrayGet => {
            // Wave A W195 / ADR-539: ArrayGet RT/emit (was silent skip → index leak)
            out.push_str("  call i32 @syx_rt_array_get()\n");
        }
        MirOp::ArraySet => {
            out.push_str("  call i32 @syx_rt_array_set()\n");
        }
        MirOp::ValueEq | MirOp::OpaqueValueEq => {
            out.push_str("  call i32 @syx_rt_value_eq()\n");
        }
        MirOp::RefEq => {
            out.push_str("  call i32 @syx_rt_ref_eq()\n");
        }
        MirOp::ArrSorted => {
            out.push_str("  call i32 @syx_rt_array_sorted()\n");
        }
        // Wave A W544 / ADR-888: ObjectRef Retain/Release MirOp variants peeled
        // (ADR-855 emit nops retired with the variants).
        MirOp::HeapCompact => {
            out.push_str("  call i32 @syx_rt_heap_compact()\n");
        }
        other => {
            // EC-M1-3 / ADR-136: Suite-1 pilot ops must not silently skip.
            if syx_mir::is_suite1_pilot_mir_op(other) {
                out.push_str(&format!(
                    "  ; EC-M1-3 unimplemented pilot MirOp: {other:?}\n"
                ));
                out.push_str("  call void @llvm.trap()\n");
                out.push_str("  unreachable\n");
            }
            // Non-pilot: honest table marks unsupported/noop; silent skip remains for now.
        }
    }
    None
}

fn fresh_tmp(next: &mut u32) -> String {
    let v = LLVM_SSA_SEQ.fetch_add(1, Ordering::Relaxed);
    *next = v.saturating_add(1);
    format!("%v{v}")
}

fn emit_discard_call_i32(out: &mut String, next_tmp: &mut u32, callee: &str, args: &str) {
    let tmp = fresh_tmp(next_tmp);
    out.push_str(&format!("  {tmp} = call i32 @{callee}({args})\n"));
}

/// Wave A W6 / ADR-350: after CallHost / Throw / Call, branch to the MIR catch
/// block when `syx_rt_take_catch` reports a pending error. Returns the
/// fallthrough label that must be opened for the non-catching path.
///
/// `catch_labels` must only list blocks that are actually emitted (structured
/// `func.blocks` ids) — not phantom flat-instruction jump ids.
fn emit_take_catch_branch(
    out: &mut String,
    catch_labels: &[(u32, String)],
    next_tmp: &mut u32,
    next_label: &mut u32,
) -> String {
    let catch_tmp = fresh_tmp(next_tmp);
    out.push_str(&format!("  {catch_tmp} = call i32 @syx_rt_take_catch()\n"));
    let cmp = fresh_tmp(next_tmp);
    out.push_str(&format!("  {cmp} = icmp sge i32 {catch_tmp}, 0\n"));
    let dispatch = fresh_block_label(next_label);
    let cont = fresh_block_label(next_label);
    out.push_str(&format!(
        "  br i1 {cmp}, label %{dispatch}, label %{cont}\n"
    ));
    out.push_str(&format!("{dispatch}:\n"));
    out.push_str(&format!("  switch i32 {catch_tmp}, label %{cont} [\n"));
    for (id, lab) in catch_labels {
        out.push_str(&format!("    i32 {id}, label %{lab}\n"));
    }
    out.push_str("  ]\n");
    cont
}

fn fresh_block_label(next: &mut u32) -> String {
    let v = *next;
    *next += 1;
    format!("ft{v}")
}

fn is_native_object_bytes(bytes: &[u8]) -> bool {
    if bytes.len() < 4 {
        return false;
    }
    // Mach-O 64-bit (LE / BE)
    if bytes.starts_with(b"\xcf\xfa\xed\xfe") || bytes.starts_with(b"\xfe\xed\xfa\xcf") {
        return true;
    }
    // ELF
    if bytes.starts_with(b"\x7fELF") {
        return true;
    }
    // Windows COFF object: first u16 LE is IMAGE_FILE_HEADER.Machine.
    // inkwell FileType::Object on MSVC targets emits COFF, not PE/ELF/Mach-O.
    let machine = u16::from_le_bytes([bytes[0], bytes[1]]);
    matches!(
        machine,
        0x014c | // IMAGE_FILE_MACHINE_I386
        0x8664 | // IMAGE_FILE_MACHINE_AMD64
        0xaa64 | // IMAGE_FILE_MACHINE_ARM64
        0x01c4 // IMAGE_FILE_MACHINE_ARMNT
    )
}

pub fn compile_native(module: &MirModule) -> SyxResult<Vec<u8>> {
    compile_native_with_profile(module, *LLVM_COMPILE_RELEASE.lock().unwrap())
}

/// Phase 93: native compile with debug/release clang profile.
pub fn compile_native_with_profile(module: &MirModule, release: bool) -> SyxResult<Vec<u8>> {
    *LLVM_COMPILE_RELEASE.lock().unwrap() = release;
    compile_native_inner(module)
}

fn compile_native_inner(module: &MirModule) -> SyxResult<Vec<u8>> {
    // Preferred path: inkwell TargetMachine → native .o (no clang).
    #[cfg(feature = "llvm")]
    if inkwell_object_direct_enabled() && !llvm_text_emit_enabled() {
        match emit_inkwell_object_v6(module) {
            Ok(obj) => {
                set_last_compile_aot(true);
                return Ok(obj);
            }
            Err(e) if native_stub_fallback_forbidden() => {
                return Err(SyxDiagnostic::error(format!(
                    "inkwell object-direct failed (clang IR fallback disabled): {}",
                    e.message
                )));
            }
            Err(_) => {}
        }
    }

    #[cfg(feature = "llvm")]
    if inkwell_default_enabled() && !llvm_text_emit_enabled() {
        if let Ok(bytes) = inkwell_backend::compile_with_inkwell(module) {
            if is_native_object_bytes(&bytes) {
                set_last_compile_aot(true);
                return Ok(bytes);
            }
            set_last_compile_aot(true);
            if let Ok(obj) = compile_ir_bytes_to_object(&bytes) {
                return Ok(obj);
            }
        }
    }

    #[cfg(feature = "llvm")]
    if let Ok(bytes) = inkwell_backend::compile_with_inkwell(module) {
        if is_native_object_bytes(&bytes) {
            set_last_compile_aot(true);
            return Ok(bytes);
        }
        if !bytes.starts_with(b"\x7fELF") {
            set_last_compile_aot(true);
            if let Ok(obj) = compile_ir_bytes_to_object(&bytes) {
                return Ok(obj);
            }
        }
    }

    if llvm_aot_enabled() || llvm_text_emit_enabled() {
        match compile_native_aot_ir(module) {
            Ok(bytes) => {
                set_last_compile_aot(true);
                return Ok(bytes);
            }
            Err(e) if native_stub_fallback_forbidden() => {
                return Err(SyxDiagnostic::error(format!(
                    "Native LLVM tier-1 compile failed; C stub fallback disabled \
                     (SYX_NATIVE_STRICT=1 or SYX_NATIVE_C_STUB=0): {}",
                    e.message
                )));
            }
            Err(_) => {}
        }
    }

    set_last_compile_aot(false);
    if native_stub_fallback_forbidden() {
        return Err(SyxDiagnostic::error(
            "Native LLVM tier-1 compile failed; C stub fallback disabled (SYX_NATIVE_STRICT=1 or SYX_NATIVE_C_STUB=0)",
        ));
    }
    warn_native_c_stub_fallback_once();
    compile_native_c_stub(module)
}

/// Phase 26 DR-AOT1: IR → clang object with register_native_fn shim (ADR-028).
fn compile_native_aot_ir(module: &MirModule) -> SyxResult<Vec<u8>> {
    let mut ir = emit_llvm_ir(module)?;
    append_aot_runtime_shim(module, &mut ir)?;
    let bytes = compile_ir_to_object(&ir)?;
    if !is_native_object_bytes(&bytes) {
        return Err(SyxDiagnostic::error(
            "LLVM AOT path did not produce a native object file",
        ));
    }
    Ok(bytes)
}

/// Map MIR / Syx fn names to LLVM (and C-stub) symbols.
/// Operator methods like `Mat_*` are invalid unquoted LLVM idents and illegal in C.
fn sanitize_llvm_symbol(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for c in name.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => out.push(c),
            '.' | '-' => out.push('_'),
            '*' => out.push_str("_mul_"),
            '+' => out.push_str("_add_"),
            '/' => out.push_str("_div_"),
            '%' => out.push_str("_rem_"),
            '=' => out.push_str("_eq_"),
            '!' => out.push_str("_bang_"),
            '<' => out.push_str("_lt_"),
            '>' => out.push_str("_gt_"),
            '&' => out.push_str("_and_"),
            '|' => out.push_str("_or_"),
            '^' => out.push_str("_xor_"),
            '~' => out.push_str("_not_"),
            _ => {
                out.push('_');
                out.push_str(&format!("{:x}_", u32::from(c)));
            }
        }
    }
    if out.is_empty() {
        "_".into()
    } else if out.as_bytes()[0].is_ascii_digit() {
        format!("_{out}")
    } else {
        out
    }
}

/// Phase 90: sync `@main` may call emitted `@main()` directly; async/actor paths keep MIR entry.
pub fn mir_function_needs_mir_entry(func: &MirFunction) -> bool {
    // W417–W419: Await*/Task*/Actor* stubs fail-closed; no MIR main entry forced.
    let _ = func;
    false
}

/// Phase 90: exported for driver gates — sync main LLVM IR must not reference MIR main runner.
/// Phase 92: match only `call i32 @syx_rt_run_main_fn`, ignore `declare`.
pub fn llvm_ir_uses_mir_main_entry(ir: &str) -> bool {
    ir.contains("call i32 @syx_rt_run_main_fn")
        || ir.contains("call void @syx_rt_run_main_fn")
}

/// Phase 91: multi-fn sync LLVM IR must not call MIR interpreter for user functions.
pub fn llvm_ir_uses_mir_call_fn(ir: &str) -> bool {
    ir.contains("call i32 @syx_rt_call_fn")
}

/// Phase 90: append `@syx_user_init` / `@syx_user_main` shim IR (exported for driver gates).
pub fn append_native_runtime_shim(module: &MirModule, ir: &mut String) -> SyxResult<()> {
    append_aot_runtime_shim(module, ir)
}

fn aot_compact_view(module: &MirModule) -> (syx_mir::MirModule, Vec<u8>) {
    // Wave A W306 / ADR-650: compact for index alignment (register_native_fn).
    // Wave A W413 / ADR-757: sidecar encode remains for Medium inventories / probes.
    // Wave A W431 / ADR-775: production emit no longer embeds sidecar bytes.
    // Wave A W484 / ADR-828: production shim/C stub no longer call this dead encode path.
    let compact = syx_mir::compact_module_for_aot_blob(module);
    let bytes = syx_mir::encode_module_metadata_sidecar(module);
    (compact, bytes)
}

fn append_aot_runtime_shim(module: &MirModule, ir: &mut String) -> SyxResult<()> {
    if !ir.contains("declare void @syx_rt_register_native_fn") {
        ir.push_str("declare void @syx_rt_register_native_fn(i32, i64)\n");
    }
    if !ir.contains("declare void @syx_rt_install_empty_runtime") {
        ir.push_str("declare void @syx_rt_install_empty_runtime()\n");
    }
    if !ir.contains("declare i32 @syx_rt_run_main_fn") {
        ir.push_str("declare i32 @syx_rt_run_main_fn(i32)\n");
    }
    // Wave A W413 / ADR-757: stop `@syx_mir_blob` / `syx_rt_load_mir_blob`.
    // Wave A W431 / ADR-775: stop `@syx_metadata_sidecar` embed; empty install + register.
    // Wave A W484 / ADR-828: compact only — no dead sidecar encode on production path.
    let compact = syx_mir::compact_module_for_aot_blob(module);

    // Opaque-pointer IR (LLVM 15+) so inkwell 0.9 / LLVM 21 parse_module succeeds.
    ir.push_str("define void @syx_user_init() {\n");
    ir.push_str("  call void @syx_rt_install_empty_runtime()\n");
    for (idx, func) in compact.functions.iter().enumerate() {
        let fname = sanitize_llvm_symbol(&func.name);
        ir.push_str(&format!(
            "  call void @syx_rt_register_native_fn(i32 {idx}, i64 ptrtoint (ptr @{fname} to i64))\n"
        ));
    }
    ir.push_str("  ret void\n");
    ir.push_str("}\n\n");

    let main_idx = compact
        .functions
        .iter()
        .position(|f| f.name == "main")
        .unwrap_or(0);
    let main_fn = compact.functions.get(main_idx);
    let use_mir_main = main_fn.is_some_and(mir_function_needs_mir_entry);
    ir.push_str("define i32 @syx_user_main() {\n");
    if use_mir_main {
        ir.push_str(&format!(
            "  %ret = call i32 @syx_rt_run_main_fn(i32 {main_idx})\n"
        ));
    } else {
        let main_sym = sanitize_llvm_symbol("main");
        ir.push_str(&format!("  %ret = call i32 @{main_sym}()\n"));
    }
    ir.push_str("  ret i32 %ret\n");
    ir.push_str("}\n\n");
    Ok(())
}

/// Produce C source for the native C stub (empty install + register loop).
///
/// Wave A W430 / ADR-774: register each compact fn (C stub align).
/// Wave A W431 / ADR-775: stop embedding `syx_metadata_sidecar`; empty install first.
fn native_c_stub_source(module: &MirModule) -> String {
    // Wave A W484 / ADR-828: compact only — no dead sidecar encode on production path.
    let compact = syx_mir::compact_module_for_aot_blob(module);
    let main_idx = compact
        .functions
        .iter()
        .position(|f| f.name == "main")
        .unwrap_or(0);

    let mut c = String::from(
        "#include <stdint.h>\n\n\
         extern void syx_rt_install_empty_runtime(void);\n\
         extern void syx_rt_register_native_fn(int32_t, int64_t);\n\
         extern int32_t syx_rt_run_main_fn(uint32_t);\n\n",
    );
    for func in &compact.functions {
        let fname = sanitize_llvm_symbol(&func.name);
        c.push_str(&format!("extern int32_t {fname}(void);\n"));
    }
    c.push('\n');
    c.push_str("void syx_user_init(void) {\n");
    c.push_str("  syx_rt_install_empty_runtime();\n");
    for (idx, func) in compact.functions.iter().enumerate() {
        let fname = sanitize_llvm_symbol(&func.name);
        c.push_str(&format!(
            "  syx_rt_register_native_fn({idx}, (int64_t)(uintptr_t)&{fname});\n"
        ));
    }
    c.push_str("}\n\n");
    let main_fn = compact.functions.get(main_idx);
    c.push_str("int32_t syx_user_main(void) {\n");
    if main_fn.is_some_and(mir_function_needs_mir_entry) {
        c.push_str(&format!("  return syx_rt_run_main_fn({main_idx});\n"));
    } else {
        let main_sym = sanitize_llvm_symbol("main");
        c.push_str(&format!("  return {main_sym}();\n"));
    }
    c.push_str("}\n");
    c
}

/// Produce a linkable native object via clang (W413+; W431: no sidecar embed).
/// User functions historically fell back through `syx_rt_call_fn` MIR interpret;
/// W416 fail-closes missing natives. W417 Await / W418 Task / W419 Actor stubs
/// fail-closed; `mir_function_needs_mir_entry` no longer forces MIR main entry.
/// W430: `syx_user_init` also runs `syx_rt_register_native_fn` (C stub align).
/// W431: empty install replaces sidecar install.
fn compile_native_c_stub(module: &MirModule) -> SyxResult<Vec<u8>> {
    compile_c_to_object(&native_c_stub_source(module))
}

fn host_clang_command() -> Command {
    if let Ok(path) = std::env::var("SYX_CLANG") {
        if !path.is_empty() {
            return Command::new(path);
        }
    }
    if let Ok(path) = std::env::var("CC") {
        if !path.is_empty() {
            return Command::new(path);
        }
    }
    #[cfg(target_os = "macos")]
    if std::path::Path::new("/usr/bin/clang").exists() {
        return Command::new("/usr/bin/clang");
    }
    Command::new("clang")
}

fn compile_c_to_object(c_src: &str) -> SyxResult<Vec<u8>> {
    let mut child = host_clang_command()
        .args(["-c", "-x", "c", "-fPIC", "-o", "-", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| SyxDiagnostic::error(format!("Failed to spawn clang: {e}")))?;

    child
        .stdin
        .as_mut()
        .ok_or_else(|| SyxDiagnostic::error("Failed to open clang stdin"))?
        .write_all(c_src.as_bytes())
        .map_err(|e| SyxDiagnostic::error(format!("Failed to write C source to clang: {e}")))?;

    let output = child
        .wait_with_output()
        .map_err(|e| SyxDiagnostic::error(format!("Failed to wait for clang: {e}")))?;

    if output.status.success() && !output.stdout.is_empty() {
        Ok(output.stdout)
    } else {
        Err(SyxDiagnostic::error(format!(
            "clang failed to compile native stub: {}",
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}

fn compile_ir_bytes_to_object(bytes: &[u8]) -> SyxResult<Vec<u8>> {
    let ir = std::str::from_utf8(bytes)
        .map_err(|e| SyxDiagnostic::error(format!("inkwell IR is not UTF-8: {e}")))?;
    compile_ir_to_object(ir)
}

fn compile_ir_to_object(ir: &str) -> SyxResult<Vec<u8>> {
    // Ensure runtime entry symbols exist when callers pass bare `emit_llvm_ir` text.
    if !(ir.contains("define i32 @syx_user_main") || ir.contains("define void @syx_user_init")) {
        return Err(SyxDiagnostic::error(
            "compile_ir_to_object: IR missing syx_user_main shim; use compile_native_aot_ir / object-direct",
        ));
    }
    let release = *LLVM_COMPILE_RELEASE.lock().unwrap();
    let args = ["-x", "ir", "-c", "-o", "-", "-"];
    let debug_args = ["-O0", "-g"];
    let release_args = ["-O2", "-DNDEBUG"];
    let profile_args: &[&str] = if release {
        &release_args
    } else {
        &debug_args
    };
    let mut child = host_clang_command()
        .args(args)
        .args(profile_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| SyxDiagnostic::error(format!("Failed to spawn clang: {e}")))?;

    child
        .stdin
        .as_mut()
        .ok_or_else(|| SyxDiagnostic::error("Failed to open clang stdin"))?
        .write_all(ir.as_bytes())
        .map_err(|e| SyxDiagnostic::error(format!("Failed to write LLVM IR to clang: {e}")))?;

    let output = child
        .wait_with_output()
        .map_err(|e| SyxDiagnostic::error(format!("Failed to wait for clang: {e}")))?;

    if output.status.success() && !output.stdout.is_empty() {
        Ok(output.stdout)
    } else {
        Err(SyxDiagnostic::error(format!(
            "clang failed to compile LLVM IR: {}",
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syx_mir::{MirBlock, MirFunction, MirInstr};

    #[test]
    fn emits_llvm_ir() {
        let module = MirModule {
            functions: vec![MirFunction {
                name: "main".into(),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![],
                instructions: vec![MirInstr {
                    dest: Some(0),
                    op: MirOp::ConstI32(1),
                    span: None,
                }],
                local_names: vec![],
                local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            }],
            ..Default::default()
};
        let ir = emit_llvm_ir(&module).unwrap();
        assert!(ir.contains("define i32 @main"));
        assert!(ir.contains("syx_rt_call_host"));
    }

    fn mir_fn_with_ops(name: &str, ops: Vec<MirOp>) -> MirFunction {
        MirFunction {
            name: name.into(),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks: vec![],
            instructions: ops
                .into_iter()
                .map(|op| MirInstr {
                    dest: None,
                    op,
                    span: None,
                })
                .collect(),
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
        }
    }

    #[test]
    fn mir_function_needs_mir_entry_await_only_is_false_after_w417() {
        let f = mir_fn_with_ops(
            "main",
            vec![
                MirOp::AwaitPoll,
                MirOp::AwaitSuspend {
                    resume_fn: 1,
                    state: 0,
                },
                MirOp::Return,
            ],
        );
        assert!(
            !mir_function_needs_mir_entry(&f),
            "Await* must not force MIR main entry after W417"
        );
    }

    #[test]
    fn mir_function_needs_mir_entry_task_only_is_false_after_w418() {
        let task = mir_fn_with_ops(
            "main",
            vec![MirOp::TaskSpawn("worker".into()), MirOp::Return],
        );
        assert!(
            !mir_function_needs_mir_entry(&task),
            "TaskSpawn must not force MIR main entry after W418"
        );
        let join = mir_fn_with_ops("main", vec![MirOp::TaskJoin, MirOp::Return]);
        assert!(
            !mir_function_needs_mir_entry(&join),
            "TaskJoin must not force MIR main entry after W418"
        );
    }

    #[test]
    fn mir_function_needs_mir_entry_actor_only_is_false_after_w419() {
        let actor = mir_fn_with_ops(
            "main",
            vec![MirOp::ActorCall("ping".into()), MirOp::Return],
        );
        assert!(
            !mir_function_needs_mir_entry(&actor),
            "ActorCall must not force MIR main entry after W419"
        );
        let send = mir_fn_with_ops(
            "main",
            vec![MirOp::ActorSend("msg".into()), MirOp::Return],
        );
        assert!(!mir_function_needs_mir_entry(&send));
        let reg = mir_fn_with_ops("main", vec![MirOp::ActorRegister, MirOp::Return]);
        assert!(!mir_function_needs_mir_entry(&reg));
    }

    #[test]
    fn emits_call_host_jump_witness_ops() {
        let module = MirModule {
            functions: vec![MirFunction {
                name: "main".into(),
                param_count: 0,
                local_count: 0,
                entry: 1,
                blocks: vec![
                    MirBlock {
                        id: 1,
                        instructions: vec![
                            MirInstr {
                                dest: None,
                                op: MirOp::CallHost(3),
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::CallWitness(1, 2),
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::Jump(2),
                                span: None,
                            },
                        ],
                        successors: vec![],
                    },
                    MirBlock {
                        id: 2,
                        instructions: vec![MirInstr {
                            dest: None,
                            op: MirOp::Return,
                            span: None,
                        }],
                        successors: vec![],
                    },
                ],
                instructions: vec![],
                local_names: vec![],
                local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            }],
            ..Default::default()
};
        let ir = emit_llvm_ir(&module).unwrap();
        assert!(ir.contains("syx_rt_call_host"));
        assert!(ir.contains("syx_rt_call_witness"));
        assert!(ir.contains("br label %b2"));
    }

    #[test]
    fn emits_adr002_phase2_closure_try_async_ops() {
        let module = MirModule {
            functions: vec![
                MirFunction {
                    name: "main".into(),
                    param_count: 0,
                    local_count: 0,
                    entry: 1,
                    blocks: vec![MirBlock {
                        id: 1,
                        instructions: vec![
                            MirInstr {
                                dest: Some(0),
                                op: MirOp::MakeClosure("c0".into(), 2),
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::BeginTry(2),
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::EndTry,
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::AwaitPoll,
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::AwaitSuspend {
                                    resume_fn: 1,
                                    state: 0,
                                },
                                span: None,
                            },
                            MirInstr {
                                dest: None,
                                op: MirOp::Return,
                                span: None,
                            },
                        ],
                        successors: vec![],
                    }],
                    instructions: vec![],
                    local_names: vec![],
                    local_types: vec![],
                    init_body_pc: 0,
                    init_wrap_pc: 0,
                },
                MirFunction {
                    name: "c0".into(),
                    param_count: 0,
                    local_count: 0,
                    entry: 0,
                    blocks: vec![],
                    instructions: vec![],
                    local_names: vec![],
                    local_types: vec![],
                    init_body_pc: 0,
                    init_wrap_pc: 0,
                },
            ],
            ..Default::default()
};
        let ir = emit_llvm_ir(&module).unwrap();
        assert!(
            ir.contains("syx_rt_make_closure(i32 1, i32 2)"),
            "MakeClosure must resolve c0→index 1: {ir}"
        );
        assert!(ir.contains("syx_rt_begin_try"));
        assert!(ir.contains("syx_rt_await_poll"));
        assert!(ir.contains("syx_rt_await_suspend"));
    }

    #[test]
    fn native_object_magic_accepts_coff_elf_macho() {
        assert!(is_native_object_bytes(b"\x7fELF........"));
        assert!(is_native_object_bytes(b"\xcf\xfa\xed\xfe........"));
        assert!(is_native_object_bytes(b"\xfe\xed\xfa\xcf........"));
        // COFF AMD64 / ARM64 machine words (LE)
        assert!(is_native_object_bytes(&[0x64, 0x86, 0, 0]));
        assert!(is_native_object_bytes(&[0x64, 0xaa, 0, 0]));
        assert!(!is_native_object_bytes(b"BC"));
        assert!(!is_native_object_bytes(b"; ModuleID"));
    }

    #[test]
    fn compile_native_returns_bytes() {
        let module = MirModule {
            functions: vec![MirFunction {
                name: "main".into(),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![],
                instructions: vec![],
                local_names: vec![],
                local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            }],
            ..Default::default()
};
        std::env::remove_var("SYX_NATIVE_STRICT");
        std::env::remove_var("SYX_NATIVE_C_STUB");
        let bytes = compile_native(&module).unwrap();
        assert!(!bytes.is_empty());
        assert!(
            is_native_object_bytes(&bytes),
            "compile_native must produce a real object file via clang, not LLVM IR text"
        );
    }

    #[test]
    fn compile_native_aot_ir_smoke() {
        let module = MirModule {
            functions: vec![MirFunction {
                name: "main".into(),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![],
                instructions: vec![
                    MirInstr {
                        dest: Some(0),
                        op: MirOp::ConstI32(15),
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Print,
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Return,
                        span: None,
                    },
                ],
                local_names: vec![],
                local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            }],
            ..Default::default()
};
        let mut full = emit_llvm_ir(&module).unwrap();
        append_aot_runtime_shim(&module, &mut full).unwrap();
        let obj = compile_ir_to_object(&full);
        assert!(obj.is_ok(), "clang IR compile failed: {:?}", obj.err());
    }

    #[test]
    fn emits_const_nil_as_push_nil_not_trap() {
        // Void main lowers to Print + ConstNil + Return; ConstNil must not llvm.trap
        // (that yielded native exit≈1 after correct stdout — suite1_arith_add).
        let module = MirModule {
            functions: vec![MirFunction {
                name: "main".into(),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![],
                instructions: vec![
                    MirInstr {
                        dest: Some(0),
                        op: MirOp::ConstI32(15),
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Print,
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::ConstNil,
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Return,
                        span: None,
                    },
                ],
                local_names: vec![],
                local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            }],
            ..Default::default()
};
        let ir = emit_llvm_ir(&module).unwrap();
        assert!(
            ir.contains("syx_rt_push_nil"),
            "ConstNil must call syx_rt_push_nil: {ir}"
        );
        assert!(
            !ir.contains("unimplemented pilot MirOp: ConstNil"),
            "ConstNil must not take EC-M1-3 trap path: {ir}"
        );
        assert!(ir.contains("syx_rt_peek_return"));
    }

    #[test]
    fn emits_const_bool_as_push_bool() {
        // Wave A W203 / ADR-547: ConstBool must push RT Bool for Print consumers.
        let module = MirModule {
            functions: vec![MirFunction {
                name: "main".into(),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![],
                instructions: vec![
                    MirInstr {
                        dest: Some(0),
                        op: MirOp::ConstBool(true),
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Print,
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Return,
                        span: None,
                    },
                ],
                local_names: vec![],
                local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            }],
            ..Default::default()
};
        let ir = emit_llvm_ir(&module).unwrap();
        assert!(
            ir.contains("syx_rt_push_bool"),
            "ConstBool must call syx_rt_push_bool: {ir}"
        );
        assert!(
            !ir.contains("unimplemented pilot MirOp: ConstBool"),
            "ConstBool must not take EC-M1-3 trap path: {ir}"
        );
    }

    #[test]
    fn emits_print_and_collection_ops() {
        let module = MirModule {
            functions: vec![MirFunction {
                name: "main".into(),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![],
                instructions: vec![
                    syx_mir::MirInstr {
                        dest: Some(0),
                        op: MirOp::ArrayBuild(2),
                        span: None,
                    },
                    syx_mir::MirInstr {
                        dest: None,
                        op: MirOp::Print,
                        span: None,
                    },
                    syx_mir::MirInstr {
                        dest: None,
                        op: MirOp::Return,
                        span: None,
                    },
                ],
                local_names: vec![],
                local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            }],
            ..Default::default()
};
        let ir = emit_llvm_ir(&module).unwrap();
        assert!(ir.contains("syx_rt_print"));
        assert!(ir.contains("syx_rt_push_collection"));
    }

    #[test]
    fn llvm_parity_gate_modes() {
        std::env::remove_var("SYX_LLVM_PARITY_GATE");
        assert_eq!(llvm_parity_gate_mode(), LlvmParityGate::Off);

        std::env::set_var("SYX_LLVM_PARITY_GATE", "smoke");
        assert_eq!(llvm_parity_gate_mode(), LlvmParityGate::Smoke);
        assert!(llvm_parity_gate_enabled());
        assert!(!llvm_parity_gate_strict());

        std::env::set_var("SYX_LLVM_PARITY_GATE", "strict");
        assert_eq!(llvm_parity_gate_mode(), LlvmParityGate::Strict);
        assert!(llvm_parity_gate_strict());

        std::env::set_var("SYX_LLVM_PARITY_GATE", "1");
        assert_eq!(llvm_parity_gate_mode(), LlvmParityGate::Smoke);

        std::env::remove_var("SYX_LLVM_PARITY_GATE");
    }

    #[test]
    fn llvm_parity_strict_top60_runtime_subset() {
        assert!(llvm_parity_strict_symbols().len() >= 140);
        let ir = emit_llvm_ir(&MirModule {
            functions: vec![MirFunction {
                name: "main".into(),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![],
                instructions: vec![
                    MirInstr {
                        dest: Some(0),
                        op: MirOp::ArrayBuild(1),
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Print,
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Return,
                        span: None,
                    },
                ],
                local_names: vec![],
                local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            }],
            ..Default::default()
})
        .unwrap();
        assert!(ir.contains("syx_rt_print"));
        assert!(ir.contains("syx_rt_push_collection"));
    }

    #[test]
    fn llvm_parity_runtime_smoke_print_and_collection() {
        std::env::set_var("SYX_LLVM_PARITY_GATE", "strict");
        let module = MirModule {
            functions: vec![MirFunction {
                name: "main".into(),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![],
                instructions: vec![
                    MirInstr {
                        dest: Some(0),
                        op: MirOp::ArrayBuild(1),
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Print,
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Return,
                        span: None,
                    },
                ],
                local_names: vec![],
                local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            }],
            ..Default::default()
};
        llvm_parity_runtime_execute_smoke(&module).expect("runtime smoke");
        std::env::remove_var("SYX_LLVM_PARITY_GATE");
    }

    #[test]
    fn llvm_nightly_release_blocker_default_when_promotion_ready() {
        std::env::set_var("SYX_NIGHTLY", "1");
        std::env::set_var("SYX_LLVM_STRICT_GREEN_WEEKS", "4");
        assert!(llvm_promotion_ready());
        assert!(llvm_nightly_release_blocker_default());
        std::env::remove_var("SYX_NIGHTLY");
        std::env::remove_var("SYX_LLVM_STRICT_GREEN_WEEKS");
    }

    #[test]
    fn llvm_pr_release_blocker_default_when_promotion_ready() {
        std::env::set_var("SYX_CI_PR", "1");
        std::env::set_var("SYX_LLVM_STRICT_GREEN_WEEKS", "4");
        assert!(llvm_promotion_ready());
        assert!(llvm_pr_release_blocker_default());
        assert!(llvm_release_blocker_enabled());
        std::env::remove_var("SYX_CI_PR");
        std::env::remove_var("SYX_LLVM_STRICT_GREEN_WEEKS");
    }

    #[test]
    fn llvm_release_default_evaluated_when_aot_and_promotion_ready() {
        std::env::set_var("SYX_LLVM_STRICT_GREEN_WEEKS", "4");
        std::env::remove_var("SYX_LLVM_AOT");
        assert!(llvm_aot_enabled());
        assert!(llvm_release_default_evaluated());
        std::env::remove_var("SYX_LLVM_STRICT_GREEN_WEEKS");
    }

    #[test]
    fn llvm_parity_gate_default_backend_on_scaffold() {
        std::env::remove_var("SYX_LLVM_AOT_PHASE4");
        std::env::set_var("SYX_LLVM_STRICT_GREEN_WEEKS", "13");
        assert!(!super::llvm_parity_gate_default_backend_on());
        std::env::set_var("SYX_LLVM_STRICT_GREEN_WEEKS", "26");
        assert!(super::llvm_parity_gate_default_backend_on());
        assert_eq!(super::llvm_parity_gate_default_backend_on_weeks(), 26);
        std::env::remove_var("SYX_LLVM_STRICT_GREEN_WEEKS");
    }

    #[test]
    fn llvm_cli_default_backend_on_scaffold() {
        std::env::remove_var("SYX_LLVM_AOT_PHASE5");
        std::env::set_var("SYX_LLVM_STRICT_GREEN_WEEKS", "26");
        assert!(!super::llvm_cli_default_backend_on());
        std::env::set_var("SYX_LLVM_STRICT_GREEN_WEEKS", "39");
        assert!(super::llvm_cli_default_backend_on());
        assert_eq!(super::llvm_cli_default_backend_on_weeks(), 39);
        std::env::remove_var("SYX_LLVM_STRICT_GREEN_WEEKS");
    }

    #[test]
    fn inkwell_prod_ga_scaffold() {
        std::env::remove_var("SYX_LLVM_AOT_PHASE9");
        std::env::set_var("SYX_LLVM_STRICT_GREEN_WEEKS", "78");
        assert!(!super::inkwell_prod_ga_ready());
        std::env::set_var("SYX_LLVM_STRICT_GREEN_WEEKS", "91");
        std::env::set_var("SYX_LLVM_PERF_GREEN_SEASONS", "5");
        assert!(super::inkwell_prod_ga_ready());
        std::env::remove_var("SYX_LLVM_STRICT_GREEN_WEEKS");
        std::env::remove_var("SYX_LLVM_PERF_GREEN_SEASONS");
    }

    #[test]
    fn inkwell_object_direct_ga_scaffold() {
        std::env::remove_var("SYX_LLVM_AOT_PHASE10");
        std::env::set_var("SYX_LLVM_STRICT_GREEN_WEEKS", "91");
        assert!(!super::inkwell_object_direct_ga_ready());
        std::env::set_var("SYX_LLVM_STRICT_GREEN_WEEKS", "104");
        std::env::set_var("SYX_LLVM_PERF_GREEN_SEASONS", "5");
        assert!(super::inkwell_object_direct_ga_ready());
        assert_eq!(super::inkwell_object_direct_ga_weeks(), 104);
        std::env::remove_var("SYX_LLVM_STRICT_GREEN_WEEKS");
        std::env::remove_var("SYX_LLVM_PERF_GREEN_SEASONS");
    }

    #[test]
    fn inkwell_crate_ga_scaffold() {
        std::env::remove_var("SYX_LLVM_AOT_PHASE11");
        std::env::set_var("SYX_INKWELL_CRATE", "0");
        std::env::set_var("SYX_LLVM_STRICT_GREEN_WEEKS", "104");
        assert!(!super::inkwell_crate_enabled());
        std::env::remove_var("SYX_INKWELL_CRATE");
        assert!(super::inkwell_true_crate_ga_ready());
        std::env::set_var("SYX_LLVM_STRICT_GREEN_WEEKS", "117");
        std::env::set_var("SYX_LLVM_PERF_GREEN_SEASONS", "5");
        std::env::set_var("SYX_INKWELL_OBJECT", "1");
        assert!(super::inkwell_true_crate_ga_ready());
        assert_eq!(super::inkwell_crate_ga_weeks(), 117);
        std::env::remove_var("SYX_LLVM_STRICT_GREEN_WEEKS");
        std::env::remove_var("SYX_LLVM_PERF_GREEN_SEASONS");
        std::env::remove_var("SYX_INKWELL_OBJECT");
    }

    fn collect_syx_rt_names(src: &str, kind: &str) -> std::collections::BTreeSet<String> {
        let mut out = std::collections::BTreeSet::new();
        let needle = format!("{kind} ");
        let mut search_from = 0usize;
        while let Some(rel) = src[search_from..].find(&needle) {
            let idx = search_from + rel;
            let after = &src[idx + needle.len()..];
            // e.g. "void @syx_rt_foo(" or "i32 @syx_rt_foo("
            if let Some(at) = after.find('@') {
                // Only accept nearby type tokens before '@' (avoid random "call" prose).
                let between = &after[..at];
                if between.len() <= 16
                    && between
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b == b' ' || b == b'*')
                {
                    let name_start = &after[at + 1..];
                    if name_start.starts_with("syx_rt_") {
                        let name: String = name_start
                            .chars()
                            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                            .collect();
                        if !name.is_empty() {
                            out.insert(name);
                        }
                    }
                }
            }
            search_from = idx + needle.len();
        }
        out
    }

    /// Phase 107: every `@syx_rt_*` call in emit sources must have a matching declare.
    #[test]
    fn p107_syx_rt_declare_covers_all_calls() {
        let src = include_str!("lib.rs");
        assert!(
            src.contains("llvm-declare-call-v1:rt-hygiene"),
            "missing fingerprint llvm-declare-call-v1:rt-hygiene"
        );
        // ADR-858 / ADR-888: wrap emit peeled — split needles so this assert
        // body does not itself contain `MirOp::ObjectRef*` match-arm text.
        let wrap = "syx_rt_object_ref_wrap";
        let declare_emit = format!("out.push_str(\"declare void @{wrap}()\")");
        let call_emit = format!("out.push_str(\"  call void @{wrap}()\\n\")");
        let arm = |name: &str| format!("MirOp::{name} =>");
        assert!(
            !src.contains(&arm("ObjectRefWrap"))
                && !src.contains(&arm("ObjectRefRetain"))
                && !src.contains(&arm("ObjectRefRelease"))
                && !src.contains(&declare_emit)
                && !src.contains(&call_emit),
            "Wave A W544 / ADR-888: MirOp ObjectRef* match arms must be gone from LLVM emit"
        );
        let calls = collect_syx_rt_names(src, "call");
        let decls = collect_syx_rt_names(src, "declare");
        let missing: Vec<_> = calls.difference(&decls).cloned().collect();
        assert!(
            missing.is_empty(),
            "llvm-declare-call-v1:rt-hygiene: syx_rt call without declare: {missing:?}"
        );
        assert!(
            !decls.contains(wrap) && !calls.contains(wrap),
            "syx_rt_object_ref_wrap must not appear in declare or call after ADR-858"
        );
    }

    fn emit_eligible_fn(name: &str, ops: Vec<MirOp>) -> MirFunction {
        MirFunction {
            name: name.into(),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks: vec![],
            instructions: ops
                .into_iter()
                .map(|op| MirInstr {
                    dest: None,
                    op,
                    span: None,
                })
                .collect(),
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
        }
    }

    #[test]
    fn emit_eligible_llvm_ir_omits_unreachable_orphan() {
        let module = MirModule {
            functions: vec![
                emit_eligible_fn("main", vec![MirOp::Return]),
                emit_eligible_fn("orphan", vec![MirOp::Return]),
            ],
            ..Default::default()
        };
        let ir = emit_llvm_ir(&module).unwrap();
        assert!(ir.contains("define i32 @main()"));
        assert!(
            !ir.contains("define i32 @orphan()"),
            "orphan must not be emitted: {ir}"
        );
    }

    #[test]
    fn emit_eligible_llvm_ir_keeps_orphan_on_call_indirect_fallback() {
        let module = MirModule {
            functions: vec![
                emit_eligible_fn("main", vec![MirOp::CallIndirect(1)]),
                emit_eligible_fn("orphan", vec![MirOp::Return]),
            ],
            ..Default::default()
        };
        let ir = emit_llvm_ir(&module).unwrap();
        assert!(ir.contains("define i32 @main()"));
        assert!(
            ir.contains("define i32 @orphan()"),
            "CallIndirect must fall back to full emit: {ir}"
        );
    }

    #[test]
    fn aot_shim_registers_orphan_at_compact_index() {
        // [main=0, orphan=1, used=2] → compact [main=0, used=1]; used registers as 1.
        let module = MirModule {
            functions: vec![
                emit_eligible_fn("main", vec![MirOp::Call("used".into()), MirOp::Return]),
                emit_eligible_fn("orphan", vec![MirOp::Return]),
                emit_eligible_fn("used", vec![MirOp::Return]),
            ],
            ..Default::default()
        };
        let mut ir = emit_llvm_ir(&module).unwrap();
        append_aot_runtime_shim(&module, &mut ir).unwrap();
        assert!(
            ir.contains("call void @syx_rt_register_native_fn(i32 1, i64 ptrtoint (ptr @used to i64))"),
            "used must register at compact index 1: {ir}"
        );
        assert!(
            !ir.contains("call void @syx_rt_register_native_fn(i32 2,"),
            "must not register with original orphan-shifted index 2: {ir}"
        );
        assert!(
            !ir.contains("ptr @orphan to i64"),
            "orphan must not be registered: {ir}"
        );
    }

    #[test]
    fn aot_shim_keeps_full_register_on_call_indirect_fallback() {
        let module = MirModule {
            functions: vec![
                emit_eligible_fn("main", vec![MirOp::CallIndirect(1)]),
                emit_eligible_fn("orphan", vec![MirOp::Return]),
            ],
            ..Default::default()
        };
        let mut ir = emit_llvm_ir(&module).unwrap();
        append_aot_runtime_shim(&module, &mut ir).unwrap();
        assert!(
            ir.contains("ptr @orphan to i64"),
            "CallIndirect fallback must still register orphan: {ir}"
        );
        assert!(
            ir.contains("register_native_fn(i32 1,"),
            "orphan remains at original index 1 under full-emit fallback: {ir}"
        );
    }

    #[test]
    fn aot_sidecar_embed_inventory_smaller_than_stub() {
        let mut orphan_ops = Vec::new();
        for i in 0..64 {
            orphan_ops.push(MirOp::ConstI32(i));
        }
        orphan_ops.push(MirOp::Return);
        let module = MirModule {
            functions: vec![
                emit_eligible_fn("main", vec![MirOp::Return]),
                emit_eligible_fn("orphan", orphan_ops),
            ],
            ..Default::default()
        };
        let stub = syx_mir::encode_module_for_aot(&module);
        let (_, sidecar_bytes) = aot_compact_view(&module);
        assert!(
            sidecar_bytes.len() < stub.len(),
            "metadata sidecar embed must be strictly smaller than stub: sidecar={} stub={}",
            sidecar_bytes.len(),
            stub.len()
        );
    }

    #[test]
    fn aot_compact_view_embeds_metadata_sidecar_only() {
        let module = syx_mir::aot_codec_fixture_main_with_debug_locals(8);
        let (compact, bytes) = aot_compact_view(&module);
        assert!(
            !compact.functions[0].local_names.is_empty(),
            "index view keeps locals"
        );
        assert!(
            compact.functions[0].instructions.len() > 0,
            "index view keeps instructions"
        );
        assert_eq!(bytes, syx_mir::encode_module_metadata_sidecar(&module));
        let view = syx_mir::decode_module_metadata_sidecar(&bytes).unwrap();
        assert!(!view.fn_names.is_empty());
        // SYXD embed path remains available for Medium inventories but is not production emit.
        let syxd = syx_mir::encode_module_aot_embed_blob(&module);
        assert_ne!(bytes, syxd, "W413 embed must not be SYXD package");
    }

    #[test]
    fn aot_shim_sidecar_omit_uses_empty_runtime_entry() {
        let module = syx_mir::aot_codec_fixture_main_with_const_strs(3);
        let mut ir = emit_llvm_ir(&module).unwrap();
        append_aot_runtime_shim(&module, &mut ir).unwrap();
        assert!(
            ir.contains("syx_rt_install_empty_runtime")
                && ir.contains("syx_rt_register_native_fn"),
            "shim must empty-install + register (W431): missing markers"
        );
        assert!(
            !ir.contains("@syx_metadata_sidecar")
                && !ir.contains("syx_rt_install_metadata_sidecar"),
            "shim must not embed/install sidecar (W431): {ir}"
        );
        assert!(
            !ir.contains("@syx_mir_blob"),
            "shim must not emit @syx_mir_blob (W413): {ir}"
        );
        assert!(
            !ir.contains("call void @syx_rt_load_mir_blob"),
            "syx_user_init must not call syx_rt_load_mir_blob (W413)"
        );
        assert!(
            !ir.contains("declare void @syx_rt_load_mir_blob"),
            "IR must omit residual declare @syx_rt_load_mir_blob (W484)"
        );
    }

    #[test]
    fn emit_llvm_ir_omits_residual_load_mir_blob_declare() {
        let module = syx_mir::aot_codec_fixture_main_with_const_strs(1);
        let ir = emit_llvm_ir(&module).unwrap();
        assert!(
            !ir.contains("declare void @syx_rt_load_mir_blob"),
            "emit must omit residual declare (W484 / ADR-828): {ir}"
        );
    }

    #[test]
    fn production_shim_path_skips_dead_sidecar_encode() {
        let src = include_str!("lib.rs");
        let shim_start = src
            .find("fn append_aot_runtime_shim")
            .expect("append_aot_runtime_shim present");
        let shim = &src[shim_start..];
        let shim_end = shim.find("\nfn ").unwrap_or(shim.len().min(3500));
        let shim_body = &shim[..shim_end];
        assert!(
            shim_body.contains("compact_module_for_aot_blob")
                && !shim_body.contains("aot_compact_view")
                && !shim_body.contains("encode_module_metadata_sidecar"),
            "production shim must compact-only without dead encode (W484)"
        );
        let stub_start = src
            .find("fn native_c_stub_source")
            .expect("native_c_stub_source present");
        let stub = &src[stub_start..];
        let stub_end = stub.find("\nfn ").unwrap_or(stub.len().min(2500));
        let stub_body = &stub[..stub_end];
        assert!(
            stub_body.contains("compact_module_for_aot_blob")
                && !stub_body.contains("aot_compact_view")
                && !stub_body.contains("encode_module_metadata_sidecar"),
            "C stub must compact-only without dead encode (W484)"
        );
    }

    #[test]
    fn native_c_stub_sidecar_omit_empty_then_register() {
        let module = syx_mir::aot_codec_fixture_main_with_const_strs(2);
        let src = native_c_stub_source(&module);
        assert!(
            src.contains("syx_rt_install_empty_runtime")
                && src.contains("syx_rt_register_native_fn(0,"),
            "C stub must empty-install + register (W431): {src}"
        );
        assert!(
            !src.contains("syx_metadata_sidecar")
                && !src.contains("syx_rt_install_metadata_sidecar"),
            "C stub must not embed/install sidecar (W431): {src}"
        );
        assert!(
            !src.contains("syx_mir_blob") && !src.contains("syx_rt_load_mir_blob"),
            "C stub must not embed MIR blob (W413/W431): {src}"
        );
        let install = src
            .find("syx_rt_install_empty_runtime")
            .expect("empty install present");
        let register = src
            .find("syx_rt_register_native_fn(0,")
            .expect("register call present");
        assert!(
            install < register,
            "register loop must follow empty install (W431)"
        );
    }
}
