//! S3-I53 S3-03 cache roundtrip contracts.
//! Hinge landed · ≠ S3-03 [x] · ≠ hello < 0.15 claim · ≠ Stage-3 exit.
//! See docs/adr/stage3-s3-03-cache-roundtrip.md · docs/census/stage3-stdlib-cache-census.md.

use smol_str::SmolStr;
use syx_codegen::{CompiledProgram, FunctionUnit, Instruction};

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_03_cache_roundtrip_function_unit_and_program() {
    // Empty optional fields previously used skip_serializing_if → bincode UnexpectedEof.
    let fu = FunctionUnit {
        name: SmolStr::new_static("main"),
        param_count: 0,
        local_count: 0,
        instructions: vec![Instruction::ConstI32(0), Instruction::Return],
        instruction_spans: Vec::new(),
        local_names: Vec::new(),
        local_types: Vec::new(),
        source_name: SmolStr::default(),
        init_body_pc: 0,
        init_wrap_pc: 0,
        has_begin_try: false,
        can_stack_bind: false,
    };
    let bytes = bincode::serialize(&fu).expect("serialize FunctionUnit");
    let back: FunctionUnit = bincode::deserialize(&bytes).expect("deserialize FunctionUnit");
    assert_eq!(back, fu);

    let prog = CompiledProgram {
        functions: vec![fu],
        globals: Vec::new(),
        strings: Vec::new(),
        host_fn_names: Vec::new(),
        entry_point: SmolStr::new_static("main"),
        mir_functions: indexmap::IndexMap::new(),
        mir_module: None,
        opaque_witnesses: indexmap::IndexMap::new(),
        debug_source: None,
    };
    let bytes = bincode::serialize(&prog).expect("serialize CompiledProgram");
    let back: CompiledProgram = bincode::deserialize(&bytes).expect("deserialize CompiledProgram");
    assert_eq!(back.functions, prog.functions);
    assert_eq!(back.entry_point, prog.entry_point);
    assert!(back.opaque_witnesses.is_empty());
}

#[test]
fn s3_03_cache_roundtrip_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-03-cache-roundtrip.md");
    assert!(
        adr.contains("S3-I53"),
        "cache-roundtrip ADR must name S3-I53"
    );
    assert!(
        adr.contains("skip_serializing_if") || adr.contains("bincode"),
        "ADR must name serde/bincode hinge"
    );
    assert!(
        adr.contains("S3-I54") && (adr.contains("prelude-embed") || adr.contains("embed")),
        "ADR must cite landed S3-I54 prelude embed"
    );
    assert!(
        adr.contains("S3-I55") && (adr.contains("stage3-s3-03-claim") || adr.contains("claim")),
        "ADR must cite landed S3-I55 claim"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-3"),
        "ADR must Non-claim Stage-3 exit"
    );

    let census = repo_file("docs/census/stage3-stdlib-cache-census.md");
    assert!(
        census.contains("Current:** S3-I55") || census.contains("Current: S3-I55"),
        "census Current must be S3-I55"
    );
    assert!(
        census.contains("cache_roundtrip_helper"),
        "census must publish cache_roundtrip_helper"
    );
    assert!(
        census.contains("S3-I55") && census.contains("S3-04"),
        "census ladder must name S3-I55 claim and S3-04 next"
    );

    let codegen = repo_file("crates/syx-codegen/src/lib.rs");
    assert!(
        codegen.contains("S3-I53") && !codegen.contains("skip_serializing_if = \"Vec::is_empty\""),
        "FunctionUnit must drop Vec skip_serializing_if"
    );
    assert!(
        !codegen.contains("skip_serializing_if = \"SmolStr::is_empty\""),
        "FunctionUnit must drop SmolStr skip_serializing_if"
    );
    assert!(
        !codegen.contains("skip_serializing_if = \"indexmap::IndexMap::is_empty\""),
        "CompiledProgram must drop opaque_witnesses skip_serializing_if"
    );

    let prior = repo_file("docs/adr/stage3-s3-03-warm-residual.md");
    assert!(
        prior.contains("S3-I53") || prior.contains("stage3-s3-03-cache-roundtrip"),
        "warm-residual ADR must cite landed I53"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s3_03 = plan02
        .lines()
        .find(|l| l.contains("| S3-03 |"))
        .expect("S3-03 row");
    assert!(
        s3_03.contains("| [x] |") || s3_03.contains("| `[x]` |"),
        "S3-03 must be [x]; got: {s3_03}"
    );
    assert!(
        plan02.contains("S3-I53") || plan02.contains("cache-roundtrip"),
        "plan/02 must cite S3-I53"
    );
}

#[test]
fn s3_03_cache_roundtrip_census_script_emits_helper() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_03_stdlib_cache_census.sh")
        .current_dir(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .output()
        .expect("run stdlib-cache census");
    assert!(
        output.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("cache_roundtrip_helper="),
        "script must emit cache_roundtrip_helper, got:\n{stdout}"
    );
    let n = stdout
        .lines()
        .find(|l| l.starts_with("cache_roundtrip_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        n >= 1,
        "expected cache_roundtrip_helper>=1, got {n}\n{stdout}"
    );
}
