//! ADR-142 / M1 W6: CST trivia round-trip vs AST comment loss.

use syx_diagnostics::SourceFile;
use syx_fmt::{format_source, format_source_cst};

const FIXTURE: &str = "// ahead\nfunc main() {\n  // inside\n}\n/* trail */\n";

#[test]
fn cst_trivia_cst_trivia_roundtrip_byte_identical() {
    let file = SourceFile::new("fixture.sy", FIXTURE);
    let out = format_source_cst(&file).expect("cst format");
    assert_eq!(out, FIXTURE, "CST format must be byte-identical for normalized fixture");
    assert!(out.contains("// ahead"));
    assert!(out.contains("// inside"));
    assert!(out.contains("/* trail */"));
}

#[test]
fn cst_trivia_ast_format_drops_comments() {
    let file = SourceFile::new("fixture.sy", FIXTURE);
    let out = format_source(&file).expect("ast format");
    assert!(
        !out.contains("// ahead") && !out.contains("// inside") && !out.contains("/* trail */"),
        "AST fmt should drop comments (migration motive); got {out:?}"
    );
    assert!(out.contains("func main"));
}
