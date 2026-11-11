//! SYX MIR optimization pipeline (fold / DCE / inline / LICM / opt-v* gates).
//! Extracted in S2-I169 (zero semantics; re-exported from crate root).
//! Uses `crate::flatten_blocks` from lowering helpers in lib.rs.

use indexmap::IndexSet;
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

use crate::{flatten_blocks, MirFunction, MirInstr, MirModule, MirOp};

pub fn const_fold(module: &mut MirModule) {
    for func in &mut module.functions {
        fold_function(&mut func.instructions);
        for block in &mut func.blocks {
            fold_function(&mut block.instructions);
        }
    }
}

pub(crate) fn fold_function(instructions: &mut Vec<MirInstr>) {
    let mut i = 0;
    while i + 2 < instructions.len() {
        let a = &instructions[i];
        let b = &instructions[i + 1];
        let c = &instructions[i + 2];
        if let (
            MirInstr {
                dest: Some(d),
                op: MirOp::ConstI32(x),
                span: None,
            },
            MirInstr {
                dest: Some(_),
                op: MirOp::ConstI32(y),
                span: None,
            },
            MirInstr {
                dest: Some(d2),
                op: MirOp::Add { .. },
                span: None,
            },
        ) = (a, b, c)
        {
            if d == d2 {
                instructions[i] = MirInstr {
                    dest: Some(*d),
                    op: MirOp::ConstI32(x + y),
                    span: None,
                };
                instructions.remove(i + 1);
                instructions.remove(i + 1);
                continue;
            }
        }
        if let MirInstr {
            op: MirOp::ConstI32(0),
            span: None,
            ..
        } = &instructions[i]
        {
            if i + 1 < instructions.len() {
                if matches!(instructions[i + 1].op, MirOp::Add { .. }) {
                    instructions.remove(i + 1);
                    instructions.remove(i);
                    continue;
                }
            }
        }
        i += 1;
    }
}


pub fn dead_code_elim(module: &mut MirModule) {
    for func in &mut module.functions {
        dce_function(&mut func.instructions);
        for block in &mut func.blocks {
            dce_function(&mut block.instructions);
        }
        if func.instructions.is_empty() {
            func.instructions.push(MirInstr {
                dest: None,
                op: MirOp::Return,
                span: None,
            });
        }
    }
}

fn dce_function(instructions: &mut Vec<MirInstr>) {
    let mut reachable = vec![false; instructions.len()];
    if instructions.is_empty() {
        return;
    }
    let mut work = vec![instructions.len() - 1];
    while let Some(idx) = work.pop() {
        if idx >= instructions.len() || reachable[idx] {
            continue;
        }
        reachable[idx] = true;
        match &instructions[idx].op {
            MirOp::JumpIfFalse(_) | MirOp::Jump(_) => {
                if idx > 0 {
                    work.push(idx - 1);
                }
            }
            MirOp::Return => {
                if idx > 0 {
                    work.push(idx - 1);
                }
            }
            _ => {
                if idx > 0 {
                    work.push(idx - 1);
                }
            }
        }
    }
    let mut out = Vec::new();
    for (i, instr) in instructions.iter().cloned().enumerate() {
        if reachable[i] || matches!(instr.op, MirOp::Return) {
            out.push(instr);
        }
    }
    *instructions = out;
}

pub fn optimize_mir_module(module: &mut MirModule) {
    optimize(module);
}

pub fn remove_unreachable_blocks(module: &mut MirModule) {
    for func in &mut module.functions {
        remove_unreachable_blocks_in_function(func);
    }
}

fn remove_unreachable_blocks_in_function(func: &mut MirFunction) {
    if func.blocks.is_empty() {
        return;
    }
    let mut reachable = IndexSet::new();
    let mut work = vec![func.entry];
    while let Some(id) = work.pop() {
        if !reachable.insert(id) {
            continue;
        }
        let Some(block) = func.blocks.iter().find(|b| b.id == id) else {
            continue;
        };
        for succ in &block.successors {
            work.push(*succ);
        }
        for instr in &block.instructions {
            match &instr.op {
                MirOp::Jump(t) | MirOp::JumpIfFalse(t) | MirOp::JumpIfNil(t) => work.push(*t),
                // R-30: `try` / `do` pads are exception edges, not only Jump successors.
                MirOp::BeginTry(t) | MirOp::BindResult(t) => work.push(*t),
                _ => {}
            }
        }
    }
    func.blocks.retain(|b| reachable.contains(&b.id));
}

const INLINE_MAX_INSTRS: usize = 10;

fn inline_threshold() -> usize {
    std::env::var("SYX_MIR_INLINE_THRESHOLD")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(INLINE_MAX_INSTRS)
}

fn func_flat_instructions(func: &MirFunction) -> &[MirInstr] {
    if !func.instructions.is_empty() {
        &func.instructions
    } else if func.blocks.len() == 1 {
        &func.blocks[0].instructions
    } else {
        &[]
    }
}

fn is_inline_candidate(func: &MirFunction, escaping_closures: &std::collections::HashSet<SmolStr>) -> bool {
    if escaping_closures.contains(&func.name) {
        return false;
    }
    if !func.name.starts_with("__syx_") {
        return false;
    }
    if func.param_count > 1 || func.blocks.len() > 1 {
        return false;
    }
    let ins = func_flat_instructions(func);
    if ins.is_empty() || ins.len() > inline_threshold() {
        return false;
    }
    ins.iter().all(|i| {
        !matches!(
            i.op,
            MirOp::Call(_)
                | MirOp::Jump(_)
                | MirOp::JumpIfFalse(_)
                | MirOp::JumpIfNil(_)
                | MirOp::BeginTry(_)
                | MirOp::BindResult(_)
                | MirOp::ReturnErr
                | MirOp::MakeClosure(_, _)
                | MirOp::AwaitSuspend { .. }
        )
    })
}

fn inlineable_body(
    func: &MirFunction,
    escaping_closures: &std::collections::HashSet<SmolStr>,
) -> Option<Vec<MirInstr>> {
    if !is_inline_candidate(func, escaping_closures) {
        return None;
    }
    let ins = func_flat_instructions(func);
    if !ins.last().is_some_and(|i| matches!(i.op, MirOp::Return)) {
        return None;
    }
    Some(ins[..ins.len().saturating_sub(1)].to_vec())
}

fn remap_inline_locals(body: Vec<MirInstr>, call_dest: Option<u32>) -> Vec<MirInstr> {
    let mut out = body;
    if let Some(dest) = call_dest {
        if let Some(last) = out.last_mut() {
            if last.dest.is_none() {
                last.dest = Some(dest);
            }
        }
    }
    out
}

fn inline_calls_in_instructions(
    instructions: &mut Vec<MirInstr>,
    module: &MirModule,
    escaping_closures: &std::collections::HashSet<SmolStr>,
) {
    let candidates: HashMap<SmolStr, Vec<MirInstr>> = module
        .functions
        .iter()
        .filter_map(|f| {
            inlineable_body(f, escaping_closures).map(|b| (f.name.clone(), b))
        })
        .collect();
    if candidates.is_empty() {
        return;
    }
    let mut i = 0;
    while i < instructions.len() {
        if let MirOp::Call(name) = &instructions[i].op {
            if let Some(body) = candidates.get(name) {
                let dest = instructions[i].dest;
                let inlined = remap_inline_locals(body.clone(), dest);
                instructions.remove(i);
                for (offset, instr) in inlined.into_iter().enumerate() {
                    instructions.insert(i + offset, instr);
                }
                continue;
            }
        }
        i += 1;
    }
}

/// Phase 10 AR2: mark closure functions that capture heap env slots (non-inlinable).
pub fn escape_analysis(module: &MirModule) -> std::collections::HashSet<SmolStr> {
    use std::collections::HashSet;
    let mut escaping = HashSet::new();
    for func in &module.functions {
        let ins = func_flat_instructions(func);
        for instr in ins {
            if let MirOp::MakeClosure(name, captures) = &instr.op {
                if *captures > 0 {
                    escaping.insert(name.clone());
                }
            }
        }
        for block in &func.blocks {
            for instr in &block.instructions {
                if let MirOp::MakeClosure(name, captures) = &instr.op {
                    if *captures > 0 {
                        escaping.insert(name.clone());
                    }
                }
            }
        }
    }
    for func in &module.functions {
        if func.name.starts_with("__mir_closure_") {
            let ins = func_flat_instructions(func);
            if ins.iter().any(|i| matches!(i.op, MirOp::LoadGlobal(_))) {
                escaping.insert(func.name.clone());
            }
        }
    }
    escaping
}

/// Inline small `__syx_*` helpers at direct `Call` sites (Phase 9 AM2).
pub fn inline_small_host_functions(module: &mut MirModule) {
    let escaping = escape_analysis(module);
    inline_small_host_functions_with_escape(module, &escaping);
}

fn inline_small_host_functions_with_escape(
    module: &mut MirModule,
    escaping: &std::collections::HashSet<SmolStr>,
) {
    let snapshot = module.clone();
    for func in &mut module.functions {
        inline_calls_in_instructions(&mut func.instructions, &snapshot, escaping);
        for block in &mut func.blocks {
            inline_calls_in_instructions(&mut block.instructions, &snapshot, escaping);
        }
    }
}

pub fn optimize(module: &mut MirModule) {
    const_fold(module);
    block_cse(module);
    simple_licm(module);
    loop_licm(module);
    if mir_opt_v8_enabled() {
        alias_barrier_licm(module);
    }
    inline_small_host_functions(module);
    remove_unreachable_blocks(module);
    dead_code_elim(module);
    // Phase 98: keep flat instruction stream aligned with blocks (VM vs LLVM/Wasm).
    resync_flat_from_blocks(module);
}

fn resync_flat_from_blocks(module: &mut MirModule) {
    for func in &mut module.functions {
        if func.blocks.is_empty() {
            continue;
        }
        func.instructions = flatten_blocks(&func.blocks);
    }
}

/// Phase 20 MIR opt v8: re-run effect-aware LICM with StoreLocal consumer-chain barriers.
fn alias_barrier_licm(module: &mut MirModule) {
    for func in &mut module.functions {
        let instrs = std::mem::take(&mut func.instructions);
        func.instructions = effect_aware_licm_block_with_alias_barriers(instrs);
        for block in &mut func.blocks {
            let instrs = std::mem::take(&mut block.instructions);
            block.instructions = effect_aware_licm_block_with_alias_barriers(instrs);
        }
    }
}

/// Split at alias barriers only; hoisting was already applied by `simple_licm` / `loop_licm`.
pub(crate) fn effect_aware_licm_block_with_alias_barriers(instrs: Vec<MirInstr>) -> Vec<MirInstr> {
    let mut out = Vec::new();
    let mut segment = Vec::new();
    let mut stored_slots_in_segment = HashSet::new();
    for insn in instrs {
        let alias_barrier = matches!(insn.op, MirOp::LoadLocal(slot) if stored_slots_in_segment.contains(&slot))
            || mir_op_is_v12_alias_barrier(&insn.op);
        if alias_barrier && !segment.is_empty() {
            out.extend(segment);
            segment = Vec::new();
            stored_slots_in_segment.clear();
        }
        if insn.dest.is_none() && mir_op_is_stack_barrier(&insn.op) {
            if !segment.is_empty() {
                out.extend(segment);
                segment = Vec::new();
                stored_slots_in_segment.clear();
            }
            out.push(insn);
        } else {
            if let MirOp::StoreLocal(slot) = insn.op {
                stored_slots_in_segment.insert(slot);
            }
            segment.push(insn);
        }
    }
    if !segment.is_empty() {
        out.extend(segment);
    }
    out
}

/// Phase 20 MIR opt v9: nightly default path for v8 alias-barrier LICM.
pub fn mir_opt_v9_default_enabled() -> bool {
    std::env::var("SYX_MIR_OPT_V9")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or_else(|_| {
            std::env::var("SYX_NIGHTLY")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
        })
}

/// Phase 21 MIR opt v10: production default when `SYX_MIR_OPT_V8` is unset.
pub fn mir_opt_v10_prod_default_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V10") {
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(_) => false,
        Err(_) => std::env::var("SYX_MIR_OPT_V8").is_err(),
    }
}

/// Phase 22 MIR opt v11: stability hardening alias for v10 production default.
pub fn mir_opt_v11_stability_enabled() -> bool {
    mir_opt_v10_prod_default_enabled()
}

/// Phase 23 MIR opt v12: production hardening — ActorCall/AwaitPoll alias barriers (ADR-007 v7).
pub fn mir_opt_v12_prod_hardening_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V12") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v11_stability_enabled(),
    }
}

/// Phase 24 MIR opt v13: incremental-cache barrier + TaskSpawn/Channel LICM extensions (ADR-007 v8).
pub fn mir_opt_v13_incr_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V13") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v12_prod_hardening_enabled(),
    }
}

/// Phase 25 MIR opt v14: JIT/Actor deoptimization barrier (ADR-007 v9).
pub fn mir_opt_v14_jit_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V14") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v13_incr_barrier_enabled(),
    }
}

/// Phase 26 MIR opt v15: extends v14 barriers for LLVM AOT / Iterator cross-tier consistency (ADR-007 v10).
pub fn mir_opt_v15_jit_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V15") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v14_jit_barrier_enabled(),
    }
}

/// Phase 27 MIR opt v16: AOT Phase 2 CFG/SSA hardening + dist-Actor send barriers (ADR-007 v11).
pub fn mir_opt_v16_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V16") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v15_jit_barrier_enabled(),
    }
}

/// Phase 28 MIR opt v17: AOT Phase 3 try/async/actor segment barriers (ADR-007 v12).
pub fn mir_opt_v17_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V17") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v16_aot_barrier_enabled(),
    }
}

/// Phase 29 MIR opt v18: enum/witness LICM barriers for AOT Phase 4 (ADR-007 v13).
pub fn mir_opt_v18_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V18") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v17_aot_barrier_enabled(),
    }
}

/// Phase 30 MIR opt v19: CallHost/JSON LICM barriers for AOT Phase 5 (ADR-007 v14).
pub fn mir_opt_v19_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V19") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v18_aot_barrier_enabled(),
    }
}

/// Phase 31 MIR opt v20: GC/relocating LICM barriers for AOT Phase 6 (ADR-007 v15).
pub fn mir_opt_v20_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V20") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v19_aot_barrier_enabled(),
    }
}

/// Phase 32 MIR opt v21: remembered-set / generational GC LICM barriers for AOT Phase 7 (ADR-007 v16).
pub fn mir_opt_v21_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V21") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v20_aot_barrier_enabled(),
    }
}

/// Phase 34 FF1: MIR opt v23 — unified tri-backend GC LICM barriers for AOT Phase 9 (ADR-007 v18).
pub fn mir_opt_v23_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V23") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v22_aot_barrier_enabled(),
    }
}

/// Phase 35 FK1: MIR opt v24 — unified GC promotion LICM barriers for AOT Phase 10 (ADR-007 v19).
pub fn mir_opt_v24_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V24") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v23_aot_barrier_enabled(),
    }
}

/// Phase 36 FP1: MIR opt v25 — GC compaction + JIT tier-2 deopt LICM barriers (ADR-007 v20).
pub fn mir_opt_v25_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V25") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v24_aot_barrier_enabled(),
    }
}

/// Phase 33 MIR opt v22: VM/native generational GC LICM barriers for AOT Phase 8 (ADR-007 v17).
pub fn mir_opt_v22_aot_barrier_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V22") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => mir_opt_v21_aot_barrier_enabled(),
    }
}

pub fn mir_opt_v8_enabled() -> bool {
    match std::env::var("SYX_MIR_OPT_V8") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => true,
    }
}

/// Phase 17 ADR-007: ops that fix stack evaluation order for native Cranelift.
fn mir_op_is_stack_barrier(op: &MirOp) -> bool {
    matches!(
        op,
        MirOp::Print
            | MirOp::ConcatStr
            | MirOp::AwaitSuspend { .. }
            | MirOp::Pop
            | MirOp::Return
            | MirOp::ReturnErr
            | MirOp::StoreLocal(_)
            | MirOp::StoreGlobal(_)
            | MirOp::Call(_)
            | MirOp::CallHost(_)
            | MirOp::CallClosure
            | MirOp::TaskSpawn(_)
            | MirOp::ActorSend(_)
            | MirOp::ActorCall(_)
            | MirOp::AwaitPoll
    )
}

/// Phase 23 v12 / Phase 24 v13 / Phase 25 v14 / Phase 26 v15 / Phase 27 v16 / Phase 28 v17 / Phase 29 v18 / Phase 30 v19 / Phase 31 v20: split LICM segments at actor/await/task/JIT boundaries.
fn mir_op_is_v12_alias_barrier(op: &MirOp) -> bool {
    if mir_opt_v25_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::Print
                | MirOp::CallWitness(_, _)
                | MirOp::CallHost(_)
                | MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
                | MirOp::StoreGlobal(_)
                | MirOp::StoreLocal(_)
                | MirOp::Return
        );
    }
    if mir_opt_v24_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::Print
                | MirOp::CallWitness(_, _)
                | MirOp::CallHost(_)
                | MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
                | MirOp::StoreGlobal(_)
                | MirOp::StoreLocal(_)
                | MirOp::Return
                | MirOp::Pop
        );
    }
    if mir_opt_v23_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::Print
                | MirOp::CallWitness(_, _)
                | MirOp::CallHost(_)
                | MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
                | MirOp::StoreGlobal(_)
                | MirOp::StoreLocal(_)
                | MirOp::Throw
                | MirOp::Catch
        );
    }
    if mir_opt_v22_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::Print
                | MirOp::CallWitness(_, _)
                | MirOp::CallHost(_)
                | MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
                | MirOp::StoreGlobal(_)
                | MirOp::StoreLocal(_)
                | MirOp::Throw
                | MirOp::Catch
        );
    }
    if mir_opt_v21_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::Print
                | MirOp::CallWitness(_, _)
                | MirOp::CallHost(_)
                | MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
                | MirOp::StoreGlobal(_)
                | MirOp::StoreLocal(_)
                | MirOp::Throw
        );
    }
    if mir_opt_v20_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::Print
                | MirOp::CallWitness(_, _)
                | MirOp::CallHost(_)
                | MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
                | MirOp::StoreGlobal(_)
                | MirOp::StoreLocal(_)
        );
    }
    if mir_opt_v19_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::Print
                | MirOp::CallWitness(_, _)
                | MirOp::CallHost(_)
                | MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
        );
    }
    if mir_opt_v18_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::Print
                | MirOp::CallWitness(_, _)
                | MirOp::CallHost(_)
                | MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
        );
    }
    if mir_opt_v17_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
                | MirOp::BeginTry(_)
                | MirOp::EndTry
                | MirOp::CallClosure
        );
    }
    if mir_opt_v16_aot_barrier_enabled() {
        return matches!(
            op,
            MirOp::ActorCall(_)
                | MirOp::ActorSend(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
                | MirOp::ReturnErr
        );
    }
    if mir_opt_v15_jit_barrier_enabled() {
        return matches!(
            op,
            MirOp::ActorCall(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::ActorSend(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
        );
    }
    if mir_opt_v14_jit_barrier_enabled() {
        return matches!(
            op,
            MirOp::ActorCall(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::ActorSend(_)
                | MirOp::IteratorNext
                | MirOp::MakeIterator
        );
    }
    if mir_opt_v13_incr_barrier_enabled() {
        return matches!(
            op,
            MirOp::ActorCall(_)
                | MirOp::AwaitPoll
                | MirOp::AwaitSuspend { .. }
                | MirOp::TaskSpawn(_)
                | MirOp::ActorSend(_)
        );
    }
    mir_opt_v12_prod_hardening_enabled()
        && matches!(op, MirOp::ActorCall(_) | MirOp::AwaitPoll | MirOp::AwaitSuspend { .. })
}

fn licm_hoist_constants_in_segment(mut seg: Vec<MirInstr>) -> Vec<MirInstr> {
    // Phase 98: ConstI32 / ConstBool / ConstStr all materialize onto the MIR value
    // stack. Hoisting them past LoadLocal (or each other) changes Call/CallHost
    // argument order — LLVM/Wasm emit blocks while the VM prefers flat, so a
    // block-only LICM hoist previously made UI wrappers see swapped params.
    // Keep segment order; do not hoist stack-pushing constants.
    let _ = &mut seg;
    seg
}

/// Split at stack barriers; hoist constants only within each segment.
fn effect_aware_licm_block(instrs: Vec<MirInstr>) -> Vec<MirInstr> {
    let mut out = Vec::new();
    let mut segment = Vec::new();
    for insn in instrs {
        if insn.dest.is_none() && mir_op_is_stack_barrier(&insn.op) {
            if !segment.is_empty() {
                out.extend(licm_hoist_constants_in_segment(segment));
                segment = Vec::new();
            }
            out.push(insn);
        } else {
            segment.push(insn);
        }
    }
    if !segment.is_empty() {
        out.extend(licm_hoist_constants_in_segment(segment));
    }
    out
}

/// Phase 14 MIR opt v4: block-local consecutive CSE.
fn block_cse(module: &mut MirModule) {
    for func in &mut module.functions {
        cse_instructions(&mut func.instructions);
        for block in &mut func.blocks {
            cse_instructions(&mut block.instructions);
        }
    }
}

fn cse_instructions(instrs: &mut Vec<MirInstr>) {
    let mut i = 1;
    while i < instrs.len() {
        if instrs[i] == instrs[i - 1] && instrs[i].dest.is_some() {
            instrs.remove(i);
        } else {
            i += 1;
        }
    }
}

/// Phase 15/17 MIR opt v5/v7: effect-aware LICM (ADR-007 stack barriers).
fn simple_licm(module: &mut MirModule) {
    for func in &mut module.functions {
        for block in &mut func.blocks {
            let instrs = std::mem::take(&mut block.instructions);
            block.instructions = effect_aware_licm_block(instrs);
        }
    }
}

/// Phase 16 MIR opt v6: hoist loop-invariant constants within loop body blocks.
fn loop_licm(module: &mut MirModule) {
    for func in &mut module.functions {
        let loop_targets: HashSet<u32> = func
            .blocks
            .iter()
            .flat_map(|b| b.instructions.iter())
            .filter_map(|i| match i.op {
                MirOp::Jump(t) | MirOp::JumpIfFalse(t) | MirOp::JumpIfNil(t) => Some(t),
                _ => None,
            })
            .collect();
        for block in &mut func.blocks {
            if !loop_targets.contains(&block.id) {
                continue;
            }
            let instrs = std::mem::take(&mut block.instructions);
            block.instructions = effect_aware_licm_block(instrs);
        }
    }
}


#[cfg(test)]
mod r30_try_pad_tests {
    use super::*;
    use crate::lower_module;
    use syx_diagnostics::SourceFile;
    use syx_parser::Parser;

    #[test]
    fn r30_try_keeps_return_err_after_optimize() {
        let src = r#"
enum E: Error { case bad }
func g() throws { throw E.bad }
func main() {
  do {
    try g()
    print("value")
  } catch {
    print("caught")
  }
}
"#;
        let module = Parser::parse(SourceFile::new("t.sy", src)).expect("parse");
        let mut mir = lower_module(&module).expect("lower");
        let before = mir
            .functions
            .iter()
            .find(|f| f.name == "main")
            .expect("main");
        let before_has = before
            .blocks
            .iter()
            .any(|b| b.instructions.iter().any(|i| matches!(i.op, MirOp::ReturnErr)));
        optimize(&mut mir);
        let main = mir
            .functions
            .iter()
            .find(|f| f.name == "main")
            .expect("main");
        let has_return_err = main
            .instructions
            .iter()
            .any(|i| matches!(i.op, MirOp::ReturnErr))
            || main.blocks.iter().any(|b| {
                b.instructions
                    .iter()
                    .any(|i| matches!(i.op, MirOp::ReturnErr))
            });
        assert!(
            before_has,
            "ReturnErr must exist before optimize; blocks={:?}",
            main.blocks
                .iter()
                .map(|b| {
                    (
                        b.id,
                        b.successors.clone(),
                        b.instructions
                            .iter()
                            .map(|i| format!("{:?}", i.op))
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>()
        );
        assert!(
            has_return_err,
            "ReturnErr pad must survive optimize; flat={:?} blocks={:?}",
            main.instructions
                .iter()
                .map(|i| format!("{:?}", i.op))
                .collect::<Vec<_>>(),
            main.blocks
                .iter()
                .map(|b| {
                    (
                        b.id,
                        b.successors.clone(),
                        b.instructions
                            .iter()
                            .map(|i| format!("{:?}", i.op))
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>()
        );
    }
}
