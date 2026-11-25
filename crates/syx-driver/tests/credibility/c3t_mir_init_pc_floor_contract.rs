//! C-3t: MIR init_body_pc / init_wrap_pc bounds floor fail-closed (MIR020).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    verify_mir_init_pc_floor, MirFunction, MirInstr, MirModule, MirOp,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3t"),
        decls: vec![],
    }
}

fn flat_fn(name: &str, n_instr: usize, body: u32, wrap: u32) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count: 0,
        entry: 0,
        blocks: vec![],
        instructions: (0..n_instr)
            .map(|_| MirInstr {
                dest: None,
                op: MirOp::Return,
                span: None,
            })
            .collect(),
        local_names: vec![],
        local_types: vec![],
        init_body_pc: body,
        init_wrap_pc: wrap,
        owner_module: None,
    }
}

#[test]
fn c3t_rejects_body_gt_wrap() {
    let mir = MirModule {
        functions: vec![flat_fn("bad", 5, 3, 2)],
        ..Default::default()
    };
    let err = verify_mir_init_pc_floor(&mir).expect_err("body>wrap");
    assert_eq!(err.code.as_deref(), Some("MIR020"), "{err}");
    let text = format!("{err}");
    assert!(
        text.contains("init_body_pc")
            && text.contains("init_wrap_pc")
            && text.contains("C-3t"),
        "{text}"
    );
}

#[test]
fn c3t_rejects_wrap_gt_len() {
    let mir = MirModule {
        functions: vec![flat_fn("bad", 3, 1, 5)],
        ..Default::default()
    };
    let err = verify_mir_init_pc_floor(&mir).expect_err("wrap>len");
    assert_eq!(err.code.as_deref(), Some("MIR020"), "{err}");
    assert!(format!("{err}").contains("instructions.len()"));
}

#[test]
fn c3t_ordered_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("Cls_init", 5, 1, 4)],
        ..Default::default()
    };
    verify_mir_init_pc_floor(&mir).expect("ordered");
}

#[test]
fn c3t_dual_zero_ok() {
    let mir = MirModule {
        functions: vec![flat_fn("main", 2, 0, 0)],
        ..Default::default()
    };
    verify_mir_init_pc_floor(&mir).expect("dual zero");
}

#[test]
fn c3t_empty_module_ok() {
    verify_mir_init_pc_floor(&MirModule::default()).expect("empty");
}

#[test]
fn c3t_compile_from_mir_rejects_oob() {
    let mir = MirModule {
        functions: vec![flat_fn("main", 2, 0, 9)],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("oob via codegen");
    assert_eq!(err.code.as_deref(), Some("MIR020"), "{err}");
}

#[test]
fn c3t_compile_from_mir_accepts_dual_zero() {
    let mir = MirModule {
        functions: vec![flat_fn("main", 1, 0, 0)],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program).expect("OK");
    assert!(!program.functions.is_empty());
}

#[test]
fn c3t_hooks_after_aot_export() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");

    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    let export_u = unify
        .find("verify_mir_aot_export_floor")
        .expect("aot-export unify");
    let init_u = unify
        .find("verify_mir_init_pc_floor")
        .expect("init-pc unify");
    assert!(
        init_u > export_u,
        "unify init-pc floor must follow aot-export floor"
    );

    let codegen = std::fs::read_to_string(root.join("crates/syx-codegen/src/lib.rs"))
        .expect("codegen");
    let start = codegen
        .find("pub fn compile_from_mir_with_source")
        .expect("compile_from_mir_with_source");
    let after = &codegen[start + 1..];
    let rel = after.find("\n    pub fn ").unwrap_or(5000);
    let body = &codegen[start..start + 1 + rel];
    let export_c = body
        .find("verify_mir_aot_export_floor")
        .expect("aot-export codegen");
    let init_c = body
        .find("verify_mir_init_pc_floor_functions")
        .expect("init-pc codegen");
    assert!(
        init_c > export_c,
        "codegen init-pc floor must follow aot-export floor"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_init_pc_floor.rs"))
        .expect("mir_init_pc_floor.rs");
    assert!(
        api.contains("MIR020")
            && api.contains("C-3t")
            && api.contains("init_body_pc")
            && api.contains("init_wrap_pc")
            && api.contains("Not CallSuper")
            && api.contains("verify_mir_init_pc_floor")
    );
}
