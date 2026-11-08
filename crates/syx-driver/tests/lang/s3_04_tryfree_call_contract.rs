//! S3-I61 S3-04 try-free Call (skip try_handler_frames) contracts.
//! Hinge landed · ≠ S3-04 [x] · ≠ Stage-3 exit · ≠ Value: Copy · ≠ default JIT.
//! See docs/adr/stage3-s3-04-tryfree-call.md · docs/stage3-vm-hotspot-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_04_tryfree_call_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-04-tryfree-call.md");
    assert!(adr.contains("S3-I61"), "tryfree-call ADR must name S3-I61");
    assert!(
        adr.contains("stack_args_needs_try_frame")
            || adr.contains("track_try_frame")
            || adr.contains("try-free"),
        "ADR must name try-free hinge"
    );
    assert!(
        (adr.contains("S3-I64") || adr.contains("S3-I63"))
            && (adr.contains("stage3-s3-04-begintry-cache")
                || adr.contains("stage3-s3-04-frame-swap")
                || adr.contains("begintry")
                || adr.contains("frame")
                || adr.contains("swap")),
        "ADR must cite landed I63/I64 chain"
    );
    assert!(
        adr.contains("≠")
            && (adr.contains("0.5") || adr.contains("[x]") || adr.contains("Copy")),
        "ADR must Non-claim S3-04 [x] / 0.5 / Value Copy"
    );

    let census = repo_file("docs/stage3-vm-hotspot-census.md");
    assert!(
        census.contains("Current:** S3-I71") || census.contains("Current: S3-I71"),
        "census Current must be S3-I71"
    );
    assert!(
        census.contains("try_free_call_helper"),
        "census must publish I61 KPI"
    );
    assert!(
        census.contains("S3-I71") && census.contains("S3-I72"),
        "census ladder must name S3-I71 landed and S3-I72 next"
    );

    let vm = repo_file("crates/syx-vm/src/lib.rs");
    assert!(
        vm.contains("stack_args_needs_try_frame")
            && vm.contains("track_try_frame")
            && vm.contains("S3-I61"),
        "syx-vm must mount try-free Call helpers"
    );
    assert!(
        vm.contains("run_function_with_stack_args") && vm.contains("S3-I60"),
        "I60 stack-args must remain"
    );

    let prior = repo_file("docs/adr/stage3-s3-04-call-stack-args.md");
    assert!(
        prior.contains("S3-I61")
            || prior.contains("stage3-s3-04-tryfree-call"),
        "I60 ADR must cite landed I61"
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
        plan02.contains("S3-I61") || plan02.contains("tryfree-call"),
        "plan/02 must cite S3-I61"
    );
    assert!(
        plan02.contains("S3-I62") || plan02.contains("locals-prep"),
        "plan/02 must cite S3-I62"
    );
    assert!(
        plan02.contains("S3-I63") || plan02.contains("frame-swap"),
        "plan/02 must cite S3-I63"
    );
    assert!(
        plan02.contains("S3-I72"),
        "plan/02 must lock next = S3-I72"
    );
}

#[test]
fn s3_04_tryfree_call_census_script_emits_hinge() {
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
        "try_free_call_helper=",
        "call_stack_args_helper=",
        "load_local_exec_fastpath=",
        "store_local_slot_clone=",
        "arg_bind_clone=",
        "fib30_s=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    let try_free = stdout
        .lines()
        .find(|l| l.starts_with("try_free_call_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        try_free >= 1,
        "expected try_free_call_helper>=1, got {try_free}\n{stdout}"
    );
}
