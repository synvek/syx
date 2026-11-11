//! SYX MIR lowering context (`LowerCtx`).
//! Extracted in S2-I171 (zero semantics; crate-private; used by lowering in lib.rs).

use crate::class_name_from_mangled;
use crate::lower_expr;
use crate::mir_module_index::ModuleIndex;
use crate::mir_types::{MirBlock, MirFunction, MirInstr, MirOp};
use indexmap::{IndexMap, IndexSet};
use smol_str::SmolStr;
use std::cell::Cell;
use std::collections::HashSet;
use std::rc::Rc;
use syx_ast::Expr;
use syx_diagnostics::{Span, SyxDiagnostic};

#[derive(Debug, Default, Clone)]
pub(crate) struct LowerCtx {
    pub(crate) func_name: SmolStr,
    pub(crate) blocks: Vec<MirBlock>,
    pub(crate) next_block: u32,
    pub(crate) next_reg: u32,
    pub(crate) locals: indexmap::IndexMap<SmolStr, u32>,
    pub(crate) globals: Vec<SmolStr>,
    pub(crate) imports: IndexSet<SmolStr>,
    pub(crate) module: ModuleIndex,
    /// Module-wide id source so `__mir_closure_N` / `__task_body_N` stay unique across
    /// functions (per-`LowerCtx` counters collide and codegen/MIR dedupe by name).
    pub(crate) closure_ids: Rc<Cell<u32>>,
    pub(crate) pending_closures: Vec<MirFunction>,
    pub(crate) in_async: bool,
    pub(crate) await_segment: u32,
    pub(crate) active_block: u32,
    pub(crate) local_types: indexmap::IndexMap<SmolStr, SmolStr>,
    pub(crate) prefer_set_build: bool,
    /// Lowering soft-fail: `(optional diagnostic code, message, optional span)`.
    pub(crate) error: Option<(Option<&'static str>, String, Option<Span>)>,
    pub(crate) mut_ref_aliases: IndexMap<SmolStr, SmolStr>,
    pub(crate) mut_ref_params: HashSet<SmolStr>,
    /// Function parameters: caller still holds these; skip scope-exit deinit.
    pub(crate) param_names: HashSet<SmolStr>,
    /// Local skipped on this return (Ident return of a newly built object).
    pub(crate) skip_deinit_on_return: Option<SmolStr>,
    /// Active source span stamped onto emitted instructions (DAP breakpoints).
    pub(crate) current_span: Option<(u32, u32)>,
}

impl LowerCtx {
    pub(crate) fn new(
        func_name: SmolStr,
        module: ModuleIndex,
        imports: IndexSet<SmolStr>,
        globals: Vec<SmolStr>,
        closure_ids: Rc<Cell<u32>>,
    ) -> Self {
        Self {
            func_name,
            blocks: Vec::new(),
            next_block: 0,
            next_reg: 0,
            locals: indexmap::IndexMap::new(),
            globals,
            imports,
            module,
            closure_ids,
            pending_closures: Vec::new(),
            in_async: false,
            await_segment: 0,
            active_block: 0,
            local_types: indexmap::IndexMap::new(),
            prefer_set_build: false,
            error: None,
            mut_ref_aliases: IndexMap::new(),
            mut_ref_params: HashSet::new(),
            param_names: HashSet::new(),
            skip_deinit_on_return: None,
            current_span: None,
        }
    }

    fn error_span(&self) -> Option<Span> {
        self.current_span.map(|(start, len)| Span { start, len })
    }

    pub(crate) fn next_closure_id(&self) -> u32 {
        let id = self.closure_ids.get();
        self.closure_ids.set(id + 1);
        id
    }

    pub(crate) fn resolve_binding_name<'a>(&'a self, name: &'a SmolStr) -> &'a SmolStr {
        self.mut_ref_aliases.get(name).unwrap_or(name)
    }

    pub(crate) fn is_class_instance_binding(&self, name: &SmolStr) -> bool {
        if name == "self" {
            return true;
        }
        self.local_types.get(name).is_some_and(|t| {
            self.module.class_types.contains_key(t) || self.module.actor_types.contains(t)
        })
    }

    pub(crate) fn uses_mut_ref_cell(&self, name: &SmolStr) -> bool {
        self.mut_ref_params.contains(name)
    }

    pub(crate) fn in_class_init(&self) -> bool {
        self.func_name.ends_with("_init")
    }

    pub(crate) fn enclosing_class(&self) -> Option<SmolStr> {
        class_name_from_mangled(self.func_name.as_str())
    }

    pub(crate) fn receiver_class_type(&self, obj_name: &SmolStr) -> Option<SmolStr> {
        if obj_name.as_str() == "self" {
            class_name_from_mangled(self.func_name.as_str())
        } else {
            self.local_types.get(obj_name).cloned()
        }
    }

    /// Unwrap `MutRefCell` only for explicit `inout`/mut-ref params. Class locals returned
    /// from inits stay as MutRefCell handles so `resolve_virtual_method` sees `__type`.
    pub(crate) fn should_unwrap_class_local(&self, name: &SmolStr) -> bool {
        if self.in_class_init() && name.as_str() == "self" {
            return false;
        }
        self.uses_mut_ref_cell(name)
    }

    pub(crate) fn member_assign_unwraps_object(&self, object: &Expr) -> bool {
        !(self.in_class_init()
            && matches!(object, Expr::Ident { name, .. } if name.as_str() == "self"))
    }

    /// After field write on an unwrapped class instance, write inner `ObjectRef` back into
    /// the local's `MutRefCell` wrapper (boot DI field `@Autowired` assigns).
    pub(crate) fn emit_mut_ref_cell_writeback_from_inner(
        &mut self,
        name: &SmolStr,
        inner_slot: u32,
        block: u32,
    ) {
        let resolved = self.resolve_binding_name(name).clone();
        let Some(&slot) = self.locals.get(&resolved) else {
            return;
        };
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(slot),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(inner_slot),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr("__syx_ref".into()),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ObjectSet,
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::StoreLocal(slot),
                span: None,
            },
        );
    }

    #[allow(dead_code)] // residual MutRefCell writeback path (parked)
    pub(crate) fn emit_mut_ref_cell_writeback(&mut self, name: &SmolStr, block: u32) {
        let resolved = self.resolve_binding_name(name).clone();
        let Some(&slot) = self.locals.get(&resolved) else {
            return;
        };
        let temp = self.alloc_local(&SmolStr::new(format!("__syx_store_{slot}")));
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::StoreLocal(temp),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(slot),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(temp),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr("__syx_ref".into()),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ObjectSet,
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::StoreLocal(slot),
                span: None,
            },
        );
    }

    pub(crate) fn emit_unwrap_syx_ref(&mut self, block: u32) {
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr("__syx_ref".into()),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ObjectGet,
                span: None,
            },
        );
    }

    pub(crate) fn lower_load_binding_raw(&mut self, name: &SmolStr, block: u32) {
        let resolved = self.resolve_binding_name(name).clone();
        if let Some(&slot) = self.locals.get(&resolved) {
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(slot),
                    span: None,
                },
            );
        } else if let Some(slot) = self.global_slot(&resolved) {
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadGlobal(slot),
                    span: None,
                },
            );
        } else {
            self.set_error(format!("Unknown variable '{name}'"));
        }
    }

    pub(crate) fn lower_rhs_for_field_store(&mut self, value: &Expr, block: u32) {
        if let Expr::Ident { name, .. } = value {
            if self.locals.contains_key(name) || self.global_slot(name).is_some() {
                self.lower_load_binding_raw(name, block);
                return;
            }
        }
        lower_expr(value, self.fresh_reg(), self, block);
    }

    pub(crate) fn lower_load_binding(&mut self, dest: u32, name: &SmolStr, block: u32) {
        let resolved = self.resolve_binding_name(name).clone();
        if self.should_unwrap_class_local(&resolved) {
            if let Some(&slot) = self.locals.get(&resolved) {
                self.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::LoadLocal(slot),
                        span: None,
                    },
                );
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ConstStr("__syx_ref".into()),
                        span: None,
                    },
                );
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ObjectGet,
                        span: None,
                    },
                );
            }
            return;
        }
        if let Some(&slot) = self.locals.get(&resolved) {
            self.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::LoadLocal(slot),
                    span: None,
                },
            );
        } else if self.lower_enclosing_field_load(dest, name, block) {
            // R-35 / S1-I2: prefer enclosing field over Core globals (`count`, …).
        } else if let Some(slot) = self.global_slot(&resolved) {
            self.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::LoadGlobal(slot),
                    span: None,
                },
            );
        } else {
            self.set_error(format!("Unknown variable '{name}'"));
        }
    }

    /// True when `name` is a stored field of the type owning this method.
    pub(crate) fn is_enclosing_stored_field(&self, name: &SmolStr) -> bool {
        let Some(type_name) = class_name_from_mangled(self.func_name.as_str()) else {
            return false;
        };
        self.module
            .dataclass_fields
            .get(&type_name)
            .is_some_and(|fs| fs.iter().any(|f| f == name))
            || self
                .module
                .class_types
                .get(&type_name)
                .is_some_and(|fs| fs.iter().any(|f| f == name))
    }

    /// `Counter_increment`: bare `value` → `self.value` ObjectGet.
    pub(crate) fn lower_enclosing_field_load(
        &mut self,
        dest: u32,
        name: &SmolStr,
        block: u32,
    ) -> bool {
        if !self.is_enclosing_stored_field(name) {
            return false;
        }
        let self_name = SmolStr::new("self");
        let Some(&self_slot) = self.locals.get(&self_name) else {
            return false;
        };
        let unwrap = self.uses_mut_ref_cell(&self_name) && !self.in_class_init();
        // Stack-style load: unwrap / ObjectGet consume the receiver from the stack.
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(self_slot),
                span: None,
            },
        );
        if unwrap {
            // Class methods receive MutRefCell; fields live on the inner dict.
            self.emit_unwrap_syx_ref(block);
        }
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr(name.to_string()),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: Some(dest),
                op: MirOp::ObjectGet,
                span: None,
            },
        );
        true
    }

    /// Bare field store: stack has RHS; emit `self.field = rhs` + write back `self`.
    pub(crate) fn lower_enclosing_field_store(&mut self, name: &SmolStr, block: u32) -> bool {
        if !self.is_enclosing_stored_field(name) {
            return false;
        }
        let self_name = SmolStr::new("self");
        let Some(&self_slot) = self.locals.get(&self_name) else {
            return false;
        };
        let unwrap = self.uses_mut_ref_cell(&self_name) && !self.in_class_init();
        let temp = self.alloc_local(&SmolStr::new(format!("__syx_field_{name}")));
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::StoreLocal(temp),
                span: None,
            },
        );
        if unwrap {
            let inner = self.alloc_local(&SmolStr::new(format!("__syx_inner_field_{name}")));
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(self_slot),
                    span: None,
                },
            );
            self.emit_unwrap_syx_ref(block);
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(temp),
                    span: None,
                },
            );
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr(name.to_string()),
                    span: None,
                },
            );
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ObjectSet,
                    span: None,
                },
            );
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(inner),
                    span: None,
                },
            );
            self.emit_mut_ref_cell_writeback_from_inner(&self_name, inner, block);
        } else {
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(self_slot),
                    span: None,
                },
            );
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(temp),
                    span: None,
                },
            );
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr(name.to_string()),
                    span: None,
                },
            );
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ObjectSet,
                    span: None,
                },
            );
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(self_slot),
                    span: None,
                },
            );
        }
        true
    }

    /// R-31: before storing a dataclass/struct value, shallow-copy via `{T}_copy`
    /// so assignment does not alias the source `ObjectRef` / heap slot.
    pub(crate) fn emit_dataclass_assign_copy_if_needed(&mut self, type_name: &SmolStr, block: u32) {
        if !self.module.dataclass_names.contains(type_name) {
            return;
        }
        if self.module.class_types.contains_key(type_name)
            || self.module.actor_types.contains(type_name)
        {
            return;
        }
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::Call(SmolStr::new(format!("{type_name}_copy"))),
                span: None,
            },
        );
    }

    pub(crate) fn lower_store_binding(&mut self, name: &SmolStr, block: u32) {
        let resolved = self.resolve_binding_name(name).clone();
        if self.should_unwrap_class_local(&resolved) {
            if let Some(&slot) = self.locals.get(&resolved) {
                let temp = self.alloc_local(&SmolStr::new(format!("__syx_store_{slot}")));
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::StoreLocal(temp),
                        span: None,
                    },
                );
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::LoadLocal(slot),
                        span: None,
                    },
                );
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::LoadLocal(temp),
                        span: None,
                    },
                );
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ConstStr("__syx_ref".into()),
                        span: None,
                    },
                );
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ObjectSet,
                        span: None,
                    },
                );
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::StoreLocal(slot),
                        span: None,
                    },
                );
            }
            return;
        }
        if self.is_class_instance_binding(&resolved) {
            if let Some(&slot) = self.locals.get(&resolved) {
                self.emit_binding_override_deinit(&resolved, slot, block);
                self.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::StoreLocal(slot),
                        span: None,
                    },
                );
            }
            return;
        }
        if let Some(ty) = self.local_types.get(&resolved).cloned() {
            self.emit_dataclass_assign_copy_if_needed(&ty, block);
        }
        if let Some(&slot) = self.locals.get(&resolved) {
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(slot),
                    span: None,
                },
            );
        } else if self.lower_enclosing_field_store(name, block) {
            // R-35 / S1-I2: prefer enclosing field over Core globals (`count`, …).
        } else if let Some(slot) = self.global_slot(&resolved) {
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreGlobal(slot),
                    span: None,
                },
            );
        } else {
            self.set_error(format!("Unknown variable '{name}'"));
        }
    }

    /// Phase 101: call `Type_deinit` on the old local before overwrite.
    pub(crate) fn emit_binding_override_deinit(&mut self, name: &SmolStr, slot: u32, block: u32) {
        let Some(type_name) = self.local_types.get(name).cloned() else {
            return;
        };
        if !self.module.class_types.contains_key(&type_name)
            && !self.module.actor_types.contains(&type_name)
        {
            return;
        }
        let deinit_name = SmolStr::new(format!("{type_name}_deinit"));
        if !self.module.has_deinit(&type_name) {
            return;
        }
        let temp = self.alloc_local(&SmolStr::new(format!("__p101_store_new_{name}")));
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::StoreLocal(temp),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(slot),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::Call(deinit_name),
                span: None,
            },
        );
        self.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(temp),
                span: None,
            },
        );
    }

    pub(crate) fn emit_scope_exit_deinits(&mut self, block: u32) {
        let locals: Vec<(SmolStr, SmolStr, u32)> = self
            .locals
            .iter()
            .filter_map(|(name, &slot)| {
                if self.param_names.contains(name) {
                    return None;
                }
                if self.skip_deinit_on_return.as_ref() == Some(name) {
                    return None;
                }
                let ty = self.local_types.get(name)?.clone();
                if self.module.class_types.contains_key(&ty)
                    || self.module.actor_types.contains(&ty)
                {
                    if self.module.has_deinit(&ty) {
                        return Some((name.clone(), ty, slot));
                    }
                }
                None
            })
            .collect();
        for (_name, type_name, slot) in locals {
            let deinit_name = SmolStr::new(format!("{type_name}_deinit"));
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(slot),
                    span: None,
                },
            );
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Call(deinit_name),
                    span: None,
                },
            );
            self.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Pop,
                    span: None,
                },
            );
        }
    }

    pub(crate) fn lower_mut_ref_cell(&mut self, dest: u32, target: &SmolStr, block: u32) {
        self.lower_load_binding(dest, target, block);
    }

    pub(crate) fn global_slot(&self, name: &SmolStr) -> Option<u32> {
        self.globals
            .iter()
            .position(|g| g == name)
            .map(|i| i as u32)
    }

    pub(crate) fn set_error(&mut self, message: impl Into<String>) {
        self.set_error_with_code(None, message);
    }

    pub(crate) fn set_error_with_code(
        &mut self,
        code: Option<&'static str>,
        message: impl Into<String>,
    ) {
        if self.error.is_none() {
            self.error = Some((code, message.into(), self.error_span()));
        }
    }

    pub(crate) fn take_error_diagnostic(&mut self) -> Option<SyxDiagnostic> {
        let (code, message, span) = self.error.take()?;
        let span = span.unwrap_or(Span::DUMMY);
        // R-36: attach SourceFile when available so Display has file:line:col.
        Some(crate::mir_diag::mir_error_at(
            message,
            self.module.source.as_ref(),
            span,
            code.unwrap_or("MIR001"),
        ))
    }

    pub(crate) fn fresh_block(&mut self) -> u32 {
        let id = self.next_block;
        self.next_block += 1;
        self.blocks.push(MirBlock {
            id,
            instructions: Vec::new(),
            successors: Vec::new(),
        });
        id
    }

    #[allow(dead_code)] // residual helper for MutRefCell / block queries
    pub(crate) fn current_block(&mut self) -> u32 {
        if self.blocks.is_empty() {
            self.fresh_block()
        } else {
            self.blocks.last().unwrap().id
        }
    }

    pub(crate) fn emit(&mut self, block_id: u32, mut instr: MirInstr) {
        if instr.span.is_none() {
            instr.span = self.current_span;
        }
        if let Some(block) = self.blocks.iter_mut().find(|b| b.id == block_id) {
            block.instructions.push(instr);
        }
    }

    pub(crate) fn link(&mut self, from: u32, to: u32) {
        if let Some(block) = self.blocks.iter_mut().find(|b| b.id == from) {
            if !block.successors.contains(&to) {
                block.successors.push(to);
            }
        }
    }

    pub(crate) fn alloc_local(&mut self, name: &SmolStr) -> u32 {
        if let Some(&slot) = self.locals.get(name) {
            slot
        } else {
            let slot = self.locals.len() as u32;
            self.locals.insert(name.clone(), slot);
            slot
        }
    }

    pub(crate) fn fresh_reg(&mut self) -> u32 {
        let r = self.next_reg;
        self.next_reg += 1;
        r
    }
}
