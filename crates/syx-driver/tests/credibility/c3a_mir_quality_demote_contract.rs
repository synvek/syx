//! C-3a: duplicate MIR bodies must fail closed (no silent quality pick).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{MirBlock, MirFunction, MirInstr, MirModule, MirOp};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3a"),
        decls: vec![],
    }
}

fn mir_fn(name: &str, param_count: u32, extra_nops: usize) -> MirFunction {
    let mut instructions = vec![MirInstr {
        dest: None,
        op: MirOp::ConstNil,
        span: None,
    }];
    for _ in 0..extra_nops {
        instructions.push(MirInstr {
            dest: None,
            op: MirOp::ConstNil,
            span: None,
        });
    }
    instructions.push(MirInstr {
        dest: None,
        op: MirOp::Return,
        span: None,
    });
    MirFunction {
        name: SmolStr::new(name),
        param_count,
        local_count: 0,
        entry: 0,
        blocks: vec![MirBlock {
            id: 0,
            instructions,
            successors: vec![],
        }],
        instructions: vec![],
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
    }
}

#[test]
fn c3a_duplicate_mir_body_different_fingerprint_fails() {
    let mir = MirModule {
        functions: vec![
            mir_fn("dup", 0, 0),
            mir_fn("dup", 1, 0), // different param_count → conflict
        ],
        ..Default::default()
    };
    let module = empty_module();
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &module, &mut program)
        .expect_err("C-3a must fail closed on conflicting duplicate MIR bodies");
    let text = format!("{err}");
    assert_eq!(err.code.as_deref(), Some("MIR001"), "{text}");
    assert!(
        text.contains("duplicate MIR body") && text.contains("quality heuristic forbidden"),
        "unexpected diagnostic: {text}"
    );
}

#[test]
fn c3a_duplicate_mir_body_identical_fingerprint_keeps_first() {
    let mir = MirModule {
        functions: vec![mir_fn("same", 0, 0), mir_fn("same", 0, 0)],
        ..Default::default()
    };
    let module = empty_module();
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &module, &mut program)
        .expect("identical fingerprints must keep first, not fail");
    assert!(
        program.functions.iter().any(|f| f.name == "same"),
        "expected compiled function 'same'"
    );
}
