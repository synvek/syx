//! S3-I74 S3-05 Struct/Extension type-member contracts.
//! Hinge landed · ≠ S3-05 [x] · ≠ Stage-3 exit.
//! See docs/adr/stage3-s3-05-struct-extension-type-member.md · docs/stage3-lint-ast-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_05_struct_extension_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-05-struct-extension-type-member.md");
    assert!(adr.contains("S3-I74"), "construct ADR must name S3-I74");
    assert!(
        adr.contains("Struct") && adr.contains("Extension"),
        "ADR must name Struct/Extension hinge"
    );
    assert!(
        adr.contains("S3-I75") && (adr.contains("claim-prep") || adr.contains("claim")),
        "ADR must cite landed S3-I75 claim-prep"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-3"),
        "ADR must Non-claim Stage-3 exit"
    );
    assert!(
        adr.contains("≠ S3-05 `[x]`") || adr.contains("≠ S3-05 [x]"),
        "ADR must Non-claim S3-05 [x]"
    );

    let census = repo_file("docs/stage3-lint-ast-census.md");
    assert!(
        census.contains("Current:** S3-I76") || census.contains("Current: S3-I76"),
        "census Current must advance to S3-I76"
    );
    assert!(
        census.contains("struct_type_member_helper")
            && census.contains("struct_book_point_l007_count"),
        "census must publish I74 KPIs"
    );
    assert!(
        census.contains("S3-I74") && census.contains("S3-I76") && census.contains("S3-I77"),
        "census must name I74 / I76 / I77"
    );
    assert!(census.contains("4/7"), "census must keep Stage-3 4/7");

    let lint = repo_file("crates/syx-lint/src/lib.rs");
    assert!(
        lint.contains("Decl::Struct") && lint.contains("Decl::Extension") && lint.contains("S3-I74"),
        "syx-lint must recurse Struct/Extension with S3-I74"
    );
    assert!(
        lint.contains("in_type_member"),
        "syx-lint must keep in_type_member guard"
    );

    let prior = repo_file("docs/adr/stage3-s3-05-l007-l012-false-positive.md");
    assert!(
        prior.contains("S3-I74")
            || prior.contains("stage3-s3-05-struct-extension-type-member"),
        "I73 ADR must cite landed I74"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s305 = plan02
        .lines()
        .find(|l| l.contains("| S3-05 |"))
        .expect("S3-05 row");
    assert!(
        s305.contains("[~]"),
        "S3-05 must stay [~] after I74; got: {s305}"
    );
    assert!(
        plan02.contains("S3-I74")
            && plan02.contains("S3-I75")
            && plan02.contains("S3-I76")
            && plan02.contains("S3-I77"),
        "plan/02 must cite I74…I77"
    );
    assert!(plan02.contains("4/7"), "plan/02 must keep Stage-3 4/7");

    assert!(
        repo_file("examples/struct_demo.sy").contains("struct Point"),
        "examples/struct_demo.sy must exist"
    );
}

#[test]
fn s3_05_struct_extension_census_script_cleared() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let script = root.join("scripts/s3_05_lint_ast_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
        .current_dir(&root)
        .output()
        .expect("run s3_05_lint_ast_census.sh");
    assert!(
        out.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("current=S3-I76") && stdout.contains("next=S3-I77"),
        "script ladder must be I76→I77:\n{stdout}"
    );
    assert!(
        stdout.contains("struct_type_member_helper="),
        "script must emit I74 hinge:\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(999)
    };
    assert_eq!(
        parse("class_demo_l007_count="),
        0,
        "class_demo L007 must stay 0\n{stdout}"
    );
    assert_eq!(
        parse("class_demo_l012_count="),
        0,
        "class_demo L012 must stay 0\n{stdout}"
    );
    assert_eq!(
        parse("struct_book_point_l007_count="),
        0,
        "struct_book_point L007 must be 0\n{stdout}"
    );
    assert!(
        parse("struct_type_member_helper=") >= 1,
        "struct_type_member_helper must be >=1\n{stdout}"
    );
}
