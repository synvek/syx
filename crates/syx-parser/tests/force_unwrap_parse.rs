use syx_ast::{Decl, Expr};
use syx_diagnostics::SourceFile;
use syx_parser::Parser;

#[test]
fn annotated_force_unwrap_parses() {
    let src = "let present: String? = \"hi\"\nlet s: String = present!\n";
    let file = SourceFile::new("t.sy", src);
    let module = Parser::parse(file).expect("parse");
    let Decl::VarGlobal { init: Some(expr), .. } = &module.decls[1] else {
        panic!("expected VarGlobal {:?}", module.decls.get(1));
    };
    assert!(
        matches!(expr, Expr::ForceUnwrap { .. }),
        "got {expr:?}"
    );
}
