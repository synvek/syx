//! Macro/DI pointcut `!within` + DiscardOldest reject policy.

use std::path::PathBuf;
use std::sync::Mutex;

use syx_ast::{Decl, Expr};
use syx_diagnostics::Span;
use syx_driver::{compile_and_run_embedded, run_file_printed, DriverOptions};
use syx_macro::{
    enumerate_decl_param_tokens, expand_decl_param_tokens, DeclExpandMode, MacroTokenKind,
};

static P137_POOL_LOCK: Mutex<()> = Mutex::new(());

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn run_fixture(rel: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture(rel);
    let options = DriverOptions::default();
    let (code, printed) = run_file_printed(&path, &options).expect("run fixture");
    assert_eq!(code, 0, "{printed:?}");
    printed
}

fn run(src: &str) -> Result<Vec<String>, String> {
    syx_host_plugin::register_echo_plugin_in_process();
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => Ok(r.printed),
        Err(e) => Err(format!("{e}")),
    }
}

#[test]
fn macro_di_not_within_pointcut_observable() {
    let printed = run_fixture("tests/fixtures/macros/not_within_pointcut.sy");
    assert!(
        printed.iter().any(|l| l.contains("__before__:handle")),
        "allowed type should match !within: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("not-within-ok")),
        "handle body: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("excluded-plain")),
        "excluded type body: {printed:?}"
    );
    assert!(
        !printed.iter().any(|l| l.contains("__before__:excluded")),
        "excluded type must not match !within(ExcludedProbe): {printed:?}"
    );
}

#[test]
fn macro_di_not_decl_expand_observable() {
    let tokens = vec![MacroTokenKind::Ident(smol_str::SmolStr::new("run"))];
    let decl = expand_decl_param_tokens(&tokens, DeclExpandMode::WrapPrefix("wrap_".into()))
        .expect("decl expand");
    assert!(matches!(
        decl,
        Decl::Function {
            name,
            ..
        } if name.as_str() == "wrap_run"
    ));
    let expr = Expr::Ident {
        span: Span::new(0, 0),
        name: smol_str::SmolStr::new("run"),
    };
    let enumerated = enumerate_decl_param_tokens(&expr);
    assert_eq!(
        enumerated,
        vec![MacroTokenKind::Ident(smol_str::SmolStr::new("run"))]
    );
}

#[test]
fn macro_di_not_discard_oldest_policy_roundtrip() {
    let _g = P137_POOL_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let printed = run(r#"
func main() {
    print(__syx_task_set_reject_policy("discardOldest"))
    print(__syx_task_reject_policy())
}
"#)
    .expect("discardOldest roundtrip");
    assert_eq!(printed[0].trim(), "discardOldest");
    assert_eq!(printed[1].trim(), "discardOldest");
}

#[test]
fn macro_di_not_pool_probe_discard_oldest_observable() {
    let _g = P137_POOL_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    syx_async::probe_bounded_queue_discard_oldest_mvp()
        .expect("discardOldest probe should succeed");
}
