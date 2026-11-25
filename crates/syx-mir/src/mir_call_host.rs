//! Collect absolute `CallHost` indices from a MIR module (native host shaking).

use std::collections::BTreeSet;

use crate::mir_aot::{reachable_fn_names, reachable_fn_names_from_roots};
use crate::mir_types::{mir_module_aot_export_root_refs, MirModule, MirOp};

/// Every absolute `CallHost` index in `module` (all functions — includes prelude).
pub fn collect_call_host_indices(module: &MirModule) -> BTreeSet<u32> {
    collect_call_host_indices_in(module, None)
}

/// CallHost indices reachable from `main` ∪ `aot_export_roots` (native shake source of truth).
pub fn collect_call_host_indices_reachable(module: &MirModule) -> BTreeSet<u32> {
    let roots = mir_module_aot_export_root_refs(module);
    let reachable = if roots.is_empty() {
        reachable_fn_names(module)
    } else {
        reachable_fn_names_from_roots(module, &roots)
    };
    collect_call_host_indices_in(module, Some(&reachable))
}

fn collect_call_host_indices_in(
    module: &MirModule,
    only_names: Option<&BTreeSet<smol_str::SmolStr>>,
) -> BTreeSet<u32> {
    let mut out = BTreeSet::new();
    for func in &module.functions {
        if let Some(names) = only_names {
            if !names.contains(&func.name) {
                continue;
            }
        }
        for instr in &func.instructions {
            if let MirOp::CallHost(idx) = instr.op {
                out.insert(idx);
            }
        }
        for block in &func.blocks {
            for instr in &block.instructions {
                if let MirOp::CallHost(idx) = instr.op {
                    out.insert(idx);
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::{MirBlock, MirFunction, MirInstr};
    use smol_str::SmolStr;

    #[test]
    fn reachable_skips_orphan_call_host() {
        let module = MirModule {
            functions: vec![
                MirFunction {
                    name: SmolStr::new_static("main"),
                    param_count: 0,
                    local_count: 0,
                    entry: 0,
                    blocks: vec![],
                    instructions: vec![MirInstr {
                        dest: None,
                        op: MirOp::Print,
                        span: None,
                    }],
                    local_names: vec![],
                    local_types: vec![],
                    init_body_pc: 0,
                    init_wrap_pc: 0,
                    owner_module: None,
                },
                MirFunction {
                    name: SmolStr::new_static("orphan_sql"),
                    param_count: 0,
                    local_count: 0,
                    entry: 0,
                    blocks: vec![MirBlock {
                        id: 0,
                        instructions: vec![MirInstr {
                            dest: None,
                            op: MirOp::CallHost(42),
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
                },
            ],
            aot_export_roots: vec![],
            module_names: vec![],
        };
        assert!(collect_call_host_indices_reachable(&module).is_empty());
        assert!(collect_call_host_indices(&module).contains(&42));
    }
}
