//! Interpret-path stdlib link: pull only reachable prelude functions with Call remapping.
//! Used when `DriverOptions.stdlib_mir` is false (default `syx run`).

use crate::{is_stdlib_module_path, ModuleGraph};
use indexmap::IndexMap;
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};
use syx_codegen::{CompiledProgram, Instruction};
use syx_mir::{MirModule, MirOp};
use syx_semantic::TypeId;
use syx_stdlib::NumericKind;

/// Build name → (owner program, index) for all stdlib functions in the graph.
pub(crate) fn stdlib_fn_index(
    graph: &ModuleGraph,
) -> HashMap<SmolStr, (&CompiledProgram, usize)> {
    let mut index = HashMap::new();
    for (path, unit) in &graph.modules {
        if !is_stdlib_module_path(path) {
            continue;
        }
        for (i, func) in unit.program.functions.iter().enumerate() {
            index.entry(func.name.clone()).or_insert((&unit.program, i));
        }
    }
    index
}

/// Collect callee names referenced by MIR `Call*` ops (and ConstStr that may name tasks).
pub(crate) fn mir_call_names(mir: &MirModule) -> HashSet<SmolStr> {
    let mut names = HashSet::new();
    for func in &mir.functions {
        for block in &func.blocks {
            for ins in &block.instructions {
                match &ins.op {
                    MirOp::Call(n)
                    | MirOp::CallSuper(n)
                    | MirOp::ActorCall(n)
                    | MirOp::ActorSend(n)
                    | MirOp::TaskSpawn(n) => {
                        names.insert(n.clone());
                    }
                    MirOp::ConstStr(s) => {
                        names.insert(SmolStr::new(s.as_str()));
                    }
                    _ => {}
                }
            }
        }
    }
    names
}

/// Pull `seeds` and their transitive Call-callees from the stdlib index into `dest`.
/// Remaps `Call*` indices by name so deduped module snapshots stay sound.
pub(crate) fn link_stdlib_callees(
    dest: &mut CompiledProgram,
    graph: &ModuleGraph,
    seeds: &HashSet<SmolStr>,
) {
    let index = stdlib_fn_index(graph);
    if index.is_empty() {
        return;
    }
    // Ensure dest has a full host name table so CallHost indices from cleared
    // snapshot modules (global all_host_fn_names order) stay valid.
    if dest.host_fn_names.is_empty() {
        dest.host_fn_names = syx_stdlib::all_host_fn_names()
            .into_iter()
            .map(String::from)
            .collect();
    }
    let mut stack = HashSet::new();
    for seed in seeds {
        ensure_linked(seed, dest, &index, &mut stack);
    }
}

fn ensure_linked(
    name: &SmolStr,
    dest: &mut CompiledProgram,
    index: &HashMap<SmolStr, (&CompiledProgram, usize)>,
    stack: &mut HashSet<SmolStr>,
) {
    if dest.functions.iter().any(|f| f.name == *name) {
        return;
    }
    let Some(&(src_prog, src_idx)) = index.get(name) else {
        return;
    };
    if !stack.insert(name.clone()) {
        return; // cycle
    }
    let Some(src_func) = src_prog.functions.get(src_idx) else {
        stack.remove(name);
        return;
    };
    // Link callees first (depth-first).
    let callee_names: Vec<SmolStr> = src_func
        .instructions
        .iter()
        .flat_map(call_targets)
        .filter_map(|ti| src_prog.functions.get(ti as usize).map(|f| f.name.clone()))
        .collect();
    for callee in &callee_names {
        ensure_linked(callee, dest, index, stack);
    }
    if dest.functions.iter().any(|f| f.name == *name) {
        stack.remove(name);
        return;
    }
    let mut cloned = src_func.clone();
    for ins in &mut cloned.instructions {
        remap_calls_by_name(ins, src_prog, dest);
    }
    dest.functions.push(cloned);
    stack.remove(name);
}

fn call_targets(ins: &Instruction) -> Vec<u32> {
    match *ins {
        Instruction::Call(i)
        | Instruction::CallSuper(i)
        | Instruction::ActorCall(i)
        | Instruction::ActorSend(i)
        | Instruction::TaskSpawn(i) => vec![i],
        Instruction::MakeClosure(fn_index, _) => vec![fn_index],
        Instruction::AwaitSuspend { resume_fn, .. } => vec![resume_fn],
        Instruction::ConstFnIndex(i) => vec![i],
        _ => Vec::new(),
    }
}

fn remap_calls_by_name(ins: &mut Instruction, src: &CompiledProgram, dest: &CompiledProgram) {
    let map_idx = |old: u32| -> u32 {
        let Some(callee) = src.functions.get(old as usize) else {
            return old;
        };
        dest.functions
            .iter()
            .position(|f| f.name == callee.name)
            .map(|i| i as u32)
            .unwrap_or(old)
    };
    match ins {
        Instruction::Call(i)
        | Instruction::CallSuper(i)
        | Instruction::ActorCall(i)
        | Instruction::ActorSend(i)
        | Instruction::TaskSpawn(i) => *i = map_idx(*i),
        Instruction::MakeClosure(fn_index, _) => *fn_index = map_idx(*fn_index),
        Instruction::AwaitSuspend { resume_fn, .. } => *resume_fn = map_idx(*resume_fn),
        Instruction::ConstFnIndex(i) => *i = map_idx(*i),
        _ => {}
    }
}

/// Look up cached stdlib exports for `path`, or `None` to fall back to parse.
pub(crate) fn cached_exports_for(
    graph: &ModuleGraph,
    path: &std::path::Path,
) -> Option<IndexMap<SmolStr, TypeId>> {
    if graph.stdlib_exports.is_empty() {
        return None;
    }
    graph.stdlib_exports.get(path).cloned()
}

/// Encode TypeId for prelude snapshot (stable, lossy only for exotic forms → Named/Unknown).
pub(crate) fn encode_type_id(ty: &TypeId) -> String {
    match ty {
        TypeId::Void => "void".into(),
        TypeId::Bool => "bool".into(),
        TypeId::String => "string".into(),
        TypeId::Unknown => "unknown".into(),
        TypeId::Numeric(k) => format!("num:{}", *k as u8),
        TypeId::Named(n) => format!("named:{n}"),
        TypeId::GenericParam(n) => format!("gparam:{n}"),
        TypeId::Optional(inner) => format!("opt:{}", encode_type_id(inner)),
        TypeId::Array(inner) => format!("arr:{}", encode_type_id(inner)),
        TypeId::Set(inner) => format!("set:{}", encode_type_id(inner)),
        TypeId::Reference(inner) => format!("ref:{}", encode_type_id(inner)),
        TypeId::MutReference(inner) => format!("mutref:{}", encode_type_id(inner)),
        TypeId::OpaqueSome(inner) => format!("some:{}", encode_type_id(inner)),
        TypeId::OpaqueAny(inner) => format!("any:{}", encode_type_id(inner)),
        TypeId::Dictionary { key, value } => {
            format!("dict:{}|{}", encode_type_id(key), encode_type_id(value))
        }
        TypeId::Result { ok, err } => {
            format!("result:{}|{}", encode_type_id(ok), encode_type_id(err))
        }
        TypeId::Simd { lanes, element } => format!("simd:{}:{}", lanes, *element as u8),
        TypeId::GenericFn { name, type_params } => {
            format!("gfn:{name}:{}", type_params.join(","))
        }
        TypeId::Tuple { elements } => {
            let inner = elements
                .iter()
                .map(|(lab, t)| match lab {
                    Some(l) => format!("{l}={}", encode_type_id(t)),
                    None => encode_type_id(t),
                })
                .collect::<Vec<_>>()
                .join(";");
            format!("tuple:{inner}")
        }
    }
}

pub(crate) fn decode_type_id(wire: &str) -> TypeId {
    if let Some(rest) = wire.strip_prefix("named:") {
        return TypeId::Named(SmolStr::new(rest));
    }
    if let Some(rest) = wire.strip_prefix("num:") {
        let code = rest.parse::<u8>().unwrap_or(255);
        return numeric_from_u8(code)
            .map(TypeId::Numeric)
            .unwrap_or(TypeId::Unknown);
    }
    if let Some(rest) = wire.strip_prefix("opt:") {
        return TypeId::Optional(Box::new(decode_type_id(rest)));
    }
    if let Some(rest) = wire.strip_prefix("arr:") {
        return TypeId::Array(Box::new(decode_type_id(rest)));
    }
    if let Some(rest) = wire.strip_prefix("set:") {
        return TypeId::Set(Box::new(decode_type_id(rest)));
    }
    if let Some(rest) = wire.strip_prefix("ref:") {
        return TypeId::Reference(Box::new(decode_type_id(rest)));
    }
    if let Some(rest) = wire.strip_prefix("mutref:") {
        return TypeId::MutReference(Box::new(decode_type_id(rest)));
    }
    if let Some(rest) = wire.strip_prefix("some:") {
        return TypeId::OpaqueSome(Box::new(decode_type_id(rest)));
    }
    if let Some(rest) = wire.strip_prefix("any:") {
        return TypeId::OpaqueAny(Box::new(decode_type_id(rest)));
    }
    if let Some(rest) = wire.strip_prefix("dict:") {
        let mut parts = rest.splitn(2, '|');
        let k = parts.next().unwrap_or("unknown");
        let v = parts.next().unwrap_or("unknown");
        return TypeId::Dictionary {
            key: Box::new(decode_type_id(k)),
            value: Box::new(decode_type_id(v)),
        };
    }
    if let Some(rest) = wire.strip_prefix("result:") {
        let mut parts = rest.splitn(2, '|');
        let ok = parts.next().unwrap_or("unknown");
        let err = parts.next().unwrap_or("unknown");
        return TypeId::Result {
            ok: Box::new(decode_type_id(ok)),
            err: Box::new(decode_type_id(err)),
        };
    }
    if let Some(rest) = wire.strip_prefix("simd:") {
        let mut parts = rest.splitn(2, ':');
        let lanes = parts.next().and_then(|s| s.parse().ok()).unwrap_or(4);
        let element = parts
            .next()
            .and_then(|s| s.parse().ok())
            .and_then(numeric_from_u8)
            .unwrap_or(NumericKind::I32);
        return TypeId::Simd { lanes, element };
    }
    if let Some(rest) = wire.strip_prefix("gparam:") {
        return TypeId::GenericParam(SmolStr::new(rest));
    }
    if let Some(rest) = wire.strip_prefix("gfn:") {
        let mut parts = rest.splitn(2, ':');
        let name = SmolStr::new(parts.next().unwrap_or("Fn"));
        let type_params = parts
            .next()
            .unwrap_or("")
            .split(',')
            .filter(|s| !s.is_empty())
            .map(SmolStr::new)
            .collect();
        return TypeId::GenericFn { name, type_params };
    }
    if let Some(rest) = wire.strip_prefix("tuple:") {
        let elements = rest
            .split(';')
            .filter(|s| !s.is_empty())
            .map(|part| {
                if let Some((lab, ty)) = part.split_once('=') {
                    (Some(SmolStr::new(lab)), decode_type_id(ty))
                } else {
                    (None, decode_type_id(part))
                }
            })
            .collect();
        return TypeId::Tuple { elements };
    }
    match wire {
        "void" => TypeId::Void,
        "bool" => TypeId::Bool,
        "string" => TypeId::String,
        _ => TypeId::Unknown,
    }
}

fn numeric_from_u8(code: u8) -> Option<NumericKind> {
    Some(match code {
        0 => NumericKind::I8,
        1 => NumericKind::I16,
        2 => NumericKind::I32,
        3 => NumericKind::I64,
        4 => NumericKind::U8,
        5 => NumericKind::U16,
        6 => NumericKind::U32,
        7 => NumericKind::U64,
        8 => NumericKind::Int,
        9 => NumericKind::UInt,
        10 => NumericKind::F32,
        11 => NumericKind::F64,
        12 => NumericKind::I128,
        13 => NumericKind::U128,
        14 => NumericKind::Float,
        15 => NumericKind::Double,
        _ => return None,
    })
}
