mod mir_text;
pub use mir_text::format_mir_module;

mod mir_types;
pub use mir_types::{
    is_suite1_pilot_mir_op, mir_module_aot_export_root_refs, MirBlock, MirFunction, MirInstr,
    MirModule, MirOp,
};

mod mir_codec;
mod mir_diag;
pub use mir_codec::{
    aot_embed_execution_omitted, decode_module, decode_module_dual_blob_package,
    decode_module_metadata_sidecar, dual_blob_package_execution_bytes,
    dual_blob_package_sidecar_bytes, encode_module, encode_module_aot_embed_blob,
    encode_module_aot_embed_execution_bytes, encode_module_dual_blob_package,
    encode_module_metadata_sidecar, is_dual_blob_package_bytes, mir_module_blob_stats,
    mir_module_from_sidecar_fn_stubs, mir_module_metadata_sidecar_view,
    peel_dual_blob_package_execution, peel_dual_blob_package_sidecar, MirModuleBlobStats,
    MirModuleDualBlobPackage, MirModuleMetadataSidecarView,
};

mod mir_aot;
pub use mir_aot::{
    aot_blob_size_report, aot_codec_fixture_main_with_const_strs,
    aot_codec_fixture_main_with_debug_locals, aot_size_fixture_call_indirect_full_emit,
    aot_size_fixture_main_with_orphan_body, build_fn_index_remap, compact_module_for_aot_blob,
    compact_module_for_aot_blob_with_roots, encode_module_compact_for_aot, encode_module_for_aot,
    llvm_emit_function_names, llvm_emit_function_names_with_roots,
    module_with_debug_locals_stripped, module_with_execution_string_table_stripped,
    module_with_native_instructions_omitted, reachable_fn_names, reachable_fn_names_from_roots,
    remap_fn_indices_in_module, shaken_module_for_aot_blob, shaken_module_for_aot_blob_with_roots,
    AotBlobSizeReport, FN_INDEX_DELETED,
};

mod mir_opt;
pub use mir_opt::{
    const_fold, dead_code_elim, escape_analysis, inline_small_host_functions,
    mir_opt_v10_prod_default_enabled, mir_opt_v11_stability_enabled,
    mir_opt_v12_prod_hardening_enabled, mir_opt_v13_incr_barrier_enabled,
    mir_opt_v14_jit_barrier_enabled, mir_opt_v15_jit_barrier_enabled,
    mir_opt_v16_aot_barrier_enabled, mir_opt_v17_aot_barrier_enabled,
    mir_opt_v18_aot_barrier_enabled, mir_opt_v19_aot_barrier_enabled,
    mir_opt_v20_aot_barrier_enabled, mir_opt_v21_aot_barrier_enabled,
    mir_opt_v22_aot_barrier_enabled, mir_opt_v23_aot_barrier_enabled,
    mir_opt_v24_aot_barrier_enabled, mir_opt_v25_aot_barrier_enabled, mir_opt_v8_enabled,
    mir_opt_v9_default_enabled, optimize, optimize_mir_module, remove_unreachable_blocks,
};

mod mir_module_index;
use mir_module_index::extension_init_literal_fields;

mod mir_lower_ctx;
mod mir_lower_expr_call_ident;
mod mir_lower_expr_literal;

mod mir_lower_expr_call_member;

mod mir_collect_closure_free_vars;
mod mir_lower_expr_array;
mod mir_lower_expr_assign;
mod mir_lower_expr_await;
mod mir_lower_expr_binary;
mod mir_lower_expr_borrow;
mod mir_lower_expr_call;
mod mir_lower_expr_cast;
mod mir_lower_expr_closure;
mod mir_lower_expr_compound_assign;
mod mir_lower_expr_group;
mod mir_lower_expr_ident;
mod mir_lower_expr_index;
mod mir_lower_expr_index_assign;
mod mir_lower_expr_is;
mod mir_lower_expr_keypath;
mod mir_lower_expr_macro_invoke;
mod mir_lower_expr_member;
mod mir_lower_expr_member_assign;
mod mir_lower_expr_optional_cast;
mod mir_lower_expr_range;
mod mir_lower_expr_super;
mod mir_lower_expr_task_block;
mod mir_lower_expr_ternary;
mod mir_lower_expr_tuple;
mod mir_lower_expr_type_self;
mod mir_lower_expr_unary;

mod mir_lower_switch_pattern;

mod mir_lower_stmt;
use mir_lower_stmt::lower_stmt;

mod mir_lower_shells;
pub use mir_lower_shells::{
    lower_dataclass_shells, lower_enum_shells, lower_finalclass_shells, lower_varglobal_inits,
    merge_mir_functions, merge_mir_shell_functions, prefix_user_dep_mir_batch,
};
mod mir_lower_type_members;
use mir_lower_type_members::lower_type_members;

mod mir_host_cast_helpers;
mod mir_lower_class_init;
mod mir_lower_expr;
mod mir_lower_function;
mod mir_lower_module;
mod mir_lower_support;
mod mir_member_call_dispatch;
mod mir_simd_helpers;
mod mir_type_infer_helpers;

pub(crate) use mir_host_cast_helpers::{
    binary_numeric_kind, collection_method_mir_op, emit_float_to_int_numeric_cast_if_needed,
    emit_hosted_collection_call, emit_integer_narrow_cast_if_needed,
    emit_integer_numeric_cast_if_needed, extension_call_op, is_hosted_collection_method,
    resolve_host_call, unary_numeric_kind,
};
pub(crate) use mir_type_infer_helpers::{
    binop_operator_symbol, class_name_from_mangled, expr_is_array_like, expr_is_dict_like,
    expr_is_float32_like, expr_is_float64_like, expr_is_set_like, expr_is_string_like,
    extension_or_hosted_return_type, infer_expr_init_type, infer_expr_type_name,
    infer_for_in_elem_type, infer_generic_arg_type_name, infer_object_type_name,
    infer_service_resolve_type_name, infer_static_method_type_args, instance_self_mut_ref_method,
    is_platform_value_type, is_static_type_member_call, type_name_dispatch_base, type_name_is_set,
};

pub(crate) use mir_lower_function::{lower_function_with_params, register_param_local_type};
pub(crate) use mir_lower_support::{
    expr_is_opaque_operand, flatten_blocks, keypath_from_callee, mir_local_names_for_slots,
    mir_local_types_for_slots,
};
pub(crate) use mir_member_call_dispatch::{
    classify_member_call, expr_as_i32_literal, method_id_mir_op, MemberCallKind,
};
pub(crate) use mir_simd_helpers::{
    host_simd_lowers_own_args, infer_expr_simd_type, infer_simd_element_from_array_arg,
    infer_simd_element_from_reduce_arg, member_host_simd_lowers_own_args,
    simd_member_lanes_element, simd_reduce_add_shape,
};

pub(crate) use mir_lower_expr::lower_expr;
pub use mir_lower_module::{
    extension_methods_for_module, lower_module, lower_module_with_extra_extensions,
    lower_module_with_extra_extensions_and_kinds,
    lower_module_with_extra_extensions_kinds_and_source,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_aot::aot_size_fixture_fn;
    use crate::mir_module_index::ModuleIndex;
    use crate::mir_opt::{effect_aware_licm_block_with_alias_barriers, fold_function};
    use smol_str::SmolStr;
    use syx_ast::Decl;
    use syx_diagnostics::SourceFile;
    use syx_parser::Parser;
    use syx_semantic::{type_id_to_name, SemanticAnalyzer, TypeId};
    use syx_stdlib::PLATFORM_INT;

    #[test]
    fn collection_type_params_survive_mir_names_and_dict_keys_values() {
        let source = r#"func main() {
  let d: Dictionary<String, Int> = ["a": 1]
  let ks = d.keys()
  let vs = d.values()
  let s: Set<Int> = [1]
}
"#;
        let module = Parser::parse(SourceFile::new("t.sy", source)).unwrap();
        let sema = SemanticAnalyzer::analyze_collecting(&module);
        assert!(sema.errors.is_empty(), "semantic errors: {:?}", sema.errors);
        match sema.all_bindings.get("ks").expect("ks") {
            TypeId::Array(elem) => {
                assert!(
                    !matches!(elem.as_ref(), TypeId::Unknown),
                    "keys erased to Array<Unknown>"
                );
                assert_eq!(type_id_to_name(elem).as_str(), "String");
            }
            other => panic!("ks type {}", type_id_to_name(other)),
        }
        match sema.all_bindings.get("vs").expect("vs") {
            TypeId::Array(elem) => {
                assert!(
                    matches!(elem.as_ref(), TypeId::Numeric(_)),
                    "values element erased: {}",
                    type_id_to_name(elem)
                );
            }
            other => panic!("vs type {}", type_id_to_name(other)),
        }

        let mir = lower_module(&module).unwrap();
        let main = mir
            .functions
            .iter()
            .find(|f| f.name == "main")
            .expect("main");
        assert!(
            main.local_types.iter().any(|t| t.as_str() == "Set<Int>"),
            "local types: {:?}",
            main.local_types
        );
        assert!(
            main.local_types
                .iter()
                .any(|t| t.as_str() == "Dictionary<String, Int>"),
            "local types: {:?}",
            main.local_types
        );
        let builds_set = main.blocks.iter().any(|b| {
            b.instructions
                .iter()
                .any(|i| matches!(i.op, MirOp::SetBuild(_)))
        });
        assert!(builds_set, "Set<Int> literal must still SetBuild");
    }

    #[test]
    fn empty_set_ctor_lowers_to_set_build_zero() {
        let source = r#"func main() {
  let s: Set<Int> = Set()
  s.insert(1)
}
"#;
        let module = Parser::parse(SourceFile::new("t.sy", source)).unwrap();
        let mir = lower_module(&module).unwrap();
        let main = mir
            .functions
            .iter()
            .find(|f| f.name == "main")
            .expect("main");
        let set_build_zero = main.blocks.iter().any(|b| {
            b.instructions
                .iter()
                .any(|i| matches!(i.op, MirOp::SetBuild(0)))
        });
        let calls_set_ctor = main.blocks.iter().any(|b| {
            b.instructions.iter().any(|i| {
                matches!(
                    &i.op,
                    MirOp::Call(name) if name.as_str() == "Set_init" || name.as_str() == "Set"
                )
            })
        });
        assert!(set_build_zero, "Set() must SetBuild(0); mir={main:?}");
        assert!(
            !calls_set_ctor,
            "Set() must not Call Set/Set_init; mir={main:?}"
        );
    }

    #[test]
    fn format_mir_module_greet_extension() {
        let source = SourceFile::new(
            "t.sy",
            r#"class Greeter {
    var name: String
    init(name: String) { self.name = name }
}
extension Greeter {
    func greet() { print("hi") }
}
func main() {
    let g = Greeter(name: "x")
    g.greet()
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let text = format_mir_module(&mir);
        assert!(text.contains("Greeter_greet"));
        assert!(text.contains("Call Greeter_greet"));
    }

    #[test]
    fn lowers_extension_methods() {
        let source = SourceFile::new(
            "t.sy",
            r#"class Greeter {
    var name: String
    init(name: String) { self.name = name }
    func hello() { print(self.name) }
}
extension Greeter {
    func greet() { print("Hello, \(self.name)!") }
}
func main() {
    let g = Greeter(name: "Greeter")
    g.hello()
    g.greet()
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let greet = mir
            .functions
            .iter()
            .find(|f| f.name == "Greeter_greet")
            .expect("Greeter_greet");
        assert!(
            greet
                .blocks
                .iter()
                .any(|b| { b.instructions.iter().any(|i| matches!(i.op, MirOp::Print)) }),
            "greet body: {:?}",
            greet.blocks
        );
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        let calls_greet = main.blocks.iter().any(|b| {
            b.instructions
                .iter()
                .any(|i| matches!(&i.op, MirOp::Call(name) if name == "Greeter_greet"))
        });
        assert!(calls_greet, "main blocks: {:?}", main.blocks);
    }

    #[test]
    fn resolves_jump_targets_to_instruction_pcs() {
        let blocks = vec![
            MirBlock {
                id: 0,
                instructions: vec![
                    MirInstr {
                        dest: None,
                        op: MirOp::JumpIfNil(2),
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(3),
                        span: None,
                    },
                ],
                successors: vec![],
            },
            MirBlock {
                id: 2,
                instructions: vec![MirInstr {
                    dest: Some(0),
                    op: MirOp::ConstI32(99),
                    span: None,
                }],
                successors: vec![],
            },
            MirBlock {
                id: 3,
                instructions: vec![MirInstr {
                    dest: None,
                    op: MirOp::Return,
                    span: None,
                }],
                successors: vec![],
            },
        ];
        let flat = flatten_blocks(&blocks);
        assert_eq!(flat.len(), 4);
        assert!(matches!(flat[0].op, MirOp::JumpIfNil(2)));
        assert!(matches!(flat[1].op, MirOp::Jump(3)));
    }

    #[test]
    fn lowers_nil_coalesce() {
        let source = include_str!("../../syx-driver/tests/fixtures/lang/nil_coalesce.sy");
        let module = Parser::parse(SourceFile::new("t.sy", source)).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        assert!(main
            .instructions
            .iter()
            .any(|i| matches!(i.op, MirOp::JumpIfNil(_))));
        // StoreLocal for `y = x ?? 42` must not sit in the Jump-dead gap.
        let flat = &main.instructions;
        let jump_if_nil = flat
            .iter()
            .position(|i| matches!(i.op, MirOp::JumpIfNil(_)))
            .expect("JumpIfNil");
        if let MirOp::Jump(end) = flat[jump_if_nil + 1].op {
            assert!(
                matches!(flat[end as usize].op, MirOp::StoreLocal(_)),
                "?? result StoreLocal should be at end block, got {:?}",
                flat[end as usize].op
            );
        }
    }

    #[test]
    fn while_body_let_stores_in_loop_block() {
        let source = SourceFile::new(
            "t.sy",
            r#"func main() {
    var i = 0
    while i < 3 {
        let x = i
        print(x)
        i = i + 1
    }
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        // StoreLocal for `let x` must live in a reachable body block, not after
        // the entry Jump into the loop header (that was the while+let nil bug).
        let entry = &main.blocks[0];
        let dead_after_jump = entry
            .instructions
            .windows(2)
            .any(|w| matches!(w[0].op, MirOp::Jump(_)) && matches!(w[1].op, MirOp::StoreLocal(_)));
        assert!(
            !dead_after_jump,
            "let StoreLocal must not sit after Jump in entry; blocks={:?}",
            main.blocks
                .iter()
                .map(|b| {
                    b.instructions
                        .iter()
                        .map(|i| format!("{:?}", i.op))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        );
        assert!(
            main.blocks.iter().any(|b| {
                b.instructions
                    .iter()
                    .any(|i| matches!(i.op, MirOp::StoreLocal(1)))
            }),
            "expected StoreLocal for loop-local x"
        );
    }

    #[test]
    fn while_if_else_emits_jump_to_then() {
        // While pre-allocates exit between body and if-then; without an explicit
        // Jump(then) after JumpIfFalse, a true condition falls into exit and
        // drops `j = j + 1`.
        let source = SourceFile::new(
            "t.sy",
            r#"func main() {
    var j = 0
    var total = 0
    while j < 5 {
        if j < 2 {
            total = total + 1
        } else if j < 4 {
            total = total + 10
        } else {
            total = total + 100
        }
        j = j + 1
    }
    print(total)
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        let has_jump_after_jif = main.blocks.iter().any(|b| {
            b.instructions.windows(2).any(|w| {
                matches!(w[0].op, MirOp::JumpIfFalse(_)) && matches!(w[1].op, MirOp::Jump(_))
            })
        });
        assert!(
            has_jump_after_jif,
            "If inside while must Jump(then) after JumpIfFalse; blocks={:?}",
            main.blocks
                .iter()
                .map(|b| {
                    b.instructions
                        .iter()
                        .map(|i| format!("{:?}", i.op))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn call_returning_array_keeps_array_method_host() {
        let source = SourceFile::new(
            "t.sy",
            r#"func make() -> Array<Int> {
    var a: Array<Int> = []
    return a
}
func main() {
    var msg = make()
    msg.append(1)
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        let has_append_host = main
            .instructions
            .iter()
            .any(|i| matches!(&i.op, MirOp::CallHost(_)));
        let has_object_get_append = main.instructions.windows(2).any(|w| {
            matches!(&w[0].op, MirOp::ConstStr(s) if s == "append")
                && matches!(w[1].op, MirOp::ObjectGet)
        });
        assert!(
            has_append_host && !has_object_get_append,
            "append on Array-returning call must be CallHost, not ObjectGet; ins={:?}",
            main.instructions
                .iter()
                .map(|i| format!("{:?}", i.op))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn lowers_if_let() {
        let source = SourceFile::new(
            "t.sy",
            r#"func main() {
    let opt: Int? = 7
    if let x = opt {
        print(x)
    } else {
        print(0)
    }
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        assert!(
            main.instructions
                .iter()
                .any(|i| matches!(i.op, MirOp::JumpIfNil(_))),
            "if let must lower to JumpIfNil; got {:?}",
            main.instructions.iter().map(|i| &i.op).collect::<Vec<_>>()
        );
        assert!(
            main.instructions
                .iter()
                .filter(|i| matches!(i.op, MirOp::Print))
                .count()
                >= 2,
            "both if-let branches should remain"
        );
    }

    #[test]
    fn class_iterator_witness_in_init() {
        let source = SourceFile::new(
            "t.sy",
            r#"class Counter: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? {
        self.n = self.n + 1
        if self.n > 3 { return nil }
        return self.n
    }
}
func main() { print(0) }"#,
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let init = mir
            .functions
            .iter()
            .find(|f| f.name == "Counter_init")
            .expect("init");
        // R-26-03: witness fn index is ConstFnIndex (not ConstI32) after shell merge.
        let has_witness = init.blocks.iter().any(|b| {
            b.instructions.windows(2).any(|w| {
                matches!(&w[0].op, MirOp::ConstFnIndex(_))
                    && matches!(&w[1].op, MirOp::ConstStr(s) if s == "__witness_Iterator_next")
            }) || b
                .instructions
                .iter()
                .any(|i| matches!(&i.op, MirOp::ConstStr(s) if s == "__witness_Iterator_next"))
        });
        assert!(has_witness, "Counter_init missing Iterator witness");
    }

    #[test]
    fn lowers_for_in_with_exit_block() {
        let source = SourceFile::new(
            "t.sy",
            r#"func main() {
    for item in "a,b,c".split(",") {
        print(item)
    }
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        assert!(main
            .blocks
            .iter()
            .any(|b| { b.instructions.iter().any(|i| matches!(i.op, MirOp::Return)) }));
        let exit = main
            .blocks
            .iter()
            .find(|b| b.instructions.iter().any(|i| matches!(i.op, MirOp::Return)));
        assert!(exit.is_some());
    }

    #[test]
    fn lowers_and_optimizes() {
        let source = SourceFile::new("t.sy", "func main() { return 1 + 2 }");
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        assert!(!mir.functions.is_empty());
        assert!(!mir.functions[0].blocks.is_empty());
    }

    #[test]
    fn folds_addition() {
        let mut instrs = vec![
            MirInstr {
                dest: Some(0),
                op: MirOp::ConstI32(1),
                span: None,
            },
            MirInstr {
                dest: Some(1),
                op: MirOp::ConstI32(2),
                span: None,
            },
            MirInstr {
                dest: Some(0),
                op: MirOp::Add {
                    kind: PLATFORM_INT.to_byte(),
                },
                span: None,
            },
        ];
        fold_function(&mut instrs);
        assert_eq!(instrs.len(), 1);
        assert_eq!(instrs[0].op, MirOp::ConstI32(3));
    }

    #[test]
    fn lowers_generic_instance_param() {
        use syx_mono::monomorphize_module;
        use syx_semantic::SemanticAnalyzer;
        let source = SourceFile::new(
            "t.sy",
            "func id<T>(x: Int) -> Int { return x }\nfunc main() { print(id(x: 42)) }",
        );
        let module = Parser::parse(source).unwrap();
        let semantic = SemanticAnalyzer::analyze(&module).unwrap();
        let mono = monomorphize_module(&module, &semantic.instantiations);
        let id_decl = mono
            .decls
            .iter()
            .find(|d| matches!(d, Decl::Function { name, .. } if name.as_str().starts_with("id_")))
            .expect("mono id");
        if let Decl::Function { params, .. } = id_decl {
            assert_eq!(params.len(), 1, "expected one param");
            assert_eq!(params[0].name.as_str(), "x");
        }
        let mir = lower_module(&mono).expect("lower generic instance");
        let id_fn = mir
            .functions
            .iter()
            .find(|f| f.name.as_str().starts_with("id_"))
            .expect("monomorphized id");
        assert!(
            id_fn
                .instructions
                .iter()
                .any(|i| matches!(i.op, MirOp::LoadLocal(_))),
            "expected LoadLocal for param x in {:?}",
            id_fn.instructions
        );
    }

    #[test]
    fn lowers_dataclass_shells_for_native() {
        let source = SourceFile::new(
            "t.sy",
            "data class Point(x: Int, y: Int)\nfunc main() { let p = Point(1, 2) }",
        );
        let module = Parser::parse(source).unwrap();
        let shells = lower_dataclass_shells(&module);
        assert!(shells.iter().any(|f| f.name == "Point_init"));
        assert!(shells.iter().any(|f| f.name == "Point_copy"));
        assert!(shells.iter().any(|f| f.name == "Point_describe"));
    }

    #[test]
    fn lowers_struct_shells_for_native_w467() {
        let source = SourceFile::new(
            "t.sy",
            "struct Point {\n    x: Int\n    y: Int\n}\nfunc main() { let p = Point(1, 2) }",
        );
        let module = Parser::parse(source).unwrap();
        let shells = lower_dataclass_shells(&module);
        assert!(shells.iter().any(|f| f.name == "Point_init"));
        assert!(shells.iter().any(|f| f.name == "Point_copy"));
        assert!(shells.iter().any(|f| f.name == "Point_describe"));
        assert!(shells.iter().any(|f| f.name == "Point_hash_into"));
    }

    #[test]
    fn lowers_finalclass_shells_for_native_w468() {
        let source = SourceFile::new(
            "t.sy",
            "final class Shape {\n    point\n    circle(r: Int)\n    rect(w: Int, h: Int)\n}\nfunc main() { }",
        );
        let module = Parser::parse(source).unwrap();
        let shells = lower_finalclass_shells(&module);
        assert!(shells.iter().any(|f| f.name == "Shape_point_init"));
        assert!(shells.iter().any(|f| f.name == "Shape_circle_init"));
        assert!(shells.iter().any(|f| f.name == "Shape_rect_init"));
        let circle = shells
            .iter()
            .find(|f| f.name == "Shape_circle_init")
            .expect("circle init");
        assert_eq!(circle.param_count, 1);
        let rect = shells
            .iter()
            .find(|f| f.name == "Shape_rect_init")
            .expect("rect init");
        assert_eq!(rect.param_count, 2);
        let point = shells
            .iter()
            .find(|f| f.name == "Shape_point_init")
            .expect("point init");
        assert_eq!(point.param_count, 0);
    }

    #[test]
    fn lowers_varglobal_inits_combined_for_native_w469() {
        let source = SourceFile::new(
            "t.sy",
            "var a = 1\nvar b\nvar c = 2\nfunc main() { print(a) }",
        );
        let module = Parser::parse(source).unwrap();
        let inits = lower_varglobal_inits(&module).unwrap();
        assert_eq!(inits.len(), 1);
        assert_eq!(inits[0].name, "__init_global");
        assert_eq!(inits[0].param_count, 0);
        let ops: Vec<_> = inits[0].instructions.iter().map(|i| &i.op).collect();
        assert!(
            ops.iter().any(|op| matches!(op, MirOp::StoreGlobal(0))),
            "slot 0 for first VarGlobal with init: {ops:?}"
        );
        assert!(
            ops.iter().any(|op| matches!(op, MirOp::StoreGlobal(2))),
            "slot 2 for third VarGlobal (b has no init): {ops:?}"
        );
        assert!(
            !ops.iter().any(|op| matches!(op, MirOp::StoreGlobal(1))),
            "slot 1 (b) must not StoreGlobal: {ops:?}"
        );
        let no_init = Parser::parse(SourceFile::new("t.sy", "var x\nfunc main() { }")).unwrap();
        assert!(lower_varglobal_inits(&no_init).unwrap().is_empty());
    }

    /// W470 / ADR-814: Extension methods lower via `lower_type_members` with
    /// instance self / static / builtin target parity (MIR authoritative path).
    #[test]
    fn lowers_extension_methods_parity_for_native_w470() {
        let source = SourceFile::new(
            "t.sy",
            r#"
class Box {
    var n: Int
    init(n: Int) { self.n = n }
}
extension Box {
    func doubled() -> Int { return self.n * 2 }
    static func zero() -> Int { return 0 }
}
extension Int {
    func plusOne() -> Int { return self + 1 }
}
func main() { }
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let doubled = mir
            .functions
            .iter()
            .find(|f| f.name == "Box_doubled")
            .expect("Box_doubled");
        assert_eq!(doubled.param_count, 1, "instance extension injects self");
        assert!(
            doubled.blocks.iter().any(|b| !b.instructions.is_empty()),
            "Box_doubled body must be non-empty"
        );
        let zero = mir
            .functions
            .iter()
            .find(|f| f.name == "Box_zero")
            .expect("Box_zero");
        assert_eq!(zero.param_count, 0, "static extension has no self");
        assert!(
            zero.blocks.iter().any(|b| !b.instructions.is_empty()),
            "Box_zero body must be non-empty"
        );
        let plus = mir
            .functions
            .iter()
            .find(|f| f.name == "Int_plusOne")
            .expect("Int_plusOne");
        assert_eq!(plus.param_count, 1, "builtin target extension injects self");
        assert!(
            plus.blocks.iter().any(|b| !b.instructions.is_empty()),
            "Int_plusOne body must be non-empty"
        );
    }

    /// W471 / ADR-815: Class init/method/deinit/nested lower via MIR
    /// `lower_type_members` / `lower_class_init` (Class-only migrate path).
    #[test]
    fn s1_i7_int_extension_call_on_int64_literal() {
        let source = SourceFile::new(
            "t.sy",
            r#"
extension Int {
    func doubled() -> Int { return self + self }
}
func main() {
    print(3.doubled())
}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let idx = ModuleIndex::from_module(&module);
        assert!(
            idx.extension_methods
                .contains(&(SmolStr::new("Int64"), SmolStr::new("doubled"))),
            "Int64,doubled missing: {:?}",
            idx.extension_methods
                .iter()
                .filter(|(t, _)| t.as_str().starts_with("Int"))
                .collect::<Vec<_>>()
        );
        let mir = lower_module(&module).unwrap();
        let main = mir
            .functions
            .iter()
            .find(|f| f.name == "main")
            .expect("main");
        let ops: Vec<_> = main.blocks[0]
            .instructions
            .iter()
            .map(|i| format!("{:?}", i.op))
            .collect();
        assert!(
            ops.iter().any(|op| op.contains("Int_doubled")),
            "expected Call Int_doubled in main, got {ops:?}"
        );
    }

    #[test]
    fn s1_i7_extension_point_convenience_init_lowers() {
        let source = SourceFile::new(
            "t.sy",
            r#"
struct Point {
    var x: Int
    var y: Int
}
extension Point {
    init() {
        self.init(x: 0, y: 0)
    }
}
func main() {
    let p = Point()
    print(p.x)
}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let idx = ModuleIndex::from_module(&module);
        let fields = idx
            .dataclass_fields
            .get(&SmolStr::new("Point"))
            .cloned()
            .unwrap();
        assert_eq!(fields, vec![SmolStr::new("x"), SmolStr::new("y")]);
        assert!(idx.dataclass_names.contains(&SmolStr::new("Point")));
        assert_eq!(
            idx.dataclass_field_default_i64
                .get(&(SmolStr::new("Point"), SmolStr::new("x")))
                .copied(),
            Some(0)
        );
        let mir = lower_module(&module).unwrap();
        // Prefer 0-arg convenience init with literal fields.
        let init = mir
            .functions
            .iter()
            .rev()
            .find(|f| f.name == "Point_init")
            .expect("Point_init");
        assert_eq!(init.param_count, 0, "convenience init should be 0-arg");
        let init_ops: Vec<_> = init.blocks[0]
            .instructions
            .iter()
            .map(|i| format!("{:?}", i.op))
            .collect();
        assert!(
            init_ops.iter().any(|o| o.contains("ConstNumeric")),
            "expected literal fields in convenience init: {init_ops:?}"
        );
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        let ops: Vec<_> = main.blocks[0]
            .instructions
            .iter()
            .map(|i| format!("{:?}", i.op))
            .collect();
        assert!(
            ops.iter().any(|o| o.contains("Point_init")),
            "main should Call Point_init: {ops:?}"
        );
    }

    #[test]
    fn lowers_class_members_parity_for_native_w471() {
        let source = SourceFile::new(
            "t.sy",
            r#"
class Point {
    var x: Int
    init(x: Int) { self.x = x }
    func doubled() -> Int { return self.x * 2 }
    static func origin() -> Int { return 0 }
    deinit { }
    class Nested {
        var y: Int
        init(y: Int) { self.y = y }
        func get() -> Int { return self.y }
    }
}
func main() { }
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let init = mir
            .functions
            .iter()
            .find(|f| f.name == "Point_init")
            .expect("Point_init");
        assert!(init.param_count >= 1);
        assert!(
            init.blocks.iter().any(|b| !b.instructions.is_empty()) || !init.instructions.is_empty(),
            "Point_init body must be non-empty"
        );
        let doubled = mir
            .functions
            .iter()
            .find(|f| f.name == "Point_doubled")
            .expect("Point_doubled");
        assert_eq!(doubled.param_count, 1);
        assert!(doubled.blocks.iter().any(|b| !b.instructions.is_empty()));
        let origin = mir
            .functions
            .iter()
            .find(|f| f.name == "Point_origin")
            .expect("Point_origin");
        assert_eq!(origin.param_count, 0, "static method has no self");
        assert!(origin.blocks.iter().any(|b| !b.instructions.is_empty()));
        let deinit = mir
            .functions
            .iter()
            .find(|f| f.name == "Point_deinit")
            .expect("Point_deinit");
        assert!(deinit.param_count >= 1);
        let nested_init = mir
            .functions
            .iter()
            .find(|f| f.name == "Point_Nested_init" || f.name == "Nested_init")
            .expect("nested class init");
        assert!(
            nested_init
                .blocks
                .iter()
                .any(|b| !b.instructions.is_empty())
                || !nested_init.instructions.is_empty(),
            "nested init body must be non-empty"
        );
    }

    /// W472 / ADR-816: Actor init (ActorRegister) / method / static / deinit
    /// via MIR `lower_type_members(..., is_actor=true)` / `lower_class_init`.
    #[test]
    fn lowers_actor_members_parity_for_native_w472() {
        let source = SourceFile::new(
            "t.sy",
            r#"
actor Counter {
    var n: Int
    init(n: Int) { self.n = n }
    func bump() -> Int { return self.n + 1 }
    static func zero() -> Int { return 0 }
    deinit { }
}
func main() { }
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let init = mir
            .functions
            .iter()
            .find(|f| f.name == "Counter_init")
            .expect("Counter_init");
        assert!(init.param_count >= 1);
        let has_register = init.blocks.iter().any(|b| {
            b.instructions
                .iter()
                .any(|i| matches!(i.op, MirOp::ActorRegister))
        }) || init
            .instructions
            .iter()
            .any(|i| matches!(i.op, MirOp::ActorRegister));
        assert!(
            has_register,
            "Counter_init must emit ActorRegister (is_actor=true)"
        );
        let bump = mir
            .functions
            .iter()
            .find(|f| f.name == "Counter_bump")
            .expect("Counter_bump");
        assert_eq!(bump.param_count, 1);
        assert!(bump.blocks.iter().any(|b| !b.instructions.is_empty()));
        // Typed self for actor methods (ADR-631).
        assert!(
            bump.local_types.iter().any(|t| t.as_str() == "Counter") || bump.param_count == 1,
            "actor method should carry typed self path"
        );
        let zero = mir
            .functions
            .iter()
            .find(|f| f.name == "Counter_zero")
            .expect("Counter_zero");
        assert_eq!(zero.param_count, 0, "static method has no self");
        assert!(zero.blocks.iter().any(|b| !b.instructions.is_empty()));
        let deinit = mir
            .functions
            .iter()
            .find(|f| f.name == "Counter_deinit")
            .expect("Counter_deinit");
        assert!(deinit.param_count >= 1);
    }

    #[test]
    fn lowers_dataclass_main_without_stack_overflow() {
        let source = include_str!("../../syx-driver/tests/fixtures/lang/dataclass_synth.sy");
        let module = Parser::parse(SourceFile::new("t.sy", source)).unwrap();
        let mut mir = lower_module(&module).unwrap();
        optimize(&mut mir);
        assert!(mir.functions.iter().any(|f| f.name == "main"));
    }

    #[test]
    fn enum_assoc_lowers_shells_and_switch() {
        let source = include_str!("../../syx-driver/tests/fixtures/lang/enum_associated.sy");
        let module = Parser::parse(SourceFile::new("t.sy", source)).unwrap();
        let mut mir = lower_module(&module).unwrap();
        merge_mir_functions(&mut mir, lower_enum_shells(&module));
        assert!(mir
            .functions
            .iter()
            .any(|f| f.name == "NetworkResult_success_init"));
        assert!(mir
            .functions
            .iter()
            .any(|f| f.name == "NetworkResult_failure_init"));
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        assert!(main.blocks.iter().any(|b| {
            b.instructions
                .iter()
                .any(|i| matches!(i.op, MirOp::JumpIfFalse(_)))
        }));
    }

    #[test]
    fn hof_closure_roundtrip_and_index() {
        let source = SourceFile::new(
            "t.sy",
            r#"func main() {
    let nums = [1, 2, 3, 4]
    let evens = nums.filter({ |x| x % 2 == 0 })
    print(evens.len())
    return 0
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let decoded = decode_module(&encode_module(&mir)).unwrap();
        assert_eq!(decoded.functions.len(), mir.functions.len());
        let closure = mir
            .functions
            .iter()
            .find(|f| f.name.starts_with("__mir_closure_"))
            .expect("closure fn");
        let idx = mir
            .functions
            .iter()
            .position(|f| f.name == closure.name)
            .unwrap();
        assert!(idx > 0, "closure should not be main");
        let bytes = encode_module(&mir);
        assert!(!bytes.is_empty(), "encode_module must not be empty");
        assert_eq!(bytes.len(), encode_module(&decoded).len());
    }

    #[test]
    fn mir_closure_names_unique_across_functions() {
        let source = SourceFile::new(
            "t.sy",
            r#"
func a() {
    let _ = [1].map({ (x: Int) -> Int in return x * 2 })
}
func b() {
    let _ = [1, 2, 3].reduce(0, { (acc: Int, x: Int) -> Int in return acc + x })
}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let names: Vec<_> = mir
            .functions
            .iter()
            .filter(|f| f.name.starts_with("__mir_closure_"))
            .map(|f| f.name.as_str())
            .collect();
        assert_eq!(
            names.len(),
            2,
            "expected one closure per HOF site, got {names:?}"
        );
        assert_ne!(
            names[0], names[1],
            "cross-function closure names must not collide"
        );
        let make: Vec<_> = mir
            .functions
            .iter()
            .flat_map(|f| f.instructions.iter())
            .filter_map(|i| match &i.op {
                MirOp::MakeClosure(n, _) => Some(n.as_str()),
                _ => None,
            })
            .collect();
        assert!(make.contains(&names[0]));
        assert!(make.contains(&names[1]));
    }

    #[test]
    fn escape_analysis_marks_capturing_closures_non_inlineable() {
        let source = SourceFile::new(
            "t.sy",
            r#"func main() {
    let base = 1
    let nums = [1, 2]
    let mapped = nums.map({ |x| x + base })
    print(mapped[0])
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let mut mir = lower_module(&module).unwrap();
        let escaping = escape_analysis(&mir);
        assert!(
            mir.functions
                .iter()
                .any(|f| f.name.starts_with("__mir_closure_") && escaping.contains(&f.name)),
            "capturing closure should be marked escaping"
        );
        optimize(&mut mir);
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        assert!(
            main.instructions
                .iter()
                .chain(main.blocks.iter().flat_map(|b| &b.instructions))
                .any(|i| matches!(i.op, MirOp::MakeClosure(_, _))),
            "MakeClosure call sites must remain after optimize"
        );
    }

    #[test]
    fn mir_opt_v9_nightly_default_enables_v9_flag() {
        std::env::remove_var("SYX_MIR_OPT_V9");
        std::env::set_var("SYX_NIGHTLY", "1");
        assert!(mir_opt_v9_default_enabled());
        std::env::remove_var("SYX_NIGHTLY");
    }

    #[test]
    fn mir_opt_v10_prod_default_enables_v8_without_nightly() {
        std::env::remove_var("SYX_MIR_OPT_V8");
        std::env::remove_var("SYX_MIR_OPT_V9");
        std::env::remove_var("SYX_MIR_OPT_V10");
        std::env::remove_var("SYX_NIGHTLY");
        assert!(mir_opt_v10_prod_default_enabled());
        assert!(mir_opt_v8_enabled());
    }

    #[test]
    fn mir_opt_v10_explicit_opt_in() {
        std::env::set_var("SYX_MIR_OPT_V8", "0");
        std::env::set_var("SYX_MIR_OPT_V10", "1");
        assert!(mir_opt_v10_prod_default_enabled());
        assert!(!mir_opt_v8_enabled());
        std::env::remove_var("SYX_MIR_OPT_V8");
        std::env::remove_var("SYX_MIR_OPT_V10");
    }

    #[test]
    fn mir_opt_v11_stability_aliases_v10_prod_default() {
        std::env::remove_var("SYX_MIR_OPT_V8");
        std::env::remove_var("SYX_MIR_OPT_V10");
        assert!(mir_opt_v11_stability_enabled());
        assert_eq!(
            mir_opt_v11_stability_enabled(),
            mir_opt_v10_prod_default_enabled()
        );
    }

    #[test]
    fn mir_opt_v14_jit_barrier_aliases_v13_by_default() {
        std::env::remove_var("SYX_MIR_OPT_V14");
        std::env::remove_var("SYX_MIR_OPT_V13");
        assert!(mir_opt_v14_jit_barrier_enabled());
        assert!(mir_opt_v13_incr_barrier_enabled());
    }

    #[test]
    fn mir_opt_v13_incr_barrier_aliases_v12_by_default() {
        std::env::remove_var("SYX_MIR_OPT_V13");
        std::env::remove_var("SYX_MIR_OPT_V12");
        std::env::remove_var("SYX_MIR_OPT_V8");
        assert!(mir_opt_v13_incr_barrier_enabled());
        assert!(mir_opt_v12_prod_hardening_enabled());
    }

    #[test]
    fn mir_opt_v12_prod_hardening_aliases_v11_by_default() {
        std::env::remove_var("SYX_MIR_OPT_V12");
        std::env::remove_var("SYX_MIR_OPT_V8");
        assert!(mir_opt_v12_prod_hardening_enabled());
        assert!(mir_opt_v11_stability_enabled());
    }

    #[test]
    fn mir_opt_v8_explicit_zero_disables_prod_default() {
        std::env::remove_var("SYX_MIR_OPT_V8");
        std::env::remove_var("SYX_MIR_OPT_V10");
        std::env::set_var("SYX_MIR_OPT_V8", "0");
        assert!(!mir_opt_v8_enabled());
        std::env::remove_var("SYX_MIR_OPT_V8");
    }

    #[test]
    fn alias_barrier_splits_store_load_segment() {
        let instrs = vec![
            MirInstr {
                dest: Some(10),
                op: MirOp::ConstStr("hoisted".into()),
                span: None,
            },
            MirInstr {
                dest: Some(0),
                op: MirOp::ConstI32(1),
                span: None,
            },
            MirInstr {
                dest: None,
                op: MirOp::StoreLocal(0),
                span: None,
            },
            MirInstr {
                dest: Some(1),
                op: MirOp::LoadLocal(0),
                span: None,
            },
            MirInstr {
                dest: None,
                op: MirOp::Print,
                span: None,
            },
        ];
        let out = effect_aware_licm_block_with_alias_barriers(instrs);
        let str_idx = out
            .iter()
            .position(|i| matches!(i.op, MirOp::ConstStr(_)))
            .expect("ConstStr");
        let store_idx = out
            .iter()
            .position(|i| matches!(i.op, MirOp::StoreLocal(_)))
            .expect("StoreLocal");
        let load_idx = out
            .iter()
            .position(|i| matches!(i.op, MirOp::LoadLocal(_)))
            .expect("LoadLocal");
        assert!(str_idx < store_idx, "ConstStr may hoist before StoreLocal");
        assert!(store_idx < load_idx, "StoreLocal before LoadLocal");
        assert!(load_idx < out.len(), "LoadLocal preserved before Print");
    }

    fn minimal_fn(name: &str, ops: Vec<MirOp>) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks: vec![],
            instructions: ops
                .into_iter()
                .map(|op| MirInstr {
                    dest: None,
                    op,
                    span: None,
                })
                .collect(),
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
        }
    }

    #[test]
    fn reachable_fn_names_includes_main_callee_excludes_orphan() {
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::Call(SmolStr::new("used"))]),
                minimal_fn("used", vec![MirOp::Return]),
                minimal_fn("orphan", vec![MirOp::Return]),
            ],
            ..Default::default()
        };
        let reachable = reachable_fn_names(&module);
        assert!(reachable.contains("main"));
        assert!(reachable.contains("used"));
        assert!(!reachable.contains("orphan"));
    }

    #[test]
    fn reachable_fn_names_follows_actor_call_and_call_super_edges() {
        let module = MirModule {
            functions: vec![
                minimal_fn(
                    "main",
                    vec![
                        MirOp::ActorCall(SmolStr::new("actor_fn")),
                        MirOp::CallSuper(SmolStr::new("super_init")),
                    ],
                ),
                minimal_fn("actor_fn", vec![MirOp::Return]),
                minimal_fn("super_init", vec![MirOp::Return]),
                minimal_fn("unreferenced", vec![MirOp::Return]),
            ],
            ..Default::default()
        };
        let reachable = reachable_fn_names(&module);
        assert!(reachable.contains("actor_fn"));
        assert!(reachable.contains("super_init"));
        assert!(!reachable.contains("unreferenced"));
    }

    #[test]
    fn r29_super_method_lowers_to_call_super() {
        // R-29: any super.method → CallSuper (not only super.init).
        let source = SourceFile::new(
            "t.sy",
            r#"class Base {
    func greet() { print("b") }
}
class Child: Base {
    override func greet() { print("c") }
    func up() { super.greet() }
}
func main() { Child().up() }
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let up = mir
            .functions
            .iter()
            .find(|f| f.name == "Child_up" || f.name.ends_with("_up"))
            .expect("Child_up");
        let has_call_super = up.blocks.iter().any(|b| {
            b.instructions
                .iter()
                .any(|i| matches!(&i.op, MirOp::CallSuper(n) if n == "Base_greet"))
        });
        assert!(
            has_call_super,
            "Child_up must CallSuper Base_greet; ops={:?}",
            up.blocks
                .iter()
                .flat_map(|b| b.instructions.iter().map(|i| format!("{:?}", i.op)))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn reachable_fn_names_empty_without_main() {
        let module = MirModule {
            functions: vec![minimal_fn("orphan_only", vec![MirOp::Return])],
            ..Default::default()
        };
        assert!(reachable_fn_names(&module).is_empty());
    }

    #[test]
    fn llvm_emit_function_names_excludes_orphan_when_reachable() {
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::Call(SmolStr::new("used"))]),
                minimal_fn("used", vec![MirOp::Return]),
                minimal_fn("orphan", vec![MirOp::Return]),
            ],
            ..Default::default()
        };
        let emit = llvm_emit_function_names(&module);
        assert!(emit.contains("main"));
        assert!(emit.contains("used"));
        assert!(!emit.contains("orphan"));
    }

    #[test]
    fn llvm_emit_function_names_falls_back_on_call_indirect() {
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::CallIndirect(1)]),
                minimal_fn("orphan", vec![MirOp::Return]),
            ],
            ..Default::default()
        };
        let emit = llvm_emit_function_names(&module);
        assert!(emit.contains("main"));
        assert!(emit.contains("orphan"));
    }

    #[test]
    fn shaken_module_for_aot_blob_stubs_orphan_preserves_fn_count() {
        let orphan_body = vec![
            MirOp::ConstI32(1),
            MirOp::ConstI32(2),
            MirOp::Add {
                kind: PLATFORM_INT.to_byte(),
            },
            MirOp::Return,
        ];
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::Call(SmolStr::new("used"))]),
                minimal_fn("used", vec![MirOp::Return]),
                minimal_fn("orphan", orphan_body),
            ],
            ..Default::default()
        };
        let shaken = shaken_module_for_aot_blob(&module);
        assert_eq!(shaken.functions.len(), module.functions.len());
        let orphan = shaken
            .functions
            .iter()
            .find(|f| f.name == "orphan")
            .unwrap();
        assert_eq!(orphan.blocks.len(), 0);
        assert_eq!(orphan.instructions.len(), 1);
        assert!(matches!(orphan.instructions[0].op, MirOp::Return));
    }

    #[test]
    fn shaken_module_for_aot_blob_keeps_eligible_body() {
        let used_body = vec![MirOp::ConstI32(7), MirOp::Return];
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::Call(SmolStr::new("used"))]),
                minimal_fn("used", used_body.clone()),
                minimal_fn("orphan", vec![MirOp::ConstI32(99), MirOp::Return]),
            ],
            ..Default::default()
        };
        let shaken = shaken_module_for_aot_blob(&module);
        let used = shaken.functions.iter().find(|f| f.name == "used").unwrap();
        assert_eq!(used.instructions.len(), used_body.len());
        assert!(matches!(used.instructions[0].op, MirOp::ConstI32(7)));
    }

    #[test]
    fn shaken_module_for_aot_blob_no_stub_on_call_indirect_fallback() {
        let orphan_body = vec![MirOp::ConstI32(99), MirOp::Return];
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::CallIndirect(1)]),
                minimal_fn("orphan", orphan_body.clone()),
            ],
            ..Default::default()
        };
        let shaken = shaken_module_for_aot_blob(&module);
        let orphan = shaken
            .functions
            .iter()
            .find(|f| f.name == "orphan")
            .unwrap();
        assert_eq!(orphan.instructions.len(), orphan_body.len());
    }

    #[test]
    fn encode_module_for_aot_round_trip_preserves_fn_count() {
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::Return]),
                minimal_fn("orphan", vec![MirOp::ConstI32(1), MirOp::Return]),
            ],
            ..Default::default()
        };
        let bytes = encode_module_for_aot(&module);
        let decoded = decode_module(&bytes).unwrap();
        assert_eq!(decoded.functions.len(), module.functions.len());
    }

    #[test]
    fn compact_module_for_aot_blob_deletes_orphan_reduces_fn_count() {
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::Call(SmolStr::new("used"))]),
                minimal_fn("used", vec![MirOp::Return]),
                minimal_fn("orphan", vec![MirOp::ConstI32(1), MirOp::Return]),
            ],
            ..Default::default()
        };
        let compact = compact_module_for_aot_blob(&module);
        assert_eq!(compact.functions.len(), 2);
        assert!(compact.functions.iter().any(|f| f.name == "main"));
        assert!(compact.functions.iter().any(|f| f.name == "used"));
        assert!(!compact.functions.iter().any(|f| f.name == "orphan"));
    }

    #[test]
    fn remap_fn_indices_in_module_updates_call_indirect() {
        let mut module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::CallIndirect(2)]),
                minimal_fn("orphan", vec![MirOp::Return]),
                minimal_fn("used", vec![MirOp::Return]),
            ],
            ..Default::default()
        };
        // old [main=0, orphan=1, used=2] → keep [main=0, used=1]
        let remap = vec![0, FN_INDEX_DELETED, 1];
        module.functions.retain(|f| f.name != "orphan");
        remap_fn_indices_in_module(&mut module, &remap);
        assert_eq!(module.functions.len(), 2);
        assert!(matches!(
            module.functions[0].instructions[0].op,
            MirOp::CallIndirect(1)
        ));
    }

    #[test]
    fn compact_module_for_aot_blob_no_delete_on_call_indirect_fallback() {
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::CallIndirect(1)]),
                minimal_fn("orphan", vec![MirOp::Return]),
            ],
            ..Default::default()
        };
        let compact = compact_module_for_aot_blob(&module);
        assert_eq!(compact.functions.len(), module.functions.len());
        assert!(compact.functions.iter().any(|f| f.name == "orphan"));
    }

    #[test]
    fn encode_module_compact_for_aot_round_trip_reduces_fn_count() {
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::Return]),
                minimal_fn("orphan", vec![MirOp::ConstI32(1), MirOp::Return]),
            ],
            ..Default::default()
        };
        let bytes = encode_module_compact_for_aot(&module);
        let decoded = decode_module(&bytes).unwrap();
        assert_eq!(decoded.functions.len(), 1);
        assert_eq!(decoded.functions[0].name.as_str(), "main");
    }

    #[test]
    fn encode_module_compact_size_inventory_smaller_than_stub() {
        let mut orphan_ops = Vec::new();
        for i in 0..64 {
            orphan_ops.push(MirOp::ConstI32(i));
        }
        orphan_ops.push(MirOp::Return);
        let module = MirModule {
            functions: vec![
                minimal_fn("main", vec![MirOp::Return]),
                minimal_fn("orphan", orphan_ops),
            ],
            ..Default::default()
        };
        let stub_bytes = encode_module_for_aot(&module);
        let compact_bytes = encode_module_compact_for_aot(&module);
        assert!(
            compact_bytes.len() < stub_bytes.len(),
            "compact inventory must be strictly smaller than stub encode: compact={} stub={}",
            compact_bytes.len(),
            stub_bytes.len()
        );
    }

    #[test]
    fn aot_blob_size_report_saves_bytes_with_orphan_body() {
        let module = aot_size_fixture_main_with_orphan_body(64);
        let report = aot_blob_size_report(&module);
        assert!(
            report.saved_bytes > 0 && report.compact_bytes < report.stub_bytes,
            "orphan body must yield positive saved_bytes: {report:?}"
        );
    }

    #[test]
    fn r34_protocol_shape_for_in_emits_witness_call_indirect() {
        let source = SourceFile::new(
            "r34.sy",
            r#"
protocol Shape {
    func area() -> Double
}
class Circle: Shape {
    var r: Double
    init(r: Double) { self.r = r }
    func area() -> Double { return 3.14159 * self.r * self.r }
}
func total(_ shapes: Array<Shape>) -> Double {
    var sum = 0.0
    for s in shapes {
        sum = sum + s.area()
    }
    return sum
}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let total = mir
            .functions
            .iter()
            .find(|f| f.name == "total")
            .expect("total");
        let flat: Vec<_> = total
            .blocks
            .iter()
            .flat_map(|b| b.instructions.iter())
            .map(|i| format!("{:?}", i.op))
            .collect();
        let joined = flat.join("\n");
        eprintln!("TOTAL OPS:\n{joined}");
        eprintln!("local_types={:?}", total.local_types);
        assert!(
            joined.contains("CallIndirect") && joined.contains("__witness_Shape_area"),
            "expected witness CallIndirect, got:\n{joined}"
        );
    }

    #[test]
    fn lower_module_export_func_fills_aot_export_roots() {
        let source = SourceFile::new(
            "t.sy",
            r#"export func greet() {
    print("hi")
}
func main() {
    greet()
}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        assert!(
            mir.aot_export_roots.iter().any(|n| n.as_str() == "greet"),
            "export func greet must fill aot_export_roots: {:?}",
            mir.aot_export_roots
        );
        assert!(
            mir.functions.iter().any(|f| f.name.as_str() == "greet"),
            "export func must still lower as a MIR function"
        );
    }

    #[test]
    fn lower_module_without_export_keeps_empty_aot_export_roots() {
        let source = SourceFile::new("t.sy", "func main() { return 1 }");
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        assert!(
            mir.aot_export_roots.is_empty(),
            "no export ⇒ empty aot_export_roots: {:?}",
            mir.aot_export_roots
        );
    }
    #[test]
    fn public_func_roots_drive_llvm_emit_function_names_exports_ga_evidence() {
        // Orphan public export (not called from main): roots drive emit; cleared ⇒ main-only.
        let source = SourceFile::new(
            "t.sy",
            r#"public func greet() {
    print("hi")
}
func helper() {}
func main() {
    print("main")
}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        assert!(
            mir.aot_export_roots.iter().any(|n| n.as_str() == "greet"),
            "public greet must fill aot_export_roots: {:?}",
            mir.aot_export_roots
        );
        let emit = llvm_emit_function_names(&mir);
        assert!(
            emit.contains("greet"),
            "W440 default emit must include public greet root: {emit:?}"
        );
        assert!(
            emit.contains("main"),
            "default emit must still include main: {emit:?}"
        );
        assert!(
            !emit.contains("helper"),
            "unreachable helper must not be in emit: {emit:?}"
        );
        let cleared = MirModule {
            functions: mir.functions.clone(),
            aot_export_roots: vec![],
        };
        let cleared_emit = llvm_emit_function_names(&cleared);
        assert!(
            !cleared_emit.contains("greet"),
            "cleared-roots clone must be main-only (no orphan greet): {cleared_emit:?}"
        );
        assert!(
            cleared_emit.contains("main"),
            "cleared emit must keep main: {cleared_emit:?}"
        );
        let roots = mir_module_aot_export_root_refs(&mir);
        assert_eq!(
            emit,
            reachable_fn_names_from_roots(&mir, &roots),
            "default emit must follow aot_export_roots"
        );
    }

    #[test]
    fn lower_module_public_func_fills_aot_export_roots_scan_all() {
        let source = SourceFile::new(
            "t.sy",
            r#"public func greet() {
    print("hi")
}
func helper() {}
func main() {
    greet()
}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        assert!(
            mir.aot_export_roots.iter().any(|n| n.as_str() == "greet"),
            "public func greet must fill aot_export_roots: {:?}",
            mir.aot_export_roots
        );
        assert!(
            !mir.aot_export_roots.iter().any(|n| n.as_str() == "helper"),
            "internal func helper must not fill roots: {:?}",
            mir.aot_export_roots
        );
        assert!(
            !mir.aot_export_roots.iter().any(|n| n.as_str() == "main"),
            "internal main must not fill roots: {:?}",
            mir.aot_export_roots
        );
    }

    #[test]
    fn lower_module_export_list_trailing_comma_fills_aot_export_roots() {
        let source = SourceFile::new(
            "t.sy",
            r#"export { greet, }
func greet() {
    print("hi")
}
func main() {
    greet()
}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        assert!(
            mir.aot_export_roots.iter().any(|n| n.as_str() == "greet"),
            "export {{ greet, }} must fill aot_export_roots: {:?}",
            mir.aot_export_roots
        );
    }

    #[test]
    fn lower_module_export_list_fills_aot_export_roots() {
        let source = SourceFile::new(
            "t.sy",
            r#"export { greet }
func greet() {
    print("hi")
}
func main() {
    greet()
}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        assert!(
            mir.aot_export_roots.iter().any(|n| n.as_str() == "greet"),
            "export {{ greet }} must fill aot_export_roots: {:?}",
            mir.aot_export_roots
        );
    }

    #[test]
    fn lower_module_export_list_and_export_func_dedupe_roots() {
        let source = SourceFile::new(
            "t.sy",
            r#"export { greet }
export func greet() {
    print("hi")
}
func main() {}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let mir = lower_module(&module).unwrap();
        let count = mir
            .aot_export_roots
            .iter()
            .filter(|n| n.as_str() == "greet")
            .count();
        assert_eq!(
            count, 1,
            "greet must be deduped in roots: {:?}",
            mir.aot_export_roots
        );
    }

    #[test]
    fn aot_size_fixture_orphan_saved_bytes_monotonic() {
        let small = aot_blob_size_report(&aot_size_fixture_main_with_orphan_body(64));
        let large = aot_blob_size_report(&aot_size_fixture_main_with_orphan_body(128));
        assert!(
            small.saved_bytes > 0,
            "orphan(64) must save bytes: {small:?}"
        );
        assert!(
            large.saved_bytes > small.saved_bytes,
            "orphan(128) saved must be strictly greater than orphan(64): small={small:?} large={large:?}"
        );
    }

    #[test]
    fn aot_size_fixture_call_indirect_no_fn_delete_save() {
        // CallIndirect forces full-emit: fn-delete compact cannot drop orphans.
        // Production omit+strip may still shrink bodies vs stub (≠ zero total save).
        let module = aot_size_fixture_call_indirect_full_emit();
        let stub_bytes = encode_module_for_aot(&module).len();
        let compact_only_bytes = encode_module(&compact_module_for_aot_blob(&module)).len();
        let report = aot_blob_size_report(&module);
        assert_eq!(
            stub_bytes, compact_only_bytes,
            "CallIndirect must not save via fn-delete compact"
        );
        assert_eq!(report.stub_bytes, stub_bytes);
        assert!(
            report.compact_bytes <= report.stub_bytes,
            "production path must not grow vs stub: {report:?}"
        );
    }

    #[test]
    fn mir_module_blob_stats_round_trip_preserves_fn_names() {
        let module = aot_size_fixture_main_with_orphan_body(2);
        let stats = mir_module_blob_stats(&module);
        assert_eq!(stats.fn_count, module.functions.len());
        assert!(
            stats.encoded_bytes > 0,
            "non-empty module must encode to >0 bytes: {stats:?}"
        );
        let decoded = decode_module(&encode_module(&module)).expect("decode");
        assert_eq!(decoded.functions.len(), module.functions.len());
        let names: Vec<_> = module.functions.iter().map(|f| f.name.as_str()).collect();
        let decoded_names: Vec<_> = decoded.functions.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(decoded_names, names);
        let stats_after = mir_module_blob_stats(&decoded);
        assert_eq!(stats_after.fn_count, stats.fn_count);
        assert_eq!(stats_after.encoded_bytes, stats.encoded_bytes);
    }

    #[test]
    fn mir_module_blob_stats_empty_module_round_trip() {
        let module = MirModule::default();
        let stats = mir_module_blob_stats(&module);
        assert_eq!(stats.fn_count, 0);
        let decoded = decode_module(&encode_module(&module)).expect("decode empty");
        assert_eq!(decoded.functions.len(), 0);
        let again = mir_module_blob_stats(&decoded);
        assert_eq!(again.fn_count, 0);
        assert_eq!(again.encoded_bytes, stats.encoded_bytes);
    }

    #[test]
    fn module_with_debug_locals_stripped_clears_slots_keeps_instructions() {
        let module = aot_codec_fixture_main_with_debug_locals(8);
        let stripped = module_with_debug_locals_stripped(&module);
        assert_eq!(stripped.functions.len(), module.functions.len());
        assert_eq!(
            stripped.functions[0].local_count,
            module.functions[0].local_count
        );
        assert!(stripped.functions[0].local_names.is_empty());
        assert!(stripped.functions[0].local_types.is_empty());
        assert_eq!(
            stripped.functions[0].instructions.len(),
            module.functions[0].instructions.len()
        );
        // Generic encode_module must NOT auto-strip.
        let plain = decode_module(&encode_module(&module)).unwrap();
        assert_eq!(plain.functions[0].local_names.len(), 8);
        assert_eq!(plain.functions[0].local_types.len(), 8);
    }

    #[test]
    fn encode_module_compact_for_aot_strips_debug_locals() {
        let module = aot_codec_fixture_main_with_debug_locals(8);
        let compact_only = encode_module(&compact_module_for_aot_blob(&module));
        let aot = encode_module_compact_for_aot(&module);
        assert!(
            aot.len() < compact_only.len(),
            "AOT compact+strip must be smaller than compact-only: aot={} compact_only={}",
            aot.len(),
            compact_only.len()
        );
        let decoded = decode_module(&aot).unwrap();
        assert!(!decoded.functions.is_empty());
        for func in &decoded.functions {
            assert!(
                func.local_names.is_empty(),
                "AOT blob must strip local_names"
            );
            assert!(
                func.local_types.is_empty(),
                "AOT blob must strip local_types"
            );
        }
        let stripped_path = encode_module(&module_with_debug_locals_stripped(
            &compact_module_for_aot_blob(&module),
        ));
        assert_eq!(
            aot.len(),
            stripped_path.len(),
            "compact_for_aot must match strip helper path"
        );
    }

    #[test]
    fn encode_module_compact_for_aot_no_debug_slots_neutral() {
        let module = aot_size_fixture_main_with_orphan_body(2);
        let compact_only = encode_module(&compact_module_for_aot_blob(&module));
        let aot = encode_module_compact_for_aot(&module);
        assert_eq!(
            aot.len(),
            compact_only.len(),
            "no-debug modules must not change size under strip wire"
        );
    }

    #[test]
    fn encode_module_compact_for_aot_omits_eligible_instruction_streams() {
        let mut ops = Vec::new();
        for i in 0..64 {
            ops.push(MirOp::ConstI32(i));
        }
        ops.push(MirOp::Return);
        let module = MirModule {
            functions: vec![aot_size_fixture_fn("main", ops)],
            ..Default::default()
        };
        let compact = compact_module_for_aot_blob(&module);
        let strip_only = encode_module(&module_with_debug_locals_stripped(&compact));
        let aot = encode_module_compact_for_aot(&module);
        assert!(
            aot.len() < strip_only.len(),
            "AOT compact+omit+strip must be smaller than compact+strip-only: aot={} strip_only={}",
            aot.len(),
            strip_only.len()
        );
        let decoded = decode_module(&aot).unwrap();
        assert_eq!(decoded.functions.len(), 1);
        assert_eq!(decoded.functions[0].instructions.len(), 1);
        assert!(matches!(
            decoded.functions[0].instructions[0].op,
            MirOp::Return
        ));
        // Generic encode_module must NOT auto-omit.
        let plain = decode_module(&encode_module(&module)).unwrap();
        assert!(plain.functions[0].instructions.len() > 1);
    }

    #[test]
    fn encode_module_compact_for_aot_omit_keeps_strip_and_call_indirect() {
        let module = aot_codec_fixture_main_with_debug_locals(8);
        let aot = encode_module_compact_for_aot(&module);
        let decoded = decode_module(&aot).unwrap();
        assert!(decoded.functions[0].local_names.is_empty());
        assert!(decoded.functions[0].local_types.is_empty());
        assert!(matches!(
            decoded.functions[0].instructions[0].op,
            MirOp::Return
        ));

        let mut fat_ops = Vec::new();
        for i in 0..32 {
            fat_ops.push(MirOp::ConstI32(i));
        }
        fat_ops.push(MirOp::Return);
        let ci = MirModule {
            functions: vec![
                aot_size_fixture_fn("main", vec![MirOp::CallIndirect(1)]),
                aot_size_fixture_fn("orphan", fat_ops),
            ],
            ..Default::default()
        };
        let ci_bytes = encode_module_compact_for_aot(&ci);
        let ci_decoded = decode_module(&ci_bytes).unwrap();
        assert_eq!(ci_decoded.functions.len(), 2);
        for func in &ci_decoded.functions {
            assert_eq!(func.instructions.len(), 1);
            assert!(matches!(func.instructions[0].op, MirOp::Return));
        }
    }

    #[test]
    fn encode_decode_module_metadata_sidecar_roundtrip() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let view = mir_module_metadata_sidecar_view(&module);
        let bytes = encode_module_metadata_sidecar(&module);
        let decoded = decode_module_metadata_sidecar(&bytes).expect("sidecar decode");
        let after = encode_module(&module);
        assert_eq!(before, after, "sidecar wire must not mutate encode_module");
        assert_eq!(decoded, view);
    }
    #[test]
    fn encode_decode_module_dual_blob_package_roundtrip() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module);
        let view = mir_module_metadata_sidecar_view(&module);
        let execution = dual_blob_package_execution_bytes(&module);
        let bytes = encode_module_dual_blob_package(&module);
        let decoded = decode_module_dual_blob_package(&bytes).expect("dual-blob package decode");
        let after = encode_module(&module);
        assert_eq!(
            before, after,
            "dual-blob package wire must not mutate encode_module"
        );
        assert_eq!(decoded.sidecar, view);
        assert_eq!(decoded.execution_bytes, execution);
        assert_eq!(
            dual_blob_package_sidecar_bytes(&module),
            encode_module_metadata_sidecar(&module)
        );
    }

    #[test]
    fn encode_module_aot_embed_blob_peels_to_compact_for_aot() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let embed = encode_module_aot_embed_blob(&module);
        let compact = encode_module_aot_embed_execution_bytes(&module);
        let package = encode_module_dual_blob_package(&module);
        assert!(is_dual_blob_package_bytes(&embed));
        assert_ne!(
            embed, package,
            "AOT embed omits execution; inventory package uses whole-module execution"
        );
        let peeled = peel_dual_blob_package_execution(&embed).expect("peel embed");
        assert!(peeled.is_empty(), "W409 AOT embed execution must be empty");
        assert_eq!(peeled, compact.as_slice());
        assert!(aot_embed_execution_omitted());
        assert!(embed.len() > dual_blob_package_sidecar_bytes(&module).len());
        let side = peel_dual_blob_package_sidecar(&embed).expect("peel sidecar");
        let decoded = decode_module_metadata_sidecar(side).expect("decode sidecar");
        assert_eq!(decoded, mir_module_metadata_sidecar_view(&module));
    }

    #[test]
    fn mir_module_from_sidecar_fn_stubs_preserves_fn_count() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let view = mir_module_metadata_sidecar_view(&module);
        let stubs = mir_module_from_sidecar_fn_stubs(&view);
        assert_eq!(stubs.functions.len(), module.functions.len());
        assert_eq!(stubs.functions[0].name.as_str(), "main");
        assert!(matches!(
            stubs.functions[0].instructions[0].op,
            MirOp::Return
        ));
    }
    #[test]
    fn module_with_execution_string_table_stripped_clears_const_str() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let before = encode_module(&module).len();
        let stripped = module_with_execution_string_table_stripped(&module);
        let after = encode_module(&stripped).len();
        assert!(
            after < before,
            "stripping ConstStr payloads must shrink whole-module encode: before={before} after={after}"
        );
        let view = mir_module_metadata_sidecar_view(&stripped);
        assert_eq!(view.string_literals.len(), 4);
        assert!(view.string_literals.iter().all(|s| s.is_empty()));
        let orig = mir_module_metadata_sidecar_view(&module);
        assert!(orig.string_literals.iter().all(|s| !s.is_empty()));
    }
    #[test]
    fn peel_dual_blob_package_sidecar_matches_encode() {
        let module = aot_codec_fixture_main_with_const_strs(4);
        let embed = encode_module_aot_embed_blob(&module);
        let expected = encode_module_metadata_sidecar(&module);
        let peeled = peel_dual_blob_package_sidecar(&embed).expect("peel sidecar");
        assert_eq!(peeled, expected.as_slice());
        let decoded = decode_module_metadata_sidecar(peeled).expect("decode sidecar");
        assert_eq!(decoded, mir_module_metadata_sidecar_view(&module));
    }
}

#[cfg(test)]
mod r35_struct_defaults {
    use super::*;
    use crate::mir_module_index::ModuleIndex;
    use smol_str::SmolStr;
    use syx_diagnostics::SourceFile;
    use syx_parser::Parser;
    #[test]
    fn counter_book_fields_and_defaults() {
        let source = SourceFile::new(
            "t.sy",
            r#"struct Counter {
    var value: Int = 0
    mutating func increment() {
        value = value + 1
    }
}
func main() {
    var c = Counter()
    c.increment()
    print(c.value)
}
"#,
        );
        let module = Parser::parse(source).unwrap();
        let idx = ModuleIndex::from_module(&module);
        let fields = idx
            .dataclass_fields
            .get("Counter")
            .cloned()
            .unwrap_or_default();
        assert_eq!(fields, vec![SmolStr::new("value")], "fields={fields:?}");
        assert_eq!(
            idx.dataclass_field_default_i64
                .get(&(SmolStr::new("Counter"), SmolStr::new("value")))
                .copied(),
            Some(0)
        );
        let mut mir = lower_module(&module).unwrap();
        merge_mir_shell_functions(&mut mir, lower_dataclass_shells(&module));
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        let ops: Vec<_> = main.blocks[0]
            .instructions
            .iter()
            .map(|i| format!("{:?}", i.op))
            .collect();
        assert!(
            ops.iter()
                .any(|o| o.contains("ConstI32") || o.contains("ConstNumeric")),
            "main should pad default before Counter_init: {ops:?}"
        );
    }
}
