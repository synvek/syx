//! C-3i: MIR try/catch pairing floor fail-closed (MIR009).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_try_floor, MirBlock, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3i"),
        decls: vec![],
    }
}

fn instr(op: MirOp) -> MirInstr {
    MirInstr {
        dest: None,
        op,
        span: None,
    }
}

fn flat_fn(name: &str, ops: Vec<MirOp>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count: 0,
        entry: 0,
        blocks: vec![],
        instructions: ops.into_iter().map(instr).collect(),
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
        owner_module: None,
    }
}

fn blocked_fn(name: &str, blocks: Vec<(u32, Vec<MirOp>)>) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count: 0,
        entry: blocks.first().map(|b| b.0).unwrap_or(0),
        blocks: blocks
            .into_iter()
            .map(|(id, ops)| MirBlock {
                id,
                instructions: ops.into_iter().map(instr).collect(),
                successors: vec![],
            })
            .collect(),
        instructions: vec![],
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
        owner_module: None,
    }
}

#[test]
fn c3i_rejects_endtry_without_begin() {
    let mir = MirModule {
        functions: vec![flat_fn("bad", vec![MirOp::EndTry, MirOp::Return])],
        ..Default::default()
    };
    let err = verify_mir_try_floor(&mir).expect_err("EndTry");
    assert_eq!(err.code.as_deref(), Some("MIR009"), "{err}");
    let text = format!("{err}");
    assert!(text.contains("EndTry") && text.contains("C-3i"), "{text}");
}

#[test]
fn c3i_rejects_unclosed_begin() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "bad",
            vec![MirOp::BeginTry(0), MirOp::Return],
        )],
        ..Default::default()
    };
    let err = verify_mir_try_floor(&mir).expect_err("unclosed");
    assert_eq!(err.code.as_deref(), Some("MIR009"), "{err}");
    assert!(format!("{err}").contains("unclosed"));
}

#[test]
fn c3i_rejects_orphan_catch() {
    let mir = MirModule {
        functions: vec![flat_fn("bad", vec![MirOp::Catch, MirOp::Return])],
        ..Default::default()
    };
    let err = verify_mir_try_floor(&mir).expect_err("orphan");
    assert_eq!(err.code.as_deref(), Some("MIR009"), "{err}");
    assert!(format!("{err}").contains("orphan Catch"));
}

#[test]
fn c3i_accepts_try_catch_lower_shape() {
    let mir = MirModule {
        functions: vec![blocked_fn(
            "ok",
            vec![
                (
                    0,
                    vec![
                        MirOp::BeginTry(1),
                        MirOp::ConstI32(1),
                        MirOp::EndTry,
                        MirOp::Jump(2),
                    ],
                ),
                (1, vec![MirOp::Catch, MirOp::Return]),
                (2, vec![MirOp::Return]),
            ],
        )],
        ..Default::default()
    };
    verify_mir_try_floor(&mir).expect("lower shape");
}

#[test]
fn c3i_accepts_propagating_throw() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "boom",
            vec![MirOp::ConstStr("x".into()), MirOp::Throw, MirOp::Return],
        )],
        ..Default::default()
    };
    verify_mir_try_floor(&mir).expect("depth-0 Throw OK");
}

#[test]
fn c3i_compile_from_mir_rejects_orphan_catch() {
    let mir = MirModule {
        functions: vec![flat_fn(
            "main",
            vec![MirOp::Catch, MirOp::Return],
        )],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("orphan Catch must MIR009 via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR009"), "{err}");
}

#[test]
fn c3i_hooks_and_symbols_exist() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    assert!(
        unify.contains("verify_mir_try_floor") && unify.contains("MirModIndex::build"),
        "unify must call verify_mir_try_floor after ModuleId"
    );
    let mod_idx = unify.find("MirModIndex::build").expect("MirModIndex");
    let try_f = unify.find("verify_mir_try_floor").expect("try floor");
    assert!(
        try_f > mod_idx,
        "verify_mir_try_floor must follow MirModIndex::build"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let nxt = after
        .find("\n    pub fn ")
        .map(|i| start + 1 + i)
        .unwrap_or(start + 5000);
    let body = &codegen[start..nxt];
    assert!(body.contains("verify_mir_try_floor_functions"));
    assert!(body.contains("MirModIndex::build"));
    let mod_c = body.find("MirModIndex::build").expect("mod codegen");
    let try_c = body
        .find("verify_mir_try_floor_functions")
        .expect("try codegen");
    assert!(
        try_c > mod_c,
        "codegen try floor must follow MirModIndex"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_try_floor.rs"))
        .expect("mir_try_floor.rs");
    assert!(
        api.contains("MIR009")
            && api.contains("C-3i")
            && api.contains("Not a full exception")
            && api.contains("verify_mir_try_floor")
    );
}
