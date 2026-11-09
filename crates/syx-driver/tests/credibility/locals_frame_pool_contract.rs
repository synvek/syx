//! S3-I58 S3-04 locals_buf_pool frame-reuse contracts.
//! Hinge landed · ≠ S3-04 [x] · ≠ Stage-3 exit · ≠ default JIT.
//! See docs/adr/stage3-s3-04-locals-frame-pool.md · docs/census/stage3-vm-hotspot-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_04_locals_frame_pool_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-04-locals-frame-pool.md");
    assert!(adr.contains("S3-I58"), "locals-frame-pool ADR must name S3-I58");
    assert!(
        adr.contains("locals_buf_pool")
            && (adr.contains("push_caller_locals_frame")
                || adr.contains("recycle_and_restore_locals")),
        "ADR must name pool helpers"
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
        adr.contains("≠") && (adr.contains("0.5") || adr.contains("[x]")),
        "ADR must Non-claim S3-04 [x] / 0.5 this window"
    );

    let census = repo_file("docs/census/stage3-vm-hotspot-census.md");
    assert!(
        census.contains("Current:** S3-I71") || census.contains("Current: S3-I71"),
        "census Current must be S3-I71"
    );
    assert!(
        census.contains("locals_frame_reuse_helper"),
        "census must publish I58 KPI"
    );
    assert!(
        census.contains("S3-I71") && census.contains("S3-I72"),
        "census ladder must name S3-I71 landed and S3-I72 next"
    );

    let vm = repo_file("crates/syx-vm/src/lib.rs");
    assert!(
        vm.contains("locals_buf_pool")
            && vm.contains("push_caller_locals_frame")
            && vm.contains("recycle_and_restore_locals")
            && vm.contains("S3-I58"),
        "syx-vm must mount locals frame pool"
    );
    assert!(
        !vm.contains("locals[i] = arg.clone()"),
        "syx-vm must keep I57 arg bind clone=0"
    );

    let prior = repo_file("docs/adr/stage3-s3-04-call-args-move.md");
    assert!(
        prior.contains("S3-I58")
            || prior.contains("stage3-s3-04-locals-frame-pool"),
        "I57 ADR must cite landed I58"
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
        plan02.contains("S3-I58") || plan02.contains("locals-frame-pool"),
        "plan/02 must cite S3-I58"
    );
    assert!(
        plan02.contains("S3-I59") || plan02.contains("loadlocal-fastpath"),
        "plan/02 must cite S3-I59"
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
fn s3_04_locals_frame_pool_census_script_emits_hinge() {
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
        "locals_frame_reuse_helper=",
        "arg_bind_clone=",
        "owned_args_helper=",
        "fib30_s=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    let reuse = stdout
        .lines()
        .find(|l| l.starts_with("locals_frame_reuse_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        reuse >= 1,
        "expected locals_frame_reuse_helper>=1, got {reuse}\n{stdout}"
    );
    let bind = stdout
        .lines()
        .find(|l| l.starts_with("arg_bind_clone="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(1);
    assert_eq!(bind, 0, "expected arg_bind_clone=0, got {bind}\n{stdout}");
}
