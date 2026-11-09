//! S3-I67 S3-04 can_stack_bind compile-time cache contracts.
//! Hinge landed · ≠ S3-04 [x] · ≠ Stage-3 exit · ≠ Value: Copy · ≠ default JIT.
//! See docs/adr/stage3-s3-04-can-stack-bind-cache.md · docs/census/stage3-vm-hotspot-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_04_can_stack_bind_cache_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-04-can-stack-bind-cache.md");
    assert!(adr.contains("S3-I67"), "can-stack-bind-cache ADR must name S3-I67");
    assert!(
        adr.contains("can_stack_bind")
            || adr.contains("compute_can_stack_bind")
            || adr.contains("stack-bind"),
        "ADR must name can_stack_bind hinge"
    );
    assert!(
        adr.contains("S3-I68")
            && (adr.contains("claim-prep") || adr.contains("claim") || adr.contains("stage3-s3-04-claim-prep")),
        "ADR must cite landed S3-I68 claim-prep"
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
        census.contains("can_stack_bind_cache_helper"),
        "census must publish I67 KPI"
    );
    assert!(
        census.contains("S3-I71") && census.contains("S3-I72"),
        "census ladder must name S3-I71 landed and S3-I72 next"
    );

    let vm = repo_file("crates/syx-vm/src/lib.rs");
    assert!(
        vm.contains("can_stack_bind") && vm.contains("S3-I67"),
        "syx-vm must read can_stack_bind on Call gate"
    );
    let bind_fn = vm
        .split("fn call_can_stack_bind")
        .nth(1)
        .and_then(|s| s.split("fn ").next())
        .unwrap_or("");
    assert!(
        !bind_fn.contains("ends_with") && !bind_fn.contains("parse_mangled"),
        "call_can_stack_bind must not scan name on hot path"
    );

    let codegen = repo_file("crates/syx-codegen/src/lib.rs");
    assert!(
        codegen.contains("can_stack_bind")
            && codegen.contains("compute_can_stack_bind")
            && codegen.contains("S3-I67"),
        "codegen must mount can_stack_bind cache"
    );

    let prior = repo_file("docs/adr/stage3-s3-04-jit-env-gate-cache.md");
    assert!(
        prior.contains("S3-I67")
            || prior.contains("stage3-s3-04-can-stack-bind-cache"),
        "I66 ADR must cite landed I67"
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
        plan02.contains("S3-I67") || plan02.contains("can-stack-bind"),
        "plan/02 must cite S3-I67"
    );
    assert!(
        plan02.contains("S3-I72"),
        "plan/02 must lock next = S3-I72"
    );
}

#[test]
fn s3_04_can_stack_bind_cache_census_script_emits_hinge() {
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
        "can_stack_bind_cache_helper=",
        "jit_env_gate_cache_helper=",
        "tryfree_dispatch_helper=",
        "begin_try_cache_helper=",
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
        .find(|l| l.starts_with("can_stack_bind_cache_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        cache >= 1,
        "expected can_stack_bind_cache_helper>=1, got {cache}\n{stdout}"
    );
}
