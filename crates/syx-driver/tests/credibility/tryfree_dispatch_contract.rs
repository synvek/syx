//! S3-I65 S3-04 try-free dispatch (hot interpret) contracts.
//! Hinge landed · ≠ S3-04 [x] · ≠ Stage-3 exit · ≠ Value: Copy · ≠ default JIT.
//! See docs/adr/stage3-s3-04-tryfree-dispatch.md · docs/census/stage3-vm-hotspot-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_04_tryfree_dispatch_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-04-tryfree-dispatch.md");
    assert!(
        adr.contains("S3-I65"),
        "tryfree-dispatch ADR must name S3-I65"
    );
    assert!(
        adr.contains("run_function_body_tryfree")
            || adr.contains("tryfree")
            || adr.contains("调度"),
        "ADR must name try-free dispatch hinge"
    );
    assert!(
        adr.contains("S3-I66")
            && (adr.contains("jit")
                || adr.contains("env")
                || adr.contains("stage3-s3-04-jit-env-gate-cache")),
        "ADR must cite landed S3-I66 jit-env-gate-cache"
    );
    assert!(
        adr.contains("≠") && (adr.contains("0.5") || adr.contains("[x]") || adr.contains("Copy")),
        "ADR must Non-claim S3-04 [x] / 0.5 / Value Copy"
    );

    let census = repo_file("docs/census/stage3-vm-hotspot-census.md");
    assert!(
        census.contains("Current:** S3-I71") || census.contains("Current: S3-I71"),
        "census Current must be S3-I71"
    );
    assert!(
        census.contains("tryfree_dispatch_helper"),
        "census must publish I65 KPI"
    );
    assert!(
        census.contains("S3-I71") && census.contains("S3-I72"),
        "census ladder must name S3-I71 landed and S3-I72 next"
    );

    let vm = repo_file("crates/syx-vm/src/lib.rs");
    assert!(
        vm.contains("run_function_body_tryfree") && vm.contains("S3-I65"),
        "syx-vm must mount try-free dispatch"
    );
    // tryfree must not poll/take pending_catch each insn (cold/Call may still check throw).
    let tryfree = vm
        .split("fn run_function_body_tryfree")
        .nth(1)
        .and_then(|s| s.split("fn finish_function_body_unwind").next())
        .unwrap_or("");
    assert!(
        !tryfree.contains("pending_catch"),
        "tryfree must not poll pending_catch"
    );

    let prior = repo_file("docs/adr/stage3-s3-04-begintry-cache.md");
    assert!(
        prior.contains("S3-I65") || prior.contains("stage3-s3-04-tryfree-dispatch"),
        "I64 ADR must cite landed I65"
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
        plan02.contains("S3-I65") || plan02.contains("tryfree-dispatch"),
        "plan/02 must cite S3-I65"
    );
    assert!(plan02.contains("S3-I72"), "plan/02 must lock next = S3-I72");
}

#[test]
fn s3_04_tryfree_dispatch_census_script_emits_hinge() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_04_vm_hotspot_census.sh")
        .current_dir(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .output()
        .expect("run vm-hotspot census");
    assert!(
        output.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for key in [
        "tryfree_dispatch_helper=",
        "begin_try_cache_helper=",
        "locals_frame_swap_helper=",
        "try_free_call_helper=",
        "arg_bind_clone=",
        "fib30_s=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    let dispatch = stdout
        .lines()
        .find(|l| l.starts_with("tryfree_dispatch_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        dispatch >= 1,
        "expected tryfree_dispatch_helper>=1, got {dispatch}\n{stdout}"
    );
}
