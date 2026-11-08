//! S3-I66 S3-04 JIT env gate cache contracts.
//! Hinge landed · ≠ S3-04 [x] · ≠ Stage-3 exit · ≠ Value: Copy · ≠ default JIT.
//! See docs/adr/stage3-s3-04-jit-env-gate-cache.md · docs/stage3-vm-hotspot-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_04_jit_env_gate_cache_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-04-jit-env-gate-cache.md");
    assert!(adr.contains("S3-I66"), "jit-env-gate-cache ADR must name S3-I66");
    assert!(
        adr.contains("AtomicU8")
            || adr.contains("env::var")
            || adr.contains("门闩"),
        "ADR must name JIT env gate cache hinge"
    );
    assert!(
        adr.contains("S3-I67")
            && (adr.contains("can_stack_bind") || adr.contains("stack-bind") || adr.contains("stage3-s3-04-can-stack-bind-cache")),
        "ADR must cite landed S3-I67 can-stack-bind-cache"
    );
    assert!(
        adr.contains("≠")
            && (adr.contains("0.5") || adr.contains("[x]") || adr.contains("JIT")),
        "ADR must Non-claim S3-04 [x] / 0.5 / default JIT"
    );

    let census = repo_file("docs/stage3-vm-hotspot-census.md");
    assert!(
        census.contains("Current:** S3-I71") || census.contains("Current: S3-I71"),
        "census Current must be S3-I71"
    );
    assert!(
        census.contains("jit_env_gate_cache_helper"),
        "census must publish I66 KPI"
    );
    assert!(
        census.contains("S3-I71") && census.contains("S3-I72"),
        "census ladder must name S3-I71 landed and S3-I72 next"
    );

    let jit = repo_file("crates/syx-vm/src/jit.rs");
    assert!(
        jit.contains("S3-I66")
            && jit.contains("AtomicU8")
            && (jit.contains("JIT_FASTPATH_GATE") || jit.contains("load_or_init_gate")),
        "jit.rs must mount AtomicU8 gate cache"
    );
    assert!(
        jit.contains("reset_jit_env_gates_for_test"),
        "jit.rs must expose test reset for env mutation"
    );
    // Cached helpers must not call env::var on every invocation after init —
    // env::var may appear only inside compute closures / uncached helpers.
    let fast = jit
        .split("fn fastpath_enabled")
        .nth(1)
        .and_then(|s| s.split("pub fn ").next())
        .unwrap_or("");
    assert!(
        fast.contains("load_or_init_gate") || fast.contains("JIT_FASTPATH_GATE"),
        "fastpath_enabled must use gate cache"
    );
    let tier2 = jit
        .split("fn tier2_opt_in_enabled")
        .nth(1)
        .and_then(|s| s.split("pub fn ").next())
        .unwrap_or("");
    assert!(
        tier2.contains("load_or_init_gate") || tier2.contains("JIT_TIER2_GATE"),
        "tier2_opt_in_enabled must use gate cache"
    );

    let prior = repo_file("docs/adr/stage3-s3-04-tryfree-dispatch.md");
    assert!(
        prior.contains("S3-I66")
            || prior.contains("stage3-s3-04-jit-env-gate-cache"),
        "I65 ADR must cite landed I66"
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
        plan02.contains("S3-I66") || plan02.contains("jit-env-gate"),
        "plan/02 must cite S3-I66"
    );
    assert!(
        plan02.contains("S3-I72"),
        "plan/02 must lock next = S3-I72"
    );
}

#[test]
fn s3_04_jit_env_gate_cache_census_script_emits_hinge() {
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
        "jit_env_gate_cache_helper=",
        "tryfree_dispatch_helper=",
        "begin_try_cache_helper=",
        "try_free_call_helper=",
        "arg_bind_clone=",
        "fib30_s=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    let gate = stdout
        .lines()
        .find(|l| l.starts_with("jit_env_gate_cache_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        gate >= 1,
        "expected jit_env_gate_cache_helper>=1, got {gate}\n{stdout}"
    );
}
