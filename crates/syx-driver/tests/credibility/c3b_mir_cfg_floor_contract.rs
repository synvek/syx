//! C-3b: MIR CFG / terminator floor fail-closed (MIR002).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{verify_mir_cfg_floor, MirBlock, MirFunction, MirInstr, MirModule, MirOp};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3b"),
        decls: vec![],
    }
}

fn mir_fn(name: &str, entry: u32, blocks: Vec<MirBlock>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count: 0,
        entry,
        blocks,
        instructions: vec![],
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
    }
}

fn block(id: u32, ops: Vec<MirOp>, successors: Vec<u32>) -> MirBlock {
    MirBlock {
        id,
        instructions: ops
            .into_iter()
            .map(|op| MirInstr {
                dest: None,
                op,
                span: None,
            })
            .collect(),
        successors,
    }
}

#[test]
fn c3b_verify_api_rejects_bad_entry() {
    let mir = MirModule {
        functions: vec![mir_fn(
            "bad_entry",
            7,
            vec![block(0, vec![MirOp::Return], vec![])],
        )],
        ..Default::default()
    };
    let err = verify_mir_cfg_floor(&mir).expect_err("bad entry must MIR002");
    assert_eq!(err.code.as_deref(), Some("MIR002"), "{err}");
    assert!(
        format!("{err}").contains("bad entry") && format!("{err}").contains("C-3b"),
        "{err}"
    );
}

#[test]
fn c3b_compile_from_mir_rejects_oob_jump() {
    let mir = MirModule {
        functions: vec![mir_fn(
            "oob",
            0,
            vec![block(0, vec![MirOp::Jump(42)], vec![])],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("oob jump must fail closed via compile_from_mir");
    assert_eq!(err.code.as_deref(), Some("MIR002"), "{err}");
    assert!(format!("{err}").contains("oob jump"), "{err}");
}

#[test]
fn c3b_compile_from_mir_rejects_missing_terminator() {
    let mir = MirModule {
        functions: vec![mir_fn(
            "no_term",
            0,
            vec![block(0, vec![MirOp::ConstNil], vec![])],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("missing terminator must fail closed");
    assert_eq!(err.code.as_deref(), Some("MIR002"), "{err}");
    assert!(format!("{err}").contains("missing terminator"), "{err}");
}

#[test]
fn c3b_compile_from_mir_accepts_minimal_return() {
    let mir = MirModule {
        functions: vec![mir_fn(
            "ok",
            0,
            vec![block(0, vec![MirOp::ConstNil, MirOp::Return], vec![])],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect("valid MIR must pass C-3b floor");
    assert!(program.functions.iter().any(|f| f.name == "ok"));
}

#[test]
fn c3b_hooks_and_symbols_exist() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    assert!(
        unify.contains("verify_mir_cfg_floor"),
        "build_unified_mir path must call verify_mir_cfg_floor"
    );
    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen lib.rs");
    assert!(
        codegen.contains("verify_mir_cfg_floor"),
        "compile_from_mir_with_source must call verify_mir_cfg_floor"
    );
    let verify = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_verify_floor.rs"))
        .expect("mir_verify_floor.rs");
    assert!(verify.contains("MIR002") && verify.contains("C-3b"));
}
