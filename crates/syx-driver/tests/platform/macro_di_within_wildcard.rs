//! Macro/DI within wildcard + Actor shared pool.

use std::path::PathBuf;
use std::sync::Mutex;

use syx_ast::{Decl, Expr};
use syx_diagnostics::Span;
use syx_driver::{compile_and_run_embedded, run_file_printed, DriverOptions};
use syx_macro::{
    decl_base_name_from_tokens, expand_decl_param_tokens, enumerate_decl_param_tokens,
    DeclExpandMode, MacroTokenKind,
};

static P138_POOL_LOCK: Mutex<()> = Mutex::new(());

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
fn macro_di_wildcard_within_wildcard_pointcut_observable() {
    let printed = run_fixture("tests/fixtures/macros/within_wildcard_pointcut.sy");
    assert!(
        printed.iter().any(|l| l.contains("__before__:handle")),
        "OrderService should match within(*Service): {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("within-wildcard-ok")),
        "handle body: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("repo-plain")),
        "OrderRepo body: {printed:?}"
    );
    assert!(
        !printed.iter().any(|l| l.contains("__before__:repo")),
        "OrderRepo must not match within(*Service): {printed:?}"
    );
}

#[test]
fn macro_di_wildcard_token_rewrite_observable() {
    let tokens = vec![
        MacroTokenKind::Ident(smol_str::SmolStr::new("my")),
        MacroTokenKind::Punct('_'),
        MacroTokenKind::Ident(smol_str::SmolStr::new("run")),
    ];
    assert_eq!(
        decl_base_name_from_tokens(&tokens).map(|s| s.to_string()),
        Some("my_run".into())
    );
    let decl = expand_decl_param_tokens(&tokens, DeclExpandMode::WrapPrefix("wrap_".into()))
        .expect("token rewrite");
    assert!(matches!(
        decl,
        Decl::Function {
            name,
            ..
        } if name.as_str() == "wrap_my_run"
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
fn macro_di_wildcard_actor_shared_pool_roundtrip() {
    let _g = P138_POOL_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let printed = run(
        r#"
func main() {
    print(__syx_actor_set_shared_pool_enabled(true))
    print(__syx_actor_shared_pool_enabled())
    print(__syx_actor_set_shared_pool_size(3))
    print(__syx_actor_shared_pool_size())
}
"#,
    )
    .expect("shared pool roundtrip");
    assert_eq!(printed[0].trim(), "true");
    assert_eq!(printed[1].trim(), "true");
    assert_eq!(printed[2].trim(), "3");
    assert_eq!(printed[3].trim(), "3");
}

#[test]
fn macro_di_wildcard_pool_probe_actor_shared_pool_observable() {
    let _g = P138_POOL_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    syx_async::probe_actor_shared_pool_mvp().expect("actor shared pool probe should succeed");
}

#[test]
fn macro_di_wildcard_track_e_fingerprint_strings() {
    let service_rs = include_str!("../../../syx-macro/src/service.rs");
    let user_rs = include_str!("../../../syx-macro/src/user.rs");
    let runtime_rs = include_str!("../../../syx-async/src/runtime.rs");
    let docs = include_str!("../../../../books/language-guide/19-macros.md");
    let lib = include_str!("../../src/lib.rs");
    let concurrency = include_str!("../../../../packages/core/concurrency.sy");
    assert!(service_rs.contains("macro-service-v19:pointcut-within-wildcard-mvp"));
    assert!(user_rs.contains("macro-token-v9:token-rewrite-mvp"));
    assert!(runtime_rs.contains("runtime-concurrency-v6:actor-shared-pool-mvp"));
    assert!(docs.contains("macro-docs-v20:m28-service-v19-honesty"));
    assert!(lib.contains("macro-service-v19:pointcut-within-wildcard-mvp"));
    assert!(lib.contains("macro-token-v9:token-rewrite-mvp"));
    assert!(lib.contains("runtime-concurrency-v6:actor-shared-pool-mvp"));
    assert!(lib.contains("macro-docs-v20:m28-service-v19-honesty"));
    assert!(concurrency.contains("setActorSharedPoolEnabled"));
}
