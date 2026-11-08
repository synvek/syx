//! S3-I1…I41 S3-01 diagnostic Span census + face gate contracts.
//! KPIs: prior bare = 0 + miette=1 + … + residual gates = 0 + filtered helper_calls = 0;
//! S3-01 [x] claimed (I41); ≠ Stage-3 exit.
//! See docs/adr/stage3-s3-01-claim.md · docs/stage3-diagnostic-span-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn bare_diag_count(src: &str) -> usize {
    src.matches("SyxDiagnostic::error(").count() + src.matches("SyxDiagnostic::warning(").count()
}

#[test]
fn s3_01_span_census_baseline() {
    let census = repo_file("docs/stage3-diagnostic-span-census.md");
    assert!(census.contains("S3-I1"));
    assert!(census.contains("SyxDiagnostic") && census.contains("primary_span"));
    assert!(census.contains("IR") || census.contains("syx-mir"));
    assert!(census.contains("≠") && census.contains("S3-01"));
    assert!(
        census.contains("S3-I40")
            && (census.contains("noise") || census.contains("claim-prep") || census.contains("helper_calls")),
        "census must name S3-I40 noise/claim-prep gate"
    );
    assert!(
        census.contains("S3-I41")
            && (census.contains("claim") || census.contains("[x]")),
        "census must name S3-I41 S3-01 claim"
    );
    assert!(
        census.contains("S3-I42")
            && (census.contains("S3-02") || census.contains("open")),
        "census must name S3-I42 S3-02 open"
    );
    assert!(
        census.contains("S3-I43")
            && (census.contains("demangle") || census.contains("construct") || census.contains("S3-02")),
        "census must name S3-I43 demangle"
    );
    assert!(
        census.contains("S3-I44")
            && (census.contains("debug") || census.contains("stack") || census.contains("S3-02")),
        "census must name S3-I44 debug stack"
    );
    assert!(
        census.contains("S3-I45")
            && (census.contains("runtime") || census.contains("__type")),
        "census must name S3-I45 runtime"
    );
    assert!(
        census.contains("S3-I46")
            && (census.contains("meta") || census.contains("S3-02")),
        "census must name S3-I46 meta"
    );
    assert!(
        census.contains("S3-I47")
            && census.contains("claim"),
        "census must name S3-I47 claim"
    );
    assert!(
        census.contains("S3-03"),
        "census ladder must name S3-03 next"
    );
    assert!(
        census.contains("Current:** S3-I47") || census.contains("Current: S3-I47"),
        "census Current must be S3-I47"
    );
    assert!(
        census.contains("host_residual_bare") && census.contains("**0**"),
        "census must record host_residual_bare = 0"
    );
    assert!(
        census.contains("positionless_helper_calls") && census.contains("**0**"),
        "census must record positionless_helper_calls = 0"
    );
    assert!(
        census.contains("positionless_helper_noise"),
        "census must publish positionless_helper_noise"
    );
    assert!(
        census.contains("positionless_helper_policy"),
        "census must publish positionless_helper_policy"
    );
    assert!(
        census.contains("positionless_parser_driver_bare") && census.contains("**0**"),
        "census must record positionless_parser_driver_bare = 0"
    );
    assert!(
        census.contains("positionless_driver_compile_bare") && census.contains("**0**"),
        "census must record positionless_driver_compile_bare = 0"
    );
    assert!(
        census.contains("positionless_driver_emit_test_native_bare") && census.contains("**0**"),
        "census must record positionless_driver_emit_test_native_bare = 0"
    );
    assert!(
        census.contains("positionless_vm_phase84_bare") && census.contains("**0**"),
        "census must record positionless_vm_phase84_bare = 0"
    );
    assert!(
        census.contains("positionless_vm_hosts_bare") && census.contains("**0**"),
        "census must record positionless_vm_hosts_bare = 0"
    );
    assert!(
        census.contains("positionless_vm_phase89_bare") && census.contains("**0**"),
        "census must record positionless_vm_phase89_bare = 0"
    );
    assert!(
        census.contains("positionless_vm_phase120_bare") && census.contains("**0**"),
        "census must record positionless_vm_phase120_bare = 0"
    );
    assert!(
        census.contains("positionless_vm_http_bare") && census.contains("**0**"),
        "census must record positionless_vm_http_bare = 0"
    );
    assert!(
        census.contains("positionless_vm_os_bare") && census.contains("**0**"),
        "census must record positionless_vm_os_bare = 0"
    );
    assert!(
        census.contains("positionless_vm_lib_bare") && census.contains("**0**"),
        "census must record positionless_vm_lib_bare = 0"
    );
    assert!(
        census.contains("positionless_vm_phase83_bare") && census.contains("**0**"),
        "census must record positionless_vm_phase83_bare = 0"
    );
    assert!(
        census.contains("positionless_vm_channel_bare") && census.contains("**0**"),
        "census must record positionless_vm_channel_bare = 0"
    );
    assert!(
        census.contains("positionless_vm_phase_tail_bare") && census.contains("**0**"),
        "census must record positionless_vm_phase_tail_bare = 0"
    );
    assert!(
        census.contains("positionless_vm_mid_bare") && census.contains("**0**"),
        "census must record positionless_vm_mid_bare = 0"
    );
    assert!(
        census.contains("positionless_wasm_run_bare") && census.contains("**0**"),
        "census must record positionless_wasm_run_bare = 0"
    );
    assert!(
        census.contains("positionless_pkg_bare") && census.contains("**0**"),
        "census must record positionless_pkg_bare = 0"
    );
    assert!(
        census.contains("positionless_macro_bare") && census.contains("**0**"),
        "census must record positionless_macro_bare = 0"
    );
    assert!(
        census.contains("positionless_async_bare") && census.contains("**0**"),
        "census must record positionless_async_bare = 0"
    );
    assert!(
        census.contains("positionless_llvm_bare") && census.contains("**0**"),
        "census must record positionless_llvm_bare = 0"
    );
    assert!(
        census.contains("positionless_rt_bare") && census.contains("**0**"),
        "census must record positionless_rt_bare = 0"
    );
    assert!(
        census.contains("positionless_jit_bare") && census.contains("**0**"),
        "census must record positionless_jit_bare = 0"
    );
    assert!(
        census.contains("positionless_debug_bare") && census.contains("**0**"),
        "census must record positionless_debug_bare = 0"
    );
    assert!(
        census.contains("positionless_codegen_bare") && census.contains("**0**"),
        "census must record positionless_codegen_bare = 0"
    );
    for kpi in [
        "positionless_mir_bare",
        "positionless_mir_run_bare",
        "positionless_meta_bare",
        "positionless_hir_bare",
        "positionless_cst_bare",
        "positionless_wasm_emit_bare",
        "positionless_fmt_bare",
        "positionless_compiler_wasm_bare",
        "positionless_lexer_bare",
        "positionless_lint_bare",
    ] {
        assert!(
            census.contains(kpi) && census.contains("**0**"),
            "census must record {kpi} = 0"
        );
    }
    assert!(
        census.contains("miette_help_impl") && census.contains("**1**"),
        "census must record miette_help_impl = 1"
    );
    assert!(
        census.contains("with_span_label_wired") && census.contains("**1**"),
        "census must record with_span_label_wired = 1"
    );

    let adr = repo_file("docs/adr/stage3-open.md");
    assert!(adr.contains("S3-I1"));
    assert!(adr.contains("Open Stage-3") || adr.contains("**Open Stage-3**"));
    assert!(adr.contains("Open S3-01") || adr.contains("**Open S3-01**"));
    assert!(adr.contains("≠") && adr.contains("S3-01"));
    assert!(adr.contains("≠") && adr.contains("Stage-3"));
    assert!(
        adr.contains("S3-I35") && adr.contains("stage3-s3-01-positionless-pkg"),
        "stage3-open must cite landed S3-I35 ADR"
    );
    assert!(
        adr.contains("S3-I36") && adr.contains("stage3-s3-01-positionless-macro"),
        "stage3-open must cite landed S3-I36 ADR"
    );
    assert!(
        adr.contains("S3-I37") && adr.contains("stage3-s3-01-positionless-async-llvm"),
        "stage3-open must cite landed S3-I37 ADR"
    );
    assert!(
        adr.contains("S3-I38") && adr.contains("stage3-s3-01-positionless-rt-jit-debug"),
        "stage3-open must cite landed S3-I38 ADR"
    );
    assert!(
        adr.contains("S3-I39") && adr.contains("stage3-s3-01-positionless-residual"),
        "stage3-open must cite landed S3-I39 ADR"
    );
    assert!(
        adr.contains("S3-I40") && adr.contains("stage3-s3-01-noise-claim-prep"),
        "stage3-open must cite landed S3-I40 ADR"
    );
    assert!(
        adr.contains("S3-I41") && adr.contains("stage3-s3-01-claim"),
        "stage3-open must cite landed S3-I41 claim ADR"
    );
    assert!(
        adr.contains("S3-I42") && adr.contains("stage3-s3-02-open"),
        "stage3-open must cite landed S3-I42 open ADR"
    );
    assert!(
        adr.contains("S3-I43") && adr.contains("stage3-s3-02-demangle-semantic"),
        "stage3-open must cite landed S3-I43 demangle ADR"
    );
    assert!(
        adr.contains("S3-I44") && adr.contains("stage3-s3-02-debug-stack-display"),
        "stage3-open must cite landed S3-I44 debug ADR"
    );
    assert!(
        adr.contains("S3-I45") && adr.contains("stage3-s3-02-runtime-type-display"),
        "stage3-open must cite landed S3-I45 runtime ADR"
    );
    assert!(
        adr.contains("S3-I46") && adr.contains("stage3-s3-02-meta-type-display"),
        "stage3-open must cite landed S3-I46 meta ADR"
    );
    assert!(
        adr.contains("S3-I47") && adr.contains("stage3-s3-02-claim"),
        "stage3-open must cite landed S3-I47 claim ADR"
    );
    assert!(
        adr.contains("S3-03"),
        "stage3-open must lock next = S3-03"
    );

    let macro_adr = repo_file("docs/adr/stage3-s3-01-span-host-macro.md");
    assert!(macro_adr.contains("S3-I13"));
    assert!(
        macro_adr.contains("S3-I14")
            && (macro_adr.contains("hosts") || macro_adr.contains("vm")),
        "macro ADR Consequences must cite landed S3-I14"
    );

    let vm_adr = repo_file("docs/adr/stage3-s3-01-span-host-vm-hosts.md");
    assert!(vm_adr.contains("S3-I14"));
    assert!(
        vm_adr.contains("host_vm_hosts_bare") || vm_adr.contains("vm_error"),
        "vm-hosts ADR must decide hosts bare→0 / KPI"
    );
    assert!(vm_adr.contains("≠") && vm_adr.contains("S3-01"));
    assert!(
        vm_adr.contains("S3-I15") && vm_adr.contains("phase89"),
        "vm-hosts ADR must cite landed S3-I15 phase89"
    );
    assert!(
        vm_adr.contains("do **not** change") && vm_adr.contains("primary_span: None")
    );

    let phase89_adr = repo_file("docs/adr/stage3-s3-01-span-host-vm-phase89.md");
    assert!(phase89_adr.contains("S3-I15"));
    assert!(
        phase89_adr.contains("host_vm_phase89_bare") || phase89_adr.contains("vm_error"),
        "phase89 ADR must decide phase89 bare→0 / KPI"
    );
    assert!(phase89_adr.contains("≠") && phase89_adr.contains("S3-01"));
    assert!(
        phase89_adr.contains("S3-I16") && phase89_adr.contains("phase84"),
        "phase89 ADR must cite landed S3-I16 phase84"
    );
    assert!(
        phase89_adr.contains("do **not** change") && phase89_adr.contains("primary_span: None")
    );

    let phase84_adr = repo_file("docs/adr/stage3-s3-01-span-host-vm-phase84.md");
    assert!(phase84_adr.contains("S3-I16"));
    assert!(
        phase84_adr.contains("host_vm_phase84_bare") || phase84_adr.contains("vm_error"),
        "phase84 ADR must decide phase84 bare→0 / KPI"
    );
    assert!(phase84_adr.contains("≠") && phase84_adr.contains("S3-01"));
    assert!(
        phase84_adr.contains("S3-I17") && phase84_adr.contains("phase120"),
        "phase84 ADR must cite landed S3-I17 phase120"
    );
    assert!(
        phase84_adr.contains("do **not** change") && phase84_adr.contains("primary_span: None")
    );

    let phase120_adr = repo_file("docs/adr/stage3-s3-01-span-host-vm-phase120.md");
    assert!(phase120_adr.contains("S3-I17"));
    assert!(
        phase120_adr.contains("host_vm_phase120_bare") || phase120_adr.contains("vm_error"),
        "phase120 ADR must decide phase120 bare→0 / KPI"
    );
    assert!(phase120_adr.contains("≠") && phase120_adr.contains("S3-01"));
    assert!(
        phase120_adr.contains("S3-I18") && phase120_adr.contains("phase83"),
        "phase120 ADR must cite landed S3-I18 phase83"
    );
    assert!(
        phase120_adr.contains("do **not** change") && phase120_adr.contains("primary_span: None")
    );

    let phase83_adr = repo_file("docs/adr/stage3-s3-01-span-host-vm-phase83.md");
    assert!(phase83_adr.contains("S3-I18"));
    assert!(
        phase83_adr.contains("host_vm_phase83_bare") || phase83_adr.contains("vm_error"),
        "phase83 ADR must decide phase83 bare→0 / KPI"
    );
    assert!(phase83_adr.contains("≠") && phase83_adr.contains("S3-01"));
    assert!(
        phase83_adr.contains("S3-I19") && phase83_adr.contains("phase-tail"),
        "phase83 ADR must cite landed S3-I19 phase-tail"
    );
    assert!(
        phase83_adr.contains("do **not** change") && phase83_adr.contains("primary_span: None")
    );

    let phase_tail_adr = repo_file("docs/adr/stage3-s3-01-span-host-vm-phase-tail.md");
    assert!(phase_tail_adr.contains("S3-I19"));
    assert!(
        phase_tail_adr.contains("host_vm_phase_tail_bare") || phase_tail_adr.contains("vm_error"),
        "phase-tail ADR must decide phase-tail bare→0 / KPI"
    );
    assert!(phase_tail_adr.contains("≠") && phase_tail_adr.contains("S3-01"));
    assert!(
        phase_tail_adr.contains("S3-I20") && phase_tail_adr.contains("host-vm-mid"),
        "phase-tail ADR must cite landed S3-I20 mid"
    );
    assert!(
        phase_tail_adr.contains("do **not** change")
            && phase_tail_adr.contains("primary_span: None")
    );

    let mid_adr = repo_file("docs/adr/stage3-s3-01-span-host-vm-mid.md");
    assert!(mid_adr.contains("S3-I20"));
    assert!(
        mid_adr.contains("host_vm_mid_bare") || mid_adr.contains("vm_error"),
        "mid ADR must decide mid bare→0 / KPI"
    );
    assert!(mid_adr.contains("≠") && mid_adr.contains("S3-01"));
    assert!(
        mid_adr.contains("S3-I21") && mid_adr.contains("wasm_stubs"),
        "mid ADR must cite landed S3-I21 wasm_stubs"
    );
    assert!(
        mid_adr.contains("do **not** change") && mid_adr.contains("primary_span: None")
    );

    let stubs_adr = repo_file("docs/adr/stage3-s3-01-span-host-vm-wasm-stubs.md");
    assert!(stubs_adr.contains("S3-I21"));
    assert!(
        stubs_adr.contains("host_vm_wasm_stubs_bare") || stubs_adr.contains("vm_error"),
        "wasm-stubs ADR must decide stubs bare→0 / KPI"
    );
    assert!(stubs_adr.contains("≠") && stubs_adr.contains("S3-01"));
    assert!(
        stubs_adr.contains("S3-I22")
            && (stubs_adr.contains("rt") || stubs_adr.contains("cranelift")),
        "wasm-stubs ADR must cite landed S3-I22 rt/cranelift/debug"
    );
    assert!(
        stubs_adr.contains("do **not** change") && stubs_adr.contains("primary_span: None")
    );

    let rt_adr = repo_file("docs/adr/stage3-s3-01-span-host-rt-jit-debug.md");
    assert!(rt_adr.contains("S3-I22"));
    assert!(
        rt_adr.contains("host_rt_jit_debug_bare")
            || rt_adr.contains("rt_error")
            || rt_adr.contains("jit_error"),
        "rt-jit-debug ADR must decide bare→0 / KPI"
    );
    assert!(rt_adr.contains("≠") && rt_adr.contains("S3-01"));
    assert!(
        rt_adr.contains("S3-I23") && rt_adr.contains("residual"),
        "rt-jit-debug ADR must cite landed S3-I23 residual"
    );
    assert!(
        rt_adr.contains("do **not** change") && rt_adr.contains("primary_span: None")
    );

    let residual_adr = repo_file("docs/adr/stage3-s3-01-span-host-residual.md");
    assert!(residual_adr.contains("S3-I23"));
    assert!(
        residual_adr.contains("host_residual_bare")
            || residual_adr.contains("mir_run_diag")
            || residual_adr.contains("mir_diag"),
        "residual ADR must decide bare→0 / KPI"
    );
    assert!(residual_adr.contains("≠") && residual_adr.contains("S3-01"));
    assert!(
        residual_adr.contains("S3-I24") && residual_adr.contains("claim"),
        "residual ADR must cite landed S3-I24 claim prep"
    );
    assert!(
        residual_adr.contains("do **not** change")
            && residual_adr.contains("primary_span: None")
    );

    let claim_adr = repo_file("docs/adr/stage3-s3-01-claim-prep.md");
    assert!(claim_adr.contains("S3-I24"));
    assert!(
        claim_adr.contains("positionless_helper_calls")
            && claim_adr.contains("miette_help_impl")
            && claim_adr.contains("with_span_label_wired"),
        "claim-prep ADR must name quality KPIs"
    );
    assert!(claim_adr.contains("≠") && claim_adr.contains("S3-01"));
    assert!(
        claim_adr.contains("S3-I25") && claim_adr.contains("miette-help-label"),
        "claim-prep ADR must cite landed S3-I25 miette help/label"
    );
    assert!(
        claim_adr.contains("S3-I26") && claim_adr.contains("positionless-parser-driver"),
        "claim-prep ADR must cite landed S3-I26 positionless parser/driver"
    );
    assert!(
        claim_adr.contains("S3-I27") && claim_adr.contains("positionless-driver-compile"),
        "claim-prep ADR must cite landed S3-I27 positionless driver compile"
    );
    assert!(
        claim_adr.contains("do **not**") && claim_adr.contains("primary_span: None")
            || claim_adr.contains("Do not") && claim_adr.contains("primary_span: None"),
        "claim-prep ADR must forbid changing diagnostics defaults"
    );

    let miette_adr = repo_file("docs/adr/stage3-s3-01-miette-help-label.md");
    assert!(miette_adr.contains("S3-I25"));
    assert!(
        miette_adr.contains("miette_help_impl") && miette_adr.contains("with_span_label_wired"),
        "miette ADR must decide help/label KPI=1"
    );
    assert!(miette_adr.contains("≠") && miette_adr.contains("S3-01"));
    assert!(
        miette_adr.contains("S3-I26") && miette_adr.contains("positionless-parser-driver"),
        "miette ADR must cite landed S3-I26 positionless"
    );
    assert!(
        miette_adr.contains("S3-I27") && miette_adr.contains("positionless-driver-compile"),
        "miette ADR must cite landed S3-I27 driver compile"
    );
    assert!(
        miette_adr.contains("S3-I28") && miette_adr.contains("positionless-driver-emit-test-native"),
        "miette ADR must cite landed S3-I28 emit/test/native"
    );
    assert!(
        miette_adr.contains("S3-I29") && miette_adr.contains("positionless-vm-phase84"),
        "miette ADR must cite landed S3-I29 vm phase84"
    );
    assert!(
        miette_adr.contains("S3-I30") && miette_adr.contains("positionless-vm-hosts"),
        "miette ADR must cite landed S3-I30 vm hosts"
    );
    assert!(
        miette_adr.contains("S3-I31") && miette_adr.contains("positionless-vm-phase89"),
        "miette ADR must cite landed S3-I31 vm phase89"
    );
    assert!(
        miette_adr.contains("S3-I32") && miette_adr.contains("positionless-vm-phase120"),
        "miette ADR must cite landed S3-I32 vm phase120"
    );
    assert!(
        miette_adr.contains("S3-I33") && miette_adr.contains("positionless-vm-lib"),
        "miette ADR must cite landed S3-I33 vm lib"
    );
    assert!(
        miette_adr.contains("S3-I34") && miette_adr.contains("positionless-vm-mid"),
        "miette ADR must cite landed S3-I34 vm mid"
    );
    assert!(
        miette_adr.contains("S3-I35") && miette_adr.contains("positionless-pkg"),
        "miette ADR must cite landed S3-I35 pkg"
    );
    assert!(
        miette_adr.contains("S3-I36") && miette_adr.contains("positionless-macro"),
        "miette ADR must cite landed S3-I36 macro"
    );
    assert!(
        miette_adr.contains("S3-I37") && miette_adr.contains("positionless-async-llvm"),
        "miette ADR must cite landed S3-I37 async-llvm"
    );
    assert!(
        miette_adr.contains("S3-I38") && miette_adr.contains("positionless-rt-jit-debug"),
        "miette ADR must cite landed S3-I38 rt-jit-debug"
    );
    assert!(
        miette_adr.contains("S3-I39") && miette_adr.contains("positionless-residual"),
        "miette ADR must cite landed S3-I39 residual"
    );
    assert!(
        miette_adr.contains("S3-I40") && miette_adr.contains("noise-claim-prep"),
        "miette ADR must cite landed S3-I40 noise-claim-prep"
    );
    assert!(
        miette_adr.contains("S3-I41")
            && (miette_adr.contains("claim") || miette_adr.contains("[x]")),
        "miette ADR must cite S3-I41 claim"
    );
    assert!(
        miette_adr.contains("S3-I42") && (miette_adr.contains("S3-02") || miette_adr.contains("open")),
        "miette ADR must cite S3-I42 open"
    );
    assert!(
        miette_adr.contains("S3-I43") && (miette_adr.contains("construct") || miette_adr.contains("demangle") || miette_adr.contains("S3-02")),
        "miette_adr must cite S3-I43"
    );
    assert!(
        miette_adr.contains("S3-I44") && (miette_adr.contains("debug") || miette_adr.contains("stack") || miette_adr.contains("S3-02")),
        "miette_adr must cite S3-I44"
    );
    assert!(
        miette_adr.contains("S3-I45") && (miette_adr.contains("runtime") || miette_adr.contains("__type")),
        "miette_adr must cite S3-I45 runtime"
        );
        assert!(
            miette_adr.contains("S3-I46") && (miette_adr.contains("meta") || miette_adr.contains("S3-02") || miette_adr.contains("claim")),
        "miette_adr must cite S3-I46 meta"
        );
        assert!(
            miette_adr.contains("S3-I47") && (miette_adr.contains("claim") || miette_adr.contains("S3-02")),
        "miette_adr must cite S3-I47 claim"
        );
        assert!(
            miette_adr.contains("S3-03"),
        "miette_adr must lock next = S3-03"
    );
    assert!(
        miette_adr.contains("do not") && miette_adr.contains("primary_span: None")
            || miette_adr.contains("do **not**") && miette_adr.contains("primary_span"),
        "miette ADR must forbid changing primary_span defaults"
    );

    let pos_adr = repo_file("docs/adr/stage3-s3-01-positionless-parser-driver.md");
    assert!(pos_adr.contains("S3-I26"));
    assert!(
        pos_adr.contains("positionless_parser_driver_bare")
            && (pos_adr.contains("=0") || pos_adr.contains("= 0")),
        "positionless ADR must decide parser/driver bare KPI=0"
    );
    assert!(pos_adr.contains("≠") && pos_adr.contains("S3-01"));
    assert!(
        pos_adr.contains("S3-I27") && pos_adr.contains("positionless-driver-compile"),
        "parser/driver ADR must cite landed S3-I27"
    );
    assert!(
        pos_adr.contains("S3-I28") && pos_adr.contains("positionless-driver-emit-test-native"),
        "parser/driver ADR must cite landed S3-I28"
    );
    assert!(
        pos_adr.contains("S3-I29") && pos_adr.contains("positionless-vm-phase84"),
        "parser/driver ADR must cite landed S3-I29"
    );
    assert!(
        pos_adr.contains("S3-I30") && pos_adr.contains("positionless-vm-hosts"),
        "parser/driver ADR must cite landed S3-I30"
    );
    assert!(
        pos_adr.contains("S3-I31") && pos_adr.contains("positionless-vm-phase89"),
        "parser/driver ADR must cite landed S3-I31"
    );
    assert!(
        pos_adr.contains("S3-I32") && pos_adr.contains("positionless-vm-phase120"),
        "parser/driver ADR must cite landed S3-I32"
    );
    assert!(
        pos_adr.contains("S3-I33") && pos_adr.contains("positionless-vm-lib"),
        "parser/driver ADR must cite landed S3-I33"
    );
    assert!(
        pos_adr.contains("S3-I34") && pos_adr.contains("positionless-vm-mid"),
        "parser/driver ADR must cite landed S3-I34"
    );
    assert!(
        pos_adr.contains("S3-I35") && pos_adr.contains("positionless-pkg"),
        "parser/driver ADR must cite landed S3-I35"
    );
    assert!(
        pos_adr.contains("S3-I36") && pos_adr.contains("positionless-macro"),
        "parser/driver ADR must cite landed S3-I36"
    );
    assert!(
        pos_adr.contains("S3-I37") && pos_adr.contains("positionless-async-llvm"),
        "parser/driver ADR must cite landed S3-I37"
    );
    assert!(
        pos_adr.contains("S3-I38") && pos_adr.contains("positionless-rt-jit-debug"),
        "parser/driver ADR must cite landed S3-I38"
    );
    assert!(
        pos_adr.contains("S3-I39") && pos_adr.contains("positionless-residual"),
        "parser/driver ADR must cite landed S3-I39"
    );
    assert!(
        pos_adr.contains("S3-I40") && pos_adr.contains("noise-claim-prep"),
        "parser/driver ADR must cite landed S3-I40"
    );
    assert!(
        pos_adr.contains("S3-I41")
            && (pos_adr.contains("claim") || pos_adr.contains("[x]")),
        "parser/driver ADR must cite S3-I41 claim"
    );
    assert!(
        pos_adr.contains("S3-I42") && (pos_adr.contains("S3-02") || pos_adr.contains("open")),
        "parser/driver ADR must cite S3-I42 open"
    );
    assert!(
        pos_adr.contains("S3-I43") && (pos_adr.contains("construct") || pos_adr.contains("demangle") || pos_adr.contains("S3-02")),
        "pos_adr must cite S3-I43"
    );
    assert!(
        pos_adr.contains("S3-I44") && (pos_adr.contains("debug") || pos_adr.contains("stack") || pos_adr.contains("S3-02")),
        "pos_adr must cite S3-I44"
    );
    assert!(
        pos_adr.contains("S3-I45") && (pos_adr.contains("runtime") || pos_adr.contains("__type")),
        "pos_adr must cite S3-I45 runtime"
        );
        assert!(
            pos_adr.contains("S3-I46") && (pos_adr.contains("meta") || pos_adr.contains("S3-02") || pos_adr.contains("claim")),
        "pos_adr must cite S3-I46 meta"
        );
        assert!(
            pos_adr.contains("S3-I47") && (pos_adr.contains("claim") || pos_adr.contains("S3-02")),
        "pos_adr must cite S3-I47 claim"
        );
        assert!(
            pos_adr.contains("S3-03"),
        "pos_adr must lock next = S3-03"
    );
    assert!(
        pos_adr.contains("DUMMY") && (pos_adr.contains("SourceFile") || pos_adr.contains("span")),
        "positionless ADR must document no-fake-span / DUMMY rule"
    );

    let compile_adr = repo_file("docs/adr/stage3-s3-01-positionless-driver-compile.md");
    assert!(compile_adr.contains("S3-I27"));
    assert!(
        compile_adr.contains("positionless_driver_compile_bare")
            && (compile_adr.contains("=0") || compile_adr.contains("= 0")),
        "compile ADR must decide driver compile bare KPI=0"
    );
    assert!(compile_adr.contains("≠") && compile_adr.contains("S3-01"));
    assert!(
        compile_adr.contains("S3-I28") && compile_adr.contains("positionless-driver-emit-test-native"),
        "compile ADR must cite landed S3-I28"
    );
    assert!(
        compile_adr.contains("S3-I29") && compile_adr.contains("positionless-vm-phase84"),
        "compile ADR must cite landed S3-I29"
    );
    assert!(
        compile_adr.contains("S3-I30") && compile_adr.contains("positionless-vm-hosts"),
        "compile ADR must cite landed S3-I30"
    );
    assert!(
        compile_adr.contains("S3-I31") && compile_adr.contains("positionless-vm-phase89"),
        "compile ADR must cite landed S3-I31"
    );
    assert!(
        compile_adr.contains("S3-I32") && compile_adr.contains("positionless-vm-phase120"),
        "compile ADR must cite landed S3-I32"
    );
    assert!(
        compile_adr.contains("S3-I33") && compile_adr.contains("positionless-vm-lib"),
        "compile ADR must cite landed S3-I33"
    );
    assert!(
        compile_adr.contains("S3-I34") && compile_adr.contains("positionless-vm-mid"),
        "compile ADR must cite landed S3-I34"
    );
    assert!(
        compile_adr.contains("S3-I35") && compile_adr.contains("positionless-pkg"),
        "compile ADR must cite landed S3-I35"
    );
    assert!(
        compile_adr.contains("S3-I36") && compile_adr.contains("positionless-macro"),
        "compile ADR must cite landed S3-I36"
    );
    assert!(
        compile_adr.contains("S3-I37") && compile_adr.contains("positionless-async-llvm"),
        "compile ADR must cite landed S3-I37"
    );
    assert!(
        compile_adr.contains("S3-I38") && compile_adr.contains("positionless-rt-jit-debug"),
        "compile ADR must cite landed S3-I38"
    );
    assert!(
        compile_adr.contains("S3-I39") && compile_adr.contains("positionless-residual"),
        "compile ADR must cite landed S3-I39"
    );
    assert!(
        compile_adr.contains("S3-I40") && compile_adr.contains("noise-claim-prep"),
        "compile ADR must cite landed S3-I40"
    );
    assert!(
        compile_adr.contains("S3-I41")
            && (compile_adr.contains("claim") || compile_adr.contains("[x]")),
        "compile ADR must cite S3-I41 claim"
    );
    assert!(
        compile_adr.contains("S3-I42") && (compile_adr.contains("S3-02") || compile_adr.contains("open")),
        "compile ADR must cite S3-I42 open"
    );
    assert!(
        compile_adr.contains("S3-I43") && (compile_adr.contains("construct") || compile_adr.contains("demangle") || compile_adr.contains("S3-02")),
        "compile_adr must cite S3-I43"
    );
    assert!(
        compile_adr.contains("S3-I44") && (compile_adr.contains("debug") || compile_adr.contains("stack") || compile_adr.contains("S3-02")),
        "compile_adr must cite S3-I44"
    );
    assert!(
        compile_adr.contains("S3-I45") && (compile_adr.contains("runtime") || compile_adr.contains("__type")),
        "compile_adr must cite S3-I45 runtime"
        );
        assert!(
            compile_adr.contains("S3-I46") && (compile_adr.contains("meta") || compile_adr.contains("S3-02") || compile_adr.contains("claim")),
        "compile_adr must cite S3-I46 meta"
        );
        assert!(
            compile_adr.contains("S3-I47") && (compile_adr.contains("claim") || compile_adr.contains("S3-02")),
        "compile_adr must cite S3-I47 claim"
        );
        assert!(
            compile_adr.contains("S3-03"),
        "compile_adr must lock next = S3-03"
    );

    let phase89_pos_adr = repo_file("docs/adr/stage3-s3-01-positionless-vm-phase89.md");
    assert!(phase89_pos_adr.contains("S3-I31"));
    assert!(
        phase89_pos_adr.contains("positionless_vm_phase89_bare")
            && (phase89_pos_adr.contains("=0") || phase89_pos_adr.contains("= 0")),
        "phase89 positionless ADR must decide bare KPI=0"
    );
    assert!(phase89_pos_adr.contains("≠") && phase89_pos_adr.contains("S3-01"));
    assert!(
        phase89_pos_adr.contains("S3-I32") && phase89_pos_adr.contains("positionless-vm-phase120"),
        "phase89 positionless ADR must cite landed S3-I32"
    );
    assert!(
        phase89_pos_adr.contains("S3-I33") && phase89_pos_adr.contains("positionless-vm-lib"),
        "phase89 positionless ADR must cite landed S3-I33"
    );
    assert!(
        phase89_pos_adr.contains("S3-I34") && phase89_pos_adr.contains("positionless-vm-mid"),
        "phase89 positionless ADR must cite landed S3-I34"
    );
    assert!(
        phase89_pos_adr.contains("S3-I35") && phase89_pos_adr.contains("positionless-pkg"),
        "phase89 positionless ADR must cite landed S3-I35"
    );
    assert!(
        phase89_pos_adr.contains("S3-I36") && phase89_pos_adr.contains("positionless-macro"),
        "phase89 positionless ADR must cite landed S3-I36"
    );
    assert!(
        phase89_pos_adr.contains("S3-I37") && phase89_pos_adr.contains("positionless-async-llvm"),
        "phase89 positionless ADR must cite landed S3-I37"
    );
    assert!(
        phase89_pos_adr.contains("S3-I38") && phase89_pos_adr.contains("positionless-rt-jit-debug"),
        "phase89 positionless ADR must cite landed S3-I38"
    );
    assert!(
        phase89_pos_adr.contains("S3-I39") && phase89_pos_adr.contains("positionless-residual"),
        "phase89 positionless ADR must cite landed S3-I39"
    );
    assert!(
        phase89_pos_adr.contains("S3-I40") && phase89_pos_adr.contains("noise-claim-prep"),
        "phase89 positionless ADR must cite landed S3-I40"
    );
    assert!(
        phase89_pos_adr.contains("S3-I41")
            && (phase89_pos_adr.contains("claim") || phase89_pos_adr.contains("[x]")),
        "phase89 positionless ADR must cite S3-I41 claim"
    );
    assert!(
        phase89_pos_adr.contains("S3-I42") && (phase89_pos_adr.contains("S3-02") || phase89_pos_adr.contains("open")),
        "phase89 positionless ADR must cite S3-I42 open"
    );
    assert!(
        phase89_pos_adr.contains("S3-I43") && (phase89_pos_adr.contains("construct") || phase89_pos_adr.contains("demangle") || phase89_pos_adr.contains("S3-02")),
        "phase89_pos_adr must cite S3-I43"
    );
    assert!(
        phase89_pos_adr.contains("S3-I44") && (phase89_pos_adr.contains("debug") || phase89_pos_adr.contains("stack") || phase89_pos_adr.contains("S3-02")),
        "phase89_pos_adr must cite S3-I44"
    );
    assert!(
        phase89_pos_adr.contains("S3-I45") && (phase89_pos_adr.contains("runtime") || phase89_pos_adr.contains("__type")),
        "phase89_pos_adr must cite S3-I45 runtime"
        );
        assert!(
            phase89_pos_adr.contains("S3-I46") && (phase89_pos_adr.contains("meta") || phase89_pos_adr.contains("S3-02") || phase89_pos_adr.contains("claim")),
        "phase89_pos_adr must cite S3-I46 meta"
        );
        assert!(
            phase89_pos_adr.contains("S3-I47") && (phase89_pos_adr.contains("claim") || phase89_pos_adr.contains("S3-02")),
        "phase89_pos_adr must cite S3-I47 claim"
        );
        assert!(
            phase89_pos_adr.contains("S3-03"),
        "phase89_pos_adr must lock next = S3-03"
    );
    assert!(
        phase89_pos_adr.contains("DUMMY")
            && (phase89_pos_adr.contains("SourceFile") || phase89_pos_adr.contains("span")),
        "phase89 positionless ADR must document no-fake-span / DUMMY rule"
    );

    let phase120_pos_adr = repo_file("docs/adr/stage3-s3-01-positionless-vm-phase120.md");
    assert!(phase120_pos_adr.contains("S3-I32"));
    assert!(
        phase120_pos_adr.contains("positionless_vm_phase120_bare")
            && phase120_pos_adr.contains("positionless_vm_http_bare")
            && phase120_pos_adr.contains("positionless_vm_os_bare"),
        "phase120 positionless ADR must decide three bare KPIs=0"
    );
    assert!(phase120_pos_adr.contains("≠") && phase120_pos_adr.contains("S3-01"));
    assert!(
        phase120_pos_adr.contains("S3-I33") && phase120_pos_adr.contains("positionless-vm-lib"),
        "phase120 positionless ADR must cite landed S3-I33"
    );
    assert!(
        phase120_pos_adr.contains("S3-I34") && phase120_pos_adr.contains("positionless-vm-mid"),
        "phase120 positionless ADR must cite landed S3-I34"
    );
    assert!(
        phase120_pos_adr.contains("S3-I35") && phase120_pos_adr.contains("positionless-pkg"),
        "phase120 positionless ADR must cite landed S3-I35"
    );
    assert!(
        phase120_pos_adr.contains("S3-I36") && phase120_pos_adr.contains("positionless-macro"),
        "phase120 positionless ADR must cite landed S3-I36"
    );
    assert!(
        phase120_pos_adr.contains("S3-I37") && phase120_pos_adr.contains("positionless-async-llvm"),
        "phase120 positionless ADR must cite landed S3-I37"
    );
    assert!(
        phase120_pos_adr.contains("S3-I38") && phase120_pos_adr.contains("positionless-rt-jit-debug"),
        "phase120 positionless ADR must cite landed S3-I38"
    );
    assert!(
        phase120_pos_adr.contains("S3-I39") && phase120_pos_adr.contains("positionless-residual"),
        "phase120 positionless ADR must cite landed S3-I39"
    );
    assert!(
        phase120_pos_adr.contains("S3-I40") && phase120_pos_adr.contains("noise-claim-prep"),
        "phase120 positionless ADR must cite landed S3-I40"
    );
    assert!(
        phase120_pos_adr.contains("S3-I41")
            && (phase120_pos_adr.contains("claim") || phase120_pos_adr.contains("[x]")),
        "phase120 positionless ADR must cite S3-I41 claim"
    );
    assert!(
        phase120_pos_adr.contains("S3-I42") && (phase120_pos_adr.contains("S3-02") || phase120_pos_adr.contains("open")),
        "phase120 positionless ADR must cite S3-I42 open"
    );
    assert!(
        phase120_pos_adr.contains("S3-I43") && (phase120_pos_adr.contains("construct") || phase120_pos_adr.contains("demangle") || phase120_pos_adr.contains("S3-02")),
        "phase120_pos_adr must cite S3-I43"
    );
    assert!(
        phase120_pos_adr.contains("S3-I44") && (phase120_pos_adr.contains("debug") || phase120_pos_adr.contains("stack") || phase120_pos_adr.contains("S3-02")),
        "phase120_pos_adr must cite S3-I44"
    );
    assert!(
        phase120_pos_adr.contains("S3-I45") && (phase120_pos_adr.contains("runtime") || phase120_pos_adr.contains("__type")),
        "phase120_pos_adr must cite S3-I45 runtime"
        );
        assert!(
            phase120_pos_adr.contains("S3-I46") && (phase120_pos_adr.contains("meta") || phase120_pos_adr.contains("S3-02") || phase120_pos_adr.contains("claim")),
        "phase120_pos_adr must cite S3-I46 meta"
        );
        assert!(
            phase120_pos_adr.contains("S3-I47") && (phase120_pos_adr.contains("claim") || phase120_pos_adr.contains("S3-02")),
        "phase120_pos_adr must cite S3-I47 claim"
        );
        assert!(
            phase120_pos_adr.contains("S3-03"),
        "phase120_pos_adr must lock next = S3-03"
    );
    assert!(
        phase120_pos_adr.contains("DUMMY")
            && (phase120_pos_adr.contains("SourceFile") || phase120_pos_adr.contains("span")),
        "phase120 positionless ADR must document no-fake-span / DUMMY rule"
    );

    let lib_pos_adr = repo_file("docs/adr/stage3-s3-01-positionless-vm-lib.md");
    assert!(lib_pos_adr.contains("S3-I33"));
    assert!(
        lib_pos_adr.contains("positionless_vm_lib_bare")
            && lib_pos_adr.contains("positionless_vm_phase83_bare")
            && lib_pos_adr.contains("positionless_vm_channel_bare")
            && lib_pos_adr.contains("positionless_vm_phase_tail_bare"),
        "lib positionless ADR must decide four bare KPIs"
    );
    assert!(lib_pos_adr.contains("≠") && lib_pos_adr.contains("S3-01"));
    assert!(
        lib_pos_adr.contains("S3-I34") && lib_pos_adr.contains("positionless-vm-mid"),
        "lib positionless ADR must cite landed S3-I34"
    );
    assert!(
        lib_pos_adr.contains("S3-I35") && lib_pos_adr.contains("positionless-pkg"),
        "lib positionless ADR must cite landed S3-I35"
    );
    assert!(
        lib_pos_adr.contains("S3-I36") && lib_pos_adr.contains("positionless-macro"),
        "lib positionless ADR must cite landed S3-I36"
    );
    assert!(
        lib_pos_adr.contains("S3-I37") && lib_pos_adr.contains("positionless-async-llvm"),
        "lib positionless ADR must cite landed S3-I37"
    );
    assert!(
        lib_pos_adr.contains("S3-I38") && lib_pos_adr.contains("positionless-rt-jit-debug"),
        "lib positionless ADR must cite landed S3-I38"
    );
    assert!(
        lib_pos_adr.contains("S3-I39") && lib_pos_adr.contains("positionless-residual"),
        "lib positionless ADR must cite landed S3-I39"
    );
    assert!(
        lib_pos_adr.contains("S3-I40") && lib_pos_adr.contains("noise-claim-prep"),
        "lib positionless ADR must cite landed S3-I40"
    );
    assert!(
        lib_pos_adr.contains("S3-I41")
            && (lib_pos_adr.contains("claim") || lib_pos_adr.contains("[x]")),
        "lib positionless ADR must cite S3-I41 claim"
    );
    assert!(
        lib_pos_adr.contains("S3-I42") && (lib_pos_adr.contains("S3-02") || lib_pos_adr.contains("open")),
        "lib positionless ADR must cite S3-I42 open"
    );
    assert!(
        lib_pos_adr.contains("S3-I43") && (lib_pos_adr.contains("construct") || lib_pos_adr.contains("demangle") || lib_pos_adr.contains("S3-02")),
        "lib_pos_adr must cite S3-I43"
    );
    assert!(
        lib_pos_adr.contains("S3-I44") && (lib_pos_adr.contains("debug") || lib_pos_adr.contains("stack") || lib_pos_adr.contains("S3-02")),
        "lib_pos_adr must cite S3-I44"
    );
    assert!(
        lib_pos_adr.contains("S3-I45") && (lib_pos_adr.contains("runtime") || lib_pos_adr.contains("__type")),
        "lib_pos_adr must cite S3-I45 runtime"
        );
        assert!(
            lib_pos_adr.contains("S3-I46") && (lib_pos_adr.contains("meta") || lib_pos_adr.contains("S3-02") || lib_pos_adr.contains("claim")),
        "lib_pos_adr must cite S3-I46 meta"
        );
        assert!(
            lib_pos_adr.contains("S3-I47") && (lib_pos_adr.contains("claim") || lib_pos_adr.contains("S3-02")),
        "lib_pos_adr must cite S3-I47 claim"
        );
        assert!(
            lib_pos_adr.contains("S3-03"),
        "lib_pos_adr must lock next = S3-03"
    );

    let mid_pos_adr = repo_file("docs/adr/stage3-s3-01-positionless-vm-mid.md");
    assert!(mid_pos_adr.contains("S3-I34"));
    assert!(
        mid_pos_adr.contains("positionless_vm_mid_bare")
            && mid_pos_adr.contains("positionless_wasm_run_bare"),
        "mid positionless ADR must decide mid + wasm-run KPIs"
    );
    assert!(mid_pos_adr.contains("≠") && mid_pos_adr.contains("S3-01"));
    assert!(
        mid_pos_adr.contains("S3-I35") && mid_pos_adr.contains("positionless-pkg"),
        "mid positionless ADR must cite landed S3-I35"
    );
    assert!(
        mid_pos_adr.contains("S3-I36") && mid_pos_adr.contains("positionless-macro"),
        "mid positionless ADR must cite landed S3-I36"
    );
    assert!(
        mid_pos_adr.contains("S3-I37") && mid_pos_adr.contains("positionless-async-llvm"),
        "mid positionless ADR must cite landed S3-I37"
    );
    assert!(
        mid_pos_adr.contains("S3-I38") && mid_pos_adr.contains("positionless-rt-jit-debug"),
        "mid positionless ADR must cite landed S3-I38"
    );
    assert!(
        mid_pos_adr.contains("S3-I39") && mid_pos_adr.contains("positionless-residual"),
        "mid positionless ADR must cite landed S3-I39"
    );
    assert!(
        mid_pos_adr.contains("S3-I40") && mid_pos_adr.contains("noise-claim-prep"),
        "mid positionless ADR must cite landed S3-I40"
    );
    assert!(
        mid_pos_adr.contains("S3-I41")
            && (mid_pos_adr.contains("claim") || mid_pos_adr.contains("[x]")),
        "mid positionless ADR must cite S3-I41 claim"
    );
    assert!(
        mid_pos_adr.contains("S3-I42") && (mid_pos_adr.contains("S3-02") || mid_pos_adr.contains("open")),
        "mid positionless ADR must cite S3-I42 open"
    );
    assert!(
        mid_pos_adr.contains("S3-I43") && (mid_pos_adr.contains("construct") || mid_pos_adr.contains("demangle") || mid_pos_adr.contains("S3-02")),
        "mid_pos_adr must cite S3-I43"
    );
    assert!(
        mid_pos_adr.contains("S3-I44") && (mid_pos_adr.contains("debug") || mid_pos_adr.contains("stack") || mid_pos_adr.contains("S3-02")),
        "mid_pos_adr must cite S3-I44"
    );
    assert!(
        mid_pos_adr.contains("S3-I45") && (mid_pos_adr.contains("runtime") || mid_pos_adr.contains("__type")),
        "mid_pos_adr must cite S3-I45 runtime"
        );
        assert!(
            mid_pos_adr.contains("S3-I46") && (mid_pos_adr.contains("meta") || mid_pos_adr.contains("S3-02") || mid_pos_adr.contains("claim")),
        "mid_pos_adr must cite S3-I46 meta"
        );
        assert!(
            mid_pos_adr.contains("S3-I47") && (mid_pos_adr.contains("claim") || mid_pos_adr.contains("S3-02")),
        "mid_pos_adr must cite S3-I47 claim"
        );
        assert!(
            mid_pos_adr.contains("S3-03"),
        "mid_pos_adr must lock next = S3-03"
    );

    let pkg_pos_adr = repo_file("docs/adr/stage3-s3-01-positionless-pkg.md");
    assert!(pkg_pos_adr.contains("S3-I35"));
    assert!(
        pkg_pos_adr.contains("positionless_pkg_bare")
            && (pkg_pos_adr.contains("=0") || pkg_pos_adr.contains("= 0")),
        "pkg positionless ADR must decide bare KPI=0"
    );
    assert!(pkg_pos_adr.contains("≠") && pkg_pos_adr.contains("S3-01"));
    assert!(
        pkg_pos_adr.contains("S3-I36") && pkg_pos_adr.contains("positionless-macro"),
        "pkg positionless ADR must cite landed S3-I36"
    );
    assert!(
        pkg_pos_adr.contains("S3-I37") && pkg_pos_adr.contains("positionless-async-llvm"),
        "pkg positionless ADR must cite landed S3-I37"
    );
    assert!(
        pkg_pos_adr.contains("S3-I38") && pkg_pos_adr.contains("positionless-rt-jit-debug"),
        "pkg positionless ADR must cite landed S3-I38"
    );
    assert!(
        pkg_pos_adr.contains("S3-I39") && pkg_pos_adr.contains("positionless-residual"),
        "pkg positionless ADR must cite landed S3-I39"
    );
    assert!(
        pkg_pos_adr.contains("S3-I40") && pkg_pos_adr.contains("noise-claim-prep"),
        "pkg positionless ADR must cite landed S3-I40"
    );
    assert!(
        pkg_pos_adr.contains("S3-I41")
            && (pkg_pos_adr.contains("claim") || pkg_pos_adr.contains("[x]")),
        "pkg positionless ADR must cite S3-I41 claim"
    );
    assert!(
        pkg_pos_adr.contains("S3-I42") && (pkg_pos_adr.contains("S3-02") || pkg_pos_adr.contains("open")),
        "pkg positionless ADR must cite S3-I42 open"
    );
    assert!(
        pkg_pos_adr.contains("S3-I43") && (pkg_pos_adr.contains("construct") || pkg_pos_adr.contains("demangle") || pkg_pos_adr.contains("S3-02")),
        "pkg_pos_adr must cite S3-I43"
    );
    assert!(
        pkg_pos_adr.contains("S3-I44") && (pkg_pos_adr.contains("debug") || pkg_pos_adr.contains("stack") || pkg_pos_adr.contains("S3-02")),
        "pkg_pos_adr must cite S3-I44"
    );
    assert!(
        pkg_pos_adr.contains("S3-I45") && (pkg_pos_adr.contains("runtime") || pkg_pos_adr.contains("__type")),
        "pkg_pos_adr must cite S3-I45 runtime"
        );
        assert!(
            pkg_pos_adr.contains("S3-I46") && (pkg_pos_adr.contains("meta") || pkg_pos_adr.contains("S3-02") || pkg_pos_adr.contains("claim")),
        "pkg_pos_adr must cite S3-I46 meta"
        );
        assert!(
            pkg_pos_adr.contains("S3-I47") && (pkg_pos_adr.contains("claim") || pkg_pos_adr.contains("S3-02")),
        "pkg_pos_adr must cite S3-I47 claim"
        );
        assert!(
            pkg_pos_adr.contains("S3-03"),
        "pkg_pos_adr must lock next = S3-03"
    );

    let macro_pos_adr = repo_file("docs/adr/stage3-s3-01-positionless-macro.md");
    assert!(macro_pos_adr.contains("S3-I36"));
    assert!(
        macro_pos_adr.contains("positionless_macro_bare")
            && (macro_pos_adr.contains("=0") || macro_pos_adr.contains("= 0")),
        "macro positionless ADR must decide bare KPI=0"
    );
    assert!(macro_pos_adr.contains("≠") && macro_pos_adr.contains("S3-01"));
    assert!(
        macro_pos_adr.contains("S3-I37") && macro_pos_adr.contains("positionless-async-llvm"),
        "macro positionless ADR must cite landed S3-I37"
    );
    assert!(
        macro_pos_adr.contains("S3-I38") && macro_pos_adr.contains("positionless-rt-jit-debug"),
        "macro positionless ADR must cite landed S3-I38"
    );
    assert!(
        macro_pos_adr.contains("S3-I39") && macro_pos_adr.contains("positionless-residual"),
        "macro positionless ADR must cite landed S3-I39"
    );
    assert!(
        macro_pos_adr.contains("S3-I40") && macro_pos_adr.contains("noise-claim-prep"),
        "macro positionless ADR must cite landed S3-I40"
    );
    assert!(
        macro_pos_adr.contains("S3-I41")
            && (macro_pos_adr.contains("claim") || macro_pos_adr.contains("[x]")),
        "macro positionless ADR must cite S3-I41 claim"
    );
    assert!(
        macro_pos_adr.contains("S3-I42") && (macro_pos_adr.contains("S3-02") || macro_pos_adr.contains("open")),
        "macro positionless ADR must cite S3-I42 open"
    );
    assert!(
        macro_pos_adr.contains("S3-I43") && (macro_pos_adr.contains("construct") || macro_pos_adr.contains("demangle") || macro_pos_adr.contains("S3-02")),
        "macro_pos_adr must cite S3-I43"
    );
    assert!(
        macro_pos_adr.contains("S3-I44") && (macro_pos_adr.contains("debug") || macro_pos_adr.contains("stack") || macro_pos_adr.contains("S3-02")),
        "macro_pos_adr must cite S3-I44"
    );
    assert!(
        macro_pos_adr.contains("S3-I45") && (macro_pos_adr.contains("runtime") || macro_pos_adr.contains("__type")),
        "macro_pos_adr must cite S3-I45 runtime"
        );
        assert!(
            macro_pos_adr.contains("S3-I46") && (macro_pos_adr.contains("meta") || macro_pos_adr.contains("S3-02") || macro_pos_adr.contains("claim")),
        "macro_pos_adr must cite S3-I46 meta"
        );
        assert!(
            macro_pos_adr.contains("S3-I47") && (macro_pos_adr.contains("claim") || macro_pos_adr.contains("S3-02")),
        "macro_pos_adr must cite S3-I47 claim"
        );
        assert!(
            macro_pos_adr.contains("S3-03"),
        "macro_pos_adr must lock next = S3-03"
    );
    assert!(
        macro_pos_adr.contains("DUMMY")
            && (macro_pos_adr.contains("SourceFile") || macro_pos_adr.contains("span")),
        "macro positionless ADR must document no-fake-span / DUMMY rule"
    );

    let async_llvm_pos_adr = repo_file("docs/adr/stage3-s3-01-positionless-async-llvm.md");
    assert!(async_llvm_pos_adr.contains("S3-I37"));
    assert!(
        async_llvm_pos_adr.contains("positionless_async_bare")
            && async_llvm_pos_adr.contains("positionless_llvm_bare")
            && (async_llvm_pos_adr.contains("=0") || async_llvm_pos_adr.contains("= 0")),
        "async-llvm positionless ADR must decide bare KPIs=0"
    );
    assert!(async_llvm_pos_adr.contains("≠") && async_llvm_pos_adr.contains("S3-01"));
    assert!(
        async_llvm_pos_adr.contains("S3-I38") && async_llvm_pos_adr.contains("positionless-rt-jit-debug"),
        "async-llvm positionless ADR must cite landed S3-I38"
    );
    assert!(
        async_llvm_pos_adr.contains("S3-I39") && async_llvm_pos_adr.contains("positionless-residual"),
        "async-llvm positionless ADR must cite landed S3-I39"
    );
    assert!(
        async_llvm_pos_adr.contains("S3-I40") && async_llvm_pos_adr.contains("noise-claim-prep"),
        "async-llvm positionless ADR must cite landed S3-I40"
    );
    assert!(
        async_llvm_pos_adr.contains("S3-I41")
            && (async_llvm_pos_adr.contains("claim") || async_llvm_pos_adr.contains("[x]")),
        "async-llvm positionless ADR must cite S3-I41 claim"
    );
    assert!(
        async_llvm_pos_adr.contains("S3-I42") && (async_llvm_pos_adr.contains("S3-02") || async_llvm_pos_adr.contains("open")),
        "async-llvm positionless ADR must cite S3-I42 open"
    );
    assert!(
        async_llvm_pos_adr.contains("S3-I43") && (async_llvm_pos_adr.contains("construct") || async_llvm_pos_adr.contains("demangle") || async_llvm_pos_adr.contains("S3-02")),
        "async_llvm_pos_adr must cite S3-I43"
    );
    assert!(
        async_llvm_pos_adr.contains("S3-I44") && (async_llvm_pos_adr.contains("debug") || async_llvm_pos_adr.contains("stack") || async_llvm_pos_adr.contains("S3-02")),
        "async_llvm_pos_adr must cite S3-I44"
    );
    assert!(
        async_llvm_pos_adr.contains("S3-I45") && (async_llvm_pos_adr.contains("runtime") || async_llvm_pos_adr.contains("__type")),
        "async_llvm_pos_adr must cite S3-I45 runtime"
        );
        assert!(
            async_llvm_pos_adr.contains("S3-I46") && (async_llvm_pos_adr.contains("meta") || async_llvm_pos_adr.contains("S3-02") || async_llvm_pos_adr.contains("claim")),
        "async_llvm_pos_adr must cite S3-I46 meta"
        );
        assert!(
            async_llvm_pos_adr.contains("S3-I47") && (async_llvm_pos_adr.contains("claim") || async_llvm_pos_adr.contains("S3-02")),
        "async_llvm_pos_adr must cite S3-I47 claim"
        );
        assert!(
            async_llvm_pos_adr.contains("S3-03"),
        "async_llvm_pos_adr must lock next = S3-03"
    );
    assert!(
        async_llvm_pos_adr.contains("DUMMY")
            && (async_llvm_pos_adr.contains("SourceFile") || async_llvm_pos_adr.contains("span")),
        "async-llvm positionless ADR must document no-fake-span / DUMMY rule"
    );

    let rt_pos_adr = repo_file("docs/adr/stage3-s3-01-positionless-rt-jit-debug.md");
    assert!(rt_pos_adr.contains("S3-I38"));
    assert!(
        rt_pos_adr.contains("positionless_rt_bare")
            && rt_pos_adr.contains("positionless_jit_bare")
            && rt_pos_adr.contains("positionless_debug_bare")
            && rt_pos_adr.contains("positionless_codegen_bare"),
        "rt-jit-debug ADR must decide four bare KPIs"
    );
    assert!(rt_pos_adr.contains("≠") && rt_pos_adr.contains("S3-01"));
    assert!(
        rt_pos_adr.contains("S3-I39") && rt_pos_adr.contains("positionless-residual"),
        "rt-jit-debug ADR must cite landed S3-I39"
    );
    assert!(
        rt_pos_adr.contains("S3-I40") && rt_pos_adr.contains("noise-claim-prep"),
        "rt-jit-debug ADR must cite landed S3-I40"
    );
    assert!(
        rt_pos_adr.contains("S3-I41")
            && (rt_pos_adr.contains("claim") || rt_pos_adr.contains("[x]")),
        "rt-jit-debug ADR must cite S3-I41 claim"
    );
    assert!(
        rt_pos_adr.contains("S3-I42") && (rt_pos_adr.contains("S3-02") || rt_pos_adr.contains("open")),
        "rt-jit-debug ADR must cite S3-I42 open"
    );
    assert!(
        rt_pos_adr.contains("S3-I43") && (rt_pos_adr.contains("construct") || rt_pos_adr.contains("demangle") || rt_pos_adr.contains("S3-02")),
        "rt_pos_adr must cite S3-I43"
    );
    assert!(
        rt_pos_adr.contains("S3-I44") && (rt_pos_adr.contains("debug") || rt_pos_adr.contains("stack") || rt_pos_adr.contains("S3-02")),
        "rt_pos_adr must cite S3-I44"
    );
    assert!(
        rt_pos_adr.contains("S3-I45") && (rt_pos_adr.contains("runtime") || rt_pos_adr.contains("__type")),
        "rt_pos_adr must cite S3-I45 runtime"
        );
        assert!(
            rt_pos_adr.contains("S3-I46") && (rt_pos_adr.contains("meta") || rt_pos_adr.contains("S3-02") || rt_pos_adr.contains("claim")),
        "rt_pos_adr must cite S3-I46 meta"
        );
        assert!(
            rt_pos_adr.contains("S3-I47") && (rt_pos_adr.contains("claim") || rt_pos_adr.contains("S3-02")),
        "rt_pos_adr must cite S3-I47 claim"
        );
        assert!(
            rt_pos_adr.contains("S3-03"),
        "rt_pos_adr must lock next = S3-03"
    );
    assert!(
        rt_pos_adr.contains("DUMMY")
            && (rt_pos_adr.contains("SourceFile") || rt_pos_adr.contains("span")),
        "rt-jit-debug ADR must document no-fake-span / DUMMY rule"
    );

    let residual_pos_adr = repo_file("docs/adr/stage3-s3-01-positionless-residual.md");
    assert!(residual_pos_adr.contains("S3-I39"));
    assert!(
        residual_pos_adr.contains("positionless_mir_bare")
            && residual_pos_adr.contains("positionless_lint_bare"),
        "residual ADR must decide residual bare KPIs"
    );
    assert!(residual_pos_adr.contains("≠") && residual_pos_adr.contains("S3-01"));
    assert!(
        residual_pos_adr.contains("S3-I40") && residual_pos_adr.contains("noise-claim-prep"),
        "residual ADR must cite landed S3-I40"
    );
    assert!(
        residual_pos_adr.contains("S3-I41")
            && (residual_pos_adr.contains("claim") || residual_pos_adr.contains("[x]")),
        "residual ADR must cite S3-I41 claim"
    );
    assert!(
        residual_pos_adr.contains("S3-I42") && (residual_pos_adr.contains("S3-02") || residual_pos_adr.contains("open")),
        "residual ADR must cite S3-I42 open"
    );
    assert!(
        residual_pos_adr.contains("S3-I43") && (residual_pos_adr.contains("construct") || residual_pos_adr.contains("demangle") || residual_pos_adr.contains("S3-02")),
        "residual_pos_adr must cite S3-I43"
    );
    assert!(
        residual_pos_adr.contains("S3-I44") && (residual_pos_adr.contains("debug") || residual_pos_adr.contains("stack") || residual_pos_adr.contains("S3-02")),
        "residual_pos_adr must cite S3-I44"
    );
    assert!(
        residual_pos_adr.contains("S3-I45") && (residual_pos_adr.contains("runtime") || residual_pos_adr.contains("__type")),
        "residual_pos_adr must cite S3-I45 runtime"
        );
        assert!(
            residual_pos_adr.contains("S3-I46") && (residual_pos_adr.contains("meta") || residual_pos_adr.contains("S3-02") || residual_pos_adr.contains("claim")),
        "residual_pos_adr must cite S3-I46 meta"
        );
        assert!(
            residual_pos_adr.contains("S3-I47") && (residual_pos_adr.contains("claim") || residual_pos_adr.contains("S3-02")),
        "residual_pos_adr must cite S3-I47 claim"
        );
        assert!(
            residual_pos_adr.contains("S3-03"),
        "residual_pos_adr must lock next = S3-03"
    );
    assert!(
        residual_pos_adr.contains("DUMMY")
            && (residual_pos_adr.contains("SourceFile") || residual_pos_adr.contains("span")),
        "residual ADR must document no-fake-span / DUMMY rule"
    );

    let noise_adr = repo_file("docs/adr/stage3-s3-01-noise-claim-prep.md");
    assert!(noise_adr.contains("S3-I40"));
    assert!(
        noise_adr.contains("positionless_helper_calls")
            && (noise_adr.contains("=0") || noise_adr.contains("= 0")),
        "noise ADR must decide filtered helper_calls=0"
    );
    assert!(noise_adr.contains("≠") && noise_adr.contains("S3-01"));
    assert!(
        noise_adr.contains("S3-I41")
            && (noise_adr.contains("claim") || noise_adr.contains("[x]")),
        "noise ADR must cite S3-I41 claim"
    );
    assert!(
        noise_adr.contains("S3-I42") && (noise_adr.contains("S3-02") || noise_adr.contains("open")),
        "noise ADR must cite S3-I42 open"
    );
    assert!(
        noise_adr.contains("S3-I43") && (noise_adr.contains("construct") || noise_adr.contains("demangle") || noise_adr.contains("S3-02")),
        "noise_adr must cite S3-I43"
    );
    assert!(
        noise_adr.contains("S3-I44") && (noise_adr.contains("debug") || noise_adr.contains("stack") || noise_adr.contains("S3-02")),
        "noise_adr must cite S3-I44"
    );
    assert!(
        noise_adr.contains("S3-I45") && (noise_adr.contains("runtime") || noise_adr.contains("__type")),
        "noise_adr must cite S3-I45 runtime"
        );
        assert!(
            noise_adr.contains("S3-I46") && (noise_adr.contains("meta") || noise_adr.contains("S3-02") || noise_adr.contains("claim")),
        "noise_adr must cite S3-I46 meta"
        );
        assert!(
            noise_adr.contains("S3-I47") && (noise_adr.contains("claim") || noise_adr.contains("S3-02")),
        "noise_adr must cite S3-I47 claim"
        );
        assert!(
            noise_adr.contains("S3-03"),
        "noise_adr must lock next = S3-03"
    );
    assert!(
        noise_adr.contains("lockfile_error")
            && noise_adr.contains("set_error"),
        "noise ADR must document whitelist"
    );
    assert!(
        pkg_pos_adr.contains("DUMMY")
            && (pkg_pos_adr.contains("SourceFile") || pkg_pos_adr.contains("span")),
        "pkg positionless ADR must document no-fake-span / DUMMY rule"
    );
    assert!(
        mid_pos_adr.contains("DUMMY")
            && (mid_pos_adr.contains("SourceFile") || mid_pos_adr.contains("span")),
        "mid positionless ADR must document no-fake-span / DUMMY rule"
    );
    assert!(
        lib_pos_adr.contains("DUMMY")
            && (lib_pos_adr.contains("SourceFile") || lib_pos_adr.contains("span")),
        "lib positionless ADR must document no-fake-span / DUMMY rule"
    );
    assert!(
        compile_adr.contains("DUMMY")
            && (compile_adr.contains("SourceFile") || compile_adr.contains("span")),
        "compile ADR must document no-fake-span / DUMMY rule"
    );

    let script = repo_file("scripts/s3_01_span_census.sh");
    assert!(script.contains("host_driver_compile_bare"));
    assert!(script.contains("host_driver_ops_bare"));
    assert!(script.contains("host_llvm_bare"));
    assert!(script.contains("host_async_bare"));
    assert!(script.contains("host_codegen_bare"));
    assert!(script.contains("host_wasm_run_bare"));
    assert!(script.contains("host_pkg_bare"));
    assert!(script.contains("host_macro_bare"));
    assert!(script.contains("host_vm_hosts_bare"));
    assert!(script.contains("host_vm_phase89_bare"));
    assert!(script.contains("host_vm_phase84_bare"));
    assert!(script.contains("host_vm_phase120_bare"));
    assert!(script.contains("host_vm_phase83_bare"));
    assert!(script.contains("host_vm_phase_tail_bare"));
    assert!(script.contains("host_vm_mid_bare"));
    assert!(script.contains("host_vm_wasm_stubs_bare"));
    assert!(script.contains("host_rt_jit_debug_bare"));
    assert!(script.contains("host_residual_bare"));
    assert!(script.contains("positionless_helper_calls"));
    assert!(script.contains("positionless_helper_noise"));
    assert!(script.contains("positionless_helper_policy"));
    assert!(script.contains("positionless_parser_driver_bare"));
    assert!(script.contains("positionless_driver_compile_bare"));
    assert!(script.contains("positionless_driver_emit_test_native_bare"));
    assert!(script.contains("positionless_vm_phase84_bare"));
    assert!(script.contains("positionless_vm_hosts_bare"));
    assert!(script.contains("positionless_vm_phase89_bare"));
    assert!(script.contains("positionless_vm_phase120_bare"));
    assert!(script.contains("positionless_vm_http_bare"));
    assert!(script.contains("positionless_vm_os_bare"));
    assert!(script.contains("positionless_vm_lib_bare"));
    assert!(script.contains("positionless_vm_phase83_bare"));
    assert!(script.contains("positionless_vm_channel_bare"));
    assert!(script.contains("positionless_vm_phase_tail_bare"));
    assert!(script.contains("positionless_vm_mid_bare"));
    assert!(script.contains("positionless_wasm_run_bare"));
    assert!(script.contains("positionless_pkg_bare"));
    assert!(script.contains("positionless_macro_bare"));
    assert!(script.contains("positionless_async_bare"));
    assert!(script.contains("positionless_llvm_bare"));
    assert!(script.contains("positionless_rt_bare"));
    assert!(script.contains("positionless_jit_bare"));
    assert!(script.contains("positionless_debug_bare"));
    assert!(script.contains("positionless_codegen_bare"));
    assert!(script.contains("positionless_mir_bare"));
    assert!(script.contains("positionless_mir_run_bare"));
    assert!(script.contains("positionless_meta_bare"));
    assert!(script.contains("positionless_hir_bare"));
    assert!(script.contains("positionless_cst_bare"));
    assert!(script.contains("positionless_wasm_emit_bare"));
    assert!(script.contains("positionless_fmt_bare"));
    assert!(script.contains("positionless_compiler_wasm_bare"));
    assert!(script.contains("positionless_lexer_bare"));
    assert!(script.contains("positionless_lint_bare"));
    assert!(script.contains("miette_help_impl"));
    assert!(script.contains("with_span_label_wired"));
    assert!(
        script.contains("crates/syx-driver/src/format_lint.rs"),
        "census must count driver format_lint.rs for positionless KPI"
    );
    assert!(
        script.contains("crates/syx-driver/src/build_syx.rs"),
        "census must count build_syx.rs for compile positionless KPI"
    );
    assert!(
        script.contains("crates/syx-driver/src/compile_linked.rs"),
        "census must count compile_linked.rs for compile positionless KPI"
    );
    assert!(
        script.contains("crates/syx-driver/src/emit.rs"),
        "census must count emit.rs for emit/test/native KPI"
    );
    assert!(
        script.contains("crates/syx-driver/src/test_runner.rs"),
        "census must count test_runner.rs for emit/test/native KPI"
    );
    assert!(
        script.contains("crates/syx-driver/src/native_build.rs"),
        "census must count native_build.rs for emit/test/native KPI"
    );
    assert!(
        script.contains("crates/syx-vm/src/stdlib_phase84_hosts.rs"),
        "census must count stdlib_phase84_hosts.rs for vm phase84 KPI"
    );
    assert!(
        script.contains("crates/syx-vm/src/hosts.rs"),
        "census must count hosts.rs for vm hosts KPI"
    );
    assert!(
        script.contains("crates/syx-vm/src/stdlib_phase89_hosts.rs"),
        "census must count stdlib_phase89_hosts.rs for vm phase89 KPI"
    );
    assert!(
        script.contains("crates/syx-vm/src/wasm_stubs/stdlib_phase89_hosts.rs"),
        "census must count wasm_stubs phase89 for vm phase89 KPI"
    );
    assert!(
        script.contains("crates/syx-vm/src/stdlib_phase120_hosts.rs"),
        "census must count stdlib_phase120_hosts.rs for vm phase120 KPI"
    );
    assert!(
        script.contains("crates/syx-vm/src/http.rs"),
        "census must count http.rs for vm http KPI"
    );
    assert!(
        script.contains("crates/syx-vm/src/wasm_stubs/http.rs"),
        "census must count wasm_stubs http.rs for vm http KPI"
    );
    assert!(
        script.contains("crates/syx-vm/src/os_hosts.rs"),
        "census must count os_hosts.rs for vm os KPI"
    );
    assert!(
        script.contains("crates/syx-vm/src/lib.rs"),
        "census must count lib.rs for vm lib KPI"
    );
    assert!(
        script.contains("crates/syx-vm/src/stdlib_phase83_hosts.rs"),
        "census must count phase83 for vm phase83 KPI"
    );
    assert!(
        script.contains("crates/syx-vm/src/channel_hosts.rs"),
        "census must count channel_hosts.rs for vm channel KPI"
    );
    assert!(
        script.contains("crates/syx-vm/src/stdlib_phase90_hosts.rs"),
        "census must count phase90 for vm phase_tail KPI"
    );
    assert!(
        script.contains("crates/syx-vm/src/script_hosts.rs"),
        "census must count script_hosts.rs for vm mid KPI"
    );
    assert!(
        script.contains("crates/syx-wasm-run/src/lib.rs"),
        "census must count wasm-run lib.rs for wasm_run KPI"
    );
    assert!(
        script.contains("crates/syx-wasm-run/src/bridge.rs"),
        "census must count wasm-run bridge.rs for wasm_run KPI"
    );
    assert!(
        script.contains("crates/syx-pkg/src/registry.rs"),
        "census must count pkg registry.rs for pkg KPI"
    );
    assert!(
        script.contains("crates/syx-vm/src/hosts.rs"),
        "census must count syx-vm hosts.rs"
    );
    assert!(
        script.contains("crates/syx-vm/src/stdlib_phase89_hosts.rs"),
        "census must count syx-vm stdlib_phase89_hosts.rs"
    );
    assert!(
        script.contains("crates/syx-vm/src/stdlib_phase84_hosts.rs"),
        "census must count syx-vm stdlib_phase84_hosts.rs"
    );
    assert!(
        script.contains("crates/syx-vm/src/stdlib_phase120_hosts.rs"),
        "census must count syx-vm stdlib_phase120_hosts.rs"
    );
    assert!(
        script.contains("crates/syx-vm/src/stdlib_phase83_hosts.rs"),
        "census must count syx-vm stdlib_phase83_hosts.rs"
    );
    assert!(
        script.contains("crates/syx-vm/src/stdlib_phase90_hosts.rs"),
        "census must count syx-vm stdlib_phase90_hosts.rs"
    );
    assert!(
        script.contains("crates/syx-vm/src/stdlib_phase87_hosts.rs"),
        "census must count syx-vm stdlib_phase87_hosts.rs"
    );
    assert!(
        script.contains("crates/syx-vm/src/lib.rs"),
        "census must count syx-vm lib.rs"
    );
    assert!(
        script.contains("crates/syx-vm/src/http.rs"),
        "census must count syx-vm http.rs"
    );
    assert!(
        script.contains("crates/syx-vm/src/wasm_stubs/stdlib_phase89_hosts.rs"),
        "census must count wasm_stubs stdlib_phase89_hosts.rs"
    );
    assert!(
        script.contains("crates/syx-vm/src/wasm_stubs/http.rs"),
        "census must count wasm_stubs http.rs"
    );
    assert!(
        script.contains("crates/syx-rt/src/bridge.rs"),
        "census must count syx-rt bridge.rs"
    );
    assert!(
        script.contains("crates/syx-cranelift-jit/src/lib.rs"),
        "census must count syx-cranelift-jit lib.rs"
    );
    assert!(
        script.contains("crates/syx-debug/src/lib.rs"),
        "census must count syx-debug lib.rs"
    );
    assert!(
        script.contains("crates/syx-mir-run/src/runtime.rs"),
        "census must count mir-run runtime.rs"
    );
    assert!(
        script.contains("crates/syx-mir/src/mir_codec.rs"),
        "census must count mir_codec.rs"
    );
    assert!(
        script.contains("crates/syx-parser/src/lib.rs"),
        "census must count parser lib.rs"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let stage3 = plan02
        .lines()
        .find(|l| l.contains("| **3** |"))
        .expect("stage-3 row");
    assert!(stage3.contains("[x]") && stage3.contains("退出"), "Stage-3 must be [x] 退出; got: {stage3}");
    let s3_01 = plan02
        .lines()
        .find(|l| l.contains("| S3-01 |"))
        .expect("S3-01 row");
    assert!(
        s3_01.contains("| [x] |") || s3_01.contains("| `[x]` |"),
        "plan/02 S3-01 status must be [x]; got: {s3_01}"
    );
    assert!(
        plan02.contains("S3-I41")
            || plan02.contains("stage3-s3-01-claim")
    );
    assert!(
        plan02.contains("S3-I42") && (plan02.contains("S3-02") || plan02.contains("open")),
        "plan/02 must cite S3-I42 open S3-02 open"
    );
    assert!(
        plan02.contains("S3-I43") && (plan02.contains("construct") || plan02.contains("demangle") || plan02.contains("S3-02")),
        "plan02 must cite S3-I43"
    );
    assert!(
        plan02.contains("S3-I44") && (plan02.contains("debug") || plan02.contains("stack") || plan02.contains("S3-02")),
        "plan02 must cite S3-I44"
    );
    assert!(
        plan02.contains("S3-I45") && (plan02.contains("runtime") || plan02.contains("__type")),
        "plan02 must cite S3-I45 runtime"
    );
    assert!(
        plan02.contains("S3-I46") && plan02.contains("meta"),
        "plan02 must cite S3-I46 meta"
    );
    assert!(
        plan02.contains("S3-I47") && plan02.contains("claim"),
        "plan02 must cite S3-I47 claim"
    );
    assert!(
        plan02.contains("S3-03"),
        "plan02 must lock next = S3-03"
    );

    let diag = repo_file("crates/syx-diagnostics/src/lib.rs");
    assert!(diag.contains("primary_span: None"));
    assert!(
        diag.contains("fn help"),
        "diagnostics must implement Diagnostic::help"
    );
    assert!(
        diag.contains("label: impl Into<String>")
            && !diag.contains("_label: impl Into<String>"),
        "with_span must take non-discarded label parameter"
    );
    assert!(
        diag.contains("pub label:") && diag.contains("pub help:"),
        "SyxDiagnostic must expose label and help fields"
    );
}

#[test]
fn s3_01_miette_quality_kpis_one() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("miette_help_impl=1"),
        "expected miette_help_impl=1, got:\n{stdout}"
    );
    assert!(
        stdout.contains("with_span_label_wired=1"),
        "expected with_span_label_wired=1, got:\n{stdout}"
    );
    assert!(
        stdout.contains("positionless_helper_calls=0"),
        "expected positionless_helper_calls=0, got:\n{stdout}"
    );
}

#[test]
fn s3_01_positionless_parser_driver_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_parser_driver_bare=0"),
        "expected positionless_parser_driver_bare=0, got:\n{stdout}"
    );
    for rel in [
        "crates/syx-parser/src/lib.rs",
        "crates/syx-driver/src/format_lint.rs",
        "crates/syx-driver/src/check_project.rs",
        "crates/syx-driver/src/run_exec.rs",
        "crates/syx-driver/src/debug_repl.rs",
        "crates/syx-driver/src/module_ref.rs",
        "crates/syx-driver/src/driver_options.rs",
        "crates/syx-driver/src/doc_project.rs",
    ] {
        let src = repo_file(rel);
        let bare = src.matches("parser_error(").count() + src.matches("driver_error(").count();
        // matches() also hits *_error_at( — subtract those
        let at = src.matches("parser_error_at(").count() + src.matches("driver_error_at(").count();
        assert_eq!(bare.saturating_sub(at), 0, "{rel} still has bare *_error(");
    }
}

#[test]
fn s3_01_positionless_driver_compile_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_driver_compile_bare=0"),
        "expected positionless_driver_compile_bare=0, got:\n{stdout}"
    );
    for rel in [
        "crates/syx-driver/src/build_syx.rs",
        "crates/syx-driver/src/compile_linked.rs",
    ] {
        let src = repo_file(rel);
        let bare = src.matches("driver_error(").count();
        let at = src.matches("driver_error_at(").count();
        assert_eq!(bare.saturating_sub(at), 0, "{rel} still has bare driver_error(");
    }
}


#[test]
fn s3_01_semantic_analyze_main_bare_zero() {
    for rel in [
        "crates/syx-semantic/src/semantic_analyze_expr.rs",
        "crates/syx-semantic/src/semantic_analyze_stmt.rs",
        "crates/syx-semantic/src/semantic_analyze_call.rs",
    ] {
        assert_eq!(bare_diag_count(&repo_file(rel)), 0, "{rel}");
    }
}

#[test]
fn s3_01_semantic_analyze_type_decl_bare_zero() {
    for rel in [
        "crates/syx-semantic/src/semantic_analyze_type_member.rs",
        "crates/syx-semantic/src/semantic_analyze_decl.rs",
    ] {
        assert_eq!(bare_diag_count(&repo_file(rel)), 0, "{rel}");
    }
}

#[test]
fn s3_01_semantic_concurrency_bare_zero() {
    assert_eq!(
        bare_diag_count(&repo_file(
            "crates/syx-semantic/src/semantic_concurrency.rs"
        )),
        0
    );
}

#[test]
fn s3_01_semantic_hub_bare_zero() {
    for rel in [
        "crates/syx-semantic/src/semantic_analyzer.rs",
        "crates/syx-semantic/src/type_checker.rs",
        "crates/syx-semantic/src/semantic_types.rs",
        "crates/syx-semantic/src/semantic_infer_host.rs",
        "crates/syx-semantic/src/import_resolver.rs",
    ] {
        assert_eq!(bare_diag_count(&repo_file(rel)), 0, "{rel}");
    }
}

#[test]
fn s3_01_host_driver_compile_bare_zero() {
    for rel in [
        "crates/syx-driver/src/compile_linked.rs",
        "crates/syx-driver/src/build_syx.rs",
        "crates/syx-driver/src/emit.rs",
        "crates/syx-driver/src/check_project.rs",
        "crates/syx-driver/src/module_ref.rs",
    ] {
        assert_eq!(bare_diag_count(&repo_file(rel)), 0, "{rel}");
    }
}

#[test]
fn s3_01_host_driver_ops_bare_zero() {
    let files = [
        "crates/syx-driver/src/format_lint.rs",
        "crates/syx-driver/src/test_runner.rs",
        "crates/syx-driver/src/native_build.rs",
        "crates/syx-driver/src/run_exec.rs",
        "crates/syx-driver/src/debug_repl.rs",
        "crates/syx-driver/src/doc_project.rs",
        "crates/syx-driver/src/driver_options.rs",
        "crates/syx-driver/src/run_wasm.rs",
        "crates/syx-driver/src/cranelift_stubs.rs",
    ];
    let mut total = 0usize;
    for rel in files {
        let n = bare_diag_count(&repo_file(rel));
        assert_eq!(n, 0, "{rel} must have zero bare; got {n}");
        total += n;
    }
    assert_eq!(total, 0, "host_driver_ops_bare must be 0");
}

#[test]
fn s3_01_host_llvm_bare_zero() {
    assert_eq!(
        bare_diag_count(&repo_file("crates/syx-llvm/src/lib.rs")),
        0,
        "host_llvm_bare must be 0"
    );
}

#[test]
fn s3_01_host_async_bare_zero() {
    let files = [
        "crates/syx-async/src/lib.rs",
        "crates/syx-async/src/lower.rs",
        "crates/syx-async/src/task.rs",
        "crates/syx-async/src/runtime.rs",
        "crates/syx-async/src/hosts.rs",
        "crates/syx-async/src/cancellation.rs",
        "crates/syx-async/src/dist_transport.rs",
        "crates/syx-async/src/async_diag.rs",
    ];
    let mut total = 0usize;
    for rel in files {
        let n = bare_diag_count(&repo_file(rel));
        assert_eq!(n, 0, "{rel} must have zero bare; got {n}");
        total += n;
    }
    assert_eq!(total, 0, "host_async_bare must be 0");
}

#[test]
fn s3_01_host_codegen_bare_zero() {
    assert_eq!(
        bare_diag_count(&repo_file("crates/syx-codegen/src/lib.rs")),
        0,
        "host_codegen_bare must be 0"
    );
}

#[test]
fn s3_01_host_wasm_run_bare_zero() {
    let files = [
        "crates/syx-wasm-run/src/lib.rs",
        "crates/syx-wasm-run/src/bridge.rs",
        "crates/syx-wasm-run/src/mir_runtime.rs",
        "crates/syx-wasm-run/src/atomics.rs",
        "crates/syx-wasm-run/src/wasm_run_diag.rs",
    ];
    let mut total = 0usize;
    for rel in files {
        let n = bare_diag_count(&repo_file(rel));
        assert_eq!(n, 0, "{rel} must have zero bare; got {n}");
        total += n;
    }
    assert_eq!(total, 0, "host_wasm_run_bare must be 0");
}

#[test]
fn s3_01_host_pkg_bare_zero() {
    let files = [
        "crates/syx-pkg/src/lib.rs",
        "crates/syx-pkg/src/registry.rs",
        "crates/syx-pkg/src/sources/registry.rs",
        "crates/syx-pkg/src/lock_enforce.rs",
        "crates/syx-pkg/src/sources/git.rs",
        "crates/syx-pkg/src/manifest.rs",
        "crates/syx-pkg/src/lockfile.rs",
        "crates/syx-pkg/src/sources/path.rs",
        "crates/syx-pkg/src/installer.rs",
        "crates/syx-pkg/src/resolver.rs",
        "crates/syx-pkg/src/package.rs",
        "crates/syx-pkg/src/sources/mod.rs",
        "crates/syx-pkg/src/pkg_diag.rs",
    ];
    let mut total = 0usize;
    for rel in files {
        let n = bare_diag_count(&repo_file(rel));
        assert_eq!(n, 0, "{rel} must have zero bare; got {n}");
        total += n;
    }
    assert_eq!(total, 0, "host_pkg_bare must be 0");
}

#[test]
fn s3_01_host_macro_bare_zero() {
    let files = [
        "crates/syx-macro/src/service.rs",
        "crates/syx-macro/src/builtin.rs",
        "crates/syx-macro/src/user.rs",
        "crates/syx-macro/src/external_sandbox.rs",
        "crates/syx-macro/src/external_protocol.rs",
        "crates/syx-macro/src/hygiene.rs",
        "crates/syx-macro/src/engine.rs",
        "crates/syx-macro/src/wrapper.rs",
        "crates/syx-macro/src/roles.rs",
        "crates/syx-macro/src/parse_fragment.rs",
        "crates/syx-macro/src/external.rs",
        "crates/syx-macro/src/attached.rs",
        "crates/syx-macro/src/macro_diag.rs",
    ];
    let mut total = 0usize;
    for rel in files {
        let n = bare_diag_count(&repo_file(rel));
        assert_eq!(n, 0, "{rel} must have zero bare; got {n}");
        total += n;
    }
    assert_eq!(total, 0, "host_macro_bare must be 0");
}

#[test]
fn s3_01_host_vm_hosts_bare_zero() {
    assert_eq!(
        bare_diag_count(&repo_file("crates/syx-vm/src/hosts.rs")),
        0,
        "host_vm_hosts_bare must be 0"
    );
}

#[test]
fn s3_01_host_vm_phase89_bare_zero() {
    assert_eq!(
        bare_diag_count(&repo_file(
            "crates/syx-vm/src/stdlib_phase89_hosts.rs"
        )),
        0,
        "host_vm_phase89_bare must be 0"
    );
}

#[test]
fn s3_01_host_vm_phase84_bare_zero() {
    assert_eq!(
        bare_diag_count(&repo_file(
            "crates/syx-vm/src/stdlib_phase84_hosts.rs"
        )),
        0,
        "host_vm_phase84_bare must be 0"
    );
}

#[test]
fn s3_01_host_vm_phase120_bare_zero() {
    assert_eq!(
        bare_diag_count(&repo_file(
            "crates/syx-vm/src/stdlib_phase120_hosts.rs"
        )),
        0,
        "host_vm_phase120_bare must be 0"
    );
}

#[test]
fn s3_01_host_vm_phase83_bare_zero() {
    assert_eq!(
        bare_diag_count(&repo_file(
            "crates/syx-vm/src/stdlib_phase83_hosts.rs"
        )),
        0,
        "host_vm_phase83_bare must be 0"
    );
}

#[test]
fn s3_01_host_vm_phase_tail_bare_zero() {
    let files = [
        "crates/syx-vm/src/stdlib_phase90_hosts.rs",
        "crates/syx-vm/src/stdlib_phase91_hosts.rs",
        "crates/syx-vm/src/stdlib_phase92_hosts.rs",
        "crates/syx-vm/src/stdlib_phase95_hosts.rs",
        "crates/syx-vm/src/stdlib_phase86_hosts.rs",
        "crates/syx-vm/src/stdlib_phase87_hosts.rs",
    ];
    let mut total = 0usize;
    for rel in files {
        let n = bare_diag_count(&repo_file(rel));
        assert_eq!(n, 0, "{rel} must have zero bare; got {n}");
        total += n;
    }
    assert_eq!(total, 0, "host_vm_phase_tail_bare must be 0");
}

#[test]
fn s3_01_host_vm_mid_bare_zero() {
    let files = [
        "crates/syx-vm/src/lib.rs",
        "crates/syx-vm/src/http.rs",
        "crates/syx-vm/src/os_hosts.rs",
        "crates/syx-vm/src/channel_hosts.rs",
        "crates/syx-vm/src/script_hosts.rs",
        "crates/syx-vm/src/async_iter_hosts.rs",
        "crates/syx-vm/src/gui_invoke.rs",
        "crates/syx-vm/src/gui_dlopen.rs",
        "crates/syx-vm/src/synui_hosts_stub.rs",
        "crates/syx-vm/src/jit.rs",
    ];
    let mut total = 0usize;
    for rel in files {
        let n = bare_diag_count(&repo_file(rel));
        assert_eq!(n, 0, "{rel} must have zero bare; got {n}");
        total += n;
    }
    assert_eq!(total, 0, "host_vm_mid_bare must be 0");
}

#[test]
fn s3_01_host_vm_wasm_stubs_bare_zero() {
    let files = [
        "crates/syx-vm/src/wasm_stubs/stdlib_phase89_hosts.rs",
        "crates/syx-vm/src/wasm_stubs/http.rs",
    ];
    let mut total = 0usize;
    for rel in files {
        let n = bare_diag_count(&repo_file(rel));
        assert_eq!(n, 0, "{rel} must have zero bare; got {n}");
        total += n;
    }
    assert_eq!(total, 0, "host_vm_wasm_stubs_bare must be 0");
}

#[test]
fn s3_01_host_rt_jit_debug_bare_zero() {
    let files = [
        "crates/syx-rt/src/bridge.rs",
        "crates/syx-rt/src/abi.rs",
        "crates/syx-cranelift-jit/src/lib.rs",
        "crates/syx-debug/src/lib.rs",
    ];
    let mut total = 0usize;
    for rel in files {
        let n = bare_diag_count(&repo_file(rel));
        assert_eq!(n, 0, "{rel} must have zero bare; got {n}");
        total += n;
    }
    assert_eq!(total, 0, "host_rt_jit_debug_bare must be 0");
}

#[test]
fn s3_01_host_residual_bare_zero() {
    let files = [
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-mir/src/mir_codec.rs",
        "crates/syx-mir/src/mir_lower_ctx.rs",
        "crates/syx-meta/src/lib.rs",
        "crates/syx-hir/src/lib.rs",
        "crates/syx-cst/src/lib.rs",
        "crates/syx-cst/src/lower_probe.rs",
        "crates/syx-parser/src/lib.rs",
        "crates/syx-wasm-emit/src/mir_emit.rs",
        "crates/syx-fmt/src/lib.rs",
        "crates/syx-compiler-wasm/src/lib.rs",
        "crates/syx-lexer/src/lib.rs",
        "crates/syx-lint/src/lib.rs",
    ];
    let mut total = 0usize;
    for rel in files {
        let n = bare_diag_count(&repo_file(rel));
        assert_eq!(n, 0, "{rel} must have zero bare; got {n}");
        total += n;
    }
    assert_eq!(total, 0, "host_residual_bare must be 0");
}

#[test]
fn s3_01_positionless_driver_emit_test_native_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_driver_emit_test_native_bare=0"),
        "expected positionless_driver_emit_test_native_bare=0, got:\n{stdout}"
    );
    for rel in [
        "crates/syx-driver/src/emit.rs",
        "crates/syx-driver/src/test_runner.rs",
        "crates/syx-driver/src/native_build.rs",
        "crates/syx-driver/src/run_wasm.rs",
        "crates/syx-driver/src/cranelift_stubs.rs",
    ] {
        let src = repo_file(rel);
        let bare = src.matches("driver_error(").count();
        let at = src.matches("driver_error_at(").count();
        assert_eq!(bare.saturating_sub(at), 0, "{rel} still has bare driver_error(");
    }
}

#[test]
fn s3_01_positionless_vm_phase84_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_vm_phase84_bare=0"),
        "expected positionless_vm_phase84_bare=0, got:\n{stdout}"
    );
    let src = repo_file("crates/syx-vm/src/stdlib_phase84_hosts.rs");
    let bare = src.matches("vm_error(").count();
    let at = src.matches("vm_error_at(").count();
    assert_eq!(bare.saturating_sub(at), 0, "phase84 still has bare vm_error(");
}

#[test]
fn s3_01_positionless_vm_hosts_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_vm_hosts_bare=0"),
        "expected positionless_vm_hosts_bare=0, got:\n{stdout}"
    );
    let src = repo_file("crates/syx-vm/src/hosts.rs");
    let bare = src.matches("vm_error(").count();
    let at = src.matches("vm_error_at(").count();
    assert_eq!(bare.saturating_sub(at), 0, "hosts.rs still has bare vm_error(");
}

#[test]
fn s3_01_positionless_vm_phase89_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_vm_phase89_bare=0"),
        "expected positionless_vm_phase89_bare=0, got:\n{stdout}"
    );
    for rel in [
        "crates/syx-vm/src/stdlib_phase89_hosts.rs",
        "crates/syx-vm/src/wasm_stubs/stdlib_phase89_hosts.rs",
    ] {
        let src = repo_file(rel);
        let bare = src.matches("vm_error(").count();
        let at = src.matches("vm_error_at(").count();
        assert_eq!(
            bare.saturating_sub(at),
            0,
            "{rel} still has bare vm_error("
        );
    }
}

#[test]
fn s3_01_positionless_vm_phase120_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_vm_phase120_bare=0"),
        "expected positionless_vm_phase120_bare=0, got:\n{stdout}"
    );
    let src = repo_file("crates/syx-vm/src/stdlib_phase120_hosts.rs");
    let bare = src.matches("vm_error(").count();
    let at = src.matches("vm_error_at(").count();
    assert_eq!(bare.saturating_sub(at), 0, "phase120 still has bare vm_error(");
}

#[test]
fn s3_01_positionless_vm_http_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_vm_http_bare=0"),
        "expected positionless_vm_http_bare=0, got:\n{stdout}"
    );
    for rel in [
        "crates/syx-vm/src/http.rs",
        "crates/syx-vm/src/wasm_stubs/http.rs",
    ] {
        let src = repo_file(rel);
        let bare = src.matches("vm_error(").count();
        let at = src.matches("vm_error_at(").count();
        assert_eq!(
            bare.saturating_sub(at),
            0,
            "{rel} still has bare vm_error("
        );
    }
}

#[test]
fn s3_01_positionless_vm_os_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_vm_os_bare=0"),
        "expected positionless_vm_os_bare=0, got:\n{stdout}"
    );
    let src = repo_file("crates/syx-vm/src/os_hosts.rs");
    let bare = src.matches("vm_error(").count();
    let at = src.matches("vm_error_at(").count();
    assert_eq!(bare.saturating_sub(at), 0, "os_hosts.rs still has bare vm_error(");
}

#[test]
fn s3_01_positionless_vm_lib_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_vm_lib_bare=0"),
        "expected positionless_vm_lib_bare=0, got:\n{stdout}"
    );
    let src = repo_file("crates/syx-vm/src/lib.rs");
    let bare = src.matches("vm_error(").count();
    let at = src.matches("vm_error_at(").count();
    assert_eq!(bare.saturating_sub(at), 0, "lib.rs still has bare vm_error(");
}

#[test]
fn s3_01_positionless_vm_phase83_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_vm_phase83_bare=0"),
        "expected positionless_vm_phase83_bare=0, got:\n{stdout}"
    );
    let src = repo_file("crates/syx-vm/src/stdlib_phase83_hosts.rs");
    let bare = src.matches("vm_error(").count();
    let at = src.matches("vm_error_at(").count();
    assert_eq!(bare.saturating_sub(at), 0, "phase83 still has bare vm_error(");
}

#[test]
fn s3_01_positionless_vm_channel_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_vm_channel_bare=0"),
        "expected positionless_vm_channel_bare=0, got:\n{stdout}"
    );
    let src = repo_file("crates/syx-vm/src/channel_hosts.rs");
    let bare = src.matches("vm_error(").count();
    let at = src.matches("vm_error_at(").count();
    assert_eq!(bare.saturating_sub(at), 0, "channel_hosts still has bare vm_error(");
}

#[test]
fn s3_01_positionless_vm_phase_tail_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_vm_phase_tail_bare=0"),
        "expected positionless_vm_phase_tail_bare=0, got:\n{stdout}"
    );
    for rel in [
        "crates/syx-vm/src/stdlib_phase90_hosts.rs",
        "crates/syx-vm/src/stdlib_phase91_hosts.rs",
        "crates/syx-vm/src/stdlib_phase92_hosts.rs",
        "crates/syx-vm/src/stdlib_phase95_hosts.rs",
        "crates/syx-vm/src/stdlib_phase86_hosts.rs",
        "crates/syx-vm/src/stdlib_phase87_hosts.rs",
    ] {
        let src = repo_file(rel);
        let bare = src.matches("vm_error(").count();
        let at = src.matches("vm_error_at(").count();
        assert_eq!(
            bare.saturating_sub(at),
            0,
            "{rel} still has bare vm_error("
        );
    }
}

#[test]
fn s3_01_positionless_vm_mid_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_vm_mid_bare=0"),
        "expected positionless_vm_mid_bare=0, got:\n{stdout}"
    );
    for rel in [
        "crates/syx-vm/src/script_hosts.rs",
        "crates/syx-vm/src/async_iter_hosts.rs",
        "crates/syx-vm/src/gui_invoke.rs",
        "crates/syx-vm/src/gui_dlopen.rs",
        "crates/syx-vm/src/synui_hosts_stub.rs",
        "crates/syx-vm/src/jit.rs",
    ] {
        let src = repo_file(rel);
        let bare = src.matches("vm_error(").count();
        let at = src.matches("vm_error_at(").count();
        assert_eq!(
            bare.saturating_sub(at),
            0,
            "{rel} still has bare vm_error("
        );
    }
}

#[test]
fn s3_01_positionless_wasm_run_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_wasm_run_bare=0"),
        "expected positionless_wasm_run_bare=0, got:\n{stdout}"
    );
    for rel in [
        "crates/syx-wasm-run/src/lib.rs",
        "crates/syx-wasm-run/src/bridge.rs",
    ] {
        let src = repo_file(rel);
        let bare = src.matches("wasm_run_error(").count();
        let at = src.matches("wasm_run_error_at(").count();
        assert_eq!(
            bare.saturating_sub(at),
            0,
            "{rel} still has bare wasm_run_error("
        );
    }
}

#[test]
fn s3_01_positionless_pkg_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_pkg_bare=0"),
        "expected positionless_pkg_bare=0, got:\n{stdout}"
    );
    for rel in [
        "crates/syx-pkg/src/registry.rs",
        "crates/syx-pkg/src/sources/registry.rs",
        "crates/syx-pkg/src/lock_enforce.rs",
        "crates/syx-pkg/src/sources/git.rs",
        "crates/syx-pkg/src/manifest.rs",
        "crates/syx-pkg/src/lockfile.rs",
        "crates/syx-pkg/src/lib.rs",
        "crates/syx-pkg/src/sources/path.rs",
        "crates/syx-pkg/src/installer.rs",
        "crates/syx-pkg/src/package.rs",
        "crates/syx-pkg/src/resolver.rs",
    ] {
        let src = repo_file(rel);
        let bare = src.matches("pkg_error(").count();
        let at = src.matches("pkg_error_at(").count();
        assert_eq!(
            bare.saturating_sub(at),
            0,
            "{rel} still has bare pkg_error("
        );
    }
}

#[test]
fn s3_01_positionless_macro_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_macro_bare=0"),
        "expected positionless_macro_bare=0, got:\n{stdout}"
    );
    for rel in [
        "crates/syx-macro/src/service.rs",
        "crates/syx-macro/src/builtin.rs",
        "crates/syx-macro/src/user.rs",
        "crates/syx-macro/src/external_sandbox.rs",
        "crates/syx-macro/src/external_protocol.rs",
        "crates/syx-macro/src/hygiene.rs",
        "crates/syx-macro/src/engine.rs",
        "crates/syx-macro/src/roles.rs",
        "crates/syx-macro/src/parse_fragment.rs",
        "crates/syx-macro/src/external.rs",
        "crates/syx-macro/src/attached.rs",
    ] {
        let src = repo_file(rel);
        let bare = src.matches("macro_error(").count();
        let at = src.matches("macro_error_at(").count();
        assert_eq!(
            bare.saturating_sub(at),
            0,
            "{rel} still has bare macro_error("
        );
    }
}

#[test]
fn s3_01_positionless_async_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_async_bare=0"),
        "expected positionless_async_bare=0, got:\n{stdout}"
    );
    for rel in [
        "crates/syx-async/src/runtime.rs",
        "crates/syx-async/src/hosts.rs",
        "crates/syx-async/src/task.rs",
        "crates/syx-async/src/lib.rs",
        "crates/syx-async/src/lower.rs",
    ] {
        let src = repo_file(rel);
        let bare = src.matches("async_error(").count();
        let at = src.matches("async_error_at(").count();
        assert_eq!(
            bare.saturating_sub(at),
            0,
            "{rel} still has bare async_error("
        );
    }
}

#[test]
fn s3_01_positionless_llvm_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_llvm_bare=0"),
        "expected positionless_llvm_bare=0, got:\n{stdout}"
    );
    for rel in ["crates/syx-llvm/src/lib.rs"] {
        let src = repo_file(rel);
        let bare = src.matches("llvm_error(").count();
        let at = src.matches("llvm_error_at(").count();
        assert_eq!(
            bare.saturating_sub(at),
            0,
            "{rel} still has bare llvm_error("
        );
    }
}

#[test]
fn s3_01_positionless_rt_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_rt_bare=0"),
        "expected positionless_rt_bare=0, got:\n{stdout}"
    );
    for rel in [
        "crates/syx-rt/src/bridge.rs",
        "crates/syx-rt/src/abi.rs",
    ] {
        let src = repo_file(rel);
        let bare = src.matches("rt_error(").count();
        let at = src.matches("rt_error_at(").count();
        assert_eq!(
            bare.saturating_sub(at),
            0,
            "{rel} still has bare rt_error("
        );
    }
}

#[test]
fn s3_01_positionless_jit_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_jit_bare=0"),
        "expected positionless_jit_bare=0, got:\n{stdout}"
    );
    for rel in ["crates/syx-cranelift-jit/src/lib.rs"] {
        let src = repo_file(rel);
        let bare = src.matches("jit_error(").count();
        let at = src.matches("jit_error_at(").count();
        assert_eq!(
            bare.saturating_sub(at),
            0,
            "{rel} still has bare jit_error("
        );
    }
}

#[test]
fn s3_01_positionless_debug_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_debug_bare=0"),
        "expected positionless_debug_bare=0, got:\n{stdout}"
    );
    for rel in ["crates/syx-debug/src/lib.rs"] {
        let src = repo_file(rel);
        let bare = src.matches("debug_error(").count();
        let at = src.matches("debug_error_at(").count();
        assert_eq!(
            bare.saturating_sub(at),
            0,
            "{rel} still has bare debug_error("
        );
    }
}

#[test]
fn s3_01_positionless_codegen_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_codegen_bare=0"),
        "expected positionless_codegen_bare=0, got:\n{stdout}"
    );
    for rel in ["crates/syx-codegen/src/lib.rs"] {
        let src = repo_file(rel);
        let bare = src.matches("codegen_error(").count();
        let at = src.matches("codegen_error_at(").count();
        assert_eq!(
            bare.saturating_sub(at),
            0,
            "{rel} still has bare codegen_error("
        );
    }
}

#[test]
fn s3_01_positionless_mir_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_mir_bare=0"),
        "expected positionless_mir_bare=0, got:\n{stdout}"
    );
    for rel in [
        "crates/syx-mir/src/mir_codec.rs",
        "crates/syx-mir/src/mir_lower_ctx.rs",
    ] {
        let src = repo_file(rel);
        let bare = src.matches("mir_error(").count();
        let at = src.matches("mir_error_at(").count();
        assert_eq!(bare.saturating_sub(at), 0, "{rel} still has bare mir_error(");
    }
}

#[test]
fn s3_01_positionless_mir_run_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_mir_run_bare=0"),
        "expected positionless_mir_run_bare=0, got:\n{stdout}"
    );
    let src = repo_file("crates/syx-mir-run/src/runtime.rs");
    let bare = src.matches("mir_run_error(").count();
    let at = src.matches("mir_run_error_at(").count();
    assert_eq!(bare.saturating_sub(at), 0, "runtime.rs still has bare mir_run_error(");
}

#[test]
fn s3_01_positionless_meta_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_meta_bare=0"),
        "expected positionless_meta_bare=0, got:\n{stdout}"
    );
    let src = repo_file("crates/syx-meta/src/lib.rs");
    let bare = src.matches("meta_error(").count();
    let at = src.matches("meta_error_at(").count();
    assert_eq!(bare.saturating_sub(at), 0, "meta lib still has bare meta_error(");
}

#[test]
fn s3_01_positionless_hir_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_hir_bare=0"),
        "expected positionless_hir_bare=0, got:\n{stdout}"
    );
    let src = repo_file("crates/syx-hir/src/lib.rs");
    let bare = src.matches("hir_error(").count();
    let at = src.matches("hir_error_at(").count();
    assert_eq!(bare.saturating_sub(at), 0, "hir lib still has bare hir_error(");
}

#[test]
fn s3_01_positionless_cst_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_cst_bare=0"),
        "expected positionless_cst_bare=0, got:\n{stdout}"
    );
    for rel in ["crates/syx-cst/src/lib.rs", "crates/syx-cst/src/lower_probe.rs"] {
        let src = repo_file(rel);
        let bare = src.matches("cst_error(").count();
        let at = src.matches("cst_error_at(").count();
        assert_eq!(bare.saturating_sub(at), 0, "{rel} still has bare cst_error(");
    }
}

#[test]
fn s3_01_positionless_wasm_emit_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_wasm_emit_bare=0"),
        "expected positionless_wasm_emit_bare=0, got:\n{stdout}"
    );
    let src = repo_file("crates/syx-wasm-emit/src/mir_emit.rs");
    let bare = src.matches("wasm_emit_error(").count();
    let at = src.matches("wasm_emit_error_at(").count();
    assert_eq!(bare.saturating_sub(at), 0, "mir_emit still has bare wasm_emit_error(");
}

#[test]
fn s3_01_positionless_fmt_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_fmt_bare=0"),
        "expected positionless_fmt_bare=0, got:\n{stdout}"
    );
    let src = repo_file("crates/syx-fmt/src/lib.rs");
    let bare = src.matches("fmt_error(").count();
    let at = src.matches("fmt_error_at(").count();
    assert_eq!(bare.saturating_sub(at), 0, "fmt lib still has bare fmt_error(");
}

#[test]
fn s3_01_positionless_compiler_wasm_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_compiler_wasm_bare=0"),
        "expected positionless_compiler_wasm_bare=0, got:\n{stdout}"
    );
    let src = repo_file("crates/syx-compiler-wasm/src/lib.rs");
    let bare = src.matches("compiler_wasm_error(").count();
    let at = src.matches("compiler_wasm_error_at(").count();
    assert_eq!(bare.saturating_sub(at), 0, "compiler-wasm still has bare compiler_wasm_error(");
}

#[test]
fn s3_01_positionless_lexer_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_lexer_bare=0"),
        "expected positionless_lexer_bare=0, got:\n{stdout}"
    );
    let src = repo_file("crates/syx-lexer/src/lib.rs");
    let bare = src.matches("lexer_error(").count();
    let at = src.matches("lexer_error_at(").count();
    assert_eq!(bare.saturating_sub(at), 0, "lexer lib still has bare lexer_error(");
}

#[test]
fn s3_01_positionless_lint_bare_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_lint_bare=0"),
        "expected positionless_lint_bare=0, got:\n{stdout}"
    );
    let src = repo_file("crates/syx-lint/src/lib.rs");
    let bare = src.matches("lint_error(").count();
    let at = src.matches("lint_error_at(").count();
    assert_eq!(bare.saturating_sub(at), 0, "lint lib still has bare lint_error(");
}

#[test]
fn s3_01_positionless_helper_calls_zero() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_01_span_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run census");
    assert!(output.status.success(), "census script failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("positionless_helper_calls=0"),
        "expected positionless_helper_calls=0, got:\n{stdout}"
    );
    assert!(
        stdout.contains("positionless_helper_noise="),
        "expected positionless_helper_noise echo, got:\n{stdout}"
    );
    assert!(
        stdout.contains("positionless_helper_policy="),
        "expected positionless_helper_policy echo, got:\n{stdout}"
    );
}
