//! Math llvm GA exit first-slice (W965 / ADR-1309).
//! Narrow claimed: math_llvm_ga=true on GA-exit inventory; soft-close keeps false;
//! MathFloat* 84/84 + native llvm smokes rollup; ≠ QS1 Math / ≠ status=implemented / ≠ F-08.

#[test]
fn math_llvm_ga_exit_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-driver/src/lib.rs",
        "crates/syx-driver/tests/stdlib/math.rs",
        "packages/internal/host_fn_capabilities.toml",
        "docs/lang/math-llvm-ga-exit-first-slice.md",
        "docs/lang/math-llvm-f32-soft-close-first-slice.md",
        "docs/lang/math-residual-maintenance-first-slice.md",
        "plan/topics/ga-evidence-audit.md",
        "plan/02-improvement-roadmap.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Math llvm GA exit first-slice requires evidence file: {path}"
        );
    }
    let driver = include_str!("../../src/lib.rs");
    assert!(
        driver.contains("SyxMathLlvmGaExitFirstSliceInventory")
            && driver.contains("syx_math_llvm_ga_exit_first_slice_inventory")
            && driver.contains("ADR-1309")
            && driver.contains("f32_soft_close_floor_ok")
            && driver.contains("native_llvm_smokes_all_present")
            && (driver.contains("syx_math_llvm_ga_exit_first_slice_signed: true")
                || driver.contains("syx_math_llvm_ga_exit_first_slice_signed:true")),
        "syx-driver must expose SyxMathLlvmGaExitFirstSliceInventory (W965 / ADR-1309)"
    );
    assert!(
        driver.contains("W965")
            && (driver.contains("math_llvm_ga: true") || driver.contains("math_llvm_ga:true")),
        "GA-exit inventory must claim math_llvm_ga true (W965)"
    );
    let math = include_str!("../stdlib/math.rs");
    for name in [
        "fn math_host_smoke_native_llvm",
        "fn math_hyperbolic_smoke_native_llvm",
        "fn math_soft_smoke_native_llvm",
        "fn math_special_smoke_native_llvm",
        "fn math_f32_smoke_native_llvm",
        "fn math_f32_slice2_smoke_native_llvm",
        "fn math_f32_slice3_smoke_native_llvm",
        "fn math_f32_slice4_smoke_native_llvm",
        "fn math_f32_slice5_smoke_native_llvm",
        "fn math_f32_soft_close_smoke_native_llvm",
    ] {
        assert!(
            math.contains(name),
            "math.rs must nail native llvm smoke: {name}"
        );
    }
    let caps = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let mut unsupported = Vec::new();
    for block in caps.split("[[host]]") {
        if !block.contains("name = \"MathFloat") {
            continue;
        }
        let name = block
            .lines()
            .find_map(|l| {
                l.strip_prefix("name = \"")
                    .and_then(|rest| rest.strip_suffix('"'))
            })
            .unwrap_or("?");
        if let Some(backends) = block
            .lines()
            .find(|l| l.trim_start().starts_with("backends = {"))
        {
            if backends.contains("llvm = \"unsupported\"") {
                unsupported.push(name.to_string());
            }
        }
    }
    assert!(
        unsupported.is_empty(),
        "no MathFloat* may remain llvm=unsupported after W964/W965: {unsupported:?}"
    );
}

#[test]
fn math_llvm_ga_exit_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/lang/math-llvm-ga-exit-first-slice.md");
    assert!(
        spec.contains("ADR-1309")
            && (spec.contains("GA exit")
                || spec.contains("ga exit")
                || spec.contains("math_llvm_ga=true")
                || spec.contains("`math_llvm_ga=true`")),
        "spec must cite ADR-1309 Math llvm GA exit"
    );
    assert!(
        spec.contains("eight-hundred-and-fifteenth"),
        
        "spec must cite W965 fingerprint"
    );
    assert!(
        spec.contains("SyxMathLlvmGaExitFirstSliceInventory")
            && spec.contains("syx_math_llvm_ga_exit_first_slice_inventory"),
        "spec must name GA-exit inventory API"
    );
    assert!(
        spec.contains("syx_math_llvm_ga_exit_first_slice_signed=true")
            || spec.contains("`syx_math_llvm_ga_exit_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("math_llvm_ga=true") || spec.contains("`math_llvm_ga=true`"))
            && (spec.contains("84/84") || spec.contains("math_float_llvm_all_real")),
        "spec must cite math_llvm_ga=true and 84/84"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("QS1 Math")
                || spec.contains("F-08")
                || spec.contains("status=implemented")),
        "spec Frozen must fence QS1 Math / F-08 / status=implemented"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn math_llvm_ga_exit_first_slice_inventory_nails() {
    let inv = syx_driver::syx_math_llvm_ga_exit_first_slice_inventory();
    assert!(inv.f32_soft_close_floor_ok);
    assert!(inv.native_llvm_smokes_all_present);
    assert!(inv.math_float_llvm_all_real);
    assert!(inv.math_llvm_ga);
    assert!(inv.syx_math_llvm_ga_exit_first_slice_signed);
    let soft = syx_driver::syx_math_llvm_f32_soft_close_first_slice_inventory();
    assert!(soft.syx_math_llvm_f32_soft_close_first_slice_signed);
    assert!(soft.math_float_llvm_all_real);
    assert!(!soft.math_llvm_ga);
}

