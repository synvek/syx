//! C-3h: module name → ModuleId index fail-closed (MIR008).

use smol_str::SmolStr;
use std::path::Path;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{
    collect_module_names_from_paths, MirBlock, MirFunction, MirInstr, MirModIndex, MirModule, MirOp,
    ModuleId,
};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3h"),
        decls: vec![],
    }
}

fn mir_fn(name: &str) -> MirFunction {
    MirFunction {
        name: SmolStr::new(name),
        param_count: 0,
        local_count: 0,
        entry: 0,
        blocks: vec![MirBlock {
            id: 0,
            instructions: vec![MirInstr {
                dest: None,
                op: MirOp::Return,
                span: None,
            }],
            successors: vec![],
        }],
        instructions: vec![],
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
        owner_module: None,
    }
}

#[test]
fn c3h_build_rejects_duplicate_name() {
    let mir = MirModule {
        module_names: vec![SmolStr::new("dup"), SmolStr::new("dup")],
        ..Default::default()
    };
    let err = MirModIndex::build(&mir).expect_err("duplicate name");
    assert_eq!(err.code.as_deref(), Some("MIR008"), "{err}");
    assert!(format!("{err}").contains("duplicate name") && format!("{err}").contains("C-3h"));
}

#[test]
fn c3h_require_rejects_unknown_module() {
    let mir = MirModule {
        module_names: vec![SmolStr::new("only")],
        ..Default::default()
    };
    let idx = MirModIndex::build(&mir).unwrap();
    let err = idx
        .require(&SmolStr::new("missing"))
        .expect_err("unknown module");
    assert_eq!(err.code.as_deref(), Some("MIR008"), "{err}");
    assert!(format!("{err}").contains("unknown module"));
}

#[test]
fn c3h_dense_ids_match_name_order() {
    let mir = MirModule {
        module_names: vec![SmolStr::new("a"), SmolStr::new("b"), SmolStr::new("c")],
        ..Default::default()
    };
    let idx = MirModIndex::build(&mir).unwrap();
    assert_eq!(idx.require(&SmolStr::new("a")).unwrap(), ModuleId(0));
    assert_eq!(idx.require(&SmolStr::new("b")).unwrap(), ModuleId(1));
    assert_eq!(idx.require(&SmolStr::new("c")).unwrap(), ModuleId(2));
}

#[test]
fn c3h_empty_module_names_ok() {
    let mir = MirModule::default();
    let idx = MirModIndex::build(&mir).expect("empty OK");
    assert!(idx.is_empty());
}

#[test]
fn c3h_stem_collision_rejects_mir008() {
    let a = Path::new("/tmp/pkg/a/util.sy");
    let b = Path::new("/tmp/pkg/b/util.sy");
    let err = collect_module_names_from_paths([a, b].into_iter())
        .expect_err("same stem different paths");
    assert_eq!(err.code.as_deref(), Some("MIR008"), "{err}");
    assert!(format!("{err}").contains("duplicate module stem"));
}

#[test]
fn c3h_compile_from_mir_accepts_empty_module_names() {
    let mut main = mir_fn("main");
    main.instructions = vec![MirInstr {
        dest: None,
        op: MirOp::Return,
        span: None,
    }];
    let mir = MirModule {
        functions: vec![main],
        module_names: vec![],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect("empty module_names must not break codegen");
}

#[test]
fn c3h_compile_from_mir_rejects_duplicate_module_names() {
    let mut main = mir_fn("main");
    main.instructions = vec![MirInstr {
        dest: None,
        op: MirOp::Return,
        span: None,
    }];
    let mir = MirModule {
        functions: vec![main],
        module_names: vec![SmolStr::new("x"), SmolStr::new("x")],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("dup module_names must MIR008");
    assert_eq!(err.code.as_deref(), Some("MIR008"), "{err}");
}

#[test]
fn c3h_hooks_and_symbols_exist() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    assert!(
        unify.contains("collect_module_names_from_paths")
            && unify.contains("MirModIndex::build")
            && unify.contains("verify_mir_nil_branch_floor"),
        "unify must populate module_names and build MirModIndex after nil-branch"
    );
    let nil = unify.find("verify_mir_nil_branch_floor").expect("nil");
    let mod_idx = unify.find("MirModIndex::build").expect("MirModIndex");
    assert!(
        mod_idx > nil,
        "MirModIndex::build must follow verify_mir_nil_branch_floor"
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
    assert!(body.contains("MirModIndex::build"));
    assert!(body.contains("verify_mir_nil_branch_floor_functions"));
    let nil_c = body
        .find("verify_mir_nil_branch_floor_functions")
        .expect("nil codegen");
    let mod_c = body.find("MirModIndex::build").expect("mod codegen");
    assert!(mod_c > nil_c, "codegen MirModIndex must follow nil-branch");

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_mod_index.rs"))
        .expect("mir_mod_index.rs");
    assert!(
        api.contains("pub struct ModuleId")
            && api.contains("pub struct MirModIndex")
            && api.contains("MIR008")
            && api.contains("Not a DefId")
    );
}