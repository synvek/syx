//! @builtin class `static let` + `static var { get }` load/codegen nails.
use std::fs;

use syx_diagnostics::SourceFile;
use syx_driver::{
    compile_and_run_embedded, run_native_mir_file_printed, DriverOptions,
};
use syx_frontend::{expand_macros, FrontendOptions, FrontendPipeline, StopAfter};
use syx_parser::Parser;
use syx_semantic::SemanticAnalyzer;

fn run_embed(src: &str) -> Result<Vec<String>, String> {
    syx_host_plugin::register_echo_plugin_in_process();
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => Ok(r.printed),
        Err(e) => Err(format!(
            "code={:?} msg={} source={:?}",
            e.code, e.message, e.source
        )),
    }
}

fn write_src(label: &str, src: &str) -> std::path::PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let dir = std::env::temp_dir().join(format!(
        "syx_builtin_static_{}_{}_{}",
        label,
        std::process::id(),
        SEQ.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let path = dir.join("main.sy");
    fs::write(&path, src).expect("write main");
    path
}

fn run_native(label: &str, src: &str) -> Result<Vec<String>, String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src(label, src);
    match run_native_mir_file_printed(&path, &DriverOptions::default()) {
        Ok((code, lines)) if code == 0 => Ok(lines),
        Ok((code, lines)) => Err(format!("exit {code}: {lines:?}")),
        Err(e) => Err(format!(
            "code={:?} msg={}",
            e.code, e.message
        )),
    }
}

const INT8_WITH_STATIC_LET: &str = r#"
@builtin("numeric.i8")
public class Int8: Equatable, Hashable, CustomStringConvertible, Comparable {
    public static let min: Int8 = -128i8
    public static let max: Int8 = 127i8
    public static func abs(x: Int8) -> Int8 {
        return __syx_i8_abs(x)
    }
    public static func bitWidth() -> Int {
        return 8
    }
}

extension Int8 {
    func hash(into hasher: Hasher) {
        __syx_hasher_combine(hasher, self)
    }
    func description() -> String {
        return __syx_value_description(self)
    }
    func debugDescription() -> String {
        return __syx_value_debug_description(self)
    }
    func compare(other: Int8) -> Int {
        return __syx_i8_compare(self, other)
    }
}
"#;

#[test]
fn ordinary_class_static_let_works() {
    let src = r#"
class Config {
    public static let min: Int8 = -128i8
    public static let max: Int8 = 127i8
}
func main() {
    print(Config.min)
    print(Config.max)
}
"#;
    let out = run_embed(src).expect("ordinary static let");
    assert!(out.iter().any(|l| l.trim() == "-128"), "{out:?}");
    assert!(out.iter().any(|l| l.trim() == "127"), "{out:?}");
}

#[test]
fn ordinary_class_static_computed_get_works() {
    let src = r#"
class Config {
    public static var max: Int8 {
        get { return 127i8 }
    }
}
func main() {
    print(Config.max)
}
"#;
    match run_embed(src) {
        Ok(out) => assert!(out.iter().any(|l| l.trim() == "127"), "{out:?}"),
        Err(e) => panic!("static computed get on ordinary class: {e}"),
    }
}

#[test]
fn builtin_shaped_module_static_let_analyzes() {
    let source = SourceFile::new("<diag>/Int8.sy", INT8_WITH_STATIC_LET);
    let parsed = Parser::parse(source).expect("parse");
    let expanded = expand_macros(parsed).expect("expand");
    let getters: Vec<&str> = expanded
        .decls
        .iter()
        .filter_map(|d| match d {
            syx_ast::Decl::Class { members, .. } => Some(members),
            _ => None,
        })
        .flatten()
        .filter_map(|d| match d {
            syx_ast::Decl::Function { name, attrs, .. }
                if attrs.iter().any(|a| a.name.as_str() == "static") =>
            {
                Some(name.as_str())
            }
            _ => None,
        })
        .collect();
    assert!(
        getters.iter().any(|n| *n == "__get_min"),
        "expected __get_min; got {getters:?}"
    );
    assert!(
        getters.iter().any(|n| *n == "__get_max"),
        "expected __get_max; got {getters:?}"
    );
    match SemanticAnalyzer::analyze_embedded_stdlib_with_imports(&expanded, &[]) {
        Ok(_) => {}
        Err(e) => panic!(
            "embedded semantic failed: code={:?} msg={}",
            e.code, e.message
        ),
    }
}

#[test]
fn builtin_shaped_module_frontend_prepared_ok() {
    let source = SourceFile::new("<diag>/Int8.sy", INT8_WITH_STATIC_LET);
    let prep = FrontendOptions {
        stop_after: StopAfter::Prepared,
        desugar_top_level: false,
        source: Some(source.clone()),
        skip_prelude_name_shadow: true,
        ..FrontendOptions::default()
    };
    match FrontendPipeline::run(source, &prep) {
        Ok(_) => {}
        Err(e) => panic!(
            "frontend prepared failed: code={:?} msg={}",
            e.code, e.message
        ),
    }
}

#[test]
fn builtin_int8_min_max_property_native() {
    let src = r#"
func main() {
    print(Int8.min)
    print(Int8.max)
}
"#;
    let out = run_native("int8_bounds", src).expect("Int8.min/max");
    assert!(out.iter().any(|l| l.trim() == "-128"), "{out:?}");
    assert!(out.iter().any(|l| l.trim() == "127"), "{out:?}");
}
