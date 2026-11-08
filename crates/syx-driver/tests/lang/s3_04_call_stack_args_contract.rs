//! S3-I60 S3-04 Call stack→locals (no intermediate Vec) contracts.
//! Hinge landed · ≠ S3-04 [x] · ≠ Stage-3 exit · ≠ Value: Copy · ≠ default JIT.
//! See docs/adr/stage3-s3-04-call-stack-args.md · docs/stage3-vm-hotspot-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_04_call_stack_args_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-04-call-stack-args.md");
    assert!(adr.contains("S3-I60"), "call-stack-args ADR must name S3-I60");
    assert!(
        adr.contains("run_function_with_stack_args")
            || adr.contains("stack-args")
            || adr.contains("栈参"),
        "ADR must name stack-args hinge"
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
        census.contains("call_stack_args_helper"),
        "census must publish I60 KPI"
    );
    assert!(
        census.contains("S3-I71") && census.contains("S3-I72"),
        "census ladder must name S3-I71 landed and S3-I72 next"
    );

    let vm = repo_file("crates/syx-vm/src/lib.rs");
    assert!(
        vm.contains("run_function_with_stack_args")
            && vm.contains("call_can_stack_bind")
            && vm.contains("S3-I60"),
        "syx-vm must mount stack-args API"
    );
    assert!(
        !vm.contains("locals[i] = arg.clone()"),
        "I57 arg bind clone must stay 0"
    );

    let prior = repo_file("docs/adr/stage3-s3-04-loadlocal-fastpath.md");
    assert!(
        prior.contains("S3-I60")
            || prior.contains("stage3-s3-04-call-stack-args"),
        "I59 ADR must cite landed I60"
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
        plan02.contains("S3-I60") || plan02.contains("call-stack-args"),
        "plan/02 must cite S3-I60"
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
fn s3_04_call_stack_args_census_script_emits_hinge() {
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
        "call_stack_args_helper=",
        "load_local_exec_fastpath=",
        "store_local_slot_clone=",
        "locals_frame_reuse_helper=",
        "arg_bind_clone=",
        "fib30_s=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    let stack = stdout
        .lines()
        .find(|l| l.starts_with("call_stack_args_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        stack >= 1,
        "expected call_stack_args_helper>=1, got {stack}\n{stdout}"
    );
    let bind = stdout
        .lines()
        .find(|l| l.starts_with("arg_bind_clone="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(1);
    assert_eq!(bind, 0, "expected arg_bind_clone=0, got {bind}\n{stdout}");
}
