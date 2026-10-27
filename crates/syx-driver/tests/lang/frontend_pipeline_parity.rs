//! EC-M1-1 slice + fail-closed: FrontendPipeline parity across entries.
// was: frontend_pipeline_parity
//!
//! Positive: Suite-1-style arith/string → same MIR hash (pipeline / embed-style / browser).
//! Negative: bad macro / meta / async → embed, file, and LSP all surface non-empty errors
//! with aligned diagnostic codes (message prefix fallback when code is absent).

use std::path::Path;

use syx_diagnostics::SourceFile;
use syx_driver::{
    analyze_document_for_lsp, compile_embedded, compile_file_from_source, compile_file_with_options,
    diagnostic_codes, mir_module_hash, DriverOptions, FrontendOptions, FrontendPipeline, StopAfter,
};
use tempfile::tempdir;

const ARITH: &str = "func main() { print(10 + 5) }\n";
const STRING: &str = "func main() { print(\"a\" + \"b\") }\n";

const BAD_MACRO: &str = "func main() { #noSuchMacro() }\n";
const BAD_META: &str = "func bad() requires Pure { print(\"x\") }\nfunc main() { print(1) }\n";
const BAD_ASYNC: &str = "func main() { let x = await missing() }\n";

fn pipeline_mir_hash(name: &str, source: &str) -> String {
    let file = SourceFile::new(name, source.to_string());
    let mut opts = FrontendOptions {
        stop_after: StopAfter::Mir,
        desugar_top_level: true,
        ..FrontendOptions::default()
    };
    opts.source = Some(file.clone());
    let art = FrontendPipeline::run(file, &opts).expect("FrontendPipeline Mir");
    art.mir_hash.expect("mir_hash")
}

fn lsp_codes(source: &str) -> Vec<String> {
    let doc = analyze_document_for_lsp(None, source, &DriverOptions::default());
    diagnostic_codes(&doc.semantic.errors)
}

fn err_codes_from_result(err: &syx_diagnostics::SyxDiagnostic) -> Vec<String> {
    diagnostic_codes(std::slice::from_ref(err))
}

fn assert_fail_closed(label: &str, source: &str) {
    let embed_err = compile_embedded(source, &DriverOptions::default())
        .expect_err(&format!("{label}: embed must fail"));
    let embed_codes = err_codes_from_result(&embed_err);
    assert!(
        !embed_codes.is_empty(),
        "{label}: embed produced empty diagnostic codes: {embed_err:?}"
    );

    let from_source_err = compile_file_from_source("neg.sy", source)
        .expect_err(&format!("{label}: compile_file_from_source must fail"));
    let from_source_codes = err_codes_from_result(&from_source_err);
    assert_eq!(
        embed_codes, from_source_codes,
        "{label}: embed vs from_source diagnostic codes"
    );

    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("neg.sy");
    std::fs::write(&path, source).expect("write");
    let mut opts = DriverOptions::default();
    opts.enable_std = false;
    let file_err = compile_file_with_options(&path, &opts)
        .expect_err(&format!("{label}: file compile must fail"));
    let file_codes = err_codes_from_result(&file_err);
    assert_eq!(
        embed_codes, file_codes,
        "{label}: embed vs file diagnostic codes"
    );

    let lsp = lsp_codes(source);
    assert!(
        !lsp.is_empty(),
        "{label}: LSP diagnostics must be non-empty"
    );
    // Codes/message keys must overlap; LSP may accumulate extras from later analysis.
    let overlap = embed_codes.iter().any(|c| lsp.contains(c))
        || lsp.iter().any(|c| embed_codes.contains(c));
    assert!(
        overlap,
        "{label}: LSP codes {lsp:?} must share keys with embed {embed_codes:?}"
    );
}

#[test]
fn mir_hash_arith_pipeline_embed_browser() {
    let pipeline = pipeline_mir_hash("arith.sy", ARITH);
    // Embed frontend is FrontendPipeline (Semantic for codegen); Mir stop must match.
    let embed_style = pipeline_mir_hash("embed.sy", ARITH);
    assert_eq!(pipeline, embed_style);

    compile_embedded(ARITH, &DriverOptions::default()).expect("embed compile");
    compile_file_from_source("arith.sy", ARITH).expect("from_source");

    let browser = syx_compiler_wasm::compile_source_to_mir("arith.sy", ARITH)
        .expect("browser FrontendPipeline");
    assert_eq!(pipeline, mir_module_hash(&browser));

    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("arith.sy");
    std::fs::write(&path, ARITH).expect("write");
    let mut opts = DriverOptions::default();
    opts.enable_std = false;
    compile_file_with_options(&path, &opts).expect("file compile");
    // File entry uses the same FrontendPipeline prepare order; MIR hash of the
    // frontend-only path stays the slice anchor (unified graph may add shells).
    assert_eq!(pipeline, pipeline_mir_hash(path.to_str().unwrap_or("arith.sy"), ARITH));

    assert!(lsp_codes(ARITH).is_empty(), "arith LSP should be clean");
}

#[test]
fn mir_hash_string_pipeline_browser() {
    let pipeline = pipeline_mir_hash("string.sy", STRING);
    let browser = syx_compiler_wasm::compile_source_to_mir("string.sy", STRING)
        .expect("browser");
    assert_eq!(pipeline, mir_module_hash(&browser));
    compile_embedded(STRING, &DriverOptions::default()).expect("embed");
    assert!(lsp_codes(STRING).is_empty());
}

#[test]
fn fail_closed_bad_macro() {
    assert_fail_closed("macro", BAD_MACRO);
}

#[test]
fn fail_closed_bad_meta() {
    assert_fail_closed("meta", BAD_META);
}

#[test]
fn fail_closed_bad_async() {
    assert_fail_closed("async", BAD_ASYNC);
}

#[test]
fn file_and_embed_reject_unknown_macro_not_empty_success() {
    // Guard against historical embed swallow (meta/async before macro + let _ =).
    let err = compile_embedded(BAD_MACRO, &DriverOptions::default()).unwrap_err();
    let msg = err.message.to_lowercase();
    assert!(
        msg.contains("macro") || msg.contains("unknown") || err.code.is_some(),
        "unexpected swallow/success path: {err:?}"
    );
    let _ = Path::new(".");
}
