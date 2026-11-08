//! S3-I50: trim `CompiledProgram` to entry-point reachability (warm-floor hinge).

use crate::{CompiledProgram, Instruction};
use smol_str::SmolStr;
use std::collections::{HashSet, VecDeque};

/// Drop functions unreachable from `entry_point` / `__init_global`, remapping call indices.
///
/// If any reachable function uses `CallIndirect` (vtable / stack fn index), the table is left
/// unchanged — indices cannot be soundly remapped without a full points-to analysis.
pub fn trim_program_to_entry_reachability(program: &mut CompiledProgram) {
    let n = program.functions.len();
    if n == 0 {
        return;
    }

    let mut seeds = Vec::new();
    for (i, f) in program.functions.iter().enumerate() {
        if f.name == program.entry_point || f.name == "__init_global" {
            seeds.push(i);
        }
    }
    if seeds.is_empty() {
        // Fall back: keep a function named `main` if present.
        if let Some(i) = program.functions.iter().position(|f| f.name == "main") {
            seeds.push(i);
        } else {
            return;
        }
    }

    let mut reachable = HashSet::new();
    let mut queue = VecDeque::new();
    for i in seeds {
        if reachable.insert(i) {
            queue.push_back(i);
        }
    }

    while let Some(idx) = queue.pop_front() {
        let Some(func) = program.functions.get(idx) else {
            continue;
        };
        for ins in &func.instructions {
            if matches!(ins, Instruction::CallIndirect(_)) {
                // Conservative: indirect calls may target any function index on the stack.
                return;
            }
            for callee in call_targets(ins) {
                let callee = callee as usize;
                if callee < n && reachable.insert(callee) {
                    queue.push_back(callee);
                }
            }
        }
    }

    if reachable.len() == n {
        return;
    }

    // Stable order: original index order among survivors.
    let mut kept: Vec<usize> = reachable.into_iter().collect();
    kept.sort_unstable();
    let mut old_to_new = vec![None; n];
    for (new_i, &old_i) in kept.iter().enumerate() {
        old_to_new[old_i] = Some(new_i as u32);
    }

    let mut new_functions = Vec::with_capacity(kept.len());
    for &old_i in &kept {
        let mut func = program.functions[old_i].clone();
        for ins in &mut func.instructions {
            remap_call_indices(ins, &old_to_new);
        }
        new_functions.push(func);
    }

    // Also slim host_fn_names to CallHost targets still referenced (may be empty).
    let mut used_hosts: HashSet<u32> = HashSet::new();
    for func in &new_functions {
        for ins in &func.instructions {
            if let Instruction::CallHost(i) = ins {
                used_hosts.insert(*i);
            }
        }
    }
    if used_hosts.len() < program.host_fn_names.len() {
        let mut host_kept: Vec<u32> = used_hosts.into_iter().collect();
        host_kept.sort_unstable();
        let mut host_old_to_new = vec![None; program.host_fn_names.len()];
        let mut new_hosts = Vec::with_capacity(host_kept.len());
        for (new_i, &old_i) in host_kept.iter().enumerate() {
            if let Some(name) = program.host_fn_names.get(old_i as usize) {
                host_old_to_new[old_i as usize] = Some(new_i as u32);
                new_hosts.push(name.clone());
            }
        }
        for func in &mut new_functions {
            for ins in &mut func.instructions {
                if let Instruction::CallHost(i) = ins {
                    if let Some(Some(n)) = host_old_to_new.get(*i as usize) {
                        *i = *n;
                    }
                }
            }
        }
        program.host_fn_names = new_hosts;
    }

    let kept_names: HashSet<SmolStr> = new_functions.iter().map(|f| f.name.clone()).collect();
    program.functions = new_functions;
    program
        .mir_functions
        .retain(|name, _| kept_names.contains(name));
    program
        .opaque_witnesses
        .retain(|name, _| kept_names.contains(name));
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
        _ => Vec::new(),
    }
}

fn remap_call_indices(ins: &mut Instruction, old_to_new: &[Option<u32>]) {
    let map = |idx: u32| -> u32 {
        old_to_new
            .get(idx as usize)
            .and_then(|x| *x)
            .unwrap_or(idx)
    };
    match ins {
        Instruction::Call(i)
        | Instruction::CallSuper(i)
        | Instruction::ActorCall(i)
        | Instruction::ActorSend(i)
        | Instruction::TaskSpawn(i) => *i = map(*i),
        Instruction::MakeClosure(fn_index, _) => *fn_index = map(*fn_index),
        Instruction::AwaitSuspend { resume_fn, .. } => *resume_fn = map(*resume_fn),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FunctionUnit;

    fn fu(name: &str, body: Vec<Instruction>) -> FunctionUnit {
        FunctionUnit {
            name: SmolStr::new(name),
            param_count: 0,
            local_count: 0,
            instructions: body,
            instruction_spans: Vec::new(),
            local_names: Vec::new(),
            local_types: Vec::new(),
            source_name: SmolStr::default(),
            init_body_pc: 0,
            init_wrap_pc: 0,
            has_begin_try: false,
            can_stack_bind: false,
        }
    }

    #[test]
    fn trim_drops_unreferenced_stdlib_residue() {
        let mut program = CompiledProgram {
            functions: vec![
                fu("main", vec![Instruction::Print, Instruction::Return]),
                fu("Math_abs", vec![Instruction::Return]),
                fu("Array_map", vec![Instruction::Return]),
            ],
            entry_point: SmolStr::new("main"),
            ..CompiledProgram::default()
        };
        trim_program_to_entry_reachability(&mut program);
        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "main");
    }

    #[test]
    fn trim_keeps_callee_and_remaps() {
        let mut program = CompiledProgram {
            functions: vec![
                fu("helper", vec![Instruction::Return]),
                fu("junk", vec![Instruction::Return]),
                fu(
                    "main",
                    vec![Instruction::Call(0), Instruction::Return],
                ),
            ],
            entry_point: SmolStr::new("main"),
            ..CompiledProgram::default()
        };
        trim_program_to_entry_reachability(&mut program);
        assert_eq!(program.functions.len(), 2);
        let main = program
            .functions
            .iter()
            .find(|f| f.name == "main")
            .unwrap();
        assert!(matches!(main.instructions[0], Instruction::Call(0)));
        assert_eq!(program.functions[0].name, "helper");
    }

    #[test]
    fn trim_aborts_on_call_indirect() {
        let mut program = CompiledProgram {
            functions: vec![
                fu(
                    "main",
                    vec![Instruction::CallIndirect(0), Instruction::Return],
                ),
                fu("other", vec![Instruction::Return]),
            ],
            entry_point: SmolStr::new("main"),
            ..CompiledProgram::default()
        };
        trim_program_to_entry_reachability(&mut program);
        assert_eq!(program.functions.len(), 2);
    }
}
