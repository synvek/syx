#[test]
fn f32_const_numeric_bits() {
    use smol_str::SmolStr;
    use syx_ast::{AccessLevel, Decl, Expr, Literal, Module, Stmt};
    use syx_diagnostics::Span;

    let module = Module {
        name: SmolStr::new("t"),
        decls: vec![Decl::Function {
            span: Span::new(0, 0),
            access: AccessLevel::Internal,
            attrs: vec![],
            name: SmolStr::new("main"),
            type_params: vec![],
            params: vec![],
            return_type: None,
            throws_type: None,
            effect: None,
            where_clause: vec![],
            is_async: false,
            is_override: false,
            is_final: false,
            body: vec![Stmt::VarDecl {
                span: Span::new(0, 0),
                mutable: false,
                name: SmolStr::new("i"),
                ty: None,
                init: Some(Expr::Literal {
                    span: Span::new(0, 0),
                    value: Literal::Double {
                        value: 3.4,
                        suffix: Some(SmolStr::new("f32")),
                    },
                }),
            }],
        }],
    };
    // W478 / ADR-822: MIR path (no external AST-only compile entry)
    let mir = syx_mir::lower_module(&module).unwrap();
    let mut program = syx_codegen::CompiledProgram::default();
    syx_codegen::CodeGenerator::compile_with_mir(
        &module, &mir, &mut program, None, None, None,
    )
    .unwrap();
    let main = program.functions.iter().find(|f| f.name == "main").unwrap();
    let ins = &main.instructions[0];
    if let syx_codegen::Instruction::ConstNumeric { kind, bits } = ins {
        assert_eq!(*kind, syx_stdlib::NumericKind::F32.to_byte());
        assert_eq!(*bits, syx_stdlib::float_literal_bits(syx_stdlib::NumericKind::F32, 3.4));
    } else {
        panic!("expected ConstNumeric, got {ins:?}");
    }
}
