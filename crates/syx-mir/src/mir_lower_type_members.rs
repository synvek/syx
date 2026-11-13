//! SYX MIR type-members / protocol-stub / convenience-init lowering.
//! Extracted in S2-I205 (zero semantics; crate-private).

use crate::extension_init_literal_fields;
use crate::lower_function_with_params;
use crate::mir_lower_class_init::lower_class_init;
use crate::mir_lower_shells::{mir_op, mir_shell_fn};
use crate::mir_module_index::ModuleIndex;
use crate::mir_types::{MirBlock, MirFunction, MirInstr, MirModule, MirOp};
use indexmap::IndexSet;
use smol_str::SmolStr;
use std::cell::Cell;
use std::rc::Rc;
use syx_ast::{Decl, Stmt, TypeExpr};
use syx_diagnostics::SyxResult;
use syx_stdlib::PLATFORM_INT;

pub(crate) fn merged_class_fields(module_idx: &ModuleIndex, type_name: &SmolStr) -> Vec<SmolStr> {
    let mut chain = Vec::new();
    let mut cur = Some(type_name.clone());
    while let Some(t) = cur {
        if let Some(fields) = module_idx.class_types.get(&t) {
            chain.push(fields.clone());
        }
        cur = module_idx.class_superclasses.get(&t).cloned();
    }
    chain.reverse();
    let mut out = Vec::new();
    for fields in chain {
        for field in fields {
            if !out.contains(&field) {
                out.push(field);
            }
        }
    }
    out
}

pub(crate) fn lower_type_members(
    mir: &mut MirModule,
    type_name: &SmolStr,
    members: &[Decl],
    is_actor: bool,
    conformances: &[SmolStr],
    module_idx: &ModuleIndex,
    imports: &IndexSet<SmolStr>,
    globals: &[SmolStr],
    closure_ids: Rc<Cell<u32>>,
) -> SyxResult<()> {
    let member_fields: Vec<SmolStr> = members
        .iter()
        .filter_map(syx_ast::var_global_stored_field_name)
        .collect();
    let mut merged_fields = merged_class_fields(module_idx, type_name);
    // Structs store fields in `dataclass_fields` / member VarGlobals, not `class_types`.
    if merged_fields.is_empty() {
        if let Some(dc) = module_idx.dataclass_fields.get(type_name) {
            merged_fields = dc.clone();
        } else {
            merged_fields = member_fields;
        }
    }
    let has_init = members.iter().any(|m| matches!(m, Decl::Init { .. }));
    let mut pending_inits: Vec<(&[syx_ast::Param], &[Stmt])> = Vec::new();
    for member in members {
        match member {
            Decl::Function {
                name,
                body,
                params,
                attrs,
                ..
            } => {
                let is_static = attrs.iter().any(|a| a.name.as_str() == "static");
                let method_name = SmolStr::new(format!("{type_name}_{name}"));
                let mut all_params = if is_static {
                    Vec::new()
                } else {
                    // ADR-631: typed self on actors so `self.method()` → ActorCall (not ObjectGet).
                    // Keep untyped self for classes to preserve MutRefCell / deinit lowering.
                    // Object/Dictionary bags need typed self so `self[key]` → ObjectGet/ObjectSet.
                    let typed_self =
                        is_actor || matches!(type_name.as_str(), "Object" | "Dictionary");
                    vec![self_param_typed(typed_self.then_some(type_name))]
                };
                all_params.extend_from_slice(params);
                let (func, closures) = lower_function_with_params(
                    &method_name,
                    &all_params,
                    body,
                    module_idx,
                    false,
                    imports,
                    globals,
                    &[],
                    &[],
                    Rc::clone(&closure_ids),
                )?;
                mir.functions.push(func);
                mir.functions.extend(closures);
            }
            Decl::OperatorFunc {
                op, body, params, ..
            } => {
                let method_name = SmolStr::new(format!("{type_name}_{op}"));
                let (func, closures) = lower_function_with_params(
                    &method_name,
                    params,
                    body,
                    module_idx,
                    false,
                    imports,
                    globals,
                    &[],
                    &[],
                    Rc::clone(&closure_ids),
                )?;
                mir.functions.push(func);
                mir.functions.extend(closures);
            }
            Decl::Init { params, body, .. } => {
                // S1-I7: extension `init() { self.init(x: 0, y: 0) }` → zero-arg shell
                // with literal fields (avoids recursive Call to same Point_init).
                if params.is_empty() {
                    if let Some(lit_fields) = extension_init_literal_fields(body) {
                        let init_name = SmolStr::new(format!("{type_name}_init"));
                        let shell = lower_extension_convenience_init_shell(type_name, &lit_fields);
                        if let Some(pos) = mir.functions.iter().position(|f| f.name == init_name) {
                            mir.functions[pos] = shell;
                        } else {
                            mir.functions.push(shell);
                        }
                        continue;
                    }
                }
                pending_inits.push((params, body));
            }
            Decl::Deinit { body, .. } => {
                let deinit_name = SmolStr::new(format!("{type_name}_deinit"));
                // ADR-631: only actors need typed self for method ActorCall; class deinit stays untyped.
                let all_params = vec![self_param_typed(is_actor.then_some(type_name))];
                let (func, closures) = lower_function_with_params(
                    &deinit_name,
                    &all_params,
                    body,
                    module_idx,
                    false,
                    imports,
                    globals,
                    &[],
                    &[],
                    Rc::clone(&closure_ids),
                )?;
                mir.functions.push(func);
                mir.functions.extend(closures);
            }
            _ => {}
        }
    }
    for (params, body) in pending_inits {
        lower_class_init(
            mir,
            type_name,
            &merged_fields,
            params,
            body,
            is_actor,
            conformances,
            module_idx,
            imports,
            globals,
            Rc::clone(&closure_ids),
        )?;
    }
    for member in members {
        if let Decl::Class {
            name: inner,
            members: nested,
            conformances,
            ..
        } = member
        {
            let qualified = SmolStr::new(format!("{type_name}_{inner}"));
            lower_type_members(
                mir,
                &qualified,
                nested,
                is_actor,
                conformances,
                module_idx,
                imports,
                globals,
                Rc::clone(&closure_ids),
            )?;
        } else if let Decl::Struct {
            name: inner,
            fields,
            ..
        } = member
        {
            let qualified = SmolStr::new(format!("{type_name}_{inner}"));
            let _ = (qualified, fields);
        } else if let Decl::Enum {
            name: inner, cases, ..
        } = member
        {
            let qualified = SmolStr::new(format!("{type_name}_{inner}"));
            let _ = (qualified, cases);
        } else if let Decl::FinalClass {
            name: inner,
            variants,
            ..
        } = member
        {
            let qualified = SmolStr::new(format!("{type_name}_{inner}"));
            let _ = (qualified, variants);
        }
    }
    if !has_init {
        let init_name = SmolStr::new(format!("{type_name}_init"));
        if !mir.functions.iter().any(|f| f.name == init_name) {
            if merged_fields.is_empty() {
                lower_class_init(
                    mir,
                    type_name,
                    &[],
                    &[],
                    &[],
                    is_actor,
                    conformances,
                    module_idx,
                    imports,
                    globals,
                    Rc::clone(&closure_ids),
                )?;
            } else {
                // Memberwise `{T}_init` params; call sites pad defaults. Shell merge
                // may replace this with `lower_dataclass_init_shell` (same arity).
                let params: Vec<syx_ast::Param> = merged_fields
                    .iter()
                    .map(|f| syx_ast::Param {
                        name: f.clone(),
                        external_label: syx_ast::ParamLabel::Same,
                        ty: None,
                        default: None,
                        isolated: false,
                        sending: false,
                        attrs: vec![],
                    })
                    .collect();
                lower_class_init(
                    mir,
                    type_name,
                    &merged_fields,
                    &params,
                    &[],
                    is_actor,
                    conformances,
                    module_idx,
                    imports,
                    globals,
                    Rc::clone(&closure_ids),
                )?;
            }
        }
    }
    // S1-I6: inherit protocol default bodies as `{Type}_{method}` when the type omits them.
    emit_inherited_protocol_defaults(
        mir,
        type_name,
        members,
        is_actor,
        conformances,
        module_idx,
        imports,
        globals,
        Rc::clone(&closure_ids),
    )?;
    append_protocol_stub_functions(mir, &module_idx);
    Ok(())
}

pub(crate) fn emit_inherited_protocol_defaults(
    mir: &mut MirModule,
    type_name: &SmolStr,
    members: &[Decl],
    is_actor: bool,
    conformances: &[SmolStr],
    module_idx: &ModuleIndex,
    imports: &IndexSet<SmolStr>,
    globals: &[SmolStr],
    closure_ids: Rc<Cell<u32>>,
) -> SyxResult<()> {
    for protocol in conformances {
        let Some(methods) = module_idx.protocol_methods.get(protocol) else {
            continue;
        };
        for method in methods {
            let method_name = SmolStr::new(format!("{type_name}_{method}"));
            if mir.functions.iter().any(|f| f.name == method_name) {
                continue;
            }
            if members
                .iter()
                .any(|m| matches!(m, Decl::Function { name, .. } if name == method))
            {
                continue;
            }
            let Some((params, body)) = module_idx
                .protocol_default_methods
                .get(&(protocol.clone(), method.clone()))
            else {
                continue;
            };
            let typed_self = is_actor || matches!(type_name.as_str(), "Object" | "Dictionary");
            let mut all_params = vec![self_param_typed(typed_self.then_some(type_name))];
            all_params.extend_from_slice(params);
            let (func, closures) = lower_function_with_params(
                &method_name,
                &all_params,
                body,
                module_idx,
                false,
                imports,
                globals,
                &[],
                &[],
                Rc::clone(&closure_ids),
            )?;
            mir.functions.push(func);
            mir.functions.extend(closures);
        }
    }
    Ok(())
}

pub(crate) fn append_protocol_stub_functions(mir: &mut MirModule, module_idx: &ModuleIndex) {
    for (protocol, methods) in &module_idx.protocol_methods {
        for method in methods {
            let name = SmolStr::new(format!("{protocol}_{method}"));
            if mir.functions.iter().any(|f| f.name == name) {
                continue;
            }
            let stub = vec![
                MirInstr {
                    dest: None,
                    op: MirOp::ConstNil,
                    span: None,
                },
                MirInstr {
                    dest: None,
                    op: MirOp::Return,
                    span: None,
                },
            ];
            mir.functions.push(MirFunction {
                name,
                param_count: 1,
                local_count: 1,
                entry: 0,
                blocks: vec![MirBlock {
                    id: 0,
                    instructions: stub.clone(),
                    successors: vec![],
                }],
                instructions: stub,
                local_names: vec![SmolStr::new("self")],
                local_types: vec![SmolStr::new("Unknown")],
                init_body_pc: 0,
                init_wrap_pc: 0,
            });
        }
    }
}

#[allow(dead_code)] // convenience over self_param_typed; retained for type-member peel API
pub(crate) fn self_param() -> syx_ast::Param {
    self_param_typed(None)
}

pub(crate) fn self_param_typed(type_name: Option<&SmolStr>) -> syx_ast::Param {
    syx_ast::Param {
        name: SmolStr::new("self"),
        external_label: syx_ast::ParamLabel::Omitted,
        ty: type_name.map(|n| TypeExpr::Named {
            name: n.clone(),
            generics: vec![],
        }),
        default: None,
        isolated: false,
        sending: false,
        attrs: vec![],
    }
}

pub(crate) fn lower_extension_convenience_init_shell(
    type_name: &SmolStr,
    lit_fields: &[(SmolStr, i128)],
) -> MirFunction {
    let mut ins = Vec::new();
    for (field, value) in lit_fields {
        ins.push(mir_op(MirOp::ConstNumeric {
            kind: PLATFORM_INT.to_byte(),
            bits: *value as u64,
        }));
        ins.push(mir_op(MirOp::ConstStr(field.to_string())));
    }
    ins.push(mir_op(MirOp::ConstStr(type_name.to_string())));
    ins.push(mir_op(MirOp::ConstStr("__type".to_string())));
    ins.push(mir_op(MirOp::DictBuild((lit_fields.len() + 1) as u32)));
    ins.push(mir_op(MirOp::Return));
    mir_shell_fn(SmolStr::new(format!("{type_name}_init")), 0, 0, ins)
}
