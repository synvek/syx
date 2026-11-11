//! S2-I169 S2-05 syx-mir mir_opt extract contracts.
//! Nails: mir_opt module + root pub use + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-opt-extract.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_opt_extract_baseline() {
    let opt = repo_file("crates/syx-mir/src/mir_opt.rs");
    assert!(
        opt.contains("pub fn const_fold")
            && opt.contains("pub fn dead_code_elim")
            && opt.contains("pub fn optimize"),
        "mir_opt must hold const_fold/dead_code_elim/optimize"
    );
    assert!(
        opt.contains("mir_opt_v8_enabled") && opt.contains("loop_licm"),
        "mir_opt must hold opt-v* gates and LICM"
    );
    assert!(
        !opt.contains("struct LowerCtx") && !opt.contains("struct ModuleIndex"),
        "mir_opt must not pull lowering index / LowerCtx"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    assert!(
        lib.contains("mod mir_opt;"),
        "lib.rs must declare mod mir_opt"
    );
    assert!(
        lib.contains("pub use mir_opt::") && lib.contains("optimize"),
        "lib.rs must pub-use mir_opt surface"
    );
    assert!(
        !lib.lines().any(|l| l.starts_with("pub fn dead_code_elim")),
        "dead_code_elim definition must leave lib.rs"
    );
    assert!(
        !lib.lines().any(|l| l.starts_with("pub fn optimize(")),
        "optimize definition must leave lib.rs"
    );
    assert!(
        lib.contains("mod mir_lower_ctx;")
            || lib
                .lines()
                .any(|l| l.starts_with("struct LowerCtx")
                    || l.starts_with("pub(crate) struct LowerCtx")),
        "LowerCtx remains reachable (lib or mir_lower_ctx after I171)"
    );
    assert!(
        lib.contains("pub(crate) fn flatten_blocks") || lib.contains("flatten_blocks,"),
        "flatten_blocks must stay pub(crate) in lib.rs (def or re-export)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-opt-extract.md");
    assert!(adr.contains("S2-I169"), "opt ADR must name S2-I169");
    assert!(
        adr.contains("S2-I170") && adr.contains("mir_lower"),
        "ADR must lock next knife = S2-I170 mir_lower"
    );
    assert!(
        adr.contains("≠") && adr.contains("S2-05"),
        "ADR must Non-claim S2-05 [x]"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-2"),
        "ADR must Non-claim Stage-2 exit"
    );
}
