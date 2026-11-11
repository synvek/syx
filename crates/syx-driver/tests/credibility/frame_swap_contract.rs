//! S3-I63 S3-04 locals frame swap (checkout+swap) contracts.
//! Hinge landed · ≠ S3-04 [x] · ≠ Stage-3 exit · ≠ Value: Copy · ≠ default JIT.
//! See docs/adr/stage3-s3-04-frame-swap.md · docs/census/stage3-vm-hotspot-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_04_frame_swap_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-04-frame-swap.md");
    assert!(adr.contains("S3-I63"), "frame-swap ADR must name S3-I63");
    assert!(
        adr.contains("mem::swap") || adr.contains("swap") || adr.contains("帧"),
        "ADR must name frame-swap hinge"
    );
    assert!(
        adr.contains("S3-I64")
            && (adr.contains("begintry")
                || adr.contains("BeginTry")
                || adr.contains("stage3-s3-04-begintry-cache")),
        "ADR must cite landed S3-I64 begintry-cache"
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
        census.contains("locals_frame_swap_helper"),
        "census must publish I63 KPI"
    );
    assert!(
        census.contains("S3-I71") && census.contains("S3-I72"),
        "census ladder must name S3-I71 landed and S3-I72 next"
    );

    let vm = repo_file("crates/syx-vm/src/lib.rs");
    assert!(
        vm.contains("mem::swap") && vm.contains("S3-I63"),
        "syx-vm must mount frame swap"
    );
    assert!(
        vm.contains("push_caller_locals_frame") && vm.contains("recycle_and_restore_locals"),
        "frame helpers must remain"
    );

    let prior = repo_file("docs/adr/stage3-s3-04-locals-prep.md");
    assert!(
        prior.contains("S3-I63") || prior.contains("stage3-s3-04-frame-swap"),
        "I62 ADR must cite landed I63"
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
        plan02.contains("S3-I63") || plan02.contains("frame-swap"),
        "plan/02 must cite S3-I63"
    );
    assert!(plan02.contains("S3-I72"), "plan/02 must lock next = S3-I72");
}

#[test]
fn s3_04_frame_swap_census_script_emits_hinge() {
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
        "locals_frame_swap_helper=",
        "locals_prep_helper=",
        "try_free_call_helper=",
        "call_stack_args_helper=",
        "arg_bind_clone=",
        "fib30_s=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    let swap = stdout
        .lines()
        .find(|l| l.starts_with("locals_frame_swap_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        swap >= 1,
        "expected locals_frame_swap_helper>=1, got {swap}\n{stdout}"
    );
}
