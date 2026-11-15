//! Service DI / advice / bean / lazy / Codable-attached unit probes
//! (merged from phase120–129; duplicate request-scope / weak boot nails removed).

use smol_str::SmolStr;
use syx_ast::{AccessLevel, Attribute, Decl};

use crate::attached::expand_attached_macros;
use crate::service::{
    check_service_cycles, collect_beans, collect_services, expand_advice_interceptors,
    expand_around_interceptors, expand_service_macros, pick_binding, queue_services_from_module,
    reject_service_on_non_types, spec_to_meta_entry,
};

fn diag_span() -> syx_diagnostics::Span {
    syx_diagnostics::Span { start: 0, len: 1 }
}

#[test]
fn p120_service_e006_on_function() {
    let decls = vec![Decl::Function {
        span: diag_span(),
        access: AccessLevel::Public,
        attrs: vec![Attribute {
            name: SmolStr::new("Service"),
            args: vec![],
        }],
        name: SmolStr::new("bad"),
        type_params: vec![],
        params: vec![],
        return_type: None,
        throws_type: None,
        effect: None,
        where_clause: vec![],
        body: vec![],
        is_async: false,
        is_override: false,
        is_final: false,
    }];
    let err = reject_service_on_non_types(&decls).expect_err("E006");
    assert_eq!(err.code.as_deref(), Some("E006"));
}

#[test]
fn p120_service_scan_and_boot_codegen() {
    let src = r#"
@Service
class Logger {
  init() {}
}

@Service
class App {
  init(@Inject logger: Logger) {}
}

func main() {}
"#;
    let file = syx_diagnostics::SourceFile::new("t.sy", src);
    let mut module = syx_parser::Parser::parse(file).expect("parse");
    expand_service_macros(&mut module).expect("expand service");
    let services = collect_services(&module).expect("collect");
    assert_eq!(services.len(), 2);
    let app = services.iter().find(|s| s.type_name == "App").unwrap();
    assert_eq!(app.deps, vec![SmolStr::new("Logger")]);
    assert!(module.decls.iter().any(|d| matches!(
        d,
        Decl::Function { name, .. } if name.as_str() == "__syx_service_boot_impl"
            || name.as_str() == "__syx_service_boot"
    )));
}

#[test]
fn p120_service_cycle_e007() {
    let src = r#"
@Service
class A {
  init(@Inject b: B) {}
}
@Service
class B {
  init(@Inject a: A) {}
}
"#;
    let file = syx_diagnostics::SourceFile::new("t.sy", src);
    let mut module = syx_parser::Parser::parse(file).expect("parse");
    let err = expand_service_macros(&mut module).expect_err("cycle");
    assert_eq!(err.code.as_deref(), Some("E007"));
}

#[test]
fn p120_service_missing_dep_e008() {
    let src = r#"
@Service
class App {
  init(@Inject logger: Logger) {}
}
"#;
    let file = syx_diagnostics::SourceFile::new("t.sy", src);
    let mut module = syx_parser::Parser::parse(file).expect("parse");
    let err = expand_service_macros(&mut module).expect_err("missing");
    assert_eq!(err.code.as_deref(), Some("E008"));
}

#[test]
fn p120_expand_example_service_di_has_boot() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../examples/service_di/main.sy"
    );
    let src = std::fs::read_to_string(path).expect("read example");
    let file = syx_diagnostics::SourceFile::new("main.sy", src);
    let mut module = syx_parser::Parser::parse(file).expect("parse");
    let services = crate::service::collect_services(&module).expect("collect");
    assert!(
        services.len() >= 2,
        "expected App + loggers (+ optional lazy/cycle), got {services:?}"
    );
    module = crate::expand_module(module).expect("expand");
    assert!(
        module.decls.iter().any(|d| {
            matches!(
                d,
                Decl::Function { name, .. }
                    if name.as_str() == "__syx_service_boot"
                        || name.as_str() == "__syx_service_boot_impl"
            )
        }),
        "boot missing after expand"
    );
}

#[test]
fn p121_service_field_autowired_scan() {
    let src = r#"
@Service
class Logger {
  init() {}
}

@Service
class App {
  @Autowired
  var logger: Logger
  init() {}
}
"#;
    let file = syx_diagnostics::SourceFile::new("t.sy", src);
    let module = syx_parser::Parser::parse(file).expect("parse");
    let services = collect_services(&module).expect("collect");
    let app = services.iter().find(|s| s.type_name == "App").unwrap();
    assert_eq!(app.deps, vec![SmolStr::new("Logger")]);
    assert_eq!(app.field_autowires.len(), 1);
    assert_eq!(app.field_autowires[0].field, "logger");
    assert_eq!(app.field_autowires[0].type_name, "Logger");
}

#[test]
fn p121_service_postconstruct_scan() {
    let src = r#"
@Service
class App {
  init() {}
  @PostConstruct
  func onReady() {}
}
"#;
    let file = syx_diagnostics::SourceFile::new("t.sy", src);
    let module = syx_parser::Parser::parse(file).expect("parse");
    let services = collect_services(&module).expect("collect");
    let app = services.iter().find(|s| s.type_name == "App").unwrap();
    assert_eq!(app.post_construct.as_deref(), Some("onReady"));
}

#[test]
fn p121_service_di_append_import_visible() {
    syx_meta::service_di_clear();
    let src_a = r#"
@Service
class Logger {
  init() {}
}
"#;
    let src_b = r#"
@Service
class App {
  init(@Inject logger: Logger) {}
}
"#;
    let mut mod_a =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("a.sy", src_a)).expect("parse");
    expand_service_macros(&mut mod_a).expect("expand a");
    let mut mod_b =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("b.sy", src_b)).expect("parse");
    expand_service_macros(&mut mod_b).expect("expand b");
    let queued = syx_meta::service_di_take();
    assert_eq!(queued.len(), 2);
    assert!(queued.iter().any(|e| e.type_name == "Logger"));
    assert!(queued.iter().any(|e| e.type_name == "App"));
    syx_meta::service_di_clear();
}

#[test]
fn p121_boot_codegen_emits_service_put() {
    let src = r#"
@Service
class Logger { init() {} }
@Service
class App { init(@Inject logger: Logger) {} }
func main() { print(1) }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let expanded = crate::expand_module(module).expect("expand");
    let dump = format!("{expanded:?}");
    assert!(
        dump.contains("__syx_service_put"),
        "boot must construct+put: {dump}"
    );
    assert!(dump.contains("__syx_di_Logger"), "{dump}");
}

#[test]
fn p122_service_scope_prototype_scan() {
    let src = r#"
@Scope("prototype")
@Service
class Worker {
  init() {}
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    assert_eq!(services[0].scope, SmolStr::new("prototype"));
}

#[test]
fn p122_service_predestroy_scan() {
    let src = r#"
@Service
class App {
  init() {}
  @PreDestroy
  func shutdown() {}
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    assert_eq!(services[0].pre_destroy.as_deref(), Some("shutdown"));
}

#[test]
fn p122_service_di_multi_package_queue_append() {
    syx_meta::service_di_clear();
    let lib = r#"
@Service
class Logger { init() {} }
"#;
    let app = r#"
@Service
class App { init(@Inject logger: Logger) {} }
"#;
    syx_meta::service_di_set_scan_stem(Some("lib".to_string()));
    queue_services_from_module(
        &syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("lib.sy", lib)).expect("parse"),
    )
    .expect("queue lib");
    syx_meta::service_di_set_scan_stem(Some("main".to_string()));
    queue_services_from_module(
        &syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("main.sy", app))
            .expect("parse"),
    )
    .expect("queue app");
    syx_meta::service_di_set_scan_stem(None);
    let queued = syx_meta::service_di_peek_entries();
    assert_eq!(queued.len(), 2);
    assert!(queued
        .iter()
        .any(|e| e.type_name == "Logger" && e.module_stem.as_deref() == Some("lib")));
    assert!(queued
        .iter()
        .any(|e| e.type_name == "App" && e.module_stem.as_deref() == Some("main")));
    syx_meta::service_di_clear();
}

#[test]
fn p122_service_meta_entry_carries_scope_and_predestroy() {
    let src = r#"
@Scope("prototype")
@Service
class App {
  init() {}
  @PreDestroy
  func done() {}
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    let entry = spec_to_meta_entry(&services[0]);
    assert_eq!(entry.scope, "prototype");
    assert_eq!(entry.pre_destroy.as_deref(), Some("done"));
}

#[test]
fn p123_service_scope_request_accepted() {
    let src = r#"
@Scope("request")
@Service
class App { init() {} }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    assert_eq!(services[0].scope, SmolStr::new("request"));
}

#[test]
fn p123_service_meta_entry_request_scope() {
    let src = r#"
@Scope("request")
@Service
class App { init() {} }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    let entry = spec_to_meta_entry(&services[0]);
    assert_eq!(entry.scope, "request");
}

#[test]
fn p123_expand_service_macros_emits_resolve_sugar() {
    let src = r#"
@Service
class Logger { init() {} }
func main() { print(1) }
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot"), "{dump}");
    assert!(
        dump.contains("resolve"),
        "ceremony sugar resolve wrapper: {dump}"
    );
}

#[test]
fn p124_service_qualifier_primary_profile_parsed() {
    let src = r#"
@Primary
@Qualifier("fast")
@Profile("prod")
@Service
class FastLogger { init() {} }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    assert!(services[0].primary);
    assert_eq!(services[0].qualifier, Some(SmolStr::new("fast")));
    assert_eq!(services[0].profile, Some(SmolStr::new("prod")));
    let entry = spec_to_meta_entry(&services[0]);
    assert!(entry.primary);
    assert_eq!(entry.qualifier.as_deref(), Some("fast"));
    assert_eq!(entry.profile.as_deref(), Some("prod"));
}

#[test]
fn p124_inject_site_qualifier_parsed() {
    let src = r#"
@Service
class Logger { init() {} }
@Service
class App {
  init(@Inject @Qualifier("fast") logger: Logger) {}
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    let app = services.iter().find(|s| s.type_name == "App").unwrap();
    assert_eq!(app.init_inject[0].qualifier, Some(SmolStr::new("fast")));
}

#[test]
fn p124_pick_binding_qualifier_and_primary() {
    let src = r#"
class Logger { init() {} }
@Primary
@Service
class DefaultLogger: Logger { init() {} }
@Qualifier("fast")
@Service
class FastLogger: Logger { init() {} }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    let fast = pick_binding(&services, "Logger", Some("fast")).expect("fast");
    assert_eq!(fast.type_name, "FastLogger");
    let primary = pick_binding(&services, "Logger", None).expect("primary");
    assert_eq!(primary.type_name, "DefaultLogger");
}

#[test]
fn p124_pick_binding_ambiguity_errors() {
    let src = r#"
class Logger { init() {} }
@Service
class ALogger: Logger { init() {} }
@Service
class BLogger: Logger { init() {} }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    let err = pick_binding(&services, "Logger", None).expect_err("ambiguous");
    let msg = format!("{err:?}");
    assert!(msg.contains("E013") || msg.contains("ambiguous"), "{msg}");
}

#[test]
fn p124_profile_compile_time_filter() {
    std::env::set_var("SYX_DI_PROFILE", "prod");
    syx_meta::service_di_set_active_profile(Some("prod".into()));
    let src = r#"
@Profile("prod")
@Service
class ProdSvc { init() {} }
@Profile("dev")
@Service
class DevSvc { init() {} }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    assert_eq!(services.len(), 1);
    assert_eq!(services[0].type_name, "ProdSvc");
    syx_meta::service_di_set_active_profile(None);
    std::env::remove_var("SYX_DI_PROFILE");
}

#[test]
fn p124_expand_emits_boot_with_binding_rewrite() {
    let src = r#"
class Logger { init() {} }
@Primary
@Service
class DefaultLogger: Logger { init() {} }
@Service
class App {
  init(@Inject logger: Logger) {}
}
func main() { print(1) }
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot"), "{dump}");
    assert!(dump.contains("DefaultLogger"), "{dump}");
}

#[test]
fn p125_field_qualifier_metadata_roundtrip() {
    let src = r#"
@Service
class Logger { init() {} }
@Service
class App {
  @Autowired @Qualifier("fast")
  var logger: Logger
  init() {}
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    let app = services.iter().find(|s| s.type_name == "App").unwrap();
    assert_eq!(app.field_autowires.len(), 1);
    assert_eq!(app.field_autowires[0].qualifier.as_deref(), Some("fast"));
    let entry = spec_to_meta_entry(app);
    assert_eq!(entry.field_qualifiers.len(), 1);
    assert_eq!(entry.field_qualifiers[0].as_deref(), Some("fast"));
}

#[test]
fn p125_pick_binding_rewrites_to_concrete_on_expand() {
    let src = r#"
class Logger { init() {} func tag() -> String { return "?" } }
@Primary
@Service
class DefaultLogger: Logger { override func tag() -> String { return "default" } }
@Qualifier("fast")
@Service
class FastLogger: Logger { override func tag() -> String { return "fast" } }
@Service
class App {
  init(@Inject @Qualifier("fast") logger: Logger) {}
}
func main() { print(1) }
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_di_FastLogger"), "{dump}");
    assert!(!dump.contains("__syx_di_Logger"), "{dump}");
}

#[test]
fn p125_around_expands_delegate_body() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Around
  func greet(name: String) -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_around_interceptors(&mut module).expect("around");
    let dump = format!("{module:?}");
    assert!(dump.contains("__around__:"), "{dump}");
    assert!(dump.contains("print"), "{dump}");
    assert!(dump.contains("hi"), "{dump}");
}

#[test]
fn p125_around_rejects_non_service() {
    let src = r#"
class Greeter {
  @Around
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = expand_around_interceptors(&mut module).expect_err("E014");
    let msg = format!("{err:?}");
    assert!(msg.contains("E014") || msg.contains("@Around"), "{msg}");
}

#[test]
fn p125_protocol_binding_alias_pick() {
    let src = r#"
protocol Pingable { func ping() -> String { return "?" } }
@Primary
@Service
class DefaultPing: Pingable { func ping() -> String { return "d" } }
@Qualifier("fast")
@Service
class FastPing: Pingable { func ping() -> String { return "f" } }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    let fast = pick_binding(&services, "Pingable", Some("fast")).expect("fast");
    assert_eq!(fast.type_name, "FastPing");
    assert!(fast.binding_aliases.iter().any(|a| a == "Pingable"));
}

#[test]
fn p125_field_autowire_boot_contains_fast_logger_assign() {
    let src = include_str!("../../syx-driver/tests/fixtures/macros/field_qualifier.sy");
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_di_FastLogger"), "{dump}");
    assert!(dump.contains("logger"), "{dump}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
}

#[test]
fn p125_binding_alias_includes_superclass() {
    let src = r#"
class Logger { init() {} }
@Qualifier("fast")
@Service
class FastLogger: Logger { init() {} }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    let fast = services
        .iter()
        .find(|s| s.type_name == "FastLogger")
        .unwrap();
    assert!(fast.binding_aliases.iter().any(|a| a == "Logger"));
    let picked = pick_binding(&services, "Logger", Some("fast")).expect("pick");
    assert_eq!(picked.type_name, "FastLogger");
}

#[test]
fn p126_protocol_conformance_metadata_roundtrip() {
    let src = r#"
protocol Pingable { func ping() -> String { return "?" } }
@Qualifier("fast")
@Service
class FastPing: Pingable {
  init() {}
  func ping() -> String { return "fast" }
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    let fast = services.iter().find(|s| s.type_name == "FastPing").unwrap();
    assert!(fast.protocol_conformances.iter().any(|p| p == "Pingable"));
    let entry = spec_to_meta_entry(fast);
    assert_eq!(entry.protocol_conformances, vec!["Pingable"]);
}

#[test]
fn p126_advice_chain_expands_multiple_around() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Around
  @Around
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert_eq!(dump.matches("__around__:").count(), 2, "{dump}");
}

#[test]
fn p126_before_after_expands_markers() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Before
  @After
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__before__:"), "{dump}");
    assert!(dump.contains("__after__:"), "{dump}");
}

#[test]
fn p126_register_witness_codegen_in_boot() {
    let src = r#"
protocol Pingable { func ping() -> String { return "?" } }
@Primary
@Service
class DefaultPing: Pingable { init() {} func ping() -> String { return "d" } }
func main() { print(1) }
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_register_witness"), "{dump}");
    assert!(dump.contains("Pingable"), "{dump}");
}

#[test]
fn p126_full_fixture_expands_boot_and_witness() {
    let src = include_str!("../../syx-driver/tests/fixtures/macros/protocol_witness.sy");
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    assert!(
        services.iter().any(|s| s.type_name == "App"),
        "{services:?}"
    );
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("__syx_service_register_witness"), "{dump}");
}

#[test]
fn p127_advice_proceed_expands_delegate_vars() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Around
  @Around
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_proceed_0"), "{dump}");
    assert!(dump.contains("__syx_proceed_1"), "{dump}");
    assert_eq!(dump.matches("__around__:").count(), 2, "{dump}");
}

#[test]
fn p127_advice_proceed_order_before_around_after() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Around
  @Before
  @After
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    let before = dump.find("__before__:").expect("before");
    let around = dump.find("__around__:").expect("around");
    let proceed = dump.find("__syx_proceed_0").expect("proceed");
    let after = dump.find("__after__:").expect("after");
    assert!(before < around, "{dump}");
    assert!(around < proceed, "{dump}");
    assert!(proceed < after, "{dump}");
}

#[test]
fn p127_collect_beans_from_configuration() {
    let src = r#"
@Configuration
class AppConfig {
  init() {}
  @Bean
  func greeter() -> Greeter { return Greeter() }
}
class Greeter { init() {} }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let beans = collect_beans(&module).expect("beans");
    assert_eq!(beans.len(), 1);
    assert_eq!(beans[0].config_type, "AppConfig");
    assert_eq!(beans[0].method, "greeter");
    assert_eq!(beans[0].return_type, "Greeter");
}

#[test]
fn p127_bean_boot_codegen_emits_factory_put() {
    let src = include_str!("../../syx-driver/tests/fixtures/macros/configuration_bean.sy");
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("__syx_service_put"), "{dump}");
    assert!(dump.contains("AppConfig"), "{dump}");
}

#[test]
fn p127_bean_rejects_non_configuration() {
    let src = r#"
@Service
class Bad {
  init() {}
  @Bean
  func greeter() -> Greeter { return Greeter() }
}
class Greeter { init() {} }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = collect_beans(&module).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("@Bean") || msg.contains("E015"), "{msg}");
}

#[test]
fn p128_lazy_attr_collected_on_service() {
    let src = r#"
@Lazy
@Service
class LazyBox {
  init() {}
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    assert_eq!(services.len(), 1);
    assert!(services[0].lazy, "{services:?}");
}

#[test]
fn p128_lazy_cycle_allowed_at_expand() {
    let src = r#"
@Lazy
@Service
class A {
  init(@Inject b: B) {}
}
@Service
class B {
  init(@Inject a: A) {}
}
func main() { print(1) }
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("lazy cycle should expand");
    let dump = format!("{module:?}");
    assert!(
        dump.contains("__syx_service_lazy_proxy") || dump.contains("Lazy"),
        "{dump}"
    );
}

#[test]
fn p128_cycle_without_lazy_still_e007_unit() {
    let src = r#"
@Service
class A {
  init(@Inject b: B) {}
}
@Service
class B {
  init(@Inject a: A) {}
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    let err = check_service_cycles(&services).expect_err("E007");
    assert_eq!(err.code.as_deref(), Some("E007"));
}

#[test]
fn p129_codable_associated_enum_expands_encode_decode() {
    let src = r#"
@Codable
enum WireStatus {
  ok(code: Int)
  err(message: String)
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_attached_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("encode"), "{dump}");
    assert!(dump.contains("decode"), "{dump}");
    assert!(dump.contains("__tag") || dump.contains("ok"), "{dump}");
    // L2 fingerprint must remain searchable in attached.rs
    assert!(include_str!("attached.rs").contains("macro-codable-l2:associated-enum-auto"));
}

#[test]
fn p129_bean_params_collected_as_inject() {
    let src = r#"
@Configuration
class AppConfig {
  init() {}
  @Bean
  func greeter(@Autowired logger: DepLogger) -> BeanGreeter {
    return BeanGreeter(logger: logger)
  }
}
class DepLogger { init() {} }
class BeanGreeter { init(logger: DepLogger) {} }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let beans = collect_beans(&module).expect("beans");
    assert_eq!(beans.len(), 1);
    assert_eq!(beans[0].params.len(), 1);
    assert_eq!(beans[0].params[0].dep_type, "DepLogger");
    assert_eq!(beans[0].params[0].param, "logger");
}

#[test]
fn p129_bean_param_boot_codegen_passes_args() {
    let src = include_str!("../../syx-driver/tests/fixtures/macros/bean_param_autowire.sy");
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(
        dump.contains("greeter") || dump.contains("BeanGreeter"),
        "{dump}"
    );
    assert!(include_str!("service.rs").contains("macro-service-v10:bean-param-autowire-mvp"));
}
