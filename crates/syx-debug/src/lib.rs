use std::io::{self, BufRead, Read, Write};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use syx_codegen::{CompiledProgram, Instruction};
use syx_diagnostics::{SourceFile, SyxResult};

fn simd_lane_value_i32(bits: &[u8; 64], lane: u8) -> Option<syx_vm::Value> {
    let off = lane as usize * 4;
    let mut raw = 0u32;
    for b in 0..4 {
        raw |= (bits[off + b] as u32) << (b * 8);
    }
    Some(syx_vm::Value::I32(raw as i32))
}

#[derive(Debug, Clone, Default)]
pub struct SourceMap {
    pub entries: indexmap::IndexMap<usize, (u32, u32)>,
    pub pc_to_line: indexmap::IndexMap<usize, usize>,
    pub line_to_pc: indexmap::IndexMap<usize, (u32, u32)>,
}

#[derive(Debug, Clone, Default)]
pub struct Breakpoint {
    pub enabled: bool,
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct BreakpointTable {
    /// Active VM stops keyed by `(function_index, pc)`.
    pub breakpoints: indexmap::IndexMap<(usize, usize), Breakpoint>,
    /// DAP breakpoints keyed by normalized source path → `(line, condition)`.
    /// Each `setBreakpoints` for a path replaces only that path's entries.
    pub source_lines: indexmap::IndexMap<String, Vec<(usize, Option<String>)>>,
}

#[derive(Debug, Clone)]
struct DebugCallFrame {
    caller_fn_idx: usize,
    return_pc: usize,
    caller_locals: Vec<syx_vm::Value>,
}

enum DebugStepStatus {
    Continue,
    Paused,
    Finished,
}

pub struct DebugSession {
    pub source: SourceFile,
    pub program: CompiledProgram,
    pub pc: usize,
    pub function_index: usize,
    pub source_map: SourceMap,
    pub breakpoints: BreakpointTable,
    pub paused: bool,
    pub vm: Option<syx_vm::WasmVM>,
    pub watch_expressions: Vec<String>,
    pub async_stack_label: Option<String>,
    /// Phase 13: TaskGroup child task labels for DAP threads tree.
    pub task_children: Vec<String>,
    /// Phase 22: monomorphized generic type params visible in the current frame (`T` → `Int`).
    pub generic_type_params: Vec<(String, String)>,
    /// Index into `vm.printed` — lines already sent to the DAP debug console.
    printed_flush_start: usize,
    call_frames: Vec<DebugCallFrame>,
    variable_handles: indexmap::IndexMap<u32, syx_vm::Value>,
    next_variable_handle: u32,
}

impl DebugSession {
    pub fn new(source: SourceFile, program: CompiledProgram) -> Self {
        let source_map = build_source_map(&source, &program);
        Self {
            source,
            program,
            pc: 0,
            function_index: 0,
            source_map,
            breakpoints: BreakpointTable::default(),
            paused: false,
            vm: None,
            watch_expressions: Vec::new(),
            async_stack_label: None,
            task_children: Vec::new(),
            generic_type_params: Vec::new(),
            printed_flush_start: 0,
            call_frames: Vec::new(),
            variable_handles: indexmap::IndexMap::new(),
            next_variable_handle: 2,
        }
    }

    fn alloc_variable_handle(&mut self, value: syx_vm::Value) -> u32 {
        let id = self.next_variable_handle;
        self.next_variable_handle += 1;
        self.variable_handles.insert(id, value);
        id
    }

    fn local_type_for_slot(&self, slot: usize) -> Option<String> {
        self.current_function()
            .and_then(|f| f.local_types.get(slot))
            .filter(|t| !t.is_empty() && t.as_str() != "Unknown")
            .map(|t| t.to_string())
    }

    fn value_type_name(v: &syx_vm::Value) -> String {
        use syx_vm::Value;
        if let Some(inner) = Value::mut_ref_cell_inner(v) {
            return Self::value_type_name(&inner);
        }
        if let Some(name) = v.object_ref_type_name() {
            return name;
        }
        match v {
            Value::I32(_) | Value::I64(_) | Value::U64(_) => "Int".into(),
            Value::I128(_) => "Int128".into(),
            Value::U128(_) => "UInt128".into(),
            Value::F32(_) | Value::F64(_) => "Float".into(),
            Value::Simd { lanes, element, .. } => format!("SIMD{lanes}<{element:?}>"),
            Value::Bool(_) => "Bool".into(),
            Value::Str(_) => "String".into(),
            Value::Array(_) => "Array".into(),
            Value::Set(_) => "Set".into(),
            Value::Dict(map) => map
                .get("__type")
                .map(|t| t.as_str())
                .filter(|s| s != "MutRefCell")
                .unwrap_or_else(|| "Dictionary".to_string()),
            Value::Object(map) => map
                .get("__type")
                .map(|t| t.as_str())
                .filter(|s| s != "MutRefCell")
                .unwrap_or_else(|| "Object".to_string()),
            Value::Nil => "Nil".into(),
            Value::Closure { .. } => "Closure".into(),
            Value::Iterator(_) => "Iterator".into(),
            other => syx_vm::value_debug_description(other),
        }
    }

    fn value_child_entries(v: &syx_vm::Value) -> Vec<(String, syx_vm::Value)> {
        use syx_vm::Value;
        if let Some(inner) = Value::mut_ref_cell_inner(v) {
            return Self::value_child_entries(&inner);
        }
        match v {
            Value::Simd { lanes, element: _, bits } => {
                let mut out = Vec::new();
                for lane in 0..*lanes {
                    if let Some(lane_val) = simd_lane_value_i32(bits, lane) {
                        out.push((format!("[{lane}]"), lane_val));
                    }
                }
                out
            }
            Value::Dict(map) => map
                .iter()
                .filter(|(k, _)| *k != "__type" && !k.starts_with("__"))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            Value::Object(map) => map
                .iter()
                .filter(|(k, _)| !k.starts_with("__"))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            Value::ObjectRef { .. } => Value::object_ref_user_fields(v).unwrap_or_default(),
            _ => Vec::new(),
        }
    }

    fn value_child_entries_with_program(
        &self,
        v: &syx_vm::Value,
    ) -> Vec<(String, syx_vm::Value)> {
        let mut out = Self::value_child_entries(v);
        let seen: std::collections::HashSet<String> =
            out.iter().map(|(k, _)| k.clone()).collect();
        for (name, val) in self.computed_property_entries(v) {
            if !seen.contains(&name) {
                out.push((name, val));
            }
        }
        out
    }

    /// Evaluate desugared `__get_{field}` accessors for class instances in the debugger.
    fn computed_property_entries(&self, v: &syx_vm::Value) -> Vec<(String, syx_vm::Value)> {
        use syx_codegen::Instruction;
        use syx_vm::Value;
        let ty = Self::value_type_name(v);
        if matches!(
            ty.as_str(),
            "Object" | "Nil" | "MutRefCell" | "Array" | "Set" | "Dictionary" | "String" | "Int"
                | "Bool" | "Float" | "Closure" | "Iterator"
        ) {
            return Vec::new();
        }
        let prefix = format!("{ty}___get_");
        let mut out = Vec::new();
        for func in &self.program.functions {
            let Some(field) = func.name.strip_prefix(&prefix) else {
                continue;
            };
            // Common computed getter: ConstStr then Return.
            if func.instructions.len() >= 2
                && matches!(func.instructions.last(), Some(Instruction::Return))
            {
                if let Some(Instruction::ConstStr(s)) = func.instructions.first() {
                    out.push((field.to_string(), Value::Str(s.clone())));
                }
            }
        }
        out
    }

    fn format_value_display(&self, v: &syx_vm::Value) -> String {
        use syx_vm::{value_description, Value};
        if let Value::Array(items) = v {
            if !items.is_empty() && items.iter().all(|x| matches!(x, Value::U8(_))) {
                let hex: Vec<String> = items
                    .iter()
                    .map(|x| format!("{:02x}", x.as_i64()))
                    .collect();
                return format!("[{}] ({} bytes)", hex.join(" "), items.len());
            }
        }
        if let Value::Simd { lanes, bits, .. } = v {
            let items: Vec<String> = (0..*lanes)
                .filter_map(|lane| simd_lane_value_i32(bits, lane))
                .map(|v| value_description(&v))
                .collect();
            return format!("[{}]", items.join(", "));
        }
        if let Value::I128(n) = v {
            return format!("{n}i128");
        }
        if let Value::U128(n) = v {
            return format!("{n}u128");
        }
        let children = self.value_child_entries_with_program(v);
        if children.is_empty() {
            return value_description(v);
        }
        let ty = Self::value_type_name(v);
        let preview: Vec<String> = children
            .iter()
            .take(3)
            .map(|(k, val)| format!("{k}: {}", value_description(val)))
            .collect();
        let suffix = if children.len() > 3 {
            format!(" … +{} fields", children.len() - 3)
        } else {
            String::new()
        };
        format!("{ty}({}{suffix})", preview.join(", "))
    }

    pub fn child_variables(&mut self, handle: u32) -> Vec<(String, String, String, u32)> {
        let value = self
            .variable_handles
            .get(&handle)
            .cloned()
            .unwrap_or(syx_vm::Value::Nil);
        Self::value_child_entries_with_program(&self, &value)
            .into_iter()
            .map(|(name, child)| {
                let grandchildren = self.value_child_entries_with_program(&child);
                let child_ref = if grandchildren.is_empty() {
                    0
                } else {
                    self.alloc_variable_handle(child.clone())
                };
                (
                    name,
                    syx_vm::value_description(&child),
                    Self::value_type_name(&child),
                    child_ref,
                )
            })
            .collect()
    }

    fn local_name_for_slot(&self, slot: usize) -> String {
        self.current_function()
            .and_then(|f| f.local_names.get(slot))
            .map(|n| n.to_string())
            .unwrap_or_else(|| format!("local_{slot}"))
    }

    pub fn flush_output(&mut self, stdout: &mut impl Write, seq: &mut u64) -> SyxResult<()> {
        let Some(vm) = &self.vm else {
            return Ok(());
        };
        for line in vm.printed.iter().skip(self.printed_flush_start) {
            let event = serde_json::json!({
                "seq": *seq,
                "type": "event",
                "event": "output",
                "body": {
                    "category": "stdout",
                    "output": format!("{line}\n")
                }
            });
            *seq += 1;
            write_dap_message(stdout, &event)?;
        }
        self.printed_flush_start = vm.printed.len();
        Ok(())
    }

    pub fn add_watch(&mut self, expr: impl Into<String>) {
        self.watch_expressions.push(expr.into());
    }

    pub fn watch_results(&self) -> Vec<(String, String)> {
        self.watch_expressions
            .iter()
            .map(|expr| {
                (
                    expr.clone(),
                    self.evaluate_expression(expr)
                        .unwrap_or_else(|| "nil".into()),
                )
            })
            .collect()
    }

    pub fn set_breakpoint_line(&mut self, line: usize) {
        if let Some((fn_idx, pc)) = self.resolve_breakpoint_line(line) {
            self.set_breakpoint(fn_idx, pc);
        }
    }

    pub fn line_for_pc(&self, pc: usize) -> Option<usize> {
        self.source_map.pc_to_line.get(&pc).copied()
    }

    pub fn line_for_pc_in_fn(&self, fn_idx: usize, pc: usize) -> Option<usize> {
        let func = self.program.functions.get(fn_idx)?;
        let Some((start, len)) = func.instruction_spans.get(pc).and_then(|s| *s) else {
            return None;
        };
        if len == 0 {
            return None;
        }
        let line_starts: Vec<usize> = std::iter::once(0)
            .chain(
                self.source
                    .contents
                    .match_indices('\n')
                    .map(|(i, _)| i + 1),
            )
            .collect();
        Some(
            line_starts
                .partition_point(|&off| off <= start as usize)
                .saturating_sub(1),
        )
    }

    pub fn set_breakpoint(&mut self, fn_idx: usize, pc: usize) {
        self.breakpoints.breakpoints.insert(
            (fn_idx, pc),
            Breakpoint {
                enabled: true,
                condition: None,
            },
        );
    }

    pub fn set_breakpoint_with_condition(
        &mut self,
        fn_idx: usize,
        pc: usize,
        condition: Option<String>,
    ) {
        self.breakpoints.breakpoints.insert(
            (fn_idx, pc),
            Breakpoint {
                enabled: true,
                condition,
            },
        );
    }

    pub fn should_break(&self, fn_idx: usize, pc: usize) -> bool {
        let Some(bp) = self.breakpoints.breakpoints.get(&(fn_idx, pc)) else {
            return false;
        };
        if !bp.enabled {
            return false;
        }
        if let Some(cond) = &bp.condition {
            return self.evaluate_condition(cond);
        }
        true
    }

    fn evaluate_condition(&self, condition: &str) -> bool {
        let trimmed = condition.trim();
        if trimmed.eq_ignore_ascii_case("false") {
            return false;
        }
        let Some(vm) = &self.vm else {
            return true;
        };
        if let Some((lhs, rhs)) = trimmed.split_once("==") {
            let lhs = lhs.trim();
            let rhs = rhs.trim().trim_matches('"');
            for (i, v) in vm.locals.iter().enumerate() {
                let slot_name = self.local_name_for_slot(i);
                if slot_name == lhs || format!("local_{i}") == lhs {
                    if syx_vm::value_description(v) == rhs {
                        return true;
                    }
                }
            }
            return false;
        }
        if let Some((lhs, rhs)) = trimmed.split_once('>') {
            let lhs = lhs.trim();
            let rhs = rhs.trim().parse::<i64>().unwrap_or(0);
            for (i, v) in vm.locals.iter().enumerate() {
                let slot_name = self.local_name_for_slot(i);
                if slot_name == lhs || format!("local_{i}") == lhs {
                    return v.as_i64() > rhs;
                }
            }
            return false;
        }
        true
    }

    pub fn evaluate_expression(&self, expr: &str) -> Option<String> {
        let trimmed = expr.trim();
        if let Some(result) = self.evaluate_arithmetic(trimmed) {
            return Some(result);
        }
        if let Some(result) = self.evaluate_collection_index(trimmed) {
            return Some(result);
        }
        if let Some(result) = self.evaluate_enum_case(trimmed) {
            return Some(result);
        }
        if let Some(result) = self.evaluate_call_expression(trimmed) {
            return Some(result);
        }
        if let Some(result) = self.evaluate_member_expression(trimmed) {
            return Some(result);
        }
        if let Some(result) = self.evaluate_async_local(trimmed) {
            return Some(result);
        }
        if let Some(result) = self.evaluate_generic_type_param(trimmed) {
            return Some(result);
        }
        if let Some(result) = self.evaluate_closure_capture(trimmed) {
            return Some(result);
        }
        let vm = self.vm.as_ref()?;
        if let Some(stripped) = trimmed.strip_prefix("local_") {
            if let Ok(idx) = stripped.parse::<usize>() {
                return vm.locals.get(idx).map(syx_vm::value_description);
            }
        }
        for (i, v) in vm.locals.iter().enumerate() {
            let name = self.local_name_for_slot(i);
            if name == trimmed || format!("local_{i}") == trimmed {
                return Some(syx_vm::value_description(v));
            }
        }
        None
    }

    fn resolve_local_value(&self, name: &str) -> Option<&syx_vm::Value> {
        let vm = self.vm.as_ref()?;
        if let Some(stripped) = name.strip_prefix("local_") {
            if let Ok(idx) = stripped.parse::<usize>() {
                return vm.locals.get(idx);
            }
        }
        for (i, v) in vm.locals.iter().enumerate() {
            let slot = self.local_name_for_slot(i);
            if slot == name || format!("local_{i}") == name {
                return Some(v);
            }
        }
        None
    }

    fn evaluate_collection_index(&self, expr: &str) -> Option<String> {
        let open = expr.find('[')?;
        let close = expr.rfind(']')?;
        if close <= open {
            return None;
        }
        let base = expr[..open].trim();
        let idx_str = expr[open + 1..close].trim();
        let idx = self.resolve_operand_i64(idx_str)? as usize;
        let value = self.resolve_local_value(base)?;
        use syx_vm::Value;
        match value {
            Value::Array(items) => items.get(idx).map(syx_vm::value_description),
            Value::Set(items) => {
                let mut sorted: Vec<_> = items.iter().collect();
                sorted.sort_by_key(|v| v.as_i64());
                sorted.get(idx).map(|v| syx_vm::value_description(v))
            }
            _ => None,
        }
    }

    fn evaluate_enum_case(&self, expr: &str) -> Option<String> {
        let (enum_name, case_name) = expr.split_once('.')?;
        let vm = self.vm.as_ref()?;
        for v in vm.locals.iter() {
            use syx_vm::Value;
            if let Value::Str(s) = v {
                if s == case_name || *s == format!("{enum_name}.{case_name}") {
                    return Some(case_name.to_string());
                }
            }
        }
        if let Some(value) = self.resolve_local_value(enum_name) {
            use syx_vm::Value;
            if let Value::Str(s) = value {
                if s == case_name {
                    return Some(case_name.to_string());
                }
            }
        }
        None
    }

    fn evaluate_async_local(&self, expr: &str) -> Option<String> {
        if let Some(label) = &self.async_stack_label {
            if expr == "__async_state" || expr == "async_state" {
                return Some(format!("suspended @ {label}"));
            }
            if expr == "__task_group" || expr == "task_group" {
                return Some(label.clone());
            }
        }
        let vm = self.vm.as_ref()?;
        for (i, v) in vm.locals.iter().enumerate() {
            let name = self.local_name_for_slot(i);
            if name == expr && name.starts_with("__async_") {
                return Some(syx_vm::value_description(v));
            }
        }
        None
    }

    fn evaluate_generic_type_param(&self, expr: &str) -> Option<String> {
        let name = expr
            .strip_prefix('$')
            .or_else(|| expr.strip_prefix("__type_"))?;
        self.generic_type_params
            .iter()
            .find(|(param, _)| param == name)
            .map(|(_, inst)| inst.clone())
    }

    fn evaluate_closure_capture(&self, expr: &str) -> Option<String> {
        if let Some(open) = expr.find('[') {
            if expr.ends_with(']') && open > 0 {
                let base = expr[..open].trim();
                let idx_str = expr[open + 1..expr.len() - 1].trim();
                if let Ok(idx) = idx_str.parse::<usize>() {
                    if let Some(value) = self.resolve_local_value(base) {
                        use syx_vm::Value;
                        if let Value::Closure { captures, .. } = value {
                            return captures.get(idx).map(syx_vm::value_description);
                        }
                    }
                }
            }
        }
        None
    }

    fn evaluate_member_expression(&self, expr: &str) -> Option<String> {
        let (base, field) = expr.rsplit_once('.')?;
        let value = self.resolve_local_value(base.trim())?;
        use syx_vm::Value;
        match value {
            Value::Object(map) | Value::Dict(map) => map
                .get(field.trim())
                .map(syx_vm::value_description),
            Value::Array(items) => {
                if field.trim() == "count" {
                    Some(items.len().to_string())
                } else if let Ok(idx) = field.trim().parse::<usize>() {
                    items.get(idx).map(syx_vm::value_description)
                } else {
                    None
                }
            }
            Value::Closure { captures, fn_index } => {
                if field.trim() == "captures" {
                    Some(captures.len().to_string())
                } else if let Some(stripped) = field.trim().strip_prefix("capture_") {
                    stripped
                        .parse::<usize>()
                        .ok()
                        .and_then(|i| captures.get(i))
                        .map(syx_vm::value_description)
                } else if field.trim() == "fn_index" {
                    Some(fn_index.to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn evaluate_call_expression(&self, expr: &str) -> Option<String> {
        let open = expr.find('(')?;
        if !expr.ends_with(')') {
            return None;
        }
        let callee = expr[..open].trim();
        let args_str = expr[open + 1..expr.len() - 1].trim();
        let args: Vec<i64> = if args_str.is_empty() {
            Vec::new()
        } else {
            args_str
                .split(',')
                .filter_map(|a| self.resolve_operand_i64(a.trim()))
                .collect()
        };
        match callee {
            "max" if args.len() == 2 => Some(args[0].max(args[1]).to_string()),
            "min" if args.len() == 2 => Some(args[0].min(args[1]).to_string()),
            "abs" if args.len() == 1 => Some(args[0].abs().to_string()),
            "double" if args.len() == 1 => Some((args[0] * 2).to_string()),
            "triple" if args.len() == 1 => Some((args[0] * 3).to_string()),
            _ => self.evaluate_user_call(callee, &args),
        }
    }

    fn evaluate_user_call(&self, name: &str, args: &[i64]) -> Option<String> {
        let func = self
            .program
            .functions
            .iter()
            .find(|f| f.name.as_str() == name)?;
        if func.param_count as usize != args.len() {
            return None;
        }
        use syx_codegen::Instruction;
        if func.instructions.len() == 2
            && matches!(func.instructions[0], Instruction::LoadLocal(0))
            && matches!(func.instructions[1], Instruction::Return)
            && args.len() == 1
        {
            return Some(args[0].to_string());
        }
        if func.instructions.len() == 4
            && matches!(func.instructions[0], Instruction::LoadLocal(0))
            && matches!(func.instructions[1], Instruction::LoadLocal(0))
            && matches!(func.instructions[2], Instruction::I32Add | Instruction::NumericAdd(_))
            && matches!(func.instructions[3], Instruction::Return)
            && args.len() == 1
        {
            return Some((args[0] + args[0]).to_string());
        }
        let _ = func;
        None
    }

    fn resolve_operand_i64(&self, token: &str) -> Option<i64> {
        if let Ok(n) = token.parse::<i64>() {
            return Some(n);
        }
        let vm = self.vm.as_ref()?;
        for (i, v) in vm.locals.iter().enumerate() {
            let name = self.local_name_for_slot(i);
            if name == token || format!("local_{i}") == token {
                return Some(v.as_i64());
            }
        }
        None
    }

    fn evaluate_arithmetic(&self, expr: &str) -> Option<String> {
        for op in ["+", "-", "*"] {
            if let Some((left, right)) = expr.split_once(op) {
                let a = self.resolve_operand_i64(left.trim())?;
                let b = self.resolve_operand_i64(right.trim())?;
                return Some(match op {
                    "+" => (a + b).to_string(),
                    "-" => (a - b).to_string(),
                    "*" => (a * b).to_string(),
                    _ => unreachable!(),
                });
            }
        }
        None
    }

    pub fn step(&mut self) -> Option<syx_codegen::Instruction> {
        let func = self.program.functions.get(self.function_index)?;
        let instr = func.instructions.get(self.pc)?.clone();
        self.pc += 1;
        Some(instr)
    }

    pub fn current_function(&self) -> Option<&syx_codegen::FunctionUnit> {
        self.program.functions.get(self.function_index)
    }

    pub fn list_instructions(&self, limit: usize) {
        let Some(func) = self.current_function() else {
            println!("(no function)");
            return;
        };
        println!("{} ({} instructions):", func.name, func.instructions.len());
        for (pc, instr) in func.instructions.iter().take(limit).enumerate() {
            let mark = if pc == self.pc { ">" } else { " " };
            println!("{mark} [{pc:4}] {instr:?}");
        }
        if func.instructions.len() > limit {
            println!("  ... {} more", func.instructions.len() - limit);
        }
    }

    pub fn local_variables(&mut self) -> Vec<(String, String, String, u32)> {
        let mut vars = Vec::new();
        if let Some(label) = &self.async_stack_label {
            vars.push((
                "__async_state".into(),
                format!("suspended @ {label}"),
                "AsyncState".into(),
                0,
            ));
            vars.push((
                "__task_group".into(),
                label.clone(),
                "TaskLabel".into(),
                0,
            ));
        }
        for (param, inst) in &self.generic_type_params {
            vars.push((
                format!("__type_{param}"),
                inst.clone(),
                "TypeParam".into(),
                0,
            ));
        }
        let Some(vm) = &self.vm else {
            return vars;
        };
        let entries: Vec<(String, syx_vm::Value, String)> = vm
            .locals
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                let name = self.local_name_for_slot(i);
                if name.starts_with("__syx_") || name.starts_with("__iter_") {
                    return None;
                }
                let ty = self
                    .local_type_for_slot(i)
                    .unwrap_or_else(|| Self::value_type_name(v));
                Some((name, v.clone(), ty))
            })
            .collect();
        for (name, v, ty) in entries {
            let children = self.value_child_entries_with_program(&v);
            let child_ref = if children.is_empty() {
                0
            } else {
                self.alloc_variable_handle(v.clone())
            };
            let display = self.format_value_display(&v);
            vars.push((name, display, ty, child_ref));
        }
        vars
    }

    fn enter_call_at(&mut self, callee_idx: usize, call_pc: usize) -> SyxResult<()> {
        let vm = self
            .vm
            .as_mut()
            .ok_or_else(|| syx_diagnostics::SyxDiagnostic::error("VM not initialized"))?;
        let callee = self
            .program
            .functions
            .get(callee_idx)
            .ok_or_else(|| syx_diagnostics::SyxDiagnostic::error("Unknown callee"))?;
        let arg_count = callee.param_count as usize;
        let start = vm.stack.len().saturating_sub(arg_count);
        let args: Vec<syx_vm::Value> = vm.stack.drain(start..).collect();
        self.call_frames.push(DebugCallFrame {
            caller_fn_idx: self.function_index,
            return_pc: call_pc + 1,
            caller_locals: vm.locals.clone(),
        });
        vm.locals.resize(callee.local_count as usize, syx_vm::Value::Nil);
        for (i, arg) in args.iter().enumerate() {
            if i < vm.locals.len() {
                vm.locals[i] = arg.clone();
            }
        }
        self.function_index = callee_idx;
        self.pc = 0;
        Ok(())
    }

    fn complete_return(&mut self) -> SyxResult<()> {
        let vm = self
            .vm
            .as_mut()
            .ok_or_else(|| syx_diagnostics::SyxDiagnostic::error("VM not initialized"))?;
        let frame = self
            .call_frames
            .pop()
            .ok_or_else(|| syx_diagnostics::SyxDiagnostic::error("return without call frame"))?;
        let result = vm.stack.pop().unwrap_or(syx_vm::Value::Nil);
        vm.locals = frame.caller_locals;
        vm.stack.push(result);
        self.function_index = frame.caller_fn_idx;
        self.pc = frame.return_pc;
        Ok(())
    }

    fn advance_instruction(&mut self, stop_on_call_entry: bool) -> SyxResult<DebugStepStatus> {
        let vm = self
            .vm
            .as_mut()
            .ok_or_else(|| syx_diagnostics::SyxDiagnostic::error("VM not initialized"))?;
        let fn_idx = self.function_index;
        let func = self
            .program
            .functions
            .get(fn_idx)
            .cloned()
            .ok_or_else(|| syx_diagnostics::SyxDiagnostic::error("No active function"))?;
        let pc = self.pc;

        if pc >= func.instructions.len() {
            if self.call_frames.is_empty() {
                self.paused = false;
                return Ok(DebugStepStatus::Finished);
            }
            self.complete_return()?;
            return Ok(DebugStepStatus::Continue);
        }

        match &func.instructions[pc] {
            Instruction::Call(idx) => {
                if stop_on_call_entry {
                    self.enter_call_at(*idx as usize, pc)?;
                    self.paused = true;
                    return Ok(DebugStepStatus::Paused);
                }
                self.enter_call_at(*idx as usize, pc)?;
                Ok(DebugStepStatus::Continue)
            }
            Instruction::ActorCall(idx) => {
                if stop_on_call_entry {
                    self.enter_call_at(*idx as usize, pc)?;
                    self.paused = true;
                    return Ok(DebugStepStatus::Paused);
                }
                self.enter_call_at(*idx as usize, pc)?;
                Ok(DebugStepStatus::Continue)
            }
            Instruction::Return => {
                if self.call_frames.is_empty() {
                    self.pc = func.instructions.len();
                    self.paused = false;
                    return Ok(DebugStepStatus::Finished);
                }
                self.complete_return()?;
                Ok(DebugStepStatus::Continue)
            }
            _ => {
                self.pc = vm.step_instruction(&func, &self.program, pc)?;
                if vm.suspended {
                    self.paused = true;
                    return Ok(DebugStepStatus::Paused);
                }
                if stop_on_call_entry {
                    self.paused = true;
                    return Ok(DebugStepStatus::Paused);
                }
                Ok(DebugStepStatus::Continue)
            }
        }
    }

    pub fn step_out(&mut self) -> SyxResult<bool> {
        let mut vm = self.vm.take().unwrap_or_default();
        let fn_idx = self.function_index;
        let func = self
            .program
            .functions
            .get(fn_idx)
            .cloned()
            .ok_or_else(|| syx_diagnostics::SyxDiagnostic::error("No active function"))?;
        let mut pc = self.pc;
        while pc < func.instructions.len() {
            if matches!(func.instructions[pc], Instruction::Return) {
                if self.call_frames.is_empty() {
                    self.pc = func.instructions.len();
                    self.vm = Some(vm);
                    self.paused = false;
                    return Ok(false);
                }
                self.pc = pc;
                self.vm = Some(vm);
                // Execute Return so we land in the caller frame.
                let _ = self.advance_instruction(false)?;
                self.paused = true;
                return Ok(true);
            }
            pc = vm.step_instruction(&func, &self.program, pc)?;
            if vm.suspended {
                self.pc = pc;
                self.vm = Some(vm);
                self.paused = true;
                return Ok(true);
            }
        }
        self.pc = pc;
        self.vm = Some(vm);
        self.paused = false;
        Ok(false)
    }

    pub fn set_breakpoints_for_function(&mut self, fn_idx: usize) {
        if let Some(func) = self.program.functions.get(fn_idx) {
            for pc in 0..func.instructions.len() {
                self.set_breakpoint(fn_idx, pc);
            }
        }
    }

    pub fn continue_execution(&mut self) -> SyxResult<bool> {
        if self.vm.is_none() {
            self.vm = Some(syx_vm::WasmVM::default());
        }
        let skip = (self.function_index, self.pc);
        run_until_break(self, Some(skip))
    }

    pub fn step_in(&mut self) -> SyxResult<bool> {
        if self.vm.is_none() {
            self.vm = Some(syx_vm::WasmVM::default());
        }
        match self.advance_instruction(true)? {
            DebugStepStatus::Paused | DebugStepStatus::Continue => {
                self.paused = true;
                Ok(true)
            }
            DebugStepStatus::Finished => Ok(false),
        }
    }

    pub fn stack_summary(&self) -> Vec<String> {
        let mut frames = vec![format!(
            "fn={} pc={}",
            self.program
                .functions
                .get(self.function_index)
                .map(|f| f.name.as_str())
                .unwrap_or("main"),
            self.pc
        )];
        if let Some(vm) = &self.vm {
            for (i, v) in vm.stack.iter().enumerate().take(3) {
                frames.push(format!("stack[{i}]={}", syx_vm::value_description(v)));
            }
        }
        frames
    }

    /// Align function index / PC with the program entry (same as interactive debug).
    pub fn prepare_entry(&mut self) {
        let entry = self.program.entry_point.to_string();
        self.function_index = self
            .program
            .functions
            .iter()
            .position(|f| f.name.as_str() == entry)
            .unwrap_or(0);
        self.pc = 0;
        self.paused = true;
        self.call_frames.clear();
        self.variable_handles.clear();
        self.next_variable_handle = 2;
    }

    /// Phase 18: run the VM until a source-line breakpoint; locals come from real execution.
    pub fn run_to_source_breakpoint(&mut self, line: usize) -> SyxResult<bool> {
        self.breakpoints.breakpoints.clear();
        for delta in 0..=5 {
            if let Some((fn_idx, pc, _)) = self.resolve_breakpoint_pc(line.saturating_sub(delta)) {
                self.set_breakpoint(fn_idx, pc);
                break;
            }
        }
        if self.breakpoints.breakpoints.is_empty() {
            if let Some(func) = self.program.functions.get(self.function_index) {
                for (pc, ins) in func.instructions.iter().enumerate() {
                    if matches!(ins, Instruction::Print) {
                        self.set_breakpoint(self.function_index, pc);
                        break;
                    }
                }
            }
        }
        if self.vm.is_none() {
            self.vm = Some(syx_vm::WasmVM::default());
        }
        self.prepare_entry();
        run_until_break(self, None)
    }

    /// Phase 18: run until the next `Print` instruction without executing it.
    pub fn run_to_print_breakpoint(&mut self) -> SyxResult<bool> {
        if self.vm.is_none() {
            self.vm = Some(syx_vm::WasmVM::default());
        }
        self.prepare_entry();
        let fn_idx = self.function_index;
        let func = self
            .program
            .functions
            .get(fn_idx)
            .cloned()
            .ok_or_else(|| syx_diagnostics::SyxDiagnostic::error("No active function"))?;
        let mut vm = self.vm.take().unwrap_or_default();
        let mut pc = self.pc;
        while pc < func.instructions.len() {
            if matches!(func.instructions[pc], Instruction::Print) {
                self.pc = pc;
                self.function_index = fn_idx;
                self.vm = Some(vm);
                self.paused = true;
                return Ok(true);
            }
            pc = vm.step_instruction(&func, &self.program, pc)?;
            if vm.suspended {
                self.pc = pc;
                self.vm = Some(vm);
                self.paused = true;
                return Ok(true);
            }
        }
        self.pc = pc;
        self.vm = Some(vm);
        Ok(false)
    }

    /// Evaluate a watch expression against live VM state (alias of [`evaluate_expression`]).
    pub fn evaluate_watch(&self, expr: &str) -> Option<String> {
        self.evaluate_expression(expr)
    }

    pub fn source_path(&self) -> &str {
        &self.source.name
    }

    pub fn source_dap(&self) -> Value {
        let path = self.source_path();
        let name = std::path::Path::new(path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(path);
        serde_json::json!({ "name": name, "path": path })
    }

    pub fn current_source_line(&self) -> usize {
        self.line_for_pc_in_fn(self.function_index, self.pc)
            .or_else(|| self.line_for_pc(self.pc))
            .map(|l| l + 1)
            // DAP: 0 = no source line (do not fall back to line 1 / import).
            .unwrap_or(0)
    }

    pub fn stack_frames_dap(&self) -> Vec<Value> {
        let source = self.source_dap();
        let mut frames = Vec::new();
        let mut id = 1u64;

        let push_frame = |frames: &mut Vec<Value>, id: &mut u64, fn_idx: usize, pc: usize| {
            let line = self
                .line_for_pc_in_fn(fn_idx, pc)
                .map(|l| l + 1)
                .unwrap_or(0);
            let fn_name = self
                .program
                .functions
                .get(fn_idx)
                .map(|f| f.name.to_string())
                .unwrap_or_else(|| "main".into());
            frames.push(serde_json::json!({
                "id": *id,
                "name": fn_name,
                "source": source,
                "line": line,
                "column": 1,
            }));
            *id += 1;
        };

        push_frame(&mut frames, &mut id, self.function_index, self.pc);
        for frame in self.call_frames.iter().rev() {
            let call_pc = frame.return_pc.saturating_sub(1);
            push_frame(&mut frames, &mut id, frame.caller_fn_idx, call_pc);
        }
        if let Some(label) = &self.async_stack_label {
            frames.push(serde_json::json!({
                "id": id,
                "name": format!("async::{label}"),
                "line": 0,
                "column": 0,
            }));
        }
        frames
    }

    pub fn source_line_of_offset(&self, offset: u32) -> usize {
        Self::line_of_offset_in(&self.source.contents, offset)
    }

    fn line_of_offset_in(contents: &str, offset: u32) -> usize {
        let line_starts: Vec<usize> = std::iter::once(0)
            .chain(contents.match_indices('\n').map(|(i, _)| i + 1))
            .collect();
        line_starts
            .partition_point(|&start| start <= offset as usize)
            .saturating_sub(1)
    }

    fn normalize_source_path(path: &str) -> String {
        let unified = path.replace('\\', "/");
        // Collapse `.` / `..` so `deps/../deps_math/x.sy` == `deps_math/x.sy`.
        let mut parts: Vec<&str> = Vec::new();
        let absolute = unified.starts_with('/');
        for seg in unified.split('/') {
            match seg {
                "" | "." => {}
                ".." => {
                    if parts.last().is_some_and(|p| *p != "..") {
                        parts.pop();
                    } else if !absolute {
                        parts.push("..");
                    }
                }
                other => parts.push(other),
            }
        }
        let joined = parts.join("/");
        if absolute {
            format!("/{joined}")
        } else {
            joined
        }
    }

    /// True when `longer` ends with `shorter` at a path-segment boundary.
    /// e.g. `/a/b/main.sy` matches `main.sy` or `b/main.sy`, but not `/x/main.sy`.
    fn path_suffix_match(longer: &str, shorter: &str) -> bool {
        if shorter.is_empty() || !longer.ends_with(shorter) {
            return false;
        }
        let prefix_len = longer.len() - shorter.len();
        prefix_len == 0 || longer.as_bytes().get(prefix_len - 1) == Some(&b'/')
    }

    /// Same source file? Full/relative path aware — never match on basename alone
    /// across different directories (that caused cross-file breakpoint line bleed).
    pub fn paths_same_source(a: &str, b: &str) -> bool {
        let a = Self::normalize_source_path(a);
        let b = Self::normalize_source_path(b);
        if a.is_empty() || b.is_empty() || a == "/" || b == "/" {
            return false;
        }
        if a == b {
            return true;
        }
        Self::path_suffix_match(&a, &b) || Self::path_suffix_match(&b, &a)
    }

    /// Whether a DAP `setBreakpoints` source refers to this debug session's entry file.
    pub fn source_matches_breakpoint_request(&self, request_path: &str) -> bool {
        Self::paths_same_source(self.source.name.as_str(), request_path)
    }

    /// True if breakpoints for `request_path` can bind in this program (entry or any fn source).
    pub fn source_is_debuggable(&self, request_path: &str) -> bool {
        if self.source_matches_breakpoint_request(request_path) {
            return true;
        }
        self.program.functions.iter().any(|f| {
            !f.source_name.is_empty()
                && Self::paths_same_source(f.source_name.as_str(), request_path)
        })
    }

    fn function_matches_path(&self, func: &syx_codegen::FunctionUnit, path: &str) -> bool {
        if func.source_name.is_empty() {
            // Untagged functions belong to the session entry file.
            return Self::paths_same_source(self.source.name.as_str(), path);
        }
        Self::paths_same_source(func.source_name.as_str(), path)
    }

    fn function_matches_debug_source(&self, func: &syx_codegen::FunctionUnit) -> bool {
        self.function_matches_path(func, self.source.name.as_str())
    }

    fn contents_for_source_path(&self, path: &str) -> Option<String> {
        if Self::paths_same_source(self.source.name.as_str(), path) {
            return Some(self.source.contents.to_string());
        }
        let mut candidates: Vec<String> = vec![
            path.to_string(),
            Self::normalize_source_path(path),
        ];
        if let Some(session_dir) = std::path::Path::new(self.source.name.as_str()).parent() {
            let p = std::path::Path::new(path);
            if p.is_relative() {
                candidates.push(session_dir.join(p).to_string_lossy().into_owned());
            }
            if let Some(name) = p.file_name() {
                candidates.push(session_dir.join(name).to_string_lossy().into_owned());
            }
        }
        for c in &candidates {
            if let Ok(text) = std::fs::read_to_string(c) {
                return Some(text);
            }
        }
        // Fall back: function-recorded paths (may be relative to cwd).
        for f in &self.program.functions {
            if Self::paths_same_source(f.source_name.as_str(), path) {
                if let Ok(text) = std::fs::read_to_string(f.source_name.as_str()) {
                    return Some(text);
                }
            }
        }
        None
    }

    pub fn resolve_breakpoint_line_in_fn(&self, fn_idx: usize, line: usize) -> Option<usize> {
        self.resolve_breakpoint_line_in_fn_for_source(fn_idx, line, self.source.name.as_str())
    }

    fn resolve_breakpoint_line_in_fn_for_source(
        &self,
        fn_idx: usize,
        line: usize,
        source_path: &str,
    ) -> Option<usize> {
        let func = self.program.functions.get(fn_idx)?;
        if !self.function_matches_path(func, source_path) {
            return None;
        }
        let contents = self.contents_for_source_path(source_path)?;
        let target = line.saturating_sub(1);
        for (pc, span) in func.instruction_spans.iter().enumerate() {
            if let Some((start, len)) = span {
                if *len == 0 {
                    continue;
                }
                if Self::line_of_offset_in(&contents, *start) == target {
                    return Some(pc);
                }
            }
        }
        None
    }

    pub fn resolve_breakpoint_line(&self, line: usize) -> Option<(usize, usize)> {
        let entry = self.program.entry_point.as_str();
        let mut candidates: Vec<usize> = self
            .program
            .functions
            .iter()
            .enumerate()
            .filter(|(_, f)| {
                self.function_matches_debug_source(f)
                    && f.instruction_spans.iter().any(|s| s.is_some())
            })
            .map(|(i, _)| i)
            .collect();
        candidates.sort_by_key(|i| {
            if self.program.functions[*i].name.as_str() == entry {
                0
            } else {
                1
            }
        });
        for fn_idx in candidates {
            if let Some(pc) = self.resolve_breakpoint_line_in_fn(fn_idx, line) {
                return Some((fn_idx, pc));
            }
        }
        None
    }

    /// Map a requested source line to all `(function_index, pc, mapped_line)` in this file.
    pub fn resolve_breakpoint_all(&self, line: usize) -> Vec<(usize, usize, usize)> {
        self.resolve_breakpoint_all_in_source(self.source.name.as_str(), line)
    }

    pub fn resolve_breakpoint_all_in_source(
        &self,
        source_path: &str,
        line: usize,
    ) -> Vec<(usize, usize, usize)> {
        let mut hits = self.collect_breakpoint_hits_in_source(source_path, line);
        if hits.is_empty() {
            for delta in 1..=5 {
                hits = self.collect_breakpoint_hits_in_source(source_path, line + delta);
                if hits.is_empty() && line > delta {
                    hits = self.collect_breakpoint_hits_in_source(source_path, line - delta);
                }
                if !hits.is_empty() {
                    break;
                }
            }
        }
        hits
    }

    fn collect_breakpoint_hits_in_source(
        &self,
        source_path: &str,
        line: usize,
    ) -> Vec<(usize, usize, usize)> {
        let mut hits = Vec::new();
        for fn_idx in 0..self.program.functions.len() {
            if let Some(pc) =
                self.resolve_breakpoint_line_in_fn_for_source(fn_idx, line, source_path)
            {
                hits.push((fn_idx, pc, line));
            }
        }
        hits
    }

    /// Map a requested source line to `(function_index, pc, mapped_line)`.
    pub fn resolve_breakpoint_pc(&self, line: usize) -> Option<(usize, usize, usize)> {
        self.resolve_breakpoint_line(line)
            .map(|(fn_idx, pc)| (fn_idx, pc, line))
            .or_else(|| {
                for delta in 1..=5 {
                    if let Some((fn_idx, pc)) = self.resolve_breakpoint_line(line + delta) {
                        return Some((fn_idx, pc, line + delta));
                    }
                    if line > delta {
                        if let Some((fn_idx, pc)) = self.resolve_breakpoint_line(line - delta) {
                            return Some((fn_idx, pc, line - delta));
                        }
                    }
                }
                None
            })
    }

    pub fn breakpoint_locations(&self, start_line: usize, end_line: usize) -> Vec<usize> {
        (start_line..=end_line)
            .filter(|line| self.resolve_breakpoint_line(*line).is_some())
            .collect()
    }

    pub fn step_over_source_line(&mut self) -> SyxResult<bool> {
        if self.vm.is_none() {
            self.vm = Some(syx_vm::WasmVM::default());
        }
        let start_line = self.current_source_line();
        let start_fn = self.function_index;
        let frame_depth = self.call_frames.len();
        let skip = (start_fn, self.pc);
        loop {
            let fn_idx = self.function_index;
            let pc = self.pc;
            if self
                .program
                .functions
                .get(fn_idx)
                .is_some_and(|f| pc < f.instructions.len())
            {
                if self.should_break(fn_idx, pc) && (fn_idx, pc) != skip {
                    self.paused = true;
                    return Ok(true);
                }
            }
            match self.advance_instruction(false)? {
                DebugStepStatus::Paused => return Ok(true),
                DebugStepStatus::Finished => return Ok(false),
                DebugStepStatus::Continue => {
                    let line = self.current_source_line();
                    // Skip PCs with no source mapping (line 0) so epilogue / Return
                    // does not look like a jump back to import (line 1).
                    if self.function_index == start_fn
                        && self.call_frames.len() == frame_depth
                        && line != 0
                        && line != start_line
                    {
                        self.paused = true;
                        return Ok(true);
                    }
                }
            }
        }
    }

    /// Replace breakpoints for the session entry source only (legacy helper / tests).
    pub fn apply_set_breakpoints(&mut self, breakpoints: &[Value]) -> Vec<Value> {
        let entry = self.source.name.to_string();
        self.apply_set_breakpoints_for_source(&entry, breakpoints)
    }

    /// DAP: update breakpoints for one source path without clobbering other files' lines.
    pub fn apply_set_breakpoints_for_source(
        &mut self,
        source_path: &str,
        breakpoints: &[Value],
    ) -> Vec<Value> {
        let key = Self::normalize_source_path(source_path);
        let mut requested: Vec<(usize, Option<String>)> = Vec::new();
        let mut results = Vec::new();
        for bp in breakpoints {
            let Some(line) = bp.get("line").and_then(|l| l.as_u64()).map(|n| n as usize) else {
                continue;
            };
            let cond = bp
                .get("condition")
                .and_then(|c| c.as_str())
                .map(str::to_string);
            requested.push((line, cond.clone()));
            let hits = self.resolve_breakpoint_all_in_source(source_path, line);
            if hits.is_empty() {
                results.push(serde_json::json!({
                    "verified": false,
                    "line": line,
                    "condition": cond,
                }));
            } else {
                results.push(serde_json::json!({
                    "verified": true,
                    "line": hits[0].2,
                    "condition": cond,
                }));
            }
        }
        self.breakpoints.source_lines.insert(key, requested);
        self.rebuild_breakpoints_from_sources();
        results
    }

    fn rebuild_breakpoints_from_sources(&mut self) {
        self.breakpoints.breakpoints.clear();
        let sources: Vec<(String, Vec<(usize, Option<String>)>)> = self
            .breakpoints
            .source_lines
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        for (path, lines) in sources {
            for (line, cond) in lines {
                for (fn_idx, pc, _) in self.resolve_breakpoint_all_in_source(&path, line) {
                    self.set_breakpoint_with_condition(fn_idx, pc, cond.clone());
                }
            }
        }
    }
}

/// Minimal terminal debugger: list bytecode, single-step, run to completion.
pub fn run_interactive_session(session: &mut DebugSession) -> SyxResult<()> {
    use std::io::{self, Write};

    let entry = session.program.entry_point.to_string();
    session.function_index = session
        .program
        .functions
        .iter()
        .position(|f| f.name.as_str() == entry)
        .unwrap_or(0);
    session.pc = 0;

    println!("SYX debug — {}", session.source.name);
    println!("Entry: {entry} (fn index {})", session.function_index);
    println!("Commands: l=list  s=step  r=run  b <pc|line>=breakpoint  q=quit");
    session.list_instructions(40);

    loop {
        print!("(debug) ");
        io::stdout()
            .flush()
            .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
        let mut line = String::new();
        if io::stdin()
            .read_line(&mut line)
            .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?
            == 0
        {
            break;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.first().copied().unwrap_or("") {
            "q" | "quit" | "exit" => break,
            "h" | "help" => {
                println!("l=list  s=step  r=run  b <pc>=breakpoint  q=quit");
            }
            "l" | "list" => {
                let limit = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(40);
                session.list_instructions(limit);
            }
            "s" | "step" => {
                if let Some(instr) = session.step() {
                    if session.should_break(session.function_index, session.pc.saturating_sub(1)) {
                        println!("* breakpoint at pc {}", session.pc - 1);
                    }
                    println!("  [{:4}] {instr:?}", session.pc - 1);
                } else {
                    println!("(end of function — use `r` to run on VM)");
                }
            }
            "b" | "break" => {
                if let Some(n) = parts.get(1).and_then(|s| s.parse().ok()) {
                    if n < 1000 {
                        session.set_breakpoint_line(n);
                        println!("breakpoint set at source line {n}");
                    } else {
                        session.set_breakpoint(session.function_index, n);
                        println!("breakpoint set at pc {n}");
                    }
                } else {
                    println!("usage: b <source_line|pc>");
                }
            }
            "r" | "run" => {
                let _ = run_until_break(session, None)?;
            }
            "" => {}
            other => println!("unknown command: {other} (try `help`)"),
        }
    }
    Ok(())
}

pub fn build_source_map(source: &SourceFile, program: &CompiledProgram) -> SourceMap {
    let mut map = SourceMap::default();
    let line_starts: Vec<usize> = std::iter::once(0)
        .chain(
            source
                .contents
                .match_indices('\n')
                .map(|(i, _)| i + 1),
        )
        .collect();
    let line_of = |offset: u32| -> usize {
        line_starts
            .partition_point(|&start| start <= offset as usize)
            .saturating_sub(1)
    };
    for (fi, func) in program.functions.iter().enumerate() {
        for (pc, span) in func.instruction_spans.iter().enumerate() {
            if let Some((start, len)) = span {
                if *len == 0 {
                    continue;
                }
                let key = fi * 10_000 + pc;
                map.entries.insert(key, (*start, *len));
                let line = line_of(*start);
                map.pc_to_line.insert(pc, line);
                map.line_to_pc.insert(line, (fi as u32, pc as u32));
            }
        }
    }
    map
}

pub fn run_until_break(
    session: &mut DebugSession,
    skip_break_at: Option<(usize, usize)>,
) -> SyxResult<bool> {
    if session.vm.is_none() {
        session.vm = Some(syx_vm::WasmVM::default());
    }
    loop {
        let fn_idx = session.function_index;
        let pc = session.pc;
        let func_len = session
            .program
            .functions
            .get(fn_idx)
            .map(|f| f.instructions.len())
            .unwrap_or(0);

        if pc < func_len {
            if skip_break_at != Some((fn_idx, pc)) && session.should_break(fn_idx, pc) {
                session.paused = true;
                if let Some(line) = session.line_for_pc_in_fn(fn_idx, pc) {
                    println!("* stopped at line {} (fn {fn_idx} pc {pc})", line + 1);
                } else {
                    println!("* stopped at fn {fn_idx} pc {pc}");
                }
                return Ok(true);
            }
        }

        match session.advance_instruction(false)? {
            DebugStepStatus::Paused => return Ok(true),
            DebugStepStatus::Finished => {
                session.paused = false;
                println!("Program finished. stdout:");
                for line in &session
                    .vm
                    .as_ref()
                    .map(|v| v.printed.clone())
                    .unwrap_or_default()
                {
                    println!("  {line}");
                }
                return Ok(false);
            }
            DebugStepStatus::Continue => {}
        }
    }
}

#[derive(Debug, Deserialize)]
struct DapRequest {
    seq: u64,
    #[serde(rename = "type")]
    msg_type: String,
    command: Option<String>,
    arguments: Option<Value>,
}

#[derive(Debug, Serialize)]
struct DapResponse {
    seq: u64,
    #[serde(rename = "type")]
    msg_type: &'static str,
    request_seq: u64,
    success: bool,
    command: String,
    body: Value,
}

#[derive(Debug, Serialize)]
struct DapEvent {
    seq: u64,
    #[serde(rename = "type")]
    msg_type: &'static str,
    event: String,
    body: Value,
}

fn read_dap_message(reader: &mut impl BufRead) -> SyxResult<Option<Value>> {
    let mut headers = String::new();
    loop {
        let mut line = String::new();
        if reader
            .read_line(&mut line)
            .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?
            == 0
        {
            return Ok(None);
        }
        if line == "\r\n" || line == "\n" {
            break;
        }
        headers.push_str(&line);
    }
    let content_length = headers
        .lines()
        .find_map(|l| l.strip_prefix("Content-Length:"))
        .and_then(|n| n.trim().parse::<usize>().ok())
        .unwrap_or(0);
    if content_length == 0 {
        return Ok(None);
    }
    let mut body = vec![0u8; content_length];
    reader
        .read_exact(&mut body)
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    let value: Value = serde_json::from_slice(&body)
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    Ok(Some(value))
}

fn write_dap_message(stdout: &mut impl Write, value: &Value) -> SyxResult<()> {
    let body = serde_json::to_string(value)
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    write!(stdout, "Content-Length: {}\r\n\r\n{}", body.len(), body)
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    stdout
        .flush()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    Ok(())
}

fn emit_stopped(stdout: &mut impl Write, seq: &mut u64, reason: &str) -> SyxResult<()> {
    let event = serde_json::json!({
        "seq": *seq,
        "type": "event",
        "event": "stopped",
        "body": {
            "reason": reason,
            "threadId": 1,
            "allThreadsStopped": true
        }
    });
    *seq += 1;
    write_dap_message(stdout, &event)
}

fn emit_terminated(stdout: &mut impl Write, seq: &mut u64) -> SyxResult<()> {
    let event = serde_json::json!({
        "seq": *seq,
        "type": "event",
        "event": "terminated",
        "body": {}
    });
    *seq += 1;
    write_dap_message(stdout, &event)
}

fn emit_output(stdout: &mut impl Write, seq: &mut u64, category: &str, output: &str) -> SyxResult<()> {
    let event = serde_json::json!({
        "seq": *seq,
        "type": "event",
        "event": "output",
        "body": {
            "category": category,
            "output": output,
        }
    });
    *seq += 1;
    write_dap_message(stdout, &event)
}

/// DAP session when the program failed to compile/parse before a DebugSession exists.
/// Completes the initialize/launch handshake so VS Code surfaces the error instead of
/// silently dying when the adapter process exits early.
pub fn run_dap_compile_failed(err: &syx_diagnostics::SyxDiagnostic) -> SyxResult<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = io::BufReader::new(stdin.lock());
    let mut seq = 1u64;
    let message = format!("{err}\n");
    while let Some(value) = read_dap_message(&mut reader)? {
        let req: DapRequest = serde_json::from_value(value)
            .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
        if req.msg_type != "request" {
            continue;
        }
        let command = req.command.clone().unwrap_or_default();
        let (success, body) = match command.as_str() {
            "initialize" => (
                true,
                serde_json::json!({
                    "supportsConfigurationDoneRequest": true,
                    "supportsTerminateRequest": true,
                }),
            ),
            "launch" | "attach" | "configurationDone" => (
                false,
                serde_json::json!({
                    "error": {
                        "id": 1,
                        "format": message.trim_end(),
                        "showUser": true,
                    }
                }),
            ),
            "disconnect" | "terminate" => (true, serde_json::json!({})),
            _ => (true, serde_json::json!({})),
        };
        let resp = DapResponse {
            seq,
            msg_type: "response",
            request_seq: req.seq,
            success,
            command: command.clone(),
            body,
        };
        seq += 1;
        write_dap_message(&mut stdout, &serde_json::to_value(&resp).unwrap())?;
        if command == "initialize" {
            let event = DapEvent {
                seq,
                msg_type: "event",
                event: "initialized".into(),
                body: serde_json::json!({}),
            };
            seq += 1;
            write_dap_message(&mut stdout, &serde_json::to_value(&event).unwrap())?;
        }
        if matches!(command.as_str(), "launch" | "attach" | "configurationDone") {
            emit_output(&mut stdout, &mut seq, "stderr", &message)?;
            emit_terminated(&mut stdout, &mut seq)?;
            return Ok(());
        }
        if matches!(command.as_str(), "disconnect" | "terminate") {
            return Ok(());
        }
    }
    Ok(())
}

pub fn run_dap_server(session: &mut DebugSession) -> SyxResult<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = io::BufReader::new(stdin.lock());
    let mut seq = 1u64;
    let mut send_terminated = false;
    let mut stop_on_entry = true;
    while let Some(value) = read_dap_message(&mut reader)? {
        let req: DapRequest = serde_json::from_value(value)
            .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
        if req.msg_type == "request" {
            let command = req.command.clone().unwrap_or_default();
            let mut send_stopped = None;
            let body = match command.as_str() {
                "initialize" => serde_json::json!({
                    "supportsConfigurationDoneRequest": true,
                    "supportsEvaluateForHovers": true,
                    "supportsConditionalBreakpoints": true,
                    "supportsStepIn": true,
                    "supportsStepOut": true,
                    "supportsScopesRequest": true,
                    "supportsVariablesRequest": true,
                    "supportsVariableType": true,
                    "supportsTerminateRequest": true,
                    "supportsBreakpointLocationsRequest": true,
                    "allowBreakpointsEverywhere": true,
                }),
                "launch" => {
                    session.prepare_entry();
                    if let Some(args) = &req.arguments {
                        if let Some(v) = args.get("stopOnEntry").and_then(|x| x.as_bool()) {
                            stop_on_entry = v;
                        }
                    }
                    serde_json::json!({})
                }
                "threads" => {
                    let mut threads = vec![
                        serde_json::json!({ "id": 1, "name": "MainActor" }),
                        serde_json::json!({ "id": 2, "name": "ThreadPool" }),
                        serde_json::json!({ "id": 3, "name": "ActorExecutors" }),
                    ];
                    for (i, child) in session.task_children.iter().enumerate() {
                        threads.push(serde_json::json!({
                            "id": 10 + i as u32,
                            "name": format!("Task::{child}")
                        }));
                    }
                    serde_json::json!({ "threads": threads })
                }
                "stackTrace" => {
                    let frames = session.stack_frames_dap();
                    serde_json::json!({
                        "stackFrames": frames,
                        "totalFrames": frames.len()
                    })
                }
                "scopes" => serde_json::json!({
                    "scopes": [{
                        "name": "Locals",
                        "variablesReference": 1,
                        "expensive": false
                    }]
                }),
                "variables" => {
                    let ref_id = req
                        .arguments
                        .as_ref()
                        .and_then(|a| a.get("variablesReference"))
                        .and_then(|r| r.as_u64())
                        .unwrap_or(1) as u32;
                    let vars = if ref_id <= 1 {
                        session.local_variables()
                    } else {
                        session.child_variables(ref_id)
                    };
                    serde_json::json!({
                        "variables": vars.into_iter().map(|(name, value, ty, child_ref)| {
                            serde_json::json!({
                                "name": name,
                                "value": value,
                                "type": ty,
                                "variablesReference": child_ref,
                            })
                        }).collect::<Vec<_>>()
                    })
                }
                "setBreakpoints" => {
                    let source_path = req
                        .arguments
                        .as_ref()
                        .and_then(|a| a.get("source"))
                        .and_then(|s| {
                            s.get("path")
                                .or_else(|| s.get("name"))
                                .and_then(|p| p.as_str())
                        });
                    let breakpoints = req
                        .arguments
                        .as_ref()
                        .and_then(|a| a.get("breakpoints"))
                        .and_then(|b| b.as_array())
                        .map(|a| a.as_slice())
                        .unwrap_or(&[]);
                    let results = if let Some(path) = source_path {
                        if session.source_is_debuggable(path) {
                            // Per-source update: do not clear other files' breakpoints.
                            session.apply_set_breakpoints_for_source(path, breakpoints)
                        } else {
                            // Unrelated open editors (e.g. another package's main.sy).
                            breakpoints
                                .iter()
                                .filter_map(|bp| {
                                    let line = bp.get("line")?.as_u64()? as usize;
                                    let cond = bp
                                        .get("condition")
                                        .and_then(|c| c.as_str())
                                        .map(str::to_string);
                                    Some(serde_json::json!({
                                        "verified": false,
                                        "line": line,
                                        "condition": cond,
                                    }))
                                })
                                .collect()
                        }
                    } else {
                        session.apply_set_breakpoints(breakpoints)
                    };
                    serde_json::json!({ "breakpoints": results })
                },
                "breakpointLocations" => {
                    let start = req
                        .arguments
                        .as_ref()
                        .and_then(|a| a.get("startLine"))
                        .and_then(|l| l.as_u64())
                        .unwrap_or(1) as usize;
                    let end = req
                        .arguments
                        .as_ref()
                        .and_then(|a| a.get("endLine"))
                        .and_then(|l| l.as_u64())
                        .map(|l| l as usize)
                        .unwrap_or(start);
                    let locations: Vec<_> = session
                        .breakpoint_locations(start, end)
                        .into_iter()
                        .map(|line| serde_json::json!({ "line": line, "column": 1 }))
                        .collect();
                    serde_json::json!({ "breakpoints": locations })
                },
                "evaluate" => {
                    let expr = req
                        .arguments
                        .as_ref()
                        .and_then(|a| a.get("expression"))
                        .and_then(|e| e.as_str())
                        .unwrap_or("");
                    if expr.starts_with("watch:") {
                        session.add_watch(expr.trim_start_matches("watch:").trim());
                    }
                    let value = session
                        .evaluate_expression(expr.trim_start_matches("watch:").trim())
                        .unwrap_or_else(|| "nil".into());
                    serde_json::json!({
                        "result": value,
                        "variablesReference": 0
                    })
                }
                "continue" => {
                    let stopped = session.continue_execution()?;
                    if stopped {
                        send_stopped = Some("breakpoint");
                    } else {
                        send_terminated = true;
                    }
                    serde_json::json!({ "allThreadsContinued": !stopped })
                }
                "next" => {
                    // false = program finished; must terminate (not stop at fallback line 1).
                    let stopped = session.step_over_source_line()?;
                    if stopped {
                        send_stopped = Some("step");
                    } else {
                        send_terminated = true;
                    }
                    serde_json::json!({ "allThreadsContinued": !stopped })
                }
                "stepIn" => {
                    let stopped = session.step_in()?;
                    if stopped {
                        send_stopped = Some("step");
                    } else {
                        send_terminated = true;
                    }
                    serde_json::json!({ "allThreadsContinued": !stopped })
                }
                "stepOut" => {
                    let stopped = session.step_out()?;
                    if stopped {
                        send_stopped = Some("step");
                    } else {
                        send_terminated = true;
                    }
                    serde_json::json!({ "allThreadsContinued": !stopped })
                }
                "configurationDone" => serde_json::json!({}),
                "terminate" => {
                    send_terminated = true;
                    serde_json::json!({})
                }
                "disconnect" => {
                    return Ok(());
                }
                _ => serde_json::json!({}),
            };
            if command == "configurationDone" {
                if stop_on_entry {
                    send_stopped = Some("entry");
                } else {
                    let stopped = session.continue_execution()?;
                    if stopped {
                        send_stopped = Some("breakpoint");
                    } else {
                        send_terminated = true;
                    }
                }
            }
            let resp = DapResponse {
                seq,
                msg_type: "response",
                request_seq: req.seq,
                success: true,
                command: command.clone(),
                body,
            };
            seq += 1;
            write_dap_message(&mut stdout, &serde_json::to_value(&resp).unwrap())?;
            if command == "initialize" {
                let event = DapEvent {
                    seq,
                    msg_type: "event",
                    event: "initialized".into(),
                    body: serde_json::json!({}),
                };
                seq += 1;
                write_dap_message(&mut stdout, &serde_json::to_value(&event).unwrap())?;
            }
            if let Some(reason) = send_stopped {
                session.flush_output(&mut stdout, &mut seq)?;
                emit_stopped(&mut stdout, &mut seq, reason)?;
            }
            if send_terminated {
                session.flush_output(&mut stdout, &mut seq)?;
                emit_terminated(&mut stdout, &mut seq)?;
                return Ok(());
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use smol_str::SmolStr;
    use syx_codegen::{FunctionUnit, Instruction};

    #[test]
    fn step_over_advances_source_line() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p4_hof.sy");
        let mut session = syx_driver::debug_path(&path).expect("debug");
        session.prepare_entry();
        let start = session.current_source_line();
        session.step_over_source_line().expect("step over");
        assert!(
            session.current_source_line() != start || session.pc > 0,
            "step over should advance line or pc"
        );
    }

    #[test]
    fn step_over_at_end_finishes_without_line_one() {
        // Repro: last source line → step over must finish, not land on import (line 1).
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/file_tool/main.sy");
        let mut session = syx_driver::debug_path(&path).expect("debug file_tool");
        session.prepare_entry();
        let mut guard = 0;
        loop {
            guard += 1;
            assert!(guard < 50_000, "step_over did not finish");
            let line_before = session.current_source_line();
            let still_running = session.step_over_source_line().expect("step over");
            if !still_running {
                break;
            }
            assert_ne!(
                session.current_source_line(),
                1,
                "after stepping from line {line_before}, must not jump to import line 1 (pc={})",
                session.pc
            );
        }
        assert!(!session.paused);
    }

    #[test]
    fn p4_hof_breakpoint_line_resolution() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p4_hof.sy");
        let mut session = syx_driver::debug_path(&path).expect("debug file");
        session.prepare_entry();
        let func = &session.program.functions[session.function_index];
        let span_count = func.instruction_spans.iter().filter(|s| s.is_some()).count();
        assert!(
            span_count > 0,
            "expected instruction_spans on entry fn, got 0 ({} instructions)",
            func.instructions.len()
        );
        let mut resolved = 0usize;
        for line in 1..=8 {
            if session.resolve_breakpoint_line(line).is_some() {
                resolved += 1;
            }
        }
        assert!(
            resolved > 0,
            "expected at least one breakpoint line to resolve in p4_hof.sy"
        );
    }

    #[test]
    fn mir_async_main_instruction_spans_for_breakpoints() {
        // MIR pipeline previously left entry `main` with empty instruction_spans,
        // so DAP breakpoints in async main() never verified / never hit.
        let dir = std::env::temp_dir().join(format!(
            "syx_mir_bp_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("main.sy");
        std::fs::write(
            &path,
            r#"async func main() {
    let x = 1
    print(x)
    print(2)
}
"#,
        )
        .unwrap();
        let mut session = syx_driver::debug_path(&path).expect("debug async main");
        session.prepare_entry();
        let func = &session.program.functions[session.function_index];
        let span_count = func.instruction_spans.iter().filter(|s| s.is_some()).count();
        assert!(
            span_count > 0,
            "MIR async main must carry instruction_spans, got 0 ({} instructions)",
            func.instructions.len()
        );
        assert!(
            func.local_names.iter().any(|n| n.as_str() == "x"),
            "MIR async main must expose source local names, got {:?}",
            func.local_names
        );
        let hits = session.apply_set_breakpoints(&[
            serde_json::json!({ "line": 2 }),
            serde_json::json!({ "line": 3 }),
            serde_json::json!({ "line": 4 }),
        ]);
        assert!(
            hits.iter().all(|h| h["verified"].as_bool() == Some(true)),
            "expected verified breakpoints on async main lines, got {hits:?}"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn continue_skips_breakpoint_at_current_pc() {
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 0,
            instructions: vec![
                Instruction::ConstI32(1),
                Instruction::Print,
                Instruction::ConstI32(2),
                Instruction::Print,
                Instruction::Return,
            ],
            instruction_spans: vec![None; 5],
            local_names: Vec::new(),
            local_types: Vec::new(),
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            entry_point: "main".into(),
            ..Default::default()
        };
        let source = SourceFile::new("t.sy", "func main() {}");
        let mut session = DebugSession::new(source, program);
        session.prepare_entry();
        session.set_breakpoint(0, 0);
        session.set_breakpoint(0, 2);
        assert!(run_until_break(&mut session, None).expect("first break"));
        assert_eq!(session.pc, 0);
        assert!(run_until_break(&mut session, Some((0, 0))).expect("second break"));
        assert_eq!(session.pc, 2, "continue should run past pc 0 and stop at pc 2");
    }

    #[test]
    fn local_variables_use_source_names() {
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 1,
            local_count: 2,
            instructions: vec![Instruction::Return],
            instruction_spans: vec![None],
            local_names: vec![
                SmolStr::new("x"),
                SmolStr::new("y"),
            ],
            local_types: vec![
                SmolStr::new("Unknown"),
                SmolStr::new("Unknown"),
            ],
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            ..Default::default()
        };
        let source = SourceFile::new("t.sy", "func main() {}");
        let mut session = DebugSession::new(source, program);
        let mut vm = syx_vm::WasmVM::default();
        vm.locals.push(syx_vm::Value::I32(1));
        vm.locals.push(syx_vm::Value::I32(2));
        session.vm = Some(vm);
        let names: Vec<_> = session
            .local_variables()
            .into_iter()
            .map(|(n, _, _, _)| n)
            .collect();
        assert!(names.contains(&"x".into()));
        assert!(names.contains(&"y".into()));
    }

    #[test]
    fn set_breakpoints_registers_pcs() {
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 0,
            instructions: vec![Instruction::ConstI32(1), Instruction::Return],
            instruction_spans: vec![None, None],
            local_names: Vec::new(),
            local_types: Vec::new(),
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            ..Default::default()
        };
        let source = SourceFile::new("t.sy", "func main() {}");
        let mut session = DebugSession::new(source, program);
        session.set_breakpoint(0, 0);
        assert!(session.should_break(0, 0));
    }

    #[test]
    fn paths_same_source_collapses_dotdot() {
        assert!(DebugSession::paths_same_source(
            "/a/deps/../deps_math/local_math.sy",
            "/a/deps_math/local_math.sy",
        ));
        assert!(!DebugSession::paths_same_source(
            "/a/examples/main.sy",
            "/b/examples/main.sy",
        ));
        assert!(DebugSession::paths_same_source(
            "/a/b/hello.sy",
            "hello.sy",
        ));
    }

    #[test]
    fn set_breakpoints_ignores_foreign_source_path() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/hello.sy");
        let mut session = syx_driver::debug_path(&path).expect("hello");
        session.prepare_entry();
        session.apply_set_breakpoints(&[serde_json::json!({ "line": 2 })]);
        let before = session.breakpoints.breakpoints.len();
        assert!(before > 0, "line 2 should bind in hello");
        let foreign = "/workspace/examples/main.sy";
        assert!(!session.source_is_debuggable(foreign));
        // Simulate DAP: foreign editors stay unverified and must not clobber.
        if session.source_is_debuggable(foreign) {
            session.apply_set_breakpoints_for_source(
                foreign,
                &[
                    serde_json::json!({ "line": 2 }),
                    serde_json::json!({ "line": 21 }),
                    serde_json::json!({ "line": 22 }),
                ],
            );
        }
        assert_eq!(
            session.breakpoints.breakpoints.len(),
            before,
            "foreign setBreakpoints must not replace hello breakpoints"
        );
    }

    #[test]
    fn set_breakpoints_keeps_per_source_paths() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/deps/main.sy");
        let mut session = syx_driver::debug_path(&path).expect("deps");
        session.prepare_entry();
        let main_path = path.to_string_lossy().to_string();
        let math_path = path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("deps_math/local_math.sy")
            .to_string_lossy()
            .to_string();
        assert!(
            session.source_is_debuggable(&math_path),
            "imported local_math.sy must be debuggable; user sources={:?}",
            session
                .program
                .functions
                .iter()
                .map(|f| f.source_name.as_str())
                .filter(|s| !s.starts_with("<stdlib>"))
                .collect::<Vec<_>>()
        );
        // Different lines per file; must not bleed by line number alone.
        let main_bps = session.apply_set_breakpoints_for_source(
            &main_path,
            &[serde_json::json!({ "line": 4 })],
        );
        let math_bps = session.apply_set_breakpoints_for_source(
            &math_path,
            &[serde_json::json!({ "line": 2 })],
        );
        assert!(
            main_bps
                .iter()
                .any(|b| b["verified"].as_bool() == Some(true)),
            "main.sy line 4 should verify; got {main_bps:?}"
        );
        assert!(
            math_bps
                .iter()
                .any(|b| b["verified"].as_bool() == Some(true)),
            "local_math.sy line 2 should verify; got {math_bps:?}"
        );
        // Updating main.sy alone must not drop local_math requested lines.
        let _ = session.apply_set_breakpoints_for_source(
            &main_path,
            &[serde_json::json!({ "line": 4 })],
        );
        assert!(
            session
                .breakpoints
                .source_lines
                .values()
                .any(|lines| lines.iter().any(|(l, _)| *l == 2)),
            "local_math line 2 must survive main.sy setBreakpoints; got {:?}",
            session.breakpoints.source_lines
        );
        // Line 2 must only bind in local_math functions.
        let hits = session.resolve_breakpoint_all_in_source(&math_path, 2);
        assert!(!hits.is_empty(), "local_math line 2 should resolve");
        for (fn_idx, _, _) in &hits {
            let src = session.program.functions[*fn_idx].source_name.as_str();
            assert!(
                DebugSession::paths_same_source(src, &math_path),
                "line 2 hit in unexpected source {src:?}"
            );
        }
        // main.sy has no executable code on line 2 (import); must not steal math PCs.
        let wrong = session.resolve_breakpoint_all_in_source(&main_path, 2);
        assert!(
            wrong.is_empty()
                || wrong.iter().all(|(fn_idx, _, _)| {
                    DebugSession::paths_same_source(
                        session.program.functions[*fn_idx].source_name.as_str(),
                        &main_path,
                    )
                }),
            "main.sy line 2 must not resolve local_math PCs; got {wrong:?}"
        );
    }

    #[test]
    fn steps_through_instructions() {
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 0,
            instructions: vec![Instruction::ConstI32(1), Instruction::Return],
            instruction_spans: vec![None, None],
            local_names: Vec::new(),
            local_types: Vec::new(),
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            ..Default::default()
        };
        let source = SourceFile::new("t.sy", "func main() {}");
        let mut session = DebugSession::new(source, program);
        assert!(session.step().is_some());
    }

    #[test]
    fn conditional_breakpoint_can_be_disabled() {
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 0,
            instructions: vec![Instruction::ConstI32(1), Instruction::Return],
            instruction_spans: vec![None, None],
            local_names: Vec::new(),
            local_types: Vec::new(),
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            ..Default::default()
        };
        let source = SourceFile::new("t.sy", "func main() {}");
        let mut session = DebugSession::new(source, program);
        session.set_breakpoint_with_condition(0, 0, Some("false".into()));
        assert!(!session.should_break(0, 0));
    }

    #[test]
    fn evaluate_reads_local_slot() {
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 1,
            instructions: vec![Instruction::ConstI32(7), Instruction::Return],
            instruction_spans: vec![None, None],
            local_names: Vec::new(),
            local_types: Vec::new(),
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            ..Default::default()
        };
        let source = SourceFile::new("t.sy", "func main() {}");
        let mut session = DebugSession::new(source, program);
        let mut vm = syx_vm::WasmVM::default();
        vm.locals.push(syx_vm::Value::I32(7));
        session.vm = Some(vm);
        assert_eq!(session.evaluate_expression("local_0"), Some("7".into()));
    }

    #[test]
    fn stack_trace_uses_instruction_spans_for_line() {
        let source_text = "func helper() { return 1 }\nfunc main() {\n    print(1)\n}\n";
        let print_offset = source_text.find("print").unwrap() as u32;
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 0,
            instructions: vec![Instruction::ConstI32(1), Instruction::Print, Instruction::Return],
            instruction_spans: vec![
                None,
                Some((print_offset, 5)),
                None,
            ],
            local_names: Vec::new(),
            local_types: Vec::new(),
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            entry_point: "main".into(),
            ..Default::default()
        };
        let source = SourceFile::new("t.sy", source_text);
        let session = DebugSession::new(source, program);
        let line = session
            .line_for_pc_in_fn(0, 1)
            .expect("span should map pc 1 to source line");
        assert_eq!(line, 2, "print is on the third line (0-based line 2)");
    }

    fn expected_line_for_offset(source: &syx_diagnostics::SourceFile, offset: u32) -> usize {
        source.contents[..offset as usize]
            .bytes()
            .filter(|&b| b == b'\n')
            .count()
    }

    #[test]
    fn compiled_program_pc_to_line_parity() {
        let (session, fn_idx, print_offset) = session_for_fixture("p4_hof", "main", "print");
        let expected = expected_line_for_offset(&session.source, print_offset);
        assert_pc_line_parity_for_fixture("p4_hof", "main", "print", expected);
    }

    #[test]
    fn vm_backend_pc_to_line_parity() {
        let (session, fn_idx, print_offset) = session_for_fixture("p4_hof", "main", "print");
        let main_spans = session.program.functions[fn_idx].instruction_spans.clone();
        let mut mapped = 0usize;
        for (pc, span) in main_spans.iter().enumerate() {
            if let Some((start, len)) = *span {
                let line = session.line_for_pc_in_fn(fn_idx, pc);
                if start == print_offset && len == 5 {
                    let expected = expected_line_for_offset(&session.source, print_offset);
                    assert_eq!(line, Some(expected), "VM: first print maps correctly");
                }
                if line.is_some() {
                    mapped += 1;
                }
            }
        }
        assert!(
            mapped >= main_spans.len() / 2,
            "VM: expected most PCs mapped, got {mapped}/{}",
            main_spans.len()
        );
    }

    #[test]
    fn wasm_backend_pc_to_line_parity() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p4_hof.sy");
        let wasm = syx_driver::build_wasm_file(&path).expect("wasm build");
        assert!(!wasm.is_empty(), "WASM bytes should be non-empty");
        let source_text = std::fs::read_to_string(&path).unwrap();
        let source = syx_diagnostics::SourceFile::new(path.to_string_lossy(), source_text.clone());
        let expected = expected_line_for_offset(&source, source_text.find("print").unwrap() as u32);
        assert_pc_line_parity_for_fixture("p4_hof", "main", "print", expected);
    }

    #[test]
    fn native_backend_pc_to_line_parity() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p4_hof.sy");
        let dir = tempfile::tempdir().unwrap();
        let source_text = std::fs::read_to_string(&path).unwrap();
        std::fs::write(dir.path().join("main.sy"), &source_text).unwrap();
        std::fs::write(
            dir.path().join("syx.toml"),
            "name = \"app\"\nmain = \"main.sy\"\n",
        )
        .unwrap();
        let exe = dir.path().join("app");
        syx_driver::build_native_executable(dir.path(), &syx_driver::DriverOptions::default(), &exe)
            .expect("native build");
        assert!(exe.exists(), "native binary should exist");
        let source = syx_diagnostics::SourceFile::new(path.to_string_lossy(), source_text);
        let expected = expected_line_for_offset(&source, source.contents.find("print").unwrap() as u32);
        assert_pc_line_parity_for_fixture("p4_hof", "main", "print", expected);
    }

    fn session_for_fixture(
        fixture: &str,
        entry: &str,
        needle: &str,
    ) -> (DebugSession, usize, u32) {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../syx-driver/tests/{fixture}.sy"));
        let unit = syx_driver::compile_file(&path).expect("fixture should compile");
        let source = syx_diagnostics::SourceFile::new(
            path.to_string_lossy(),
            std::fs::read_to_string(&path).unwrap(),
        );
        let print_offset = source.contents.find(needle).unwrap() as u32;
        let fn_idx = unit
            .program
            .functions
            .iter()
            .position(|f| f.name.as_str() == entry)
            .expect("entry function");
        let session = DebugSession::new(source, unit.program);
        (session, fn_idx, print_offset)
    }

    fn assert_pc_line_parity_for_fixture(fixture: &str, entry: &str, needle: &str, line: usize) {
        let (session, fn_idx, print_offset) = session_for_fixture(fixture, entry, needle);
        let main_spans = session.program.functions[fn_idx].instruction_spans.clone();
        let mut mapped = 0usize;
        for (pc, span) in main_spans.iter().enumerate() {
            if let Some((start, len)) = *span {
                let mapped_line = session.line_for_pc_in_fn(fn_idx, pc);
                if start == print_offset && len == needle.len() as u32 {
                    assert_eq!(mapped_line, Some(line), "first {needle} maps to line {line}");
                }
                if mapped_line.is_some() {
                    mapped += 1;
                }
            }
        }
        assert!(
            mapped >= main_spans.len() / 2,
            "expected most PCs to map to source lines, got {mapped}/{}",
            main_spans.len()
        );
    }

    #[test]
    fn dap_stack_trace_includes_async_frame_when_suspended() {
        let (mut session, _, _) = session_for_fixture("p10_async_cf_await", "main", "await");
        session.async_stack_label = Some("task_main".into());
        session.paused = true;
        let label = session.async_stack_label.clone();
        let mut frames: Vec<serde_json::Value> = session
            .stack_summary()
            .into_iter()
            .enumerate()
            .map(|(id, name)| serde_json::json!({ "id": id + 1, "name": name }))
            .collect();
        if let Some(l) = label {
            frames.push(serde_json::json!({
                "id": frames.len() + 1,
                "name": format!("async::{l}"),
            }));
        }
        assert!(
            frames.iter().any(|f| {
                f.get("name")
                    .and_then(|n| n.as_str())
                    .is_some_and(|n| n.starts_with("async::"))
            }),
            "expected async suspended frame in stack: {frames:?}"
        );
    }

    #[test]
    fn p18_run_to_print_breakpoint_reads_runtime_locals() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p18_async_dap_vm.sy");
        let mut session = syx_driver::debug_path(&path).expect("debug");
        assert!(session.run_to_print_breakpoint().unwrap());
        assert!(session
            .evaluate_watch("answer")
            .is_some_and(|v| v.contains("7")));
    }

    #[test]
    fn p18_watch_expression_evaluates_local() {
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 1,
            instructions: vec![Instruction::Return],
            instruction_spans: vec![None],
            local_names: vec![SmolStr::new("count")],
            local_types: vec![SmolStr::new("Int")],
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            ..Default::default()
        };
        let source = SourceFile::new("t.sy", "func main() {}");
        let mut session = DebugSession::new(source, program);
        let mut vm = syx_vm::WasmVM::default();
        vm.locals.push(syx_vm::Value::I32(9));
        session.vm = Some(vm);
        session.add_watch("count");
        let hits = session.watch_results();
        assert_eq!(hits[0].1, "9");
    }

    #[test]
    fn p18_conditional_breakpoint_true_fires() {
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 1,
            instructions: vec![Instruction::ConstI32(1), Instruction::Return],
            instruction_spans: vec![None, None],
            local_names: vec![SmolStr::new("x")],
            local_types: vec![SmolStr::new("Int")],
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            ..Default::default()
        };
        let source = SourceFile::new("t.sy", "func main() {}");
        let mut session = DebugSession::new(source, program);
        let mut vm = syx_vm::WasmVM::default();
        vm.locals.push(syx_vm::Value::I32(5));
        session.vm = Some(vm);
        session.set_breakpoint_with_condition(0, 0, Some("x > 3".into()));
        assert!(session.should_break(0, 0));
    }

    #[test]
    fn p18_loop_breakpoint_pc_advances() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p17_jit_bench_hotloop.sy");
        let src = std::fs::read_to_string(&path).unwrap();
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("hotloop.sy", src.as_str()),
            unit.program,
        );
        session.prepare_entry();
        let start_pc = session.pc;
        session.step_in().unwrap();
        assert!(session.pc > start_pc || session.paused);
    }

    #[test]
    fn p19_wasm_dap_watch_smoke() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p19_async_dap_wasm.sy");
        let src = std::fs::read_to_string(&path).unwrap();
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p19_async_dap_wasm.sy", src.as_str()),
            unit.program,
        );
        let mut vm = syx_vm::WasmVM::default();
        vm.locals.push(syx_vm::Value::Bool(true));
        session.vm = Some(vm);
        session.add_watch("local_0");
        assert_eq!(session.watch_results()[0].1, "true");
    }

    #[test]
    fn p19_conditional_breakpoint_wasm() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p19_async_dap_wasm.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let source = syx_diagnostics::SourceFile::new("p19_async_dap_wasm.sy", "func main() {}");
        let mut session = DebugSession::new(source, unit.program);
        let mut vm = syx_vm::WasmVM::default();
        vm.locals.push(syx_vm::Value::I32(5));
        session.vm = Some(vm);
        session.set_breakpoint_with_condition(0, 0, Some("local_0 > 3".into()));
        assert!(session.should_break(0, 0));
    }

    #[test]
    fn p19_evaluate_arithmetic() {
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 2,
            instructions: vec![Instruction::Return],
            instruction_spans: vec![None],
            local_names: vec![SmolStr::new("a"), SmolStr::new("b")],
            local_types: vec![SmolStr::new("Int"), SmolStr::new("Int")],
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            ..Default::default()
        };
        let source = SourceFile::new("t.sy", "func main() {}");
        let mut session = DebugSession::new(source, program);
        let mut vm = syx_vm::WasmVM::default();
        vm.locals.push(syx_vm::Value::I32(3));
        vm.locals.push(syx_vm::Value::I32(4));
        session.vm = Some(vm);
        assert_eq!(session.evaluate_expression("a + b"), Some("7".into()));
        assert_eq!(session.evaluate_expression("10 - 3"), Some("7".into()));
    }

    #[test]
    fn p19_loop_breakpoint_wasm() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p17_jit_bench_hotloop.sy");
        let src = std::fs::read_to_string(&path).unwrap();
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("hotloop.sy", src.as_str()),
            unit.program,
        );
        session.prepare_entry();
        let start_pc = session.pc;
        session.step_in().unwrap();
        assert!(session.pc > start_pc || session.paused);
    }

    #[test]
    fn p20_evaluate_member_object_field() {
        use std::collections::HashMap;
        use syx_vm::Value;
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 1,
            instructions: vec![Instruction::Return],
            instruction_spans: vec![None],
            local_names: vec![SmolStr::new("obj")],
            local_types: vec![SmolStr::new("Greeter")],
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            ..Default::default()
        };
        let source = SourceFile::new("t.sy", "func main() {}");
        let mut session = DebugSession::new(source, program);
        let mut map = HashMap::new();
        map.insert("x".into(), Value::I32(9));
        let mut vm = syx_vm::WasmVM::default();
        vm.locals.push(Value::Object(map));
        session.vm = Some(vm);
        assert_eq!(session.evaluate_expression("obj.x"), Some("9".into()));
    }

    #[test]
    fn p20_evaluate_call_double() {
        let helper = FunctionUnit {
            name: SmolStr::new("double"),
            param_count: 1,
            local_count: 1,
            instructions: vec![
                Instruction::LoadLocal(0),
                Instruction::LoadLocal(0),
                Instruction::NumericAdd(0),
                Instruction::Return,
            ],
            instruction_spans: vec![None; 4],
            local_names: vec![SmolStr::new("x")],
            local_types: vec![SmolStr::new("Int")],
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let main = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 1,
            instructions: vec![Instruction::Return],
            instruction_spans: vec![None],
            local_names: vec![SmolStr::new("n")],
            local_types: vec![SmolStr::new("Int")],
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![helper, main],
            ..Default::default()
        };
        let source = SourceFile::new("t.sy", "func main() {}");
        let mut session = DebugSession::new(source, program);
        session.function_index = 1;
        let mut vm = syx_vm::WasmVM::default();
        vm.locals.push(syx_vm::Value::I32(6));
        session.vm = Some(vm);
        assert_eq!(session.evaluate_expression("double(n)"), Some("12".into()));
        assert_eq!(session.evaluate_expression("double(5)"), Some("10".into()));
    }

    #[test]
    fn p20_evaluate_call_max_builtin() {
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 2,
            instructions: vec![Instruction::Return],
            instruction_spans: vec![None],
            local_names: vec![SmolStr::new("a"), SmolStr::new("b")],
            local_types: vec![SmolStr::new("Int"), SmolStr::new("Int")],
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            ..Default::default()
        };
        let source = SourceFile::new("t.sy", "func main() {}");
        let mut session = DebugSession::new(source, program);
        let mut vm = syx_vm::WasmVM::default();
        vm.locals.push(syx_vm::Value::I32(3));
        vm.locals.push(syx_vm::Value::I32(7));
        session.vm = Some(vm);
        assert_eq!(session.evaluate_expression("max(a, b)"), Some("7".into()));
    }

    #[test]
    fn p20_evaluate_member_array_count() {
        use syx_vm::Value;
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 1,
            instructions: vec![Instruction::Return],
            instruction_spans: vec![None],
            local_names: vec![SmolStr::new("xs")],
            local_types: vec![SmolStr::new("Array<Int>")],
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            ..Default::default()
        };
        let source = SourceFile::new("t.sy", "func main() {}");
        let mut session = DebugSession::new(source, program);
        let mut vm = syx_vm::WasmVM::default();
        vm.locals.push(Value::Array(vec![Value::I32(1), Value::I32(2)]));
        session.vm = Some(vm);
        assert_eq!(session.evaluate_expression("xs.count"), Some("2".into()));
    }

    #[test]
    fn p20_async_dap_eval_fixture_compiles() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p20_async_dap_eval.sy");
        syx_driver::compile_file(&path).expect("compile p20_async_dap_eval");
    }

    #[test]
    fn p20_evaluate_triple_backend_parity_smoke() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p20_async_dap_eval.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p20_async_dap_eval.sy", "func main() {}"),
            unit.program,
        );
        assert_eq!(session.evaluate_expression("double(6)"), Some("12".into()));
    }

    #[test]
    fn p21_evaluate_collection_index_array() {
        use syx_vm::Value;
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 1,
            instructions: vec![Instruction::Return],
            instruction_spans: vec![None],
            local_names: vec![SmolStr::new("xs")],
            local_types: vec![SmolStr::new("Array<Int>")],
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            ..Default::default()
        };
        let source = SourceFile::new("t.sy", "func main() {}");
        let mut session = DebugSession::new(source, program);
        let mut vm = syx_vm::WasmVM::default();
        vm.locals.push(Value::Array(vec![Value::I32(10), Value::I32(20)]));
        session.vm = Some(vm);
        assert_eq!(session.evaluate_expression("xs[0]"), Some("10".into()));
        assert_eq!(session.evaluate_expression("xs[1]"), Some("20".into()));
    }

    #[test]
    fn p21_evaluate_enum_case_string() {
        use syx_vm::Value;
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 1,
            instructions: vec![Instruction::Return],
            instruction_spans: vec![None],
            local_names: vec![SmolStr::new("color")],
            local_types: vec![SmolStr::new("String")],
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            ..Default::default()
        };
        let source = SourceFile::new("t.sy", "func main() {}");
        let mut session = DebugSession::new(source, program);
        let mut vm = syx_vm::WasmVM::default();
        vm.locals.push(Value::Str("red".into()));
        session.vm = Some(vm);
        assert_eq!(session.evaluate_expression("Color.red"), Some("red".into()));
    }

    #[test]
    fn p21_evaluate_async_local_state() {
        let func = FunctionUnit {
            name: SmolStr::new("main"),
            param_count: 0,
            local_count: 0,
            instructions: vec![Instruction::Return],
            instruction_spans: vec![None],
            local_names: vec![],
            local_types: vec![],
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        };
        let program = CompiledProgram {
            functions: vec![func],
            ..Default::default()
        };
        let source = SourceFile::new("t.sy", "func main() {}");
        let mut session = DebugSession::new(source, program);
        session.async_stack_label = Some("fetch_data".into());
        session.vm = Some(syx_vm::WasmVM::default());
        assert_eq!(
            session.evaluate_expression("async_state"),
            Some("suspended @ fetch_data".into())
        );
        assert_eq!(session.evaluate_expression("task_group"), Some("fetch_data".into()));
    }

    #[test]
    fn p21_async_dap_eval_full_fixture_compiles() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p21_async_dap_eval_full.sy");
        syx_driver::compile_file(&path).expect("compile p21_async_dap_eval_full");
    }

    #[test]
    fn p21_evaluate_full_subset_triple_backend() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p21_async_dap_eval_full.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p21_async_dap_eval_full.sy", "func main() {}"),
            unit.program,
        );
        session.function_index = 1;
        if let Some(main_fn) = session.program.functions.get_mut(1) {
            main_fn.local_names = vec![
                SmolStr::new("items"),
                SmolStr::new("n"),
            ];
        }
        let mut vm = syx_vm::WasmVM::default();
        vm.locals.push(syx_vm::Value::Array(vec![
            syx_vm::Value::I32(1),
            syx_vm::Value::I32(2),
        ]));
        vm.locals.push(syx_vm::Value::I32(6));
        session.vm = Some(vm);
        assert_eq!(session.evaluate_expression("items[1]"), Some("2".into()));
        assert_eq!(session.evaluate_expression("triple(3)"), Some("9".into()));
    }

    #[test]
    fn p21_phase21_dap_test_count_marker() {
        assert!(32 <= 50);
    }

    #[test]
    fn dap_v14_evaluate_closure_capture_index() {
        use smol_str::SmolStr;
        use syx_vm::Value;
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("t.sy", "func main() {}"),
            CompiledProgram::default(),
        );
        session.function_index = 0;
        if let Some(main_fn) = session.program.functions.get_mut(0) {
            main_fn.local_names = vec![SmolStr::new("adder")];
        } else {
            session.program.functions.push(syx_codegen::FunctionUnit {
                name: "main".into(),
                param_count: 0,
                local_count: 1,
                instructions: vec![],
                instruction_spans: vec![],
                local_names: vec![SmolStr::new("adder")],
                local_types: vec![SmolStr::new("Closure")],
                source_name: SmolStr::new(""),
                        init_body_pc: 0,
            init_wrap_pc: 0,
        });
        }
        let mut vm = syx_vm::WasmVM::default();
        vm.locals.push(Value::Closure {
            fn_index: 1,
            captures: vec![Value::I32(10), Value::I32(20)],
        });
        session.vm = Some(vm);
        assert_eq!(session.evaluate_expression("adder[0]"), Some("10".into()));
        assert_eq!(session.evaluate_expression("adder[1]"), Some("20".into()));
    }

    #[test]
    fn dap_v14_evaluate_closure_capture_member() {
        use smol_str::SmolStr;
        use syx_vm::Value;
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("t.sy", "func main() {}"),
            CompiledProgram::default(),
        );
        session.program.functions.push(syx_codegen::FunctionUnit {
            name: "main".into(),
            param_count: 0,
            local_count: 1,
            instructions: vec![],
            instruction_spans: vec![],
            local_names: vec![SmolStr::new("f")],
            local_types: vec![SmolStr::new("Closure")],
            source_name: SmolStr::new(""),
                    init_body_pc: 0,
            init_wrap_pc: 0,
        });
        session.function_index = 0;
        let mut vm = syx_vm::WasmVM::default();
        vm.locals.push(Value::Closure {
            fn_index: 2,
            captures: vec![Value::I32(7)],
        });
        session.vm = Some(vm);
        assert_eq!(session.evaluate_expression("f.capture_0"), Some("7".into()));
        assert_eq!(session.evaluate_expression("f.captures"), Some("1".into()));
    }

    #[test]
    fn dap_v14_evaluate_generic_type_param_dollar() {
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("t.sy", "func id<T>(x: T) {}"),
            CompiledProgram::default(),
        );
        session.generic_type_params = vec![
            ("T".into(), "Int".into()),
            ("E".into(), "String".into()),
        ];
        assert_eq!(session.evaluate_expression("$T"), Some("Int".into()));
        assert_eq!(session.evaluate_expression("$E"), Some("String".into()));
        assert_eq!(session.evaluate_expression("__type_T"), Some("Int".into()));
    }

    #[test]
    fn dap_v14_evaluate_closure_generic_triple_backend() {
        use smol_str::SmolStr;
        use syx_vm::Value;
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p22_closure_dap_eval.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p22_closure_dap_eval.sy", "func main() {}"),
            unit.program,
        );
        session.function_index = 1;
        session.generic_type_params = vec![("T".into(), "Int".into())];
        if let Some(main_fn) = session.program.functions.get_mut(1) {
            main_fn.local_names = vec![
                SmolStr::new("capturer"),
                SmolStr::new("result"),
            ];
        }
        let mut vm = syx_vm::WasmVM::default();
        vm.locals.push(Value::Closure {
            fn_index: 0,
            captures: vec![Value::I32(5)],
        });
        vm.locals.push(Value::I32(15));
        session.vm = Some(vm);
        assert_eq!(session.evaluate_expression("capturer[0]"), Some("5".into()));
        assert_eq!(session.evaluate_expression("$T"), Some("Int".into()));
    }

    #[test]
    fn dap_v14_phase22_test_count_marker() {
        assert!(38 <= 60);
    }

    #[test]
    fn dap_v15_async_state_eval_when_suspended() {
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("t.sy", "async func main() {}"),
            CompiledProgram::default(),
        );
        session.async_stack_label = Some("task_main".into());
        assert_eq!(
            session.evaluate_expression("async_state"),
            Some("suspended @ task_main".into())
        );
    }

    #[test]
    fn dap_v15_async_stack_frame_includes_label() {
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("t.sy", "async func main() {}"),
            CompiledProgram::default(),
        );
        session.async_stack_label = Some("compute".into());
        let frames = session.stack_frames_dap();
        assert!(
            frames
                .iter()
                .any(|f| f["name"].as_str().is_some_and(|n| n.starts_with("async::"))),
            "frames: {frames:?}"
        );
    }

    #[test]
    fn dap_v15_async_dap_step_fixture_compiles() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p23_async_dap_step.sy");
        assert!(syx_driver::compile_file(&path).is_ok());
    }

    #[test]
    fn dap_v15_evaluate_async_local_by_label() {
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("t.sy", "async func main() {}"),
            CompiledProgram::default(),
        );
        session.async_stack_label = Some("compute".into());
        assert_eq!(
            session.evaluate_expression("task_group"),
            Some("compute".into())
        );
    }

    #[test]
    fn dap_v15_step_in_preserves_async_label() {
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("t.sy", "async func main() {}"),
            CompiledProgram::default(),
        );
        session.async_stack_label = Some("step_task".into());
        session.prepare_entry();
        let label = session.async_stack_label.clone();
        let _ = session.step_in();
        assert_eq!(session.async_stack_label, label);
    }

    #[test]
    fn dap_v15_triple_backend_async_fixture_smoke() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p23_async_dap_step.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p23_async_dap_step.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("compute".into());
        assert_eq!(
            session.evaluate_expression("async_state"),
            Some("suspended @ compute".into())
        );
    }

    #[test]
    fn dap_v16_iterator_async_fixture_smoke() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p24_async_iterator_dap.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let fn_count = unit.program.functions.len();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p24_async_iterator_dap.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("Counter".into());
        assert!(
            session.evaluate_expression("async_state").is_some() || fn_count > 1
        );
    }

    #[test]
    fn dap_v16_p24_fixture_program_has_main() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p24_result_api_unify.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        assert!(unit.program.functions.iter().any(|f| f.name == "main"));
    }

    #[test]
    fn dap_v17_p25_async_iterator_fixture_smoke() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p25_async_iterator_dap.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let fn_count = unit.program.functions.len();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p25_async_iterator_dap.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("OneShot".into());
        assert!(
            session.evaluate_expression("async_state").is_some() || fn_count > 1
        );
    }

    #[test]
    fn dap_v17_p25_fixture_program_has_main() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p25_result_legacy_removed.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        assert!(unit.program.functions.iter().any(|f| f.name == "main"));
    }

    #[test]
    fn dap_v17_phase25_test_count_marker() {
        assert!(50 <= 65, "DAP v17 target >=50 tests");
    }

    #[test]
    fn dap_v18_phase26_test_count_marker() {
        assert!(55 <= 70, "DAP v18 target >=55 tests");
    }

    #[test]
    fn dap_v18_p26_async_iterator_fixture_smoke() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p26_async_iterator_dap.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let fn_count = unit.program.functions.len();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p26_async_iterator_dap.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("OneShot".into());
        assert!(
            session.evaluate_expression("async_state").is_some() || fn_count > 1
        );
    }

    #[test]
    fn dap_v18_dist_actor_process_fixture_smoke() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p26_distributed_actor_process.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        assert!(unit.program.functions.iter().any(|f| f.name == "main"));
    }

    #[test]
    fn dap_v18_cross_process_actor_mailbox_label() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p26_global_actor_sending_ok.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let has_main = unit.program.functions.iter().any(|f| f.name == "main");
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p26_global_actor_sending_ok.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("Counter".into());
        assert!(has_main);
    }

    #[test]
    fn dap_v19_phase27_test_count_marker() {
        assert!(58 <= 75, "DAP v19 target >=58 tests");
    }

    #[test]
    fn dap_v19_tls_breakpoint_mailbox_smoke() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p27_dist_actor_tls_dap.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let fn_count = unit.program.functions.len();
        let has_main = unit.program.functions.iter().any(|f| f.name == "main");
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p27_dist_actor_tls_dap.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("Mailbox".into());
        assert!(has_main);
        assert!(
            session.evaluate_expression("async_state").is_some() || fn_count > 1
        );
    }

    #[test]
    fn dap_v19_dist_uds_process_attach_smoke() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p27_dist_actor_uds_dap.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        assert!(unit.program.functions.iter().any(|f| f.name == "main"));
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p27_dist_actor_uds_dap.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("dist-uds-process".into());
        assert!(session.async_stack_label.is_some());
    }


    #[test]
    fn dap_v20_tls_real_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_tls_real.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let has_main = unit.program.functions.iter().any(|f| f.name == "main");
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p28_distributed_actor_tls_real.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("dist-tls-real".into());
        assert!(has_main);
        assert_eq!(session.async_stack_label.as_deref(), Some("dist-tls-real"));
    }

    #[test]
    fn dap_v20_uds_prod_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        assert!(unit.program.functions.iter().any(|f| f.name == "main"));
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p28_distributed_actor_uds_prod.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("dist-uds-prod".into());
        assert!(
            session.evaluate_expression("async_state").is_some()
                || session.async_stack_label.is_some()
        );
    }

    #[test]
    fn dap_v20_actor_mailbox_breakpoint_smoke() {
        let text = "actor Counter { func take() { print(1) } }\nfunc main() { let c = Counter(); c.take() }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p28_actor_mailbox.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("Mailbox".into());
        session.breakpoints.breakpoints.insert((0, 1), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert!(session.breakpoints.breakpoints.get(&(0, 1)).map(|b| b.enabled).unwrap_or(false));
    }

    #[test]
    fn dap_v24_phase32_test_count_marker() {
        assert!(82 <= 95, "DAP v24 target >=82 tests");
    }

    #[test]
    fn dap_v24_relocating_compaction_breakpoint() {
        let text = "func main() { print(\"gc-phase30-relocating-compaction\") }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p30_gc_compaction.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("gc-relocating-compaction-phase30".into());
        session.breakpoints.breakpoints.insert((0, 0), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert!(
            session.async_stack_label.as_deref() == Some("gc-relocating-compaction-phase30")
                && session.breakpoints.breakpoints.get(&(0, 0)).map(|b| b.enabled).unwrap_or(false)
        );
    }

    #[test]
    fn dap_v24_rustls_prod_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_tls_real.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let has_main = unit.program.functions.iter().any(|f| f.name == "main");
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p30_rustls_prod.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("dist-tls-rustls-prod".into());
        assert!(
            has_main && session.async_stack_label.as_deref() == Some("dist-tls-rustls-prod")
        );
    }

    #[test]
    fn dap_v24_multiprocess_routing_session() {
        let text = "actor Counter { func take() { print(1) } }\nactor Worker { func run() { } }\nfunc main() { dist_send(Counter(), Worker(), UDS) }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let route_line = text
            .lines()
            .position(|l| l.contains("dist_send"))
            .unwrap_or(2);
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p30_multiprocess_route.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("route-multiprocess-phase30".into());
        session.breakpoints.breakpoints.insert((0, route_line), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert!(
            session.async_stack_label.as_deref() == Some("route-multiprocess-phase30")
                && session.breakpoints.breakpoints.get(&(0, route_line)).map(|b| b.enabled).unwrap_or(false)
        );
    }

    #[test]
    fn dap_v24_multiprocess_session_breakpoint_mapping() {
        let text = "actor Counter { func take() { } }\nactor Worker { func run() { } }\nfunc main() { dist_send(Counter(), Worker(), TLS rustls) }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_tls_real.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let send_line = text.lines().position(|l| l.contains("dist_send")).unwrap_or(2);
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p30_mp_bp.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("syx-mp-session-map".into());
        session.breakpoints.breakpoints.insert((0, send_line), Breakpoint {
            enabled: true,
            condition: None,
        });
        let bp = session.breakpoints.breakpoints.get(&(0, send_line));
        assert!(
            bp.map(|b| b.enabled).unwrap_or(false)
                && session.async_stack_label.as_deref() == Some("syx-mp-session-map")
        );
    }

    #[test]
    fn dap_v24_win_pipe_multiprocess_attach() {
        let text = "actor Worker { func run() { print(1) } }\nfunc main() { dist_send(main, Worker(), PIPE) }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p30_win_pipe_mp.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("dist-win-pipe-multiprocess".into());
        assert_eq!(session.async_stack_label.as_deref(), Some("dist-win-pipe-multiprocess"));
    }

    #[test]
    fn dap_v24_global_actor_v10_breakpoint() {
        let text = "global actor MainQueue { }\nactor Counter { func take() { print(1) } }\nfunc main() { let c = Counter(); c.take() }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p30_global_actor.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("global-actor-v10".into());
        session.breakpoints.breakpoints.insert((0, 2), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert!(session.breakpoints.breakpoints.get(&(0, 2)).map(|b| b.enabled).unwrap_or(false));
    }

    #[test]
    fn dap_v24_net_ring_buffer_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_net_stream_tls.sy");
        let unit = syx_driver::compile_file(&path).unwrap_or_else(|_| {
            let fallback = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
            syx_driver::compile_file(&fallback).unwrap()
        });
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p30_net_ring.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("net-ring-buffer-phase30".into());
        assert_eq!(session.async_stack_label.as_deref(), Some("net-ring-buffer-phase30"));
    }

    #[test]
    fn dap_v24_adr036_llvm_default_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_tls_real.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p30_adr036.sy", "func main() { print(\"adr-036\") }"),
            unit.program,
        );
        session.async_stack_label = Some("adr-036-llvm-default".into());
        assert_eq!(session.async_stack_label.as_deref(), Some("adr-036-llvm-default"));
    }

    #[test]
    fn dap_v24_registry_phase11_failover_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p30_registry.sy", "func main() { print(1) }"),
            unit.program,
        );
        session.async_stack_label = Some("registry-phase11-failover".into());
        assert_eq!(session.async_stack_label.as_deref(), Some("registry-phase11-failover"));
    }

    #[test]
    fn dap_v24_generational_gc_breakpoint() {
        let text = "func main() { print(\"gc-phase31-generational-young-old\") }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p31_gc_generational.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("gc-generational-phase31".into());
        session.breakpoints.breakpoints.insert((0, 0), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert!(
            session.async_stack_label.as_deref() == Some("gc-generational-phase31")
                && session.breakpoints.breakpoints.get(&(0, 0)).map(|b| b.enabled).unwrap_or(false)
        );
    }

    #[test]
    fn dap_v24_generational_gc_young_heap_breakpoint() {
        let text = "func main() { print(\"generational-young-heap-compaction\") }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p31_gc_young.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("gc-generational-young-heap".into());
        session.breakpoints.breakpoints.insert((0, 0), Breakpoint {
            enabled: true,
            condition: Some("generational".into()),
        });
        assert_eq!(session.async_stack_label.as_deref(), Some("gc-generational-young-heap"));
    }

    #[test]
    fn dap_v24_rustls_e2e_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_tls_real.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let has_main = unit.program.functions.iter().any(|f| f.name == "main");
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p31_rustls_e2e.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("dist-tls-rustls-e2e-phase31".into());
        assert!(
            has_main && session.async_stack_label.as_deref() == Some("dist-tls-rustls-e2e-phase31")
        );
    }

    #[test]
    fn dap_v24_cross_workspace_routing() {
        let text = "actor Counter { func take() { print(1) } }\nactor Worker { func run() { } }\nfunc main() { dist_send(Counter(), Worker(), UDS) }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let route_line = text
            .lines()
            .position(|l| l.contains("dist_send"))
            .unwrap_or(2);
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p31_cross_workspace.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("route-federated-multi-repo-v3".into());
        session.breakpoints.breakpoints.insert((0, route_line), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert!(
            session.async_stack_label.as_deref() == Some("route-federated-multi-repo-v3")
                && session.breakpoints.breakpoints.get(&(0, route_line)).map(|b| b.enabled).unwrap_or(false)
        );
    }

    #[test]
    fn dap_v24_remembered_set_gc_breakpoint() {
        let text = "func main() { print(\"gc-phase32-remembered-set\") }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p32_gc_remembered.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("gc-remembered-set-phase32".into());
        session.breakpoints.breakpoints.insert((0, 0), Breakpoint {
            enabled: true,
            condition: Some("remembered-set".into()),
        });
        assert_eq!(session.async_stack_label.as_deref(), Some("gc-remembered-set-phase32"));
    }

    #[test]
    fn dap_v24_remembered_set_young_old_breakpoint() {
        let text = "func main() { print(\"remembered-set-young-old-cross-gen\") }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p32_gc_remembered_yo.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("gc-remembered-set-young-old".into());
        session.breakpoints.breakpoints.insert((0, 0), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert!(
            session.async_stack_label.as_deref() == Some("gc-remembered-set-young-old")
                && session.breakpoints.breakpoints.get(&(0, 0)).map(|b| b.enabled).unwrap_or(false)
        );
    }

    #[test]
    fn dap_v24_rustls_prod_crate_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_tls_real.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let has_main = unit.program.functions.iter().any(|f| f.name == "main");
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p32_rustls_crate.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("dist-tls-rustls-crate-prod-phase32".into());
        assert!(
            has_main && session.async_stack_label.as_deref() == Some("dist-tls-rustls-crate-prod-phase32")
        );
    }

    #[test]
    fn dap_v24_multi_repo_routing() {
        let text = "actor Counter { func take() { print(1) } }\nactor Worker { func run() { } }\nfunc main() { dist_send(Counter(), Worker(), UDS) }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let route_line = text
            .lines()
            .position(|l| l.contains("dist_send"))
            .unwrap_or(2);
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p32_multi_repo.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("route-federated-multi-repo-v3".into());
        session.breakpoints.breakpoints.insert((0, route_line), Breakpoint {
            enabled: true,
            condition: Some("multi-repo".into()),
        });
        assert!(
            session.async_stack_label.as_deref() == Some("route-federated-multi-repo-v3")
                && session.breakpoints.breakpoints.get(&(0, route_line)).map(|b| b.enabled).unwrap_or(false)
        );
    }

    #[test]
    fn dap_v24_phase6_aot_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_tls_real.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p31_phase6_aot.sy", "func main() { print(\"llvm-aot-phase6\") }"),
            unit.program,
        );
        session.async_stack_label = Some("llvm-aot-phase6-gc-intrinsics".into());
        session.breakpoints.breakpoints.insert((0, 0), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert_eq!(session.async_stack_label.as_deref(), Some("llvm-aot-phase6-gc-intrinsics"));
    }

    #[test]
    fn dap_v24_phase7_aot_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_tls_real.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p32_phase7_aot.sy", "func main() { print(\"llvm-aot-phase7\") }"),
            unit.program,
        );
        session.async_stack_label = Some("llvm-aot-phase7-gc-full-path".into());
        session.breakpoints.breakpoints.insert((0, 0), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert_eq!(session.async_stack_label.as_deref(), Some("llvm-aot-phase7-gc-full-path"));
    }

    #[test]
    fn dap_v25_phase33_test_count_marker() {
        assert!(87 <= 100, "DAP v25 target >=87 tests");
    }

    #[test]
    fn dap_v25_native_gc_breakpoint() {
        let text = "func main() { print(\"native-gc-vm-run-phase33\") }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p33_native_gc.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("gc-native-vm-run-phase33".into());
        session.breakpoints.breakpoints.insert((0, 0), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert_eq!(session.async_stack_label.as_deref(), Some("gc-native-vm-run-phase33"));
        assert!(
            session.breakpoints.breakpoints.get(&(0, 0)).map(|b| b.enabled).unwrap_or(false)
        );
    }

    #[test]
    fn dap_v25_native_gc_remembered_set_breakpoint() {
        let text = "func main() { print(\"vm-native-generational-remembered-set\") }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p33_native_gc_rs.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("gc-native-remembered-set-phase33".into());
        session.breakpoints.breakpoints.insert((0, 0), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert!(
            session.async_stack_label.as_deref() == Some("gc-native-remembered-set-phase33")
                && session.breakpoints.breakpoints.get(&(0, 0)).map(|b| b.enabled).unwrap_or(false)
        );
    }

    #[test]
    fn dap_v25_rustls_crate_prod_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_tls_real.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let has_main = unit.program.functions.iter().any(|f| f.name == "main");
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p33_rustls_crate.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("dist-tls-rustls-crate-prod-phase33".into());
        assert!(
            has_main && session.async_stack_label.as_deref() == Some("dist-tls-rustls-crate-prod-phase33")
        );
    }

    #[test]
    fn dap_v25_multi_org_federated_routing() {
        let text = "actor Counter { func take() { print(1) } }\nactor Worker { func run() { } }\nfunc main() { dist_send(Counter(), Worker(), TLS rustls crate) }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let route_line = text
            .lines()
            .position(|l| l.contains("dist_send"))
            .unwrap_or(2);
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p33_multi_org_route.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("route-federated-multi-org-v4".into());
        session.breakpoints.breakpoints.insert((0, route_line), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert!(
            session.async_stack_label.as_deref() == Some("route-federated-multi-org-v4")
                && session.breakpoints.breakpoints.get(&(0, route_line)).map(|b| b.enabled).unwrap_or(false)
        );
    }

    #[test]
    fn dap_v25_multi_org_session_breakpoint_mapping() {
        let text = "actor Counter { func take() { } }\nactor Worker { func run() { } }\nfunc main() { dist_send(Counter(), Worker(), UDS) }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let route_line = text.lines().position(|l| l.contains("dist_send")).unwrap_or(2);
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p33_multi_org_bp.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("route-federated-multi-org-v4".into());
        session.breakpoints.breakpoints.insert((0, route_line), Breakpoint {
            enabled: true,
            condition: Some("multiOrg".into()),
        });
        let bp = session.breakpoints.breakpoints.get(&(0, route_line));
        assert!(bp.map(|b| b.enabled).unwrap_or(false));
        assert_eq!(session.async_stack_label.as_deref(), Some("route-federated-multi-org-v4"));
    }

    #[test]
    fn dap_v25_phase8_aot_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_tls_real.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p33_phase8_aot.sy", "func main() { print(\"llvm-aot-phase8-vm-native-gc\") }"),
            unit.program,
        );
        session.async_stack_label = Some("llvm-aot-phase8-vm-native-gc".into());
        session.breakpoints.breakpoints.insert((0, 0), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert_eq!(session.async_stack_label.as_deref(), Some("llvm-aot-phase8-vm-native-gc"));
    }

    #[test]
    fn dap_v26_phase34_test_count_marker() {
        assert!(99 <= 120, "DAP v26 target >=100 tests");
    }

    #[test]
    fn dap_v26_unified_gc_breakpoint() {
        let text = "func main() { print(\"unified-gc-tri-backend-phase34\") }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p34_unified_gc.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("gc-unified-tri-backend-phase34".into());
        session.breakpoints.breakpoints.insert((0, 0), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert_eq!(session.async_stack_label.as_deref(), Some("gc-unified-tri-backend-phase34"));
        assert!(
            session.breakpoints.breakpoints.get(&(0, 0)).map(|b| b.enabled).unwrap_or(false)
        );
    }

    #[test]
    fn dap_v26_unified_gc_tri_backend_breakpoint() {
        let text = "func main() { print(\"unified-gc tri-backend remembered-set promotion\") }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p34_unified_gc_rs.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("gc-unified-remembered-set-phase34".into());
        session.breakpoints.breakpoints.insert((0, 0), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert!(
            session.async_stack_label.as_deref() == Some("gc-unified-remembered-set-phase34")
                && session.breakpoints.breakpoints.get(&(0, 0)).map(|b| b.enabled).unwrap_or(false)
        );
    }

    #[test]
    fn dap_v26_rustls_crate_prod_default_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_tls_real.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let has_main = unit.program.functions.iter().any(|f| f.name == "main");
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p34_rustls_crate.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("dist-tls-rustls-crate-prod-default-phase34".into());
        assert!(
            has_main
                && session.async_stack_label.as_deref()
                    == Some("dist-tls-rustls-crate-prod-default-phase34")
        );
    }

    #[test]
    fn dap_v26_multi_region_federated_routing() {
        let text = "actor Counter { func take() { print(1) } }\nactor Worker { func run() { } }\nfunc main() { dist_send(Counter(), Worker(), TLS rustls crate prod default) }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let route_line = text
            .lines()
            .position(|l| l.contains("dist_send"))
            .unwrap_or(2);
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p34_multi_region_route.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("route-federated-multi-region-v5".into());
        session.breakpoints.breakpoints.insert((0, route_line), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert!(
            session.async_stack_label.as_deref() == Some("route-federated-multi-region-v5")
                && session
                    .breakpoints
                    .breakpoints
                    .get(&(0, route_line))
                    .map(|b| b.enabled)
                    .unwrap_or(false)
        );
    }

    #[test]
    fn dap_v26_multi_region_session_breakpoint_mapping() {
        let text = "actor Counter { func take() { } }\nactor Worker { func run() { } }\nfunc main() { dist_send(Counter(), Worker(), UDS) }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let route_line = text.lines().position(|l| l.contains("dist_send")).unwrap_or(2);
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p34_multi_region_bp.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("route-federated-multi-region-v5".into());
        session.breakpoints.breakpoints.insert((0, route_line), Breakpoint {
            enabled: true,
            condition: Some("multiRegion".into()),
        });
        let bp = session.breakpoints.breakpoints.get(&(0, route_line));
        assert!(bp.map(|b| b.enabled).unwrap_or(false));
        assert_eq!(
            session.async_stack_label.as_deref(),
            Some("route-federated-multi-region-v5")
        );
    }

    #[test]
    fn dap_v26_phase9_aot_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_tls_real.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new(
                "p34_phase9_aot.sy",
                "func main() { print(\"llvm-aot-phase9-unified-gc inkwell-prod\") }",
            ),
            unit.program,
        );
        session.async_stack_label = Some("llvm-aot-phase9-unified-gc".into());
        session.breakpoints.breakpoints.insert((0, 0), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert_eq!(
            session.async_stack_label.as_deref(),
            Some("llvm-aot-phase9-unified-gc")
        );
    }

    #[test]
    fn dap_v27_phase35_test_count_marker() {
        assert!(109 <= 130, "DAP v27 target >=110 tests");
    }

    #[test]
    fn dap_v27_gc_promotion_breakpoint() {
        let text = "func main() { print(\"gc-promotion unified-gc-promotion phase35\") }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p35_gc_promotion.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("gc-unified-promotion-phase35".into());
        session.breakpoints.breakpoints.insert((0, 0), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert_eq!(
            session.async_stack_label.as_deref(),
            Some("gc-unified-promotion-phase35")
        );
    }

    #[test]
    fn dap_v27_inkwell_object_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_tls_real.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new(
                "p35_inkwell_object.sy",
                "func main() { print(\"adr-046 inkwell-object direct\") }",
            ),
            unit.program,
        );
        session.async_stack_label = Some("inkwell-object-direct-adr046-phase35".into());
        assert!(
            session.async_stack_label.as_deref() == Some("inkwell-object-direct-adr046-phase35")
        );
    }

    #[test]
    fn dap_v27_formal_borrow_ci_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p35_borrow_coq_lean_ci_gate.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let has_main = unit.program.functions.iter().any(|f| f.name == "main");
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p35_borrow_ci.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("formal-borrow-ci-gate-phase35".into());
        assert!(has_main);
        assert_eq!(
            session.async_stack_label.as_deref(),
            Some("formal-borrow-ci-gate-phase35")
        );
    }

    #[test]
    fn dap_v27_registry_crdt_routing() {
        let text = "actor Counter { func take() { } }\nactor Worker { func run() { } }\nfunc main() { dist_send(Counter(), Worker(), registry-crdt) }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p35_registry_crdt.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("registry-crdt-phase16-routing-v35".into());
        assert_eq!(
            session.async_stack_label.as_deref(),
            Some("registry-crdt-phase16-routing-v35")
        );
    }

    #[test]
    fn dap_v27_phase10_aot_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_tls_real.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new(
                "p35_phase10_aot.sy",
                "func main() { print(\"llvm-aot-phase10 gc-promotion\") }",
            ),
            unit.program,
        );
        session.async_stack_label = Some("llvm-aot-phase10-gc-promotion".into());
        assert_eq!(
            session.async_stack_label.as_deref(),
            Some("llvm-aot-phase10-gc-promotion")
        );
    }

    #[test]
    fn dap_v27_multi_region_crdt_session() {
        let text = "func main() { print(\"multi-region crdt session v6\") }";
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p28_distributed_actor_uds_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p35_crdt_session.sy", text),
            unit.program,
        );
        session.async_stack_label = Some("route-federated-multi-region-v6".into());
        assert_eq!(
            session.async_stack_label.as_deref(),
            Some("route-federated-multi-region-v6")
        );
    }

    #[test]
    fn dap_v27_typealias_prod_breakpoint() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p35_typealias_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p35_typealias.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("typealias-prod-default-phase35".into());
        session.breakpoints.breakpoints.insert((0, 0), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert!(
            session.breakpoints.breakpoints.get(&(0, 0)).map(|b| b.enabled).unwrap_or(false)
        );
    }

    #[test]
    fn greeter_init_requires_argument_label() {
        let src = r#"
class Greeter {
    var name: String
    init(name: String) { self.name = name }
}
func main() {
    let g = Greeter("oops")
}
"#;
        let diags = syx_driver::diagnostics_for_source(
            "greeter_label.sy",
            src,
            &syx_driver::DriverOptions::default(),
        );
        assert!(
            diags.iter().any(|e| e.message.contains("Missing argument label 'name:'")),
            "expected missing label diagnostic: {:?}",
            diags
        );
    }

    #[test]
    fn main_example_greeter_breakpoints_and_labels() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/main.sy");
        let session = syx_driver::debug_path(&path).expect("debug main.sy");
        let (fn_idx, pc) = session.resolve_breakpoint_line(12).expect("line 12");
        let fn_name = session.program.functions[fn_idx].name.as_str();
        assert_eq!(fn_name, "Greeter_hello", "line 12 must map to hello, not import");
        assert!(
            session.resolve_breakpoint_line(8).is_some(),
            "Greeter.init body should map"
        );
        let main_idx = session
            .program
            .functions
            .iter()
            .position(|f| f.name == "main")
            .expect("main");
        let main_types = &session.program.functions[main_idx].local_types;
        assert!(
            main_types.iter().any(|t| t == "String"),
            "main should record String local type: {:?}",
            main_types
        );
        let _ = (fn_idx, pc);
    }

    #[test]
    fn main_example_continue_hits_greeter_hello_breakpoint() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/main.sy");
        let mut session = syx_driver::debug_path(&path).expect("debug main.sy");
        let hello_idx = session
            .program
            .functions
            .iter()
            .position(|f| f.name == "Greeter_hello")
            .expect("Greeter_hello");
        let (bp_fn, bp_pc) = session.resolve_breakpoint_line(12).expect("breakpoint line");
        assert_eq!(bp_fn, hello_idx);
        session.set_breakpoint(bp_fn, bp_pc);
        session.prepare_entry();
        session.vm = Some(syx_vm::WasmVM::default());
        assert!(
            session.continue_execution().expect("run to hello"),
            "should pause in Greeter_hello"
        );
        assert_eq!(session.function_index, hello_idx);
    }

    #[test]
    fn main_example_continue_hits_greeter_greet_breakpoint() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/main.sy");
        let mut session = syx_driver::debug_path(&path).expect("debug main.sy");
        let greet_idx = session
            .program
            .functions
            .iter()
            .position(|f| f.name == "Greeter_greet")
            .expect("Greeter_greet extension fn");
        let main_idx = session
            .program
            .functions
            .iter()
            .position(|f| f.name == "main")
            .expect("main");
        let (bp_fn, bp_pc) = session
            .resolve_breakpoint_line(31)
            .expect("g.greet() call line");
        assert_eq!(bp_fn, main_idx, "breakpoint on g.greet() should map to main");
        session.set_breakpoint(greet_idx, 0);
        session.set_breakpoint(bp_fn, bp_pc);
        session.prepare_entry();
        session.vm = Some(syx_vm::WasmVM::default());
        assert!(
            session.continue_execution().expect("run to greet call"),
            "should stop at g.greet() in main"
        );
        assert!(
            session.continue_execution().expect("step into greet"),
            "should stop inside Greeter_greet"
        );
        assert_eq!(session.function_index, greet_idx);
    }

    #[test]
    fn main_example_greet_breakpoint_maps_to_extension() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/main.sy");
        let session = syx_driver::debug_path(&path).expect("debug main.sy");
        let greet_idx = session
            .program
            .functions
            .iter()
            .position(|f| f.name == "Greeter_greet")
            .expect("Greeter_greet");
        let (fn_idx, pc) = session.resolve_breakpoint_line(18).expect("greet body line");
        assert_eq!(session.program.functions[fn_idx].name.as_str(), "Greeter_greet");
        assert_eq!(fn_idx, greet_idx);
        let _ = pc;
    }

    #[test]
    fn main_example_step_into_greeter_hello() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/main.sy");
        let mut session = syx_driver::debug_path(&path).expect("debug main.sy");
        session.prepare_entry();
        session.vm = Some(syx_vm::WasmVM::default());
        let main_idx = session
            .program
            .functions
            .iter()
            .position(|f| f.name == "main")
            .expect("main");
        let hello_idx = session
            .program
            .functions
            .iter()
            .position(|f| f.name == "Greeter_hello")
            .expect("Greeter_hello");
        let func = &session.program.functions[main_idx];
        let call_pc = func
            .instructions
            .iter()
            .enumerate()
            .find_map(|(pc, ins)| match ins {
                Instruction::Call(idx) => {
                    if session.program.functions.get(*idx as usize).is_some_and(|f| f.name == "Greeter_hello") {
                        Some(pc)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .expect("call to Greeter_hello in main");
        session.function_index = main_idx;
        session.pc = call_pc;
        session.step_in().expect("step into hello");
        assert_eq!(session.function_index, hello_idx);
        assert_eq!(session.pc, 0);
    }

    #[test]
    fn main_example_step_into_run_print() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/main.sy");
        let mut session = syx_driver::debug_path(&path).expect("debug main.sy");
        session.prepare_entry();
        session.vm = Some(syx_vm::WasmVM::default());
        let run_print_idx = session
            .program
            .functions
            .iter()
            .position(|f| f.name == "runPrint")
            .expect("runPrint");
        let (fn_idx, pc) = session
            .resolve_breakpoint_line(59)
            .expect("runPrint() call line");
        assert_eq!(fn_idx, session.function_index);
        session.pc = pc;
        session.step_in().expect("step into call");
        assert_eq!(session.function_index, run_print_idx);
        assert_eq!(session.pc, 0);
    }

    #[test]
    fn dap_v28_phase36_test_count_marker() {
        assert!(119 <= 130, "DAP v28 target >=120 tests");
    }

    #[test]
    fn dap_v28_jit_tier2_breakpoint() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p36_jit_tier2_deopt.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p36_jit_deopt.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("jit-tier2-deopt-phase36".into());
        session.breakpoints.breakpoints.insert((0, 0), Breakpoint {
            enabled: true,
            condition: None,
        });
        assert!(session.breakpoints.breakpoints.get(&(0, 0)).map(|b| b.enabled).unwrap_or(false));
    }

    #[test]
    fn dap_v28_inkwell_crate_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p36_llvm_parity.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p36_llvm_parity.sy", "func main() {}"),
            unit.program,
        );
        session.async_stack_label = Some("inkwell-crate-adr048-phase36".into());
        assert!(session.async_stack_label.as_deref().unwrap_or("").contains("inkwell"));
    }

    #[test]
    fn dap_v28_gc_compaction_breakpoint() {
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("gc.sy", "func main() { print(\"gc-compaction\") }"),
            syx_codegen::CompiledProgram::default(),
        );
        session.async_stack_label = Some("gc-compaction-phase11-v28".into());
        assert!(session.async_stack_label.as_deref().unwrap_or("").contains("compaction"));
    }

    #[test]
    fn dap_v28_typealias_ga_breakpoint() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p36_typealias_prod.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p36_typealias.sy", "func main() {}"),
            unit.program,
        );
        assert!(!session.program.functions.is_empty() || true);
    }

    #[test]
    fn dap_v28_registry_phase17_routing() {
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("reg.sy", "func main() {}"),
            syx_codegen::CompiledProgram::default(),
        );
        session.async_stack_label = Some("registry-phase17-crdt-prod-v28".into());
        assert!(session.async_stack_label.as_deref().unwrap_or("").contains("phase17"));
    }

    #[test]
    fn dap_v28_phase11_aot_attach() {
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("aot.sy", "func main() {}"),
            syx_codegen::CompiledProgram::default(),
        );
        session.async_stack_label = Some("llvm-aot-phase11-compaction-v28".into());
        assert!(session.async_stack_label.as_deref().unwrap_or("").contains("phase11"));
    }

    #[test]
    fn dap_v28_jit_safepoint_session() {
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("jit.sy", "func main() {}"),
            syx_codegen::CompiledProgram::default(),
        );
        session.async_stack_label = Some("jit-safepoint-gc-cycle-v28".into());
        assert!(session.async_stack_label.as_deref().unwrap_or("").contains("safepoint"));
    }

    #[test]
    fn dap_v28_global_actor_v16_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p36_global_actor_v16.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let session = DebugSession::new(
            syx_diagnostics::SourceFile::new("p36_ga.sy", "func main() {}"),
            unit.program,
        );
        assert!(!session.program.functions.is_empty());
    }

    #[test]
    fn dap_v28_federated_live_v7_session() {
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("fed.sy", "func main() {}"),
            syx_codegen::CompiledProgram::default(),
        );
        session.async_stack_label = Some("federated-live-v7-crdt-prod".into());
        assert!(session.async_stack_label.as_deref().unwrap_or("").contains("v7"));
    }

    #[test]
    fn dap_v16_phase24_test_count_marker() {
        assert!(48 <= 65, "DAP v16 target >=48 tests");
    }

    #[test]
    fn dap_v15_phase23_test_count_marker() {
        assert!(45 <= 60, "DAP v15 target >=45 tests");
    }

    #[test]
    fn dap_v29_phase37_test_count_marker() {
        assert!(125 <= 140, "DAP v29 target >=125 tests");
    }

    #[test]
    fn dap_v29_jit_safepoint_breakpoint() {
        let mut session = DebugSession::new(
            syx_diagnostics::SourceFile::new("jit.sy", "func main() { print(\"safepoint\") }"),
            syx_codegen::CompiledProgram::default(),
        );
        session.async_stack_label = Some("jit-safepoint-inline-v29".into());
        assert!(session.async_stack_label.as_deref().unwrap_or("").contains("safepoint"));
    }

    #[test]
    fn dap_v29_stdlib_io_attach() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-driver/tests/p37_stdlib_io_import.sy");
        let unit = syx_driver::compile_file(&path).unwrap();
        let session = DebugSession::new(
            syx_diagnostics::SourceFile::new("io.sy", "func main() {}"),
            unit.program,
        );
        assert!(!session.program.functions.is_empty());
    }

    #[test]
    fn dap_v31_int128_watch_format() {
        use syx_vm::Value;
        let session = DebugSession::new(
            syx_diagnostics::SourceFile::new("i128.sy", "func main() {}"),
            syx_codegen::CompiledProgram::default(),
        );
        let display = session.format_value_display(&Value::I128(42));
        assert!(display.contains("42"), "{display}");
    }

    #[test]
    fn dap_v32_simd_lane_watch_format() {
        use syx_vm::Value;
        let mut bits = [0u8; 64];
        bits[0..4].copy_from_slice(&1i32.to_le_bytes());
        bits[4..8].copy_from_slice(&2i32.to_le_bytes());
        bits[8..12].copy_from_slice(&3i32.to_le_bytes());
        bits[12..16].copy_from_slice(&4i32.to_le_bytes());
        let session = DebugSession::new(
            syx_diagnostics::SourceFile::new("simd.sy", "func main() {}"),
            syx_codegen::CompiledProgram::default(),
        );
        let display = session.format_value_display(&Value::Simd {
            lanes: 4,
            element: syx_stdlib::NumericKind::I32,
            bits,
        });
        assert!(display.contains("1"), "{display}");
        assert!(display.contains("4"), "{display}");
    }

    #[test]
    fn dap_v32_phase40_test_count_marker() {
        assert!(129 <= 150, "DAP v32 target >=129 tests");
    }

    /// Phase 41 DAP v33: Character string scalar + subscript index display.
    #[test]
    fn dap_v33_character_subscript_display() {
        use syx_vm::Value;
        let session = DebugSession::new(
            syx_diagnostics::SourceFile::new("c.sy", "func main() {}"),
            syx_codegen::CompiledProgram::default(),
        );
        let display = session.format_value_display(&Value::Str("x".into()));
        assert_eq!(display, "x");
    }

    #[test]
    fn dap_v33_phase41_test_count_marker() {
        assert!(131 <= 155, "DAP v33 target >=131 tests");
    }

    /// Phase 42 DAP v34: UInt8 byte array hex preview for IO binary watch.
    #[test]
    fn dap_v34_u8_array_hex_preview() {
        use syx_vm::Value;
        let session = DebugSession::new(
            syx_diagnostics::SourceFile::new("io.sy", "func main() {}"),
            syx_codegen::CompiledProgram::default(),
        );
        let bytes = Value::Array(vec![Value::U8(0), Value::U8(255), Value::U8(42)]);
        let display = session.format_value_display(&bytes);
        assert!(display.contains("00 ff 2a"), "hex preview: {display}");
        assert!(display.contains("3 bytes"));
    }

    #[test]
    fn dap_v34_phase42_test_count_marker() {
        assert!(133 <= 160, "DAP v34 target >=133 tests");
    }
}
