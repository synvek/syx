//! SYX MIR AOT shaken / compact / omit / size-inventory helpers.
//! Extracted in S2-I168 as two non-contiguous slices (opt block stays in lib.rs).
//! Zero semantics; re-exported from crate root.

use smol_str::SmolStr;
use std::collections::{BTreeSet, VecDeque};

use crate::{
    encode_module, mir_module_aot_export_root_refs, MirBlock, MirFunction, MirInstr, MirModule,
    MirOp,
};

fn direct_call_targets_from_instructions(instructions: &[MirInstr]) -> Vec<SmolStr> {
    let mut out = Vec::new();
    for instr in instructions {
        match &instr.op {
            MirOp::Call(name)
            | MirOp::ActorCall(name)
            | MirOp::TaskSpawn(name)
            | MirOp::MakeClosure(name, _)
            | MirOp::CallSuper(name) => out.push(name.clone()),
            _ => {}
        }
    }
    out
}

/// Cross-function reachability from `main` via direct named call edges.
///
/// Conservative: `CallIndirect`, `CallWitness`, async resume targets, and other
/// unresolved dispatch are **not** treated as proof that a function is dead.
///
/// Default production path remains **main-only**. For explicit export roots see
/// [`reachable_fn_names_from_roots`] (W398 multi-root wire).
pub fn reachable_fn_names(module: &MirModule) -> BTreeSet<SmolStr> {
    reachable_fn_names_from_roots(module, &[])
}

/// Cross-function reachability from `main` ∪ `extra_export_roots` via direct named call edges.
///
/// Empty `extra_export_roots` is equivalent to [`reachable_fn_names`] (main-only).
/// Non-empty roots keep orphans unreachable from any root shaken out. Does **not**
/// scan semantic `Module.exports` automatically (explicit roots only; W398).
pub fn reachable_fn_names_from_roots(
    module: &MirModule,
    extra_export_roots: &[&str],
) -> BTreeSet<SmolStr> {
    let mut roots = Vec::new();
    if module.functions.iter().any(|f| f.name == "main") {
        roots.push(SmolStr::new("main"));
    }
    for name in extra_export_roots {
        let s = SmolStr::new(*name);
        if module.functions.iter().any(|f| f.name == s) {
            roots.push(s);
        }
    }
    reachable_fn_names_from_root_set(module, roots)
}

fn reachable_fn_names_from_root_set(
    module: &MirModule,
    roots: impl IntoIterator<Item = SmolStr>,
) -> BTreeSet<SmolStr> {
    let mut reachable = BTreeSet::new();
    let mut queue = VecDeque::new();
    for root in roots {
        if module.functions.iter().any(|f| f.name == root) && reachable.insert(root.clone()) {
            queue.push_back(root);
        }
    }
    while let Some(name) = queue.pop_front() {
        let Some(func) = module.functions.iter().find(|f| f.name == name) else {
            continue;
        };
        let mut callees = direct_call_targets_from_instructions(&func.instructions);
        for block in &func.blocks {
            callees.extend(direct_call_targets_from_instructions(&block.instructions));
        }
        for callee in callees {
            if reachable.insert(callee.clone()) {
                queue.push_back(callee);
            }
        }
    }
    reachable
}

fn mir_requires_full_llvm_emit(module: &MirModule) -> bool {
    fn instr_triggers_full_emit(op: &MirOp) -> bool {
        matches!(
            op,
            MirOp::CallIndirect(_) | MirOp::CallWitness(_, _) | MirOp::AwaitSuspend { .. }
        )
    }
    module.functions.iter().any(|func| {
        func.instructions
            .iter()
            .any(|i| instr_triggers_full_emit(&i.op))
            || func.blocks.iter().any(|b| {
                b.instructions
                    .iter()
                    .any(|i| instr_triggers_full_emit(&i.op))
            })
    })
}

/// Function names eligible for native LLVM body emit + `syx_rt_register_native_fn`.
///
/// Default production path (W440): uses [`MirModule::aot_export_roots`] via
/// [`llvm_emit_function_names_with_roots`]. Empty roots ≡ [`reachable_fn_names`]
/// (main-only). Falls back to **all** fn names when the module contains
/// `CallIndirect` / `CallWitness` / `AwaitSuspend` (unresolved dispatch).
/// For caller-supplied roots use [`llvm_emit_function_names_with_roots`] (W398).
pub fn llvm_emit_function_names(module: &MirModule) -> BTreeSet<SmolStr> {
    let roots = mir_module_aot_export_root_refs(module);
    llvm_emit_function_names_with_roots(module, &roots)
}

/// Emit eligibility with optional `extra_export_roots` (W398 multi-root wire).
///
/// Empty extras ≡ main-only reachability (ignores [`MirModule::aot_export_roots`];
/// use [`llvm_emit_function_names`] for the default production path). Non-empty uses
/// [`reachable_fn_names_from_roots`]. Full-emit fallback unchanged.
pub fn llvm_emit_function_names_with_roots(
    module: &MirModule,
    extra_export_roots: &[&str],
) -> BTreeSet<SmolStr> {
    if mir_requires_full_llvm_emit(module) {
        return module.functions.iter().map(|f| f.name.clone()).collect();
    }
    reachable_fn_names_from_roots(module, extra_export_roots)
}

fn stub_mir_function_for_blob(func: &MirFunction) -> MirFunction {
    MirFunction {
        name: func.name.clone(),
        param_count: func.param_count,
        local_count: func.local_count,
        entry: 0,
        blocks: vec![],
        instructions: vec![MirInstr {
            dest: None,
            op: MirOp::Return,
            span: None,
        }],
        local_names: func.local_names.clone(),
        local_types: func.local_types.clone(),
        init_body_pc: func.init_body_pc,
        init_wrap_pc: func.init_wrap_pc,
    }
}

/// Index-preserving AOT blob module: ineligible fn bodies become minimal `Return` stubs.
///
/// Preserves `functions` vec length/order so fn indices stay aligned with LLVM/register paths.
/// Does **not** delete fn entries (fn-delete / size golden remain Frozen).
pub fn shaken_module_for_aot_blob(module: &MirModule) -> MirModule {
    let roots = mir_module_aot_export_root_refs(module);
    shaken_module_for_aot_blob_with_roots(module, &roots)
}

/// [`shaken_module_for_aot_blob`] with optional export roots (W398).
pub fn shaken_module_for_aot_blob_with_roots(
    module: &MirModule,
    extra_export_roots: &[&str],
) -> MirModule {
    let eligible = llvm_emit_function_names_with_roots(module, extra_export_roots);
    MirModule {
        functions: module
            .functions
            .iter()
            .map(|func| {
                if eligible.contains(&func.name) {
                    func.clone()
                } else {
                    stub_mir_function_for_blob(func)
                }
            })
            .collect(),
        aot_export_roots: module.aot_export_roots.clone(),
    }
}

/// Encode MIR for native AOT blob embed after body-strip shake.
pub fn encode_module_for_aot(module: &MirModule) -> Vec<u8> {
    encode_module(&shaken_module_for_aot_blob(module))
}

/// Sentinel in [`build_fn_index_remap`] output for deleted (non-eligible) slots.
pub const FN_INDEX_DELETED: u32 = u32::MAX;

/// Keep eligible functions in original relative order; build old→new index remap.
///
/// `remap[old_index]` is the new index, or [`FN_INDEX_DELETED`] if the slot was removed.
pub fn build_fn_index_remap(
    functions: &[MirFunction],
    eligible: &BTreeSet<SmolStr>,
) -> (Vec<MirFunction>, Vec<u32>) {
    let mut compact = Vec::new();
    let mut remap = Vec::with_capacity(functions.len());
    for func in functions {
        if eligible.contains(&func.name) {
            remap.push(compact.len() as u32);
            compact.push(func.clone());
        } else {
            remap.push(FN_INDEX_DELETED);
        }
    }
    (compact, remap)
}

fn remap_fn_index(old: u32, remap: &[u32]) -> u32 {
    remap
        .get(old as usize)
        .copied()
        .filter(|&n| n != FN_INDEX_DELETED)
        .unwrap_or(old)
}

fn remap_fn_indices_in_instructions(instructions: &mut [MirInstr], remap: &[u32]) {
    for instr in instructions {
        match &mut instr.op {
            MirOp::CallIndirect(idx) => {
                *idx = remap_fn_index(*idx, remap);
            }
            MirOp::CallWitness(a, b) => {
                *a = remap_fn_index(*a, remap);
                *b = remap_fn_index(*b, remap);
            }
            MirOp::AwaitSuspend { resume_fn, .. } => {
                *resume_fn = remap_fn_index(*resume_fn, remap);
            }
            _ => {}
        }
    }
}

/// Remap fn-index literals in-place (`CallIndirect` / `CallWitness` / `AwaitSuspend.resume_fn`).
///
/// `MakeClosure(name, _)` is left unchanged (runtime resolves by name).
pub fn remap_fn_indices_in_module(module: &mut MirModule, remap: &[u32]) {
    for func in &mut module.functions {
        remap_fn_indices_in_instructions(&mut func.instructions, remap);
        for block in &mut func.blocks {
            remap_fn_indices_in_instructions(&mut block.instructions, remap);
        }
    }
}

/// Analysis-only fn-delete compact: drop ineligible fn entries and remap indices.
///
/// When [`mir_requires_full_llvm_emit`] is true, returns a clone of `module` (same
/// conservative fallback as emit/blob eligibility). Does **not** wire AOT production
/// encode (still [`encode_module_for_aot`] / stub path).
pub fn compact_module_for_aot_blob(module: &MirModule) -> MirModule {
    let roots = mir_module_aot_export_root_refs(module);
    compact_module_for_aot_blob_with_roots(module, &roots)
}

/// [`compact_module_for_aot_blob`] with optional export roots (W398).
pub fn compact_module_for_aot_blob_with_roots(
    module: &MirModule,
    extra_export_roots: &[&str],
) -> MirModule {
    if mir_requires_full_llvm_emit(module) {
        return module.clone();
    }
    let eligible = llvm_emit_function_names_with_roots(module, extra_export_roots);
    let (compact_fns, remap) = build_fn_index_remap(&module.functions, &eligible);
    let mut out = MirModule {
        functions: compact_fns,
        aot_export_roots: module.aot_export_roots.clone(),
    };
    remap_fn_indices_in_module(&mut out, &remap);
    out
}

/// Encode after fn-delete compact + native-instruction omit + AOT debug-local strip.
///
/// W316: clears `local_names`/`local_types` after compact.
/// W318: stubs llvm-emit-eligible instruction streams after compact, before strip.
/// Does **not** change generic `encode_module`, or claim B-05 `[x]`.
pub fn encode_module_compact_for_aot(module: &MirModule) -> Vec<u8> {
    let compact = compact_module_for_aot_blob(module);
    let omitted = module_with_native_instructions_omitted(&compact);
    encode_module(&module_with_debug_locals_stripped(&omitted))
}

/// Stub vs production AOT MIR-blob byte sizes (compat measurement helper).
///
/// `compact_bytes` is the **production** path (`encode_module_compact_for_aot` =
/// compact → omit → strip), **not** fn-delete-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AotBlobSizeReport {
    pub stub_bytes: usize,
    /// Production compact+omit+strip encode size (name retained for compat).
    pub compact_bytes: usize,
    pub saved_bytes: usize,
}

/// Compare index-preserving stub encode vs **production** compact+omit+strip encode.
pub fn aot_blob_size_report(module: &MirModule) -> AotBlobSizeReport {
    let stub_bytes = encode_module_for_aot(module).len();
    let compact_bytes = encode_module_compact_for_aot(module).len();
    AotBlobSizeReport {
        stub_bytes,
        compact_bytes,
        saved_bytes: stub_bytes.saturating_sub(compact_bytes),
    }
}

pub(crate) fn aot_size_fixture_fn(name: &str, ops: Vec<MirOp>) -> MirFunction {
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

/// Reproducible size-inventory fixture for B-04 W308.
///
/// Builds `main` plus `orphan_const_count` unreachable orphan functions, each carrying
/// one `ConstI32` then `Return`. Scale is **orphan fn count** (not a single fat body):
/// stub body-strip collapses orphan instruction streams to `Return`, so a single orphan's
/// body length does not change `aot_blob_size_report` savings; multiple orphans do
/// (stub keeps stubbed entries; compact deletes them). Relational/monotonic only —
/// **not** CI golden thresholds or absolute byte tables.
pub fn aot_size_fixture_main_with_orphan_body(orphan_const_count: usize) -> MirModule {
    let mut functions = Vec::with_capacity(orphan_const_count.saturating_add(1));
    functions.push(aot_size_fixture_fn("main", vec![MirOp::Return]));
    for i in 0..orphan_const_count {
        functions.push(aot_size_fixture_fn(
            &format!("orphan_{i}"),
            vec![MirOp::ConstI32(i as i32), MirOp::Return],
        ));
    }
    MirModule {
        functions,
        ..Default::default()
    }
}

/// Reproducible size-inventory fixture: `CallIndirect` forces full-emit / no-delete fallback.
pub fn aot_size_fixture_call_indirect_full_emit() -> MirModule {
    MirModule {
        functions: vec![
            aot_size_fixture_fn("main", vec![MirOp::CallIndirect(1)]),
            aot_size_fixture_fn("orphan", vec![MirOp::ConstI32(99), MirOp::Return]),
        ],
        ..Default::default()
    }
}

pub fn aot_codec_fixture_main_with_const_strs(n: usize) -> MirModule {
    let mut ops = Vec::with_capacity(n.saturating_add(1));
    for i in 0..n {
        ops.push(MirOp::ConstStr(format!("meta_str_{i}")));
    }
    ops.push(MirOp::Return);
    MirModule {
        functions: vec![aot_size_fixture_fn("main", ops)],
        ..Default::default()
    }
}

/// Reproducible B-05 debug-metadata fixture: `main` with `slot_count` local name/type slots.
///
/// Existing `aot_size_fixture_*` leave debug slots empty; this fixture fills both
/// `local_names` and `local_types` so strip inventory can observe `saved_bytes > 0`.
pub fn aot_codec_fixture_main_with_debug_locals(slot_count: usize) -> MirModule {
    let mut names = Vec::with_capacity(slot_count);
    let mut types = Vec::with_capacity(slot_count);
    for i in 0..slot_count {
        names.push(SmolStr::new(format!("dbg_name_{i}")));
        types.push(SmolStr::new(format!("DbgType{i}")));
    }
    let mut main = aot_size_fixture_fn("main", vec![MirOp::Return]);
    main.local_count = slot_count as u32;
    main.local_names = names;
    main.local_types = types;
    MirModule {
        functions: vec![main],
        ..Default::default()
    }
}

/// Clone and clear debug `local_names`/`local_types` only (AOT production strip helper).
///
/// Used for B-05 production debug-strip wire (W316). Does **not** change `local_count`,
/// instructions, generic `encode_module`, or claim a real codec / B-05 `[x]`.
pub fn module_with_debug_locals_stripped(module: &MirModule) -> MirModule {
    let mut stripped = module.clone();
    for func in &mut stripped.functions {
        func.local_names.clear();
        func.local_types.clear();
    }
    stripped
}

/// Clone and clear recoverable `ConstStr` string content (execution string-table strip).
///
/// Used for B-05 Medium sidecar-primary deepen (W408): empties `ConstStr` payloads so
/// the AOT execution segment no longer carries recoverable string literals (sidecar
/// remains the source of truth). Keeps `ConstStr` op slots for index alignment when
/// bodies are not yet omitted. Does **not** change generic `encode_module`, delete
/// `@syx_mir_blob`, or claim B-05 `[x]` / MIR-free.
pub fn module_with_execution_string_table_stripped(module: &MirModule) -> MirModule {
    fn clear_instr(instr: &MirInstr) -> MirInstr {
        MirInstr {
            dest: instr.dest,
            op: match &instr.op {
                MirOp::ConstStr(_) => MirOp::ConstStr(String::new()),
                other => other.clone(),
            },
            span: instr.span,
        }
    }
    MirModule {
        functions: module
            .functions
            .iter()
            .map(|func| MirFunction {
                name: func.name.clone(),
                param_count: func.param_count,
                local_count: func.local_count,
                entry: func.entry,
                blocks: func
                    .blocks
                    .iter()
                    .map(|b| MirBlock {
                        id: b.id,
                        instructions: b.instructions.iter().map(clear_instr).collect(),
                        successors: b.successors.clone(),
                    })
                    .collect(),
                instructions: func.instructions.iter().map(clear_instr).collect(),
                local_names: func.local_names.clone(),
                local_types: func.local_types.clone(),
                init_body_pc: func.init_body_pc,
                init_wrap_pc: func.init_wrap_pc,
            })
            .collect(),
        aot_export_roots: module.aot_export_roots.clone(),
    }
}

/// Clone and stub llvm-emit-eligible instruction streams to minimal `Return` (AOT omit helper).
///
/// Used for B-05 production native-instruction-omit wire (W318). Does **not** change
/// generic `encode_module`, or claim a real codec / B-05 `[x]`.
pub fn module_with_native_instructions_omitted(module: &MirModule) -> MirModule {
    let eligible = llvm_emit_function_names(module);
    MirModule {
        functions: module
            .functions
            .iter()
            .map(|func| {
                if eligible.contains(&func.name) {
                    stub_mir_function_for_blob(func)
                } else {
                    func.clone()
                }
            })
            .collect(),
        aot_export_roots: module.aot_export_roots.clone(),
    }
}
