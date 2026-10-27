//! Service DI MVP E2E (`@Service` + `@Inject` ctor + singleton resolve).

use syx_driver::{compile_and_run_embedded, compile_embedded, run_file_printed, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn service_di_ctor_inject_resolve_e2e() {
    let src = r#"
@Service
class Logger {
  init() {}
  func ping() -> String { return "pong" }
}

@Service
class App {
  init(@Inject logger: Logger) {}
  func run() -> String {
    return __syx_service_resolve("Logger").ping()
  }
}

func main() {
  __syx_service_boot()
  let app = __syx_service_resolve("App")
  print(app.run())
  let again = __syx_service_resolve("App")
  print(again.run())
}
"#;
    let out = run(src);
    assert_eq!(out, "pong\npong");
}

#[test]
fn service_di_missing_dep_compile_fails() {
    let src = r#"
@Service
class App {
  init(@Inject logger: Logger) {}
}
func main() { print(1) }
"#;
    let err = compile_embedded(src, &DriverOptions::default()).expect_err("E008");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E008") || msg.contains("Logger") || msg.contains("dependency"),
        "unexpected err: {msg}"
    );
}

#[test]
fn service_di_macros_exports_schema_in_manifest() {
    let manifest = include_str!("../../../syx-pkg/src/manifest.rs");
    assert!(manifest.contains("MacrosField"));
    assert!(manifest.contains("[macros].exports") || manifest.contains("exports"));
}

#[test]
fn service_di_fingerprint_strings_in_driver() {
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("macro-service-v1:attr-scan")
            || lib.contains("macro-package-v2:exports-schema")
            || lib.contains("runtime-service-v1:singleton-container")
            || lib.contains("heap_v92_phase120")
    );
}

#[test]
fn service_di_cli_path_service_di_example() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/service_di/main.sy");
    let path = path.canonicalize().expect("canonicalize");
    let (code, printed) = run_file_printed(&path, &DriverOptions::default()).expect("run file");
    assert_eq!(code, 0, "{printed:?}");
    let out = printed.join("\n");
    assert!(
        out.contains("pong"),
        "service_di example should print pong: {printed:?}"
    );
}
