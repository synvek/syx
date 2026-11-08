//! S3-I59 S3-04 LoadLocal scalar fastpath + StoreLocal move contracts.
//! Hinge landed · ≠ S3-04 [x] · ≠ Stage-3 exit · ≠ Value: Copy · ≠ default JIT.
//! See docs/adr/stage3-s3-04-loadlocal-fastpath.md · docs/stage3-vm-hotspot-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_04_loadlocal_fastpath_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-04-loadlocal-fastpath.md");
    assert!(adr.contains("S3-I59"), "loadlocal-fastpath ADR must name S3-I59");
    assert!(
        adr.contains("copy_value_for_stack")
            || adr.contains("fast-path")
            || adr.contains("fastpath"),
        "ADR must name scalar fastpath hinge"
    );
    assert!(
        adr.contains("S3-I60")
            && (adr.contains("stage3-s3-04-call-stack-args")
                || adr.contains("stack")
                || adr.contains("Call")),
        "ADR must cite landed S3-I60 call-stack-args"
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
        census.contains("load_local_exec_fastpath")
            && census.contains("store_local_slot_clone"),
        "census must publish I59 KPIs"
    );
    assert!(
        census.contains("S3-I71") && census.contains("S3-I72"),
        "census ladder must name S3-I71 landed and S3-I72 next"
    );

    let vm = repo_file("crates/syx-vm/src/lib.rs");
    assert!(
        vm.contains("copy_value_for_stack") && vm.contains("S3-I59"),
        "syx-vm must mount copy_value_for_stack"
    );
    assert!(
        !vm.contains("locals[*i as usize], v.clone()"),
        "StoreLocal must not clone into slot"
    );
    assert!(
        !vm.contains("locals[i] = arg.clone()"),
        "I57 arg bind clone must stay 0"
    );

    let prior = repo_file("docs/adr/stage3-s3-04-locals-frame-pool.md");
    assert!(
        prior.contains("S3-I59")
            || prior.contains("stage3-s3-04-loadlocal-fastpath"),
        "I58 ADR must cite landed I59"
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
fn s3_04_loadlocal_fastpath_census_script_emits_hinge() {
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
    let fast = stdout
        .lines()
        .find(|l| l.starts_with("load_local_exec_fastpath="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        fast >= 1,
        "expected load_local_exec_fastpath>=1, got {fast}\n{stdout}"
    );
    let store_clone = stdout
        .lines()
        .find(|l| l.starts_with("store_local_slot_clone="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(1);
    assert_eq!(
        store_clone, 0,
        "expected store_local_slot_clone=0, got {store_clone}\n{stdout}"
    );
}
