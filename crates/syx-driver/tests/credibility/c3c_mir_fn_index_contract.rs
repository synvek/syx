//! C-3c: name → FuncId index fail-closed (MIR003).

use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_mir::{FuncId, MirBlock, MirFnIndex, MirFunction, MirInstr, MirModule, MirOp};

fn empty_module() -> Module {
    Module {
        name: SmolStr::new("c3c"),
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
    }
}

#[test]
fn c3c_build_rejects_duplicate_name() {
    let mir = MirModule {
        functions: vec![mir_fn("dup"), mir_fn("dup")],
        ..Default::default()
    };
    let err = MirFnIndex::build(&mir).expect_err("duplicate name");
    assert_eq!(err.code.as_deref(), Some("MIR003"), "{err}");
    assert!(format!("{err}").contains("duplicate name") && format!("{err}").contains("C-3c"));
}

#[test]
fn c3c_require_rejects_unknown_callee() {
    let mir = MirModule {
        functions: vec![mir_fn("only")],
        ..Default::default()
    };
    let idx = MirFnIndex::build(&mir).unwrap();
    let err = idx
        .require(&SmolStr::new("missing"))
        .expect_err("unknown callee");
    assert_eq!(err.code.as_deref(), Some("MIR003"), "{err}");
    assert!(format!("{err}").contains("unknown callee"));
}

#[test]
fn c3c_dense_ids_match_function_order() {
    let mir = MirModule {
        functions: vec![mir_fn("a"), mir_fn("b"), mir_fn("c")],
        ..Default::default()
    };
    let idx = MirFnIndex::build(&mir).unwrap();
    assert_eq!(idx.require(&SmolStr::new("a")).unwrap(), FuncId(0));
    assert_eq!(idx.require(&SmolStr::new("b")).unwrap(), FuncId(1));
    assert_eq!(idx.require(&SmolStr::new("c")).unwrap(), FuncId(2));
}

#[test]
fn c3c_compile_from_mir_rejects_unknown_call() {
    // codegen mir_function_to_bytecode reads flat `instructions`, not blocks.
    let mut caller = mir_fn("main");
    caller.instructions = vec![
        MirInstr {
            dest: None,
            op: MirOp::Call(SmolStr::new("no_such_fn")),
            span: None,
        },
        MirInstr {
            dest: None,
            op: MirOp::Return,
            span: None,
        },
    ];
    let mir = MirModule {
        functions: vec![caller],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    let err = CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect_err("unknown Call must MIR003");
    assert_eq!(err.code.as_deref(), Some("MIR003"), "{err}");
    assert!(
        format!("{err}").contains("unknown callee") || format!("{err}").contains("no_such_fn"),
        "{err}"
    );
}

#[test]
fn c3c_compile_from_mir_accepts_known_call() {
    let mut callee = mir_fn("helper");
    callee.instructions = vec![MirInstr {
        dest: None,
        op: MirOp::Return,
        span: None,
    }];
    let mut caller = mir_fn("main");
    caller.instructions = vec![
        MirInstr {
            dest: Some(0),
            op: MirOp::Call(SmolStr::new("helper")),
            span: None,
        },
        MirInstr {
            dest: None,
            op: MirOp::Return,
            span: None,
        },
    ];
    let mir = MirModule {
        functions: vec![caller, callee],
        ..Default::default()
    };
    let mut program = CompiledProgram::default();
    CodeGenerator::compile_from_mir(&mir, &empty_module(), &mut program)
        .expect("known Call must succeed");
    assert!(program.functions.iter().any(|f| f.name == "helper"));
    assert!(program.functions.iter().any(|f| f.name == "main"));
}

#[test]
fn c3c_hooks_and_symbols_exist() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let unify = std::fs::read_to_string(root.join("crates/syx-driver/src/mir_unify.rs"))
        .expect("mir_unify.rs");
    assert!(
        unify.contains("MirFnIndex::build") && unify.contains("verify_mir_cfg_floor"),
        "unify must build MirFnIndex after CFG floor"
    );
    // build after verify
    let v = unify.find("verify_mir_cfg_floor").expect("verify");
    let b = unify.find("MirFnIndex::build").expect("build");
    assert!(b > v, "MirFnIndex::build must follow verify_mir_cfg_floor");

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
    assert!(body.contains("MirFnIndex::build_from_functions"));
    assert!(body.contains("verify_mir_cfg_floor"));
    assert!(
        codegen.contains("MIR003") && codegen.contains("unknown callee"),
        "resolve_fn_index must emit MIR003"
    );

    let api = std::fs::read_to_string(root.join("crates/syx-mir/src/mir_fn_index.rs"))
        .expect("mir_fn_index.rs");
    assert!(api.contains("pub struct FuncId") && api.contains("pub struct MirFnIndex"));
}
