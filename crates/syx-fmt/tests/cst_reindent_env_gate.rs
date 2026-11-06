//! S2-I150: CST reindent GA for format_source_cst (env gate superseded).
//! format_dry_run_cst stays concat; format_file migration still deferred.

use std::sync::Mutex;
use syx_diagnostics::SourceFile;
use syx_fmt::{format_dry_run_cst, format_source_auto, format_source_cst};

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn with_cleared_fmt_env<R>(f: impl FnOnce() -> R) -> R {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    std::env::remove_var("SYX_FMT_CST_REINDENT");
    std::env::remove_var("SYX_FMT_AST");
    let out = f();
    std::env::remove_var("SYX_FMT_CST_REINDENT");
    std::env::remove_var("SYX_FMT_AST");
    out
}

#[test]
fn format_source_cst_ga_defaults_to_reindent() {
    with_cleared_fmt_env(|| {
        let src = "func main() {\nprint(1)\n}\n";
        let file = SourceFile::new("t.sy", src);
        let out = format_source_cst(&file).expect("cst ga");
        assert_eq!(out, "func main() {\n    print(1)\n}\n");
        let root = syx_cst::parse_cst_subset(&file).expect("parse");
        assert_eq!(out, syx_cst::format_cst_reindent(&root));
    });
}

#[test]
fn format_source_cst_ga_idempotent_on_four_space() {
    with_cleared_fmt_env(|| {
        let src = "func main() {\n    print(1)\n}\n";
        let file = SourceFile::new("t.sy", src);
        let out = format_source_cst(&file).expect("cst");
        assert_eq!(out, src);
    });
}

#[test]
fn format_dry_run_cst_stays_concat_under_ga() {
    with_cleared_fmt_env(|| {
        let src = "func main() {\nprint(1)\n}\n";
        let file = SourceFile::new("t.sy", src);
        let report = format_dry_run_cst(&file).expect("dry_run_cst");
        assert_eq!(report.formatted, src);
        assert!(!report.would_change);
    });
}

#[test]
fn format_source_auto_ast_env_still_overrides_ga() {
    with_cleared_fmt_env(|| {
        std::env::set_var("SYX_FMT_AST", "1");
        let src = "func main() {\nprint(1)\n}\n";
        let file = SourceFile::new("t.sy", src);
        let out = format_source_auto(&file).expect("auto ast");
        let ast = syx_fmt::format_source(&file).expect("ast");
        assert_eq!(out, ast);
        assert_ne!(out, "func main() {\n    print(1)\n}\n");
    });
}
