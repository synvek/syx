//! Phase 70 WASM AOT tier breakdown stats + mir_exec opcode histogram.

use std::collections::HashMap;

use wasmparser::{Parser, Payload, TypeRef};

use crate::tier_matrix::{TIER1_NAMES, TIER2A_NAMES, TIER2B_NAMES, TIER2C_NAMES};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AotTierStats {
    pub tier1: u32,
    pub tier2a: u32,
    pub tier2b: u32,
    pub tier2c: u32,
    pub mir_exec: u32,
    /// Phase 92: `__syx_aot_run_op` import call count (runtime opcode fallback).
    pub run_op: u32,
}

impl AotTierStats {
    pub fn direct_total(&self) -> u32 {
        self.tier1 + self.tier2a + self.tier2b + self.tier2c
    }

    pub fn ratio(&self) -> f64 {
        let total = self.direct_total() + self.mir_exec;
        if total == 0 {
            1.0
        } else {
            f64::from(self.direct_total()) / f64::from(total)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CallTier {
    MirExec,
    RunOp,
    Tier1,
    Tier2a,
    Tier2b,
    Tier2c,
    Module,
}

/// Returns `(direct_ops, mir_exec_calls, ratio)` where `ratio = direct / (direct + mir_exec)`.
pub fn measure_aot_ratio(wasm: &[u8]) -> (u32, u32, f64) {
    let stats = measure_aot_tier_stats(wasm);
    (stats.direct_total(), stats.mir_exec, stats.ratio())
}

pub fn measure_aot_tier_stats(wasm: &[u8]) -> AotTierStats {
    let import_tiers = parse_import_tiers(wasm);
    let mut stats = AotTierStats::default();
    let parser = Parser::new(0);
    for payload in parser.parse_all(wasm) {
        let payload = payload.expect("wasm parse");
        if let Payload::CodeSectionEntry(body) = payload {
            for op in body.get_operators_reader().expect("operators") {
                let op = op.expect("op");
                match op {
                    wasmparser::Operator::Call { function_index } => {
                        classify_call(function_index, &import_tiers, &mut stats);
                    }
                    wasmparser::Operator::CallIndirect { .. } => {
                        stats.tier2a += 1;
                    }
                    _ => {
                        stats.tier1 += 1;
                    }
                }
            }
        }
    }
    stats
}

/// Count mir_exec opcode tags (first i32 const before mir_exec call pattern).
pub fn measure_aot_mir_exec_histogram(wasm: &[u8]) -> HashMap<u32, u32> {
    let import_tiers = parse_import_tiers(wasm);
    let mir_exec_idx = import_tiers
        .iter()
        .find(|(_, t)| **t == CallTier::MirExec)
        .map(|(i, _)| *i);
    let Some(mir_exec_idx) = mir_exec_idx else {
        return HashMap::new();
    };
    let mut hist = HashMap::new();
    let mut pending: [Option<i32>; 3] = [None; 3];
    let parser = Parser::new(0);
    for payload in parser.parse_all(wasm) {
        let payload = payload.expect("wasm parse");
        if let Payload::CodeSectionEntry(body) = payload {
            for op in body.get_operators_reader().expect("operators") {
                match op.expect("op") {
                    wasmparser::Operator::I32Const { value } => {
                        pending[0] = pending[1];
                        pending[1] = pending[2];
                        pending[2] = Some(value);
                    }
                    wasmparser::Operator::Call { function_index }
                        if function_index == mir_exec_idx =>
                    {
                        if let Some(op_tag) = pending[0] {
                            *hist.entry(op_tag as u32).or_insert(0) += 1;
                        }
                        pending = [None; 3];
                    }
                    _ => {
                        pending = [None; 3];
                    }
                }
            }
        }
    }
    hist
}

/// Phase 92: count `__syx_aot_run_op` import calls in WASM bytecode.
pub fn measure_aot_run_op_count(wasm: &[u8]) -> u32 {
    measure_aot_tier_stats(wasm).run_op
}

fn parse_import_tiers(wasm: &[u8]) -> HashMap<u32, CallTier> {
    let mut map = HashMap::new();
    let mut func_idx = 0u32;
    let parser = Parser::new(0);
    for payload in parser.parse_all(wasm) {
        let payload = payload.expect("wasm parse");
        if let Payload::ImportSection(imports) = payload {
            for import in imports {
                let import = import.expect("import");
                if matches!(import.ty, TypeRef::Func(_)) {
                    let tier = if import.name == "mir_exec" {
                        CallTier::MirExec
                    } else if import.name == "__syx_aot_run_op" {
                        CallTier::RunOp
                    } else if TIER2C_NAMES.contains(&import.name) {
                        CallTier::Tier2c
                    } else if TIER2B_NAMES.contains(&import.name) {
                        CallTier::Tier2b
                    } else if TIER2A_NAMES.contains(&import.name) {
                        CallTier::Tier2a
                    } else if TIER1_NAMES.contains(&import.name) {
                        CallTier::Tier1
                    } else {
                        CallTier::Tier1
                    };
                    map.insert(func_idx, tier);
                    func_idx += 1;
                }
            }
        }
    }
    map
}

fn classify_call(function_index: u32, import_tiers: &HashMap<u32, CallTier>, stats: &mut AotTierStats) {
    match import_tiers.get(&function_index).copied() {
        Some(CallTier::MirExec) => stats.mir_exec += 1,
        Some(CallTier::RunOp) => stats.run_op += 1,
        Some(CallTier::Tier1) => stats.tier1 += 1,
        Some(CallTier::Tier2a) => stats.tier2a += 1,
        Some(CallTier::Tier2b) => stats.tier2b += 1,
        Some(CallTier::Tier2c) => stats.tier2c += 1,
        Some(CallTier::Module) | None => stats.tier1 += 1,
    }
}
