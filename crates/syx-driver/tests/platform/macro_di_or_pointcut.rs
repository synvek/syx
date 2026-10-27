//! Macro/DI pointcut `||` + Task pool core/max/keepAlive.

use std::path::PathBuf;
use std::sync::Mutex;

use syx_ast::Expr;
use syx_diagnostics::Span;
use syx_driver::{compile_and_run_embedded, run_file_printed, DriverOptions};
use syx_macro::{enumerate_decl_param_tokens, MacroTokenKind};

static P136_POOL_LOCK: Mutex<()> = Mutex::new(());

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
fn macro_di_or_pointcut_hits_within_or_args() {
    let printed = run_fixture("tests/fixtures/macros/or_pointcut.sy");
    assert!(
        printed.iter().any(|l| l.contains("__before__:handleStr")),
        "within side: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("or-within-ok")),
        "handleStr: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("__before__:addOne")),
        "within+args: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.trim() == "4"),
        "addOne(3) => 4: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("__before__:handleInt")),
        "args side on OtherProbe: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("or-args-ok")),
        "handleInt: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("no-advice")),
        "plain without advice: {printed:?}"
    );
    assert!(
        !printed.iter().any(|l| l.contains("__before__:plain")),
        "plain should not match or-compose: {printed:?}"
    );
}

#[test]
fn macro_di_or_token_stream_enumeration_observable() {
    let expr = Expr::Binary {
        span: Span::new(0, 0),
        left: Box::new(Expr::Ident {
            span: Span::new(0, 0),
            name: smol_str::SmolStr::new("a"),
        }),
        op: syx_ast::BinOp::Add,
        right: Box::new(Expr::Group {
            span: Span::new(0, 0),
            expr: Box::new(Expr::Ident {
                span: Span::new(0, 0),
                name: smol_str::SmolStr::new("b"),
            }),
        }),
    };
    let tokens = enumerate_decl_param_tokens(&expr);
    assert_eq!(
        tokens,
        vec![
            MacroTokenKind::Ident(smol_str::SmolStr::new("a")),
            MacroTokenKind::Punct('+'),
            MacroTokenKind::Group(vec![MacroTokenKind::Ident(smol_str::SmolStr::new("b"))]),
        ]
    );
}

#[test]
fn macro_di_or_pool_core_max_keepalive_roundtrip() {
    let _g = P136_POOL_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let printed = run(
        r#"
func main() {
    print(__syx_task_set_core_pool_size(2))
    print(__syx_task_core_pool_size())
    print(__syx_task_set_maximum_pool_size(4))
    print(__syx_task_maximum_pool_size())
    print(__syx_task_set_keep_alive_ms(100))
    print(__syx_task_keep_alive_ms())
    print(__syx_task_set_reject_policy("callerRuns"))
    print(__syx_task_reject_policy())
}
"#,
    )
    .expect("pool size roundtrip");
    assert_eq!(printed[0].trim(), "2");
    assert_eq!(printed[1].trim(), "2");
    assert_eq!(printed[2].trim(), "4");
    assert_eq!(printed[3].trim(), "4");
    assert_eq!(printed[4].trim(), "100");
    assert_eq!(printed[5].trim(), "100");
    assert_eq!(printed[6].trim(), "callerRuns");
    assert_eq!(printed[7].trim(), "callerRuns");
}

#[test]
fn macro_di_or_pool_probe_core_max_keepalive_observable() {
    let _g = P136_POOL_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    syx_async::probe_core_max_keepalive_mvp()
        .expect("core/max/keepAlive probe should succeed");
}

#[test]
fn macro_di_or_track_e_fingerprint_strings() {
    let service_rs = include_str!("../../../syx-macro/src/service.rs");
    let user_rs = include_str!("../../../syx-macro/src/user.rs");
    let runtime_rs = include_str!("../../../syx-async/src/runtime.rs");
    let docs = include_str!("../../../../books/language-guide/19-macros.md");
    let lib = include_str!("../../src/lib.rs");
    let concurrency = include_str!("../../../../packages/core/concurrency.sy");
    assert!(service_rs.contains("macro-service-v17:pointcut-or-compose-mvp"));
    assert!(user_rs.contains("macro-token-v7:token-stream-mvp"));
    assert!(runtime_rs.contains("runtime-concurrency-v4:core-max-keepalive-mvp"));
    assert!(docs.contains("macro-docs-v20:m28-service-v19-honesty"));
    assert!(lib.contains("macro-service-v17:pointcut-or-compose-mvp"));
    assert!(lib.contains("macro-token-v7:token-stream-mvp"));
    assert!(lib.contains("runtime-concurrency-v4:core-max-keepalive-mvp"));
    assert!(lib.contains("macro-docs-v20:m28-service-v19-honesty"));
    assert!(
        concurrency.contains("setCorePoolSize")
            && concurrency.contains("setMaximumPoolSize")
            && concurrency.contains("setKeepAliveMs")
    );
}
