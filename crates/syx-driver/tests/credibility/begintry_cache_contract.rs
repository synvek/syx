//! S3-I64 S3-04 BeginTry compile-time cache contracts.
//! Hinge landed · ≠ S3-04 [x] · ≠ Stage-3 exit · ≠ Value: Copy · ≠ default JIT.
//! See docs/adr/stage3-s3-04-begintry-cache.md · docs/census/stage3-vm-hotspot-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_04_begintry_cache_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-04-begintry-cache.md");
    assert!(adr.contains("S3-I64"), "begintry-cache ADR must name S3-I64");
    assert!(
        adr.contains("has_begin_try")
            || adr.contains("BeginTry")
            || adr.contains("begin_try"),
        "ADR must name BeginTry cache hinge"
    );
    assert!(
        adr.contains("S3-I65")
            && (adr.contains("tryfree") || adr.contains("dispatch") || adr.contains("stage3-s3-04-tryfree-dispatch")),
        "ADR must cite landed S3-I65 tryfree-dispatch"
    );
    assert!(
        adr.contains("≠")
            && (adr.contains("0.5") || adr.contains("[x]") || adr.contains("Copy")),
        "ADR must Non-claim S3-04 [x] / 0.5 / Value Copy"
    );

    let census = repo_file("docs/census/stage3-vm-hotspot-census.md");
    assert!(
        census.contains("Current:** S3-I71") || census.contains("Current: S3-I71"),
        "census Current must be S3-I71"
    );
    assert!(
        census.contains("begin_try_cache_helper"),
        "census must publish I64 KPI"
    );
    assert!(
        census.contains("S3-I71") && census.contains("S3-I72"),
        "census ladder must name S3-I71 landed and S3-I72 next"
    );

    let vm = repo_file("crates/syx-vm/src/lib.rs");
    assert!(
        vm.contains("has_begin_try") && vm.contains("S3-I64"),
        "syx-vm must read has_begin_try on try-free gate"
    );
    assert!(
        vm.contains("stack_args_needs_try_frame"),
        "try-free gate helper must remain"
    );
    // Hot path must not re-scan via instructions.iter().any(BeginTry).
    let needs_fn = vm
        .split("fn stack_args_needs_try_frame")
        .nth(1)
        .and_then(|s| s.split("fn ").next())
        .unwrap_or("");
    assert!(
        !needs_fn.contains("Instruction::BeginTry")
            && !needs_fn.contains(".any("),
        "stack_args_needs_try_frame must not scan BeginTry on hot path"
    );

    let codegen = repo_file("crates/syx-codegen/src/lib.rs");
    assert!(
        codegen.contains("has_begin_try")
            && codegen.contains("compute_has_begin_try")
            && codegen.contains("S3-I64"),
        "codegen must mount has_begin_try cache"
    );

    let prior = repo_file("docs/adr/stage3-s3-04-frame-swap.md");
    assert!(
        prior.contains("S3-I64")
            || prior.contains("stage3-s3-04-begintry-cache"),
        "I63 ADR must cite landed I64"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s3_04 = plan02
        .lines()
        .find(|l| l.contains("| S3-04 |"))
        .expect("S3-04 row");
    assert!(
        s3_04.contains("| [x] |") || s3_04.contains("| `[x]` |"),
        "S3-04 must be [x] after I71 claim; got: {s3_04}"
    );
    assert!(
        plan02.contains("S3-I64") || plan02.contains("begintry-cache"),
        "plan/02 must cite S3-I64"
    );
    assert!(
        plan02.contains("S3-I72"),
        "plan/02 must lock next = S3-I72"
    );
}

#[test]
fn s3_04_begintry_cache_census_script_emits_hinge() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_04_vm_hotspot_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run vm-hotspot census");
    assert!(
        output.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for key in [
        "begin_try_cache_helper=",
        "locals_frame_swap_helper=",
        "locals_prep_helper=",
        "try_free_call_helper=",
        "arg_bind_clone=",
        "fib30_s=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    let cache = stdout
        .lines()
        .find(|l| l.starts_with("begin_try_cache_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        cache >= 1,
        "expected begin_try_cache_helper>=1, got {cache}\n{stdout}"
    );
}
