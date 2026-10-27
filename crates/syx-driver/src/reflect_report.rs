//! Phase 77: generic specialization reflect report for `syx doctor --reflect-report`.

#[derive(Debug, Clone, Default)]
pub struct ReflectReport {
    pub specialization_api: bool,
    pub sample_fn: String,
    pub witness_hit_rate: f64,
    /// False when a forged on-miss `has_witness: true` synthesis path is still present.
    pub ok: bool,
}

/// Phase 104: detect removed-but-forbidden runtime mono cache witness synthesis.
pub fn reflect_forged_witness_path_present() -> bool {
    let meta = include_str!("../../syx-meta/src/lib.rs");
    meta.contains("param_count: type_args.len()")
        && meta.contains("has_witness: true")
        && meta.contains("Some(MirMonoSig")
}

/// Phase 105: generic / reflect specialization usage in shipped examples and HOF gates.
pub fn reflect_usage_present() -> bool {
    // Prefer in-crate fixtures (examples paths vary by checkout layout).
    let hof = include_str!("../tests/runtime/hof_generic_witness.rs");
    hof.contains("reflect(")
        || hof.contains("__syx_reflect_specialization")
        || hof.contains(".sorted(")
        || hof.contains("__syx_array_sort_by")
}

pub fn run_reflect_report() -> ReflectReport {
    let forged = reflect_forged_witness_path_present();
    let reflect_usage = reflect_usage_present();
    let sample = crate::reflect_specialization_info("Array_len", &["Int"]);
    let witness_hit = sample.as_ref().is_some_and(|s| s.has_witness);
    let witness_hit_rate = if witness_hit { 1.0 } else { 0.0 };
    ReflectReport {
        specialization_api: include_str!("../../syx-meta/src/lib.rs")
            .contains("reflect_specialization"),
        sample_fn: sample.map(|s| s.fn_name.to_string()).unwrap_or_default(),
        witness_hit_rate,
        ok: !forged && !(reflect_usage && witness_hit_rate == 0.0),
    }
}

pub fn print_reflect_report(report: &ReflectReport) {
    println!("SYX reflect report (Phase 105)");
    println!(
        "  reflect_specialization API: {}",
        if report.specialization_api {
            "yes"
        } else {
            "no"
        }
    );
    println!("  sample fn: {}", report.sample_fn);
    println!(
        "  witness hit rate: {:.0}%",
        report.witness_hit_rate * 100.0
    );
    if !report.ok {
        if reflect_forged_witness_path_present() {
            eprintln!("syx doctor: reflect witness forged path still present (runtime_mono_cache miss must not synthesize has_witness: true)");
        } else if reflect_usage_present() && report.witness_hit_rate == 0.0 {
            eprintln!("syx doctor: reflect usage present but witness hit rate is 0% (reflect specialization GA)");
        }
    }
}
