//! S3-I56…I71 S3-04 VM-hotspot census contracts.
//! I56 open · I57–I63 construct · ≠ S3-04 [x] · ≠ Stage-3 exit.
//! See docs/adr/stage3-s3-04-claim.md · docs/census/stage3-vm-hotspot-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_04_vm_hotspot_census_baseline() {
    let census = repo_file("docs/census/stage3-vm-hotspot-census.md");
    assert!(census.contains("S3-I56"), "census must name S3-I56 open");
    assert!(
        census.contains("S3-I57"),
        "census must name S3-I57 construct"
    );
    assert!(
        census.contains("S3-I58"),
        "census must name S3-I58 construct"
    );
    assert!(
        census.contains("S3-I59"),
        "census must name S3-I59 construct"
    );
    assert!(
        census.contains("S3-I60"),
        "census must name S3-I60 construct"
    );
    assert!(
        census.contains("S3-I61"),
        "census must name S3-I61 construct"
    );
    assert!(
        census.contains("S3-I62"),
        "census must name S3-I62 construct"
    );
    assert!(
        census.contains("S3-I63"),
        "census must name S3-I63 construct"
    );
    assert!(
        census.contains("S3-I64"),
        "census must name S3-I64 construct"
    );
    assert!(
        census.contains("S3-I65"),
        "census must name S3-I65 construct"
    );
    assert!(
        census.contains("S3-I66"),
        "census must name S3-I66 construct"
    );
    assert!(
        census.contains("S3-I67"),
        "census must name S3-I67 construct"
    );
    assert!(
        census.contains("S3-I68"),
        "census must name S3-I68 construct"
    );
    assert!(
        census.contains("S3-I69"),
        "census must name S3-I69 construct"
    );
    assert!(
        census.contains("S3-I70"),
        "census must name S3-I70 construct"
    );
    assert!(
        census.contains("S3-I71"),
        "census must name S3-I71 construct"
    );
    assert!(
        census.contains("Current:** S3-I71") || census.contains("Current: S3-I71"),
        "census Current must be S3-I71"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(
        census.contains("fib30_s") && census.contains("fib30_target_s"),
        "census must publish fib30 KPIs"
    );
    assert!(
        census.contains("0.5"),
        "census must name 0.5 target without claiming"
    );
    assert!(
        census.contains("value_clone_mentions")
            && census.contains("arg_bind_clone")
            && census.contains("owned_args_helper")
            && census.contains("locals_frame_reuse_helper")
            && census.contains("load_local_exec_fastpath")
            && census.contains("store_local_slot_clone")
            && census.contains("call_stack_args_helper")
            && census.contains("try_free_call_helper")
            && census.contains("locals_prep_helper")
            && census.contains("locals_frame_swap_helper")
            && census.contains("begin_try_cache_helper")
            && census.contains("tryfree_dispatch_helper")
            && census.contains("jit_env_gate_cache_helper")
            && census.contains("can_stack_bind_cache_helper")
            && census.contains("claim_prep_helper")
            && census.contains("wall_strategy_helper")
            && census.contains("residual_scope_helper")
            && census.contains("s3_04_claim_helper")
            && census.contains("call_path_helper"),
        "census must publish call/Value/frame/load structural axes"
    );
    assert!(
        census.contains("fib_criterion_bench") && census.contains("hotspot_count_helper"),
        "census must publish criterion/JIT gap axes"
    );
    assert!(
        census.contains("S3-I71") && census.contains("S3-I72"),
        "census ladder must name S3-I71 and S3-I72"
    );
    assert!(
        census.contains("≠") && census.contains("Stage-3"),
        "census must Non-claim Stage-3 exit"
    );
    assert!(
        census.contains("≠") && (census.contains("S3-04 `[x]`") || census.contains("[x]")),
        "census must Non-claim S3-04 [x]"
    );

    let open_adr = repo_file("docs/adr/stage3-s3-04-open.md");
    assert!(open_adr.contains("S3-I56"));
    assert!(
        open_adr.contains("S3-04") && open_adr.contains("`[~]`"),
        "open ADR must Status S3-04 [~]"
    );
    assert!(
        open_adr.contains("S3-I71") && open_adr.contains("stage3-s3-04-claim"),
        "open ADR must cite landed S3-I71 claim"
    );
    assert!(
        open_adr.contains("S3-I72"),
        "open ADR must lock next = S3-I72"
    );
    assert!(
        open_adr.contains("≠") && open_adr.contains("0.5"),
        "open ADR must Non-claim 0.5 this window"
    );

    let script = repo_file("scripts/s3_04_vm_hotspot_census.sh");
    assert!(
        script.contains("fib30_s") && script.contains("fib30_target_s=0.5"),
        "census script must emit fib timings + target"
    );
    assert!(
        script.contains("locals_frame_reuse_helper")
            && script.contains("load_local_exec_fastpath")
            && script.contains("store_local_slot_clone")
            && script.contains("call_stack_args_helper")
            && script.contains("try_free_call_helper")
            && script.contains("locals_prep_helper")
            && script.contains("locals_frame_swap_helper")
            && script.contains("begin_try_cache_helper")
            && script.contains("tryfree_dispatch_helper")
            && script.contains("jit_env_gate_cache_helper")
            && script.contains("can_stack_bind_cache_helper")
            && script.contains("claim_prep_helper")
            && script.contains("wall_strategy_helper")
            && script.contains("residual_scope_helper")
            && script.contains("s3_04_claim_helper")
            && script.contains("arg_bind_clone")
            && script.contains("owned_args_helper"),
        "census script must emit structural KPIs"
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
        plan02.contains("S3-I62") || plan02.contains("locals-prep"),
        "plan/02 must cite S3-I62"
    );
    assert!(
        plan02.contains("S3-I63") || plan02.contains("frame-swap"),
        "plan/02 must cite S3-I63"
    );
    assert!(
        plan02.contains("S3-I64") || plan02.contains("begintry-cache"),
        "plan/02 must cite S3-I64"
    );
    assert!(
        plan02.contains("S3-I65") || plan02.contains("tryfree-dispatch"),
        "plan/02 must cite S3-I65"
    );
    assert!(
        plan02.contains("S3-I66") || plan02.contains("jit-env"),
        "plan/02 must cite S3-I66"
    );
    assert!(
        plan02.contains("S3-I67")
            || plan02.contains("can-stack-bind")
            || plan02.contains("can_stack_bind"),
        "plan/02 must cite S3-I67"
    );
    assert!(
        plan02.contains("S3-I68") || plan02.contains("claim-prep"),
        "plan/02 must cite S3-I68"
    );
    assert!(
        plan02.contains("S3-I69") || plan02.contains("wall-strategy"),
        "plan/02 must cite S3-I69"
    );
    assert!(
        plan02.contains("S3-I70") || plan02.contains("residual-scope"),
        "plan/02 must cite S3-I70"
    );
    assert!(
        plan02.contains("S3-I71") || plan02.contains("stage3-s3-04-claim"),
        "plan/02 must cite S3-I71"
    );
    assert!(plan02.contains("S3-I72"), "plan/02 must lock next = S3-I72");
    let stage3 = plan02
        .lines()
        .find(|l| l.contains("| **3** |"))
        .expect("stage-3 row");
    assert!(
        stage3.contains("[x]") && stage3.contains("退出"),
        "Stage-3 must be [x] 退出; got: {stage3}"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I71") && plan01.contains("S3-04"),
        "plan/01 must lock current toward S3-04 / I71"
    );
}

#[test]
fn s3_04_vm_hotspot_census_script_runs() {
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
        "value_clone_mentions=",
        "arg_clone_helper=",
        "arg_bind_clone=",
        "owned_args_helper=",
        "locals_frame_reuse_helper=",
        "load_local_exec_fastpath=",
        "store_local_slot_clone=",
        "call_stack_args_helper=",
        "try_free_call_helper=",
        "locals_prep_helper=",
        "locals_frame_swap_helper=",
        "begin_try_cache_helper=",
        "tryfree_dispatch_helper=",
        "jit_env_gate_cache_helper=",
        "can_stack_bind_cache_helper=",
        "claim_prep_helper=",
        "wall_strategy_helper=",
        "residual_scope_helper=",
        "s3_04_claim_helper=",
        "fib30_tier2_s=",
        "fib30_interp_ceiling_s=",
        "call_path_helper=",
        "suspend_frame_helper=",
        "hotspot_count_helper=",
        "syx_jit_env_mentions=",
        "fib_criterion_bench=",
        "examples_fib=",
        "fib30_target_s=0.5",
        "fib30_s=",
        "fib30_warm_s=",
        "hot_loop_s=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    assert!(
        stdout.contains("I71 scoped")
            || stdout.contains("≠ Stage-3 exit")
            || stdout.contains("I70 residual-scope")
            || stdout.contains("I69 wall-strategy")
            || stdout.contains("I68 claim-prep")
            || stdout.contains("I67 can-stack-bind-cache")
            || stdout.contains("I66 jit-env-gate-cache")
            || stdout.contains("I65 tryfree-dispatch")
            || stdout.contains("I64 begintry-cache")
            || stdout.contains("I63 frame-swap"),
        "script must non-claim S3-04 [x], got:\n{stdout}"
    );
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
    let bind = stdout
        .lines()
        .find(|l| l.starts_with("arg_bind_clone="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(1);
    assert_eq!(bind, 0, "expected arg_bind_clone=0, got {bind}\n{stdout}");
    let stack_args = stdout
        .lines()
        .find(|l| l.starts_with("call_stack_args_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        stack_args >= 1,
        "expected call_stack_args_helper>=1, got {stack_args}\n{stdout}"
    );
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
    let prep = stdout
        .lines()
        .find(|l| l.starts_with("locals_prep_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        prep >= 1,
        "expected locals_prep_helper>=1, got {prep}\n{stdout}"
    );
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
    let begin_try = stdout
        .lines()
        .find(|l| l.starts_with("begin_try_cache_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        begin_try >= 1,
        "expected begin_try_cache_helper>=1, got {begin_try}\n{stdout}"
    );
    let tryfree = stdout
        .lines()
        .find(|l| l.starts_with("tryfree_dispatch_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        tryfree >= 1,
        "expected tryfree_dispatch_helper>=1, got {tryfree}\n{stdout}"
    );
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
    let csb = stdout
        .lines()
        .find(|l| l.starts_with("can_stack_bind_cache_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        csb >= 1,
        "expected can_stack_bind_cache_helper>=1, got {csb}\n{stdout}"
    );
    let claim = stdout
        .lines()
        .find(|l| l.starts_with("claim_prep_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        claim >= 1,
        "expected claim_prep_helper>=1, got {claim}\n{stdout}"
    );
    let examples = stdout
        .lines()
        .find(|l| l.starts_with("examples_fib="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        examples >= 1,
        "expected examples_fib>=1, got {examples}\n{stdout}"
    );
    let fib_bench = stdout
        .lines()
        .find(|l| l.starts_with("fib_criterion_bench="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        fib_bench >= 1,
        "expected fib_criterion_bench>=1, got {fib_bench}\n{stdout}"
    );
    let wall = stdout
        .lines()
        .find(|l| l.starts_with("wall_strategy_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        wall >= 1,
        "expected wall_strategy_helper>=1, got {wall}\n{stdout}"
    );
    let residual = stdout
        .lines()
        .find(|l| l.starts_with("residual_scope_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        residual >= 1,
        "expected residual_scope_helper>=1, got {residual}\n{stdout}"
    );
    let claim = stdout
        .lines()
        .find(|l| l.starts_with("s3_04_claim_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        claim >= 1,
        "expected s3_04_claim_helper>=1, got {claim}\n{stdout}"
    );
}
