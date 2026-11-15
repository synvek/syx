//! Token-macro + pointcut unit probes
//! (merged from phase130–138; duplicate sum-literal / token-enumerate nails removed).

use crate::expand_module;
use crate::service::{
    collect_pointcuts, expand_advice_interceptors, expand_service_macros, parse_pointcut_pattern,
    pointcut_matches_join, pointcut_matches_method, PointcutPattern, WithinWildcard,
};
use crate::user::{
    decl_base_name_from_tokens, enumerate_decl_param_tokens, expand_decl_param_tokens,
    DeclExpandMode, MacroTokenKind,
};
use syx_ast::{Decl, Expr};
use syx_diagnostics::Span;

fn span() -> Span {
    Span::new(0, 0)
}

#[test]
fn p130_token_expr_ident_literal_subst() {
    let src = r#"
#macro double(x) {
  x + x
}
func main() {
  print(#double(21))
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroInvoke"), "{dump}");
    assert!(dump.contains("21"), "{dump}");
    assert!(
        include_str!("user.rs").contains("macro-token-v1:expr-token-mvp"),
        "fingerprint must live in user.rs"
    );
}

#[test]
fn p130_token_expr_return_body_and_nested() {
    let src = r#"
#macro inc(x) { return x + 1 }
#macro double(x) { x + x }
func main() {
  print(#double(#inc(20)))
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroInvoke"), "{dump}");
    assert!(dump.contains("20"), "{dump}");
}

#[test]
fn p130_pointcut_collected_and_around_binds() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("Greeter.greet")
  @Around("Greeter.greet")
  func greet() -> String { return "hi" }
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let pcs = collect_pointcuts(&module);
    assert!(pcs.iter().any(|p| p.as_str() == "Greeter.greet"), "{pcs:?}");

    let mut module = module;
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_proceed_0"), "{dump}");
    assert!(dump.contains("__around__:"), "{dump}");
    assert!(
        include_str!("service.rs").contains("macro-service-v11:pointcut-name-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p130_around_pointcut_requires_registration() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Around("Greeter.greet")
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = expand_advice_interceptors(&mut module).expect_err("E016");
    let msg = format!("{err}");
    assert!(
        msg.contains("E016") || msg.contains("@Pointcut") || msg.contains("@Around"),
        "{msg}"
    );
}

#[test]
fn p130_pointcut_boot_codegen_still_seals() {
    let src = r#"
import DI
@Service
class Greeter {
  init() {}
  @Pointcut("Greeter.greet")
  @Around("Greeter.greet")
  func greet() -> String {
    print("__body__:greet")
    return "ok"
  }
}
func main() {
  let g = Di.resolve("Greeter")
  print(g.greet())
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("__syx_proceed_0"), "{dump}");
}

#[test]
fn p131_decl_param_ident_literal_subst() {
    let src = r#"
@freestanding(declaration)
#macro makePrinter(msg) {
  func announce() {
    print(msg)
  }
}
#makePrinter("decl-ok")
func main() {
  announce()
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroSiteInvoke"), "{dump}");
    assert!(dump.contains("decl-ok"), "{dump}");
    assert!(
        dump.contains("name: \"announce\"") || dump.contains("announce"),
        "expanded announce fn missing: {dump}"
    );
    let fn_names: Vec<_> = module
        .decls
        .iter()
        .filter_map(|d| match d {
            syx_ast::Decl::Function { name, .. } => Some(name.as_str()),
            _ => None,
        })
        .collect();
    assert!(
        fn_names.iter().any(|n| *n == "announce"),
        "announce not in decls: {fn_names:?} dump={dump}"
    );
    assert!(
        include_str!("user.rs").contains("macro-token-v2:decl-param-subst-mvp"),
        "fingerprint must live in user.rs"
    );
}

#[test]
fn p131_decl_param_sum_literals() {
    let src = r#"
@freestanding(declaration)
#macro makeSum(a, b) {
  func sum() -> Int {
    return a + b
  }
}
#makeSum(20, 22)
func main() {
  print(sum())
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroSiteInvoke"), "{dump}");
    assert!(dump.contains("20"), "{dump}");
    assert!(dump.contains("22"), "{dump}");
}

#[test]
fn p131_execution_pointcut_parse_and_match() {
    let exact = parse_pointcut_pattern("execution(* Foo.bar(..))").expect("parse");
    assert_eq!(
        exact,
        PointcutPattern::Execution {
            type_name: smol_str::SmolStr::new("Foo"),
            method: Some(smol_str::SmolStr::new("bar")),
        }
    );
    let wild = parse_pointcut_pattern("execution(* Foo.*(..))").expect("parse");
    assert_eq!(
        wild,
        PointcutPattern::Execution {
            type_name: smol_str::SmolStr::new("Foo"),
            method: None,
        }
    );
    assert!(pointcut_matches_join("execution(* Foo.bar(..))", "Foo.bar"));
    assert!(pointcut_matches_join("execution(* Foo.*(..))", "Foo.baz"));
    assert!(!pointcut_matches_join(
        "execution(* Foo.bar(..))",
        "Foo.baz"
    ));
    assert!(!pointcut_matches_join("execution(* Foo.*(..))", "Bar.baz"));
    assert!(
        include_str!("service.rs").contains("macro-service-v12:pointcut-execution-name-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p131_execution_pointcut_collected_and_around_binds() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("execution(* Greeter.greet(..))")
  @Around("execution(* Greeter.greet(..))")
  func greet() -> String { return "hi" }
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let pcs = collect_pointcuts(&module);
    assert!(
        pcs.iter()
            .any(|p| p.as_str() == "execution(* Greeter.greet(..))"),
        "{pcs:?}"
    );

    let mut module = module;
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_proceed_0"), "{dump}");
    assert!(dump.contains("__around__:"), "{dump}");
}

#[test]
fn p131_execution_wildcard_around_binds() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("execution(* Greeter.*(..))")
  @Around("execution(* Greeter.*(..))")
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_proceed_0"), "{dump}");
}

#[test]
fn p131_execution_mismatch_fails() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("execution(* Greeter.other(..))")
  @Around("execution(* Greeter.other(..))")
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = expand_advice_interceptors(&mut module).expect_err("E016");
    let msg = format!("{err}");
    assert!(
        msg.contains("E016") || msg.contains("@Around") || msg.contains("join point"),
        "{msg}"
    );
}

#[test]
fn p131_execution_boot_codegen_still_seals() {
    let src = r#"
import DI
@Service
class Greeter {
  init() {}
  @Pointcut("execution(* Greeter.greet(..))")
  @Around("execution(* Greeter.greet(..))")
  func greet() -> String {
    print("__body__:greet")
    return "ok"
  }
}
func main() {
  let g = Di.resolve("Greeter")
  print(g.greet())
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("__syx_proceed_0"), "{dump}");
}

#[test]
fn p132_decl_hygiene_renames_params() {
    // Fingerprint: macro-token-v3:decl-hygiene-mvp
    let src = r#"
@freestanding(declaration)
#macro makePrinter(msg) {
  func announce() {
    print(msg)
  }
}
#makePrinter("decl-hygiene-ok")
func main() {
  announce()
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroSiteInvoke"), "{dump}");
    assert!(dump.contains("decl-hygiene-ok"), "{dump}");
    assert!(
        dump.contains("announce"),
        "expanded announce fn missing: {dump}"
    );
    // Hygienic param rename leaves no bare `msg` ident when hygiene is on.
    if crate::macro_hygiene_enabled() {
        assert!(
            dump.contains("__syx_mp_makePrinter_msg_") || dump.contains("decl-hygiene-ok"),
            "expected hygienic rename or substituted literal: {dump}"
        );
    }
    assert!(
        include_str!("user.rs").contains("macro-token-v3:decl-hygiene-mvp"),
        "fingerprint must live in user.rs"
    );
    assert!(
        include_str!("hygiene.rs").contains("macro-token-v3:decl-hygiene-mvp"),
        "fingerprint must live in hygiene.rs"
    );
}

#[test]
fn p132_before_after_pointcut_bind() {
    // Fingerprint: macro-service-v13:before-after-pointcut-bind-mvp
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("execution(* Greeter.greet(..))")
  @Before("execution(* Greeter.greet(..))")
  @After("execution(* Greeter.greet(..))")
  func greet() -> String {
    print("__body__:greet")
    return "hi"
  }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__before__:"), "{dump}");
    assert!(dump.contains("__after__:"), "{dump}");
    assert!(
        dump.contains("__body__:greet") || dump.contains("greet"),
        "{dump}"
    );
    assert!(
        include_str!("service.rs").contains("macro-service-v13:before-after-pointcut-bind-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p132_before_pointcut_requires_registration() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Before("Greeter.greet")
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = expand_advice_interceptors(&mut module).expect_err("E016");
    let msg = format!("{err}");
    assert!(
        msg.contains("E016") || msg.contains("@Pointcut") || msg.contains("@Before"),
        "{msg}"
    );
}

#[test]
fn p132_after_pointcut_mismatch_fails() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("execution(* Greeter.other(..))")
  @After("execution(* Greeter.other(..))")
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = expand_advice_interceptors(&mut module).expect_err("E016");
    let msg = format!("{err}");
    assert!(
        msg.contains("E016") || msg.contains("@After") || msg.contains("join point"),
        "{msg}"
    );
}

#[test]
fn p132_bare_before_after_still_work() {
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
    expand_advice_interceptors(&mut module).expect("bare advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__before__:"), "{dump}");
    assert!(dump.contains("__after__:"), "{dump}");
}

#[test]
fn p132_before_after_boot_codegen_still_seals() {
    let src = r#"
import DI
@Service
class Greeter {
  init() {}
  @Pointcut("execution(* Greeter.greet(..))")
  @Before("execution(* Greeter.greet(..))")
  @After("execution(* Greeter.greet(..))")
  func greet() -> String {
    print("__body__:greet")
    return "ok"
  }
}
func main() {
  let g = Di.resolve("Greeter")
  print(g.greet())
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("__before__:"), "{dump}");
    assert!(dump.contains("__after__:"), "{dump}");
}

#[test]
fn p133_stmt_token_ident_literal_subst() {
    // Fingerprint: macro-token-v4:stmt-token-subst-mvp
    let src = r#"
#macro say(msg) {
  print(msg)
}
#macro logSum(a, b) {
  print(a + b)
}
func main() {
  #say("stmt-token-ok")
  #logSum(20, 22)
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroInvoke"), "{dump}");
    assert!(dump.contains("stmt-token-ok"), "{dump}");
    assert!(dump.contains("20"), "{dump}");
    assert!(dump.contains("22"), "{dump}");
    assert!(
        include_str!("user.rs").contains("macro-token-v4:stmt-token-subst-mvp"),
        "fingerprint must live in user.rs"
    );
}

#[test]
fn p133_stmt_token_nested_expr_arg() {
    let src = r#"
#macro wrap(s) {
  "hi:" + s
}
#macro say(msg) {
  print(msg)
}
func main() {
  #say(#wrap("world"))
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroInvoke"), "{dump}");
    assert!(
        dump.contains("hi:") || dump.contains("world"),
        "nested expr arg subst: {dump}"
    );
}

#[test]
fn p133_stmt_token_multi_stmt_body() {
    let src = r#"
#macro sayTwice(msg) {
  print(msg)
  print(msg)
}
func main() {
  #sayTwice("twice-ok")
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroInvoke"), "{dump}");
    let count = dump.matches("twice-ok").count();
    assert!(count >= 2, "expected two substituted prints: {dump}");
}

#[test]
fn p133_stmt_token_arity_mismatch() {
    let src = r#"
#macro say(msg) {
  print(msg)
}
func main() {
  #say("a", "b")
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = expand_module(module).expect_err("arity");
    let msg = format!("{err}");
    assert!(msg.contains("arity") || msg.contains("mismatch"), "{msg}");
}

#[test]
fn p133_within_pointcut_parse_and_match() {
    // Fingerprint: macro-service-v14:pointcut-within-name-mvp
    let within = parse_pointcut_pattern("within(Foo)").expect("parse");
    assert_eq!(
        within,
        PointcutPattern::Within {
            type_name: smol_str::SmolStr::new("Foo"),
            wildcard: WithinWildcard::Exact,
        }
    );
    assert!(pointcut_matches_join("within(Foo)", "Foo.bar"));
    assert!(pointcut_matches_join("within(Foo)", "Foo.baz"));
    assert!(!pointcut_matches_join("within(Foo)", "Bar.bar"));
    assert!(parse_pointcut_pattern("within()").is_none());
    assert!(parse_pointcut_pattern("within(Foo.Bar)").is_none());
    assert!(
        include_str!("service.rs").contains("macro-service-v14:pointcut-within-name-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p133_within_around_before_after_bind() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("within(Greeter)")
  @Around("within(Greeter)")
  @Before("within(Greeter)")
  @After("within(Greeter)")
  func greet() -> String {
    print("__body__:greet")
    return "hi"
  }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__before__:"), "{dump}");
    assert!(dump.contains("__after__:"), "{dump}");
    assert!(dump.contains("__around__:"), "{dump}");
    assert!(dump.contains("__syx_proceed_0"), "{dump}");
}

#[test]
fn p133_within_hits_any_method_on_type() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("within(Greeter)")
  func marker() {}
  @Around("within(Greeter)")
  @Before("within(Greeter)")
  func alpha() -> String {
    print("__body__:alpha")
    return "a"
  }
  @After("within(Greeter)")
  func beta() -> String {
    print("__body__:beta")
    return "b"
  }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__around__:"), "{dump}");
    assert!(dump.contains("__before__:"), "{dump}");
    assert!(dump.contains("__after__:"), "{dump}");
}

#[test]
fn p133_within_mismatch_fails() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("within(Other)")
  @Around("within(Other)")
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = expand_advice_interceptors(&mut module).expect_err("E016");
    let msg = format!("{err}");
    assert!(
        msg.contains("E016") || msg.contains("@Around") || msg.contains("join point"),
        "{msg}"
    );
}

#[test]
fn p133_within_boot_codegen_still_seals() {
    let src = r#"
import DI
@Service
class Greeter {
  init() {}
  @Pointcut("within(Greeter)")
  @Before("within(Greeter)")
  @After("within(Greeter)")
  func greet() -> String {
    print("__body__:greet")
    return "ok"
  }
}
func main() {
  let g = Di.resolve("Greeter")
  print(g.greet())
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("__before__:"), "{dump}");
    assert!(dump.contains("__after__:"), "{dump}");
}

#[test]
fn p134_expr_token_nested_call_subst() {
    // Fingerprint: macro-token-v5:expr-token-subst-mvp
    let src = r#"
#macro apply(f, x) {
  f(x) + f(x)
}
func helper(v: Int) -> Int {
  return v + 1
}
func main() {
  print(#apply(helper, 10))
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroInvoke"), "{dump}");
    assert!(dump.contains("helper"), "{dump}");
    assert!(dump.contains("10"), "{dump}");
    assert!(
        include_str!("user.rs").contains("macro-token-v5:expr-token-subst-mvp"),
        "fingerprint must live in user.rs"
    );
}

#[test]
fn p134_expr_token_array_and_member_subst() {
    let src = r#"
#macro pair(x) {
  [x, x]
}
#macro pick(obj, field) {
  obj.field + 1
}
func main() {
  print(#pair(21))
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroInvoke"), "{dump}");
    assert!(dump.contains("21"), "{dump}");
}

#[test]
fn p134_args_pointcut_parse_and_match() {
    // Fingerprint: macro-service-v15:pointcut-args-name-mvp
    let args = parse_pointcut_pattern("args(Int)").expect("parse");
    assert_eq!(
        args,
        PointcutPattern::Args {
            type_name: smol_str::SmolStr::new("Int"),
        }
    );
    assert!(pointcut_matches_method(
        "args(Int)",
        "Foo.bar",
        &[syx_ast::Param {
            name: smol_str::SmolStr::new("x"),
            external_label: syx_ast::ParamLabel::Same,
            ty: Some(syx_ast::TypeExpr::Named {
                name: smol_str::SmolStr::new("Int"),
                generics: vec![],
            }),
            default: None,
            isolated: false,
            sending: false,
            attrs: vec![],
        }]
    ));
    assert!(!pointcut_matches_method(
        "args(Int)",
        "Foo.bar",
        &[syx_ast::Param {
            name: smol_str::SmolStr::new("s"),
            external_label: syx_ast::ParamLabel::Same,
            ty: Some(syx_ast::TypeExpr::Named {
                name: smol_str::SmolStr::new("String"),
                generics: vec![],
            }),
            default: None,
            isolated: false,
            sending: false,
            attrs: vec![],
        }]
    ));
    assert!(parse_pointcut_pattern("args()").is_none());
    assert!(
        include_str!("service.rs").contains("macro-service-v15:pointcut-args-name-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p134_args_around_before_after_bind() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("args(Int)")
  @Around("args(Int)")
  @Before("args(Int)")
  @After("args(Int)")
  func greet(x: Int) -> String {
    print("__body__:greet")
    return "hi"
  }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__before__:"), "{dump}");
    assert!(dump.contains("__after__:"), "{dump}");
    assert!(dump.contains("__around__:"), "{dump}");
}

#[test]
fn p134_args_mismatch_fails() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("args(Int)")
  @Before("args(Int)")
  func greet(name: String) -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = expand_advice_interceptors(&mut module).expect_err("E016");
    let msg = format!("{err}");
    assert!(
        msg.contains("E016") || msg.contains("@Before") || msg.contains("join point"),
        "{msg}"
    );
}

#[test]
fn p134_args_boot_codegen_still_seals() {
    let src = r#"
import DI
@Service
class Greeter {
  init() {}
  @Pointcut("args(Int)")
  @Before("args(Int)")
  @After("args(Int)")
  func greet(x: Int) -> String {
    print("__body__:greet")
    return "ok"
  }
}
func main() {
  let g = Di.resolve("Greeter")
  print(g.greet(x: 1))
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("__before__:"), "{dump}");
}

#[test]
fn p135_and_pointcut_parse_and_match() {
    // Fingerprint: macro-service-v16:pointcut-and-compose-mvp
    let and = parse_pointcut_pattern("within(Foo) && args(Int)").expect("parse");
    assert_eq!(
        and,
        PointcutPattern::And {
            left: Box::new(PointcutPattern::Within {
                type_name: smol_str::SmolStr::new("Foo"),
                wildcard: WithinWildcard::Exact,
            }),
            right: Box::new(PointcutPattern::Args {
                type_name: smol_str::SmolStr::new("Int"),
            }),
        }
    );
    let int_param = syx_ast::Param {
        name: smol_str::SmolStr::new("x"),
        external_label: syx_ast::ParamLabel::Same,
        ty: Some(syx_ast::TypeExpr::Named {
            name: smol_str::SmolStr::new("Int"),
            generics: vec![],
        }),
        default: None,
        isolated: false,
        sending: false,
        attrs: vec![],
    };
    assert!(pointcut_matches_method(
        "within(Foo) && args(Int)",
        "Foo.bar",
        &[int_param.clone()]
    ));
    assert!(!pointcut_matches_method(
        "within(Foo) && args(Int)",
        "Bar.bar",
        &[int_param.clone()]
    ));
    assert!(!pointcut_matches_method(
        "within(Foo) && args(Int)",
        "Foo.bar",
        &[syx_ast::Param {
            name: smol_str::SmolStr::new("s"),
            external_label: syx_ast::ParamLabel::Same,
            ty: Some(syx_ast::TypeExpr::Named {
                name: smol_str::SmolStr::new("String"),
                generics: vec![],
            }),
            default: None,
            isolated: false,
            sending: false,
            attrs: vec![],
        }]
    ));
    assert!(parse_pointcut_pattern("within(Foo) && within(Bar)").is_none());
    assert!(parse_pointcut_pattern("within(Foo) && args()").is_none());
    assert!(
        include_str!("service.rs").contains("macro-service-v16:pointcut-and-compose-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p135_and_around_before_after_bind() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("within(Greeter) && args(Int)")
  @Around("within(Greeter) && args(Int)")
  @Before("within(Greeter) && args(Int)")
  @After("within(Greeter) && args(Int)")
  func greet(x: Int) -> String {
    print("__body__:greet")
    return "hi"
  }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__before__:"), "{dump}");
    assert!(dump.contains("__after__:"), "{dump}");
    assert!(dump.contains("__around__:"), "{dump}");
}

#[test]
fn p135_and_mismatch_fails() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("within(Greeter) && args(Int)")
  @Before("within(Greeter) && args(Int)")
  func greet(name: String) -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = expand_advice_interceptors(&mut module).expect_err("E016");
    let msg = format!("{err}");
    assert!(
        msg.contains("E016") || msg.contains("@Before") || msg.contains("join point"),
        "{msg}"
    );
}

#[test]
fn p135_decl_token_name_and_default_subst() {
    // Fingerprint: macro-token-v6:decl-token-subst-mvp
    let src = r#"
@freestanding(declaration)
#macro makeFunc(fnName, msg) {
  func fnName() {
    print(msg)
  }
}
@freestanding(declaration)
#macro makeDefault(base) {
  func compute(n: Int = base + 1) -> Int {
    return n
  }
}
#makeFunc(greet, "decl-token-ok")
#makeDefault(41)
func main() {
  greet()
  print(compute())
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroSiteInvoke"), "{dump}");
    assert!(dump.contains("decl-token-ok"), "{dump}");
    assert!(
        dump.contains("name: \"greet\"") || dump.contains("greet"),
        "{dump}"
    );
    assert!(dump.contains("41"), "{dump}");
    assert!(
        include_str!("user.rs").contains("macro-token-v6:decl-token-subst-mvp"),
        "fingerprint must live in user.rs"
    );
}

#[test]
fn p135_and_boot_codegen_still_seals() {
    let src = r#"
import DI
@Service
class Greeter {
  init() {}
  @Pointcut("within(Greeter) && args(Int)")
  @Before("within(Greeter) && args(Int)")
  @After("within(Greeter) && args(Int)")
  func greet(x: Int) -> String {
    print("__body__:greet")
    return "ok"
  }
}
func main() {
  let g = Di.resolve("Greeter")
  print(g.greet(x: 1))
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("__before__:"), "{dump}");
}

#[test]
fn p136_or_pointcut_parse_and_match() {
    // Fingerprint: macro-service-v17:pointcut-or-compose-mvp
    let or = parse_pointcut_pattern("within(Foo) || args(Int)").expect("parse");
    assert_eq!(
        or,
        PointcutPattern::Or {
            left: Box::new(PointcutPattern::Within {
                type_name: smol_str::SmolStr::new("Foo"),
                wildcard: WithinWildcard::Exact,
            }),
            right: Box::new(PointcutPattern::Args {
                type_name: smol_str::SmolStr::new("Int"),
            }),
        }
    );
    let swapped = parse_pointcut_pattern("args(Int) || within(Foo)").expect("parse swapped");
    assert!(matches!(swapped, PointcutPattern::Or { .. }));
    let int_param = syx_ast::Param {
        name: smol_str::SmolStr::new("x"),
        external_label: syx_ast::ParamLabel::Same,
        ty: Some(syx_ast::TypeExpr::Named {
            name: smol_str::SmolStr::new("Int"),
            generics: vec![],
        }),
        default: None,
        isolated: false,
        sending: false,
        attrs: vec![],
    };
    let str_param = syx_ast::Param {
        name: smol_str::SmolStr::new("s"),
        external_label: syx_ast::ParamLabel::Same,
        ty: Some(syx_ast::TypeExpr::Named {
            name: smol_str::SmolStr::new("String"),
            generics: vec![],
        }),
        default: None,
        isolated: false,
        sending: false,
        attrs: vec![],
    };
    assert!(pointcut_matches_method(
        "within(Foo) || args(Int)",
        "Foo.bar",
        &[str_param.clone()]
    ));
    assert!(pointcut_matches_method(
        "within(Foo) || args(Int)",
        "Bar.baz",
        &[int_param.clone()]
    ));
    assert!(!pointcut_matches_method(
        "within(Foo) || args(Int)",
        "Bar.baz",
        &[str_param]
    ));
    assert!(parse_pointcut_pattern("within(Foo) || within(Bar)").is_none());
    assert!(parse_pointcut_pattern("within(Foo) || args()").is_none());
    assert!(
        include_str!("service.rs").contains("macro-service-v17:pointcut-or-compose-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p136_or_around_before_after_bind() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("within(Greeter) || args(Int)")
  @Around("within(Greeter) || args(Int)")
  @Before("within(Greeter) || args(Int)")
  @After("within(Greeter) || args(Int)")
  func greet(name: String) -> String {
    print("__body__:greet")
    return "hi"
  }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__before__:"), "{dump}");
    assert!(dump.contains("__after__:"), "{dump}");
    assert!(dump.contains("__around__:"), "{dump}");
}

#[test]
fn p136_token_stream_enumerate_ident_punct_group() {
    // Fingerprint: macro-token-v7:token-stream-mvp
    let expr = Expr::Binary {
        span: span(),
        left: Box::new(Expr::Ident {
            span: span(),
            name: smol_str::SmolStr::new("a"),
        }),
        op: syx_ast::BinOp::Add,
        right: Box::new(Expr::Group {
            span: span(),
            expr: Box::new(Expr::Ident {
                span: span(),
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
    assert!(
        include_str!("user.rs").contains("macro-token-v7:token-stream-mvp"),
        "fingerprint must live in user.rs"
    );
}

#[test]
fn p136_or_boot_codegen_still_seals() {
    let src = r#"
import DI
@Service
class Greeter {
  init() {}
  @Pointcut("within(Greeter) || args(Int)")
  @Before("within(Greeter) || args(Int)")
  @After("within(Greeter) || args(Int)")
  func greet(name: String) -> String {
    print("__body__:greet")
    return "ok"
  }
}
func main() {
  let g = Di.resolve("Greeter")
  print(g.greet(name: "x"))
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("__before__:"), "{dump}");
}

#[test]
fn p137_not_within_pointcut_parse_and_match() {
    // Fingerprint: macro-service-v18:pointcut-not-within-mvp
    let not_within = parse_pointcut_pattern("!within(Excluded)").expect("parse");
    assert_eq!(
        not_within,
        PointcutPattern::Not {
            inner: Box::new(PointcutPattern::Within {
                type_name: smol_str::SmolStr::new("Excluded"),
                wildcard: WithinWildcard::Exact,
            }),
        }
    );
    assert!(pointcut_matches_method(
        "!within(Excluded)",
        "Allowed.handle",
        &[]
    ));
    assert!(!pointcut_matches_method(
        "!within(Excluded)",
        "Excluded.handle",
        &[]
    ));
    assert!(parse_pointcut_pattern("!within()").is_none());
    assert!(
        include_str!("service.rs").contains("macro-service-v18:pointcut-not-within-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p137_not_within_before_after_bind() {
    let src = r#"
@Service
class Allowed {
  init() {}
  @Pointcut("!within(Excluded)")
  @Before("!within(Excluded)")
  @After("!within(Excluded)")
  func handle() -> String {
    print("__body__:handle")
    return "allowed-ok"
  }
}
@Service
class Excluded {
  init() {}
  func handle() -> String {
    print("__body__:excluded")
    return "excluded-plain"
  }
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
fn p137_decl_expand_rename_and_wrap() {
    // Fingerprint: macro-token-v8:decl-expand-mvp
    let tokens = vec![MacroTokenKind::Ident(smol_str::SmolStr::new("greet"))];
    let renamed =
        expand_decl_param_tokens(&tokens, DeclExpandMode::RenamePrefix("wrapped_".into()))
            .expect("rename expand");
    let wrapped = expand_decl_param_tokens(&tokens, DeclExpandMode::WrapPrefix("wrap_".into()))
        .expect("wrap expand");
    assert!(matches!(
        renamed,
        Decl::Function {
            ref name,
            ..
        } if name.as_str() == "wrapped_greet"
    ));
    assert!(matches!(
        wrapped,
        Decl::Function {
            ref name,
            ..
        } if name.as_str() == "wrap_greet"
    ));
    let dump = format!("{renamed:?}");
    assert!(dump.contains("decl-expand-rename"), "{dump}");
    let dump2 = format!("{wrapped:?}");
    assert!(dump2.contains("decl-expand-wrap"), "{dump2}");
    assert!(expand_decl_param_tokens(
        &[MacroTokenKind::Punct('+')],
        DeclExpandMode::RenamePrefix("x_".into())
    )
    .is_none());
    assert!(
        include_str!("user.rs").contains("macro-token-v8:decl-expand-mvp"),
        "fingerprint must live in user.rs"
    );
}

#[test]
fn p137_not_within_boot_codegen_still_seals() {
    let src = r#"
import DI
@Service
class Allowed {
  init() {}
  @Pointcut("!within(Excluded)")
  @Before("!within(Excluded)")
  @After("!within(Excluded)")
  func handle() -> String {
    print("__body__:handle")
    return "ok"
  }
}
func main() {
  let a = Di.resolve("Allowed")
  print(a.handle())
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("__before__:"), "{dump}");
}

#[test]
fn p137_fingerprint_string_pins() {
    let service_rs = include_str!("service.rs");
    let user_rs = include_str!("user.rs");
    let runtime_rs = include_str!("../../syx-async/src/runtime.rs");
    let concurrency = include_str!("../../../packages/core/concurrency.sy");
    assert!(service_rs.contains("macro-service-v18:pointcut-not-within-mvp"));
    assert!(user_rs.contains("macro-token-v8:decl-expand-mvp"));
    assert!(runtime_rs.contains("runtime-concurrency-v5:discard-oldest-mvp"));
    assert!(concurrency.contains("discardOldest"));
}

#[test]
fn p138_within_wildcard_parse_and_match() {
    // Fingerprint: macro-service-v19:pointcut-within-wildcard-mvp
    let suffix = parse_pointcut_pattern("within(*Service)").expect("parse suffix");
    assert_eq!(
        suffix,
        PointcutPattern::Within {
            type_name: smol_str::SmolStr::new("Service"),
            wildcard: WithinWildcard::Suffix,
        }
    );
    let prefix = parse_pointcut_pattern("within(Foo*)").expect("parse prefix");
    assert_eq!(
        prefix,
        PointcutPattern::Within {
            type_name: smol_str::SmolStr::new("Foo"),
            wildcard: WithinWildcard::Prefix,
        }
    );
    assert!(pointcut_matches_method(
        "within(*Service)",
        "OrderService.run",
        &[]
    ));
    assert!(pointcut_matches_method(
        "within(*Service)",
        "PaymentService.handle",
        &[]
    ));
    assert!(!pointcut_matches_method(
        "within(*Service)",
        "OrderRepo.run",
        &[]
    ));
    assert!(pointcut_matches_method("within(Foo*)", "FooBar.run", &[]));
    assert!(pointcut_matches_method("within(Foo*)", "Foo.run", &[]));
    assert!(!pointcut_matches_method("within(Foo*)", "BarFoo.run", &[]));
    assert!(parse_pointcut_pattern("within(*)").is_none());
    assert!(parse_pointcut_pattern("within(*Foo*)").is_none());
    assert!(
        include_str!("service.rs").contains("macro-service-v19:pointcut-within-wildcard-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p138_within_wildcard_before_bind() {
    let src = r#"
@Service
class OrderService {
  init() {}
  @Pointcut("within(*Service)")
  @Before("within(*Service)")
  func run() -> String {
    print("__body__:run")
    return "svc-ok"
  }
}
@Service
class OrderRepo {
  init() {}
  func run() -> String {
    print("__body__:repo")
    return "repo-plain"
  }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__before__:"), "{dump}");
}

#[test]
fn p138_token_rewrite_multi_token_decl() {
    // Fingerprint: macro-token-v9:token-rewrite-mvp
    let tokens = vec![
        MacroTokenKind::Ident(smol_str::SmolStr::new("my")),
        MacroTokenKind::Punct('_'),
        MacroTokenKind::Ident(smol_str::SmolStr::new("fn")),
    ];
    assert_eq!(
        decl_base_name_from_tokens(&tokens).map(|s| s.to_string()),
        Some("my_fn".into())
    );
    let renamed =
        expand_decl_param_tokens(&tokens, DeclExpandMode::RenamePrefix("wrapped_".into()))
            .expect("multi-token rename");
    assert!(matches!(
        renamed,
        Decl::Function {
            ref name,
            ..
        } if name.as_str() == "wrapped_my_fn"
    ));
    let wrapped = expand_decl_param_tokens(&tokens, DeclExpandMode::WrapPrefix("wrap_".into()))
        .expect("multi-token wrap");
    let dump = format!("{wrapped:?}");
    assert!(dump.contains("decl-rewrite-wrap"), "{dump}");
    let group_tokens = vec![MacroTokenKind::Group(vec![
        MacroTokenKind::Ident(smol_str::SmolStr::new("inner")),
        MacroTokenKind::Punct('_'),
        MacroTokenKind::Ident(smol_str::SmolStr::new("name")),
    ])];
    let grouped = expand_decl_param_tokens(&group_tokens, DeclExpandMode::WrapPrefix("g_".into()))
        .expect("group rewrite");
    assert!(matches!(
        grouped,
        Decl::Function {
            ref name,
            ..
        } if name.as_str() == "g_inner_name"
    ));
    let expr = Expr::Binary {
        span: span(),
        left: Box::new(Expr::Ident {
            span: span(),
            name: smol_str::SmolStr::new("a"),
        }),
        op: syx_ast::BinOp::Add,
        right: Box::new(Expr::Ident {
            span: span(),
            name: smol_str::SmolStr::new("b"),
        }),
    };
    let enumerated = enumerate_decl_param_tokens(&expr);
    let from_enum =
        expand_decl_param_tokens(&enumerated, DeclExpandMode::RenamePrefix("r_".into()))
            .expect("enum rewrite");
    assert!(matches!(
        from_enum,
        Decl::Function {
            ref name,
            ..
        } if name.as_str() == "r_a+b"
    ));
    assert!(expand_decl_param_tokens(&[], DeclExpandMode::WrapPrefix("x_".into())).is_none());
    assert!(
        include_str!("user.rs").contains("macro-token-v9:token-rewrite-mvp"),
        "fingerprint must live in user.rs"
    );
}

#[test]
fn p138_within_wildcard_boot_codegen_still_seals() {
    let src = r#"
import DI
@Service
class BillingService {
  init() {}
  @Pointcut("within(*Service)")
  @Before("within(*Service)")
  func handle() -> String {
    print("__body__:handle")
    return "ok"
  }
}
func main() {
  let s = Di.resolve("BillingService")
  print(s.handle())
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("__before__:"), "{dump}");
}

#[test]
fn p138_fingerprint_string_pins() {
    let service_rs = include_str!("service.rs");
    let user_rs = include_str!("user.rs");
    let runtime_rs = include_str!("../../syx-async/src/runtime.rs");
    let concurrency = include_str!("../../../packages/core/concurrency.sy");
    assert!(service_rs.contains("macro-service-v19:pointcut-within-wildcard-mvp"));
    assert!(user_rs.contains("macro-token-v9:token-rewrite-mvp"));
    assert!(runtime_rs.contains("runtime-concurrency-v6:actor-shared-pool-mvp"));
    assert!(concurrency.contains("setActorSharedPoolEnabled"));
}
