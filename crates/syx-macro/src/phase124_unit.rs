//! Phase 124 Macro M14 unit probes: Qualifier / Primary / Profile / withRequest hosts.

use smol_str::SmolStr;

use crate::service::{
    collect_services, expand_service_macros, pick_binding, spec_to_meta_entry,
};

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
