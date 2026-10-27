//! SYXBC Text format v1 — human-readable dump of [`CompiledProgram`].

use std::fmt::Write as FmtWrite;

use super::{CompiledProgram, Instruction};

fn escape_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn format_instruction(ins: &Instruction) -> String {
    match ins {
        Instruction::ConstI32(v) => format!("ConstI32 {v}"),
        Instruction::ConstF64(v) => format!("ConstF64 {v}"),
        Instruction::ConstNumeric { kind, bits } => format!("ConstNumeric kind={kind} bits={bits}"),
        Instruction::ConstNumeric128 { kind, lo, hi } => {
            format!("ConstNumeric128 kind={kind} lo={lo} hi={hi}")
        }
        Instruction::NumericAdd(k) => format!("NumericAdd {k}"),
        Instruction::NumericSub(k) => format!("NumericSub {k}"),
        Instruction::NumericMul(k) => format!("NumericMul {k}"),
        Instruction::NumericDiv(k) => format!("NumericDiv {k}"),
        Instruction::NumericMod(k) => format!("NumericMod {k}"),
        Instruction::NumericNeg(k) => format!("NumericNeg {k}"),
        Instruction::NumericBitAnd(k) => format!("NumericBitAnd {k}"),
        Instruction::NumericBitOr(k) => format!("NumericBitOr {k}"),
        Instruction::NumericBitXor(k) => format!("NumericBitXor {k}"),
        Instruction::NumericBitNot(k) => format!("NumericBitNot {k}"),
        Instruction::NumericShl(k) => format!("NumericShl {k}"),
        Instruction::NumericShr(k) => format!("NumericShr {k}"),
        Instruction::NumericEq(k) => format!("NumericEq {k}"),
        Instruction::NumericNe(k) => format!("NumericNe {k}"),
        Instruction::NumericLt(k) => format!("NumericLt {k}"),
        Instruction::NumericLe(k) => format!("NumericLe {k}"),
        Instruction::NumericGt(k) => format!("NumericGt {k}"),
        Instruction::NumericGe(k) => format!("NumericGe {k}"),
        Instruction::NumericCast { from, to } => format!("NumericCast from={from} to={to}"),
        Instruction::SetBuild(n) => format!("SetBuild {n}"),
        Instruction::SimdAdd { lanes, element } => format!("SimdAdd lanes={lanes} element={element}"),
        Instruction::SimdSub { lanes, element } => format!("SimdSub lanes={lanes} element={element}"),
        Instruction::SimdMul { lanes, element } => format!("SimdMul lanes={lanes} element={element}"),
        Instruction::SimdDiv { lanes, element } => format!("SimdDiv lanes={lanes} element={element}"),
        Instruction::SimdEq { lanes, element } => format!("SimdEq lanes={lanes} element={element}"),
        Instruction::SimdLt { lanes, element } => format!("SimdLt lanes={lanes} element={element}"),
        Instruction::SimdGt { lanes, element } => format!("SimdGt lanes={lanes} element={element}"),
        Instruction::SimdNe { lanes, element } => format!("SimdNe lanes={lanes} element={element}"),
        Instruction::SimdLe { lanes, element } => format!("SimdLe lanes={lanes} element={element}"),
        Instruction::SimdGe { lanes, element } => format!("SimdGe lanes={lanes} element={element}"),
        Instruction::SimdAnd { lanes, element } => format!("SimdAnd lanes={lanes} element={element}"),
        Instruction::SimdOr { lanes, element } => format!("SimdOr lanes={lanes} element={element}"),
        Instruction::SimdXor { lanes, element } => format!("SimdXor lanes={lanes} element={element}"),
        Instruction::SimdNot { lanes, element } => format!("SimdNot lanes={lanes} element={element}"),
        Instruction::SimdShl { lanes, element } => format!("SimdShl lanes={lanes} element={element}"),
        Instruction::SimdShr { lanes, element } => format!("SimdShr lanes={lanes} element={element}"),
        Instruction::SimdMin { lanes, element } => format!("SimdMin lanes={lanes} element={element}"),
        Instruction::SimdMax { lanes, element } => format!("SimdMax lanes={lanes} element={element}"),
        Instruction::SimdF32Add { lanes, element } => {
            format!("SimdF32Add lanes={lanes} element={element}")
        }
        Instruction::SimdF32Sub { lanes, element } => {
            format!("SimdF32Sub lanes={lanes} element={element}")
        }
        Instruction::SimdF32Mul { lanes, element } => {
            format!("SimdF32Mul lanes={lanes} element={element}")
        }
        Instruction::SimdF32Div { lanes, element } => {
            format!("SimdF32Div lanes={lanes} element={element}")
        }
        Instruction::SimdF64Add { lanes, element } => {
            format!("SimdF64Add lanes={lanes} element={element}")
        }
        Instruction::SimdI16Add { lanes, element } => {
            format!("SimdI16Add lanes={lanes} element={element}")
        }
                Instruction::SimdI16Sub { lanes, element } => format!("SimdI16Sub lanes={lanes} element={element}"),
        Instruction::SimdI16Mul { lanes, element } => format!("SimdI16Mul lanes={lanes} element={element}"),
        Instruction::SimdI8Add { lanes, element } => format!("SimdI8Add lanes={lanes} element={element}"),
        Instruction::SimdI8Sub { lanes, element } => format!("SimdI8Sub lanes={lanes} element={element}"),
        Instruction::SimdI8Mul { lanes, element } => format!("SimdI8Mul lanes={lanes} element={element}"),
        Instruction::SimdI8And { lanes, element } => format!("SimdI8And lanes={lanes} element={element}"),
        Instruction::SimdI8Or { lanes, element } => format!("SimdI8Or lanes={lanes} element={element}"),
        Instruction::SimdI8Xor { lanes, element } => format!("SimdI8Xor lanes={lanes} element={element}"),
        Instruction::SimdI8Not { lanes, element } => format!("SimdI8Not lanes={lanes} element={element}"),
        Instruction::SimdI8Min { lanes, element } => format!("SimdI8Min lanes={lanes} element={element}"),
        Instruction::SimdI8Max { lanes, element } => format!("SimdI8Max lanes={lanes} element={element}"),
        Instruction::SimdI8UMin { lanes, element } => format!("SimdI8UMin lanes={lanes} element={element}"),
        Instruction::SimdI8UMax { lanes, element } => format!("SimdI8UMax lanes={lanes} element={element}"),
        Instruction::SimdI8Abs { lanes, element } => format!("SimdI8Abs lanes={lanes} element={element}"),
        Instruction::SimdI8Neg { lanes, element } => format!("SimdI8Neg lanes={lanes} element={element}"),
        Instruction::SimdI8Extract { lanes, element } => format!("SimdI8Extract lanes={lanes} element={element}"),
        Instruction::SimdI8Replace { lanes, element } => format!("SimdI8Replace lanes={lanes} element={element}"),
        Instruction::SimdI8Eq { lanes, element } => format!("SimdI8Eq lanes={lanes} element={element}"),
        Instruction::SimdI8Ne { lanes, element } => format!("SimdI8Ne lanes={lanes} element={element}"),
        Instruction::SimdI8Le { lanes, element } => format!("SimdI8Le lanes={lanes} element={element}"),
        Instruction::SimdI8Ge { lanes, element } => format!("SimdI8Ge lanes={lanes} element={element}"),
        Instruction::SimdI8AvgrU { lanes, element } => format!("SimdI8AvgrU lanes={lanes} element={element}"),
        Instruction::SimdI8Popcnt { lanes, element } => format!("SimdI8Popcnt lanes={lanes} element={element}"),
        Instruction::SimdI8Bitmask { lanes, element } => format!("SimdI8Bitmask lanes={lanes} element={element}"),
        Instruction::SimdI8AllTrue { lanes, element } => format!("SimdI8AllTrue lanes={lanes} element={element}"),
        Instruction::SimdI8AnyTrue { lanes, element } => format!("SimdI8AnyTrue lanes={lanes} element={element}"),
        Instruction::SimdI8Lt { lanes, element } => format!("SimdI8Lt lanes={lanes} element={element}"),
        Instruction::SimdI8Gt { lanes, element } => format!("SimdI8Gt lanes={lanes} element={element}"),
        Instruction::SimdI8AddSat { lanes, element } => format!("SimdI8AddSat lanes={lanes} element={element}"),
        Instruction::SimdI8SubSat { lanes, element } => format!("SimdI8SubSat lanes={lanes} element={element}"),
        Instruction::SimdI8Shl { lanes, element } => format!("SimdI8Shl lanes={lanes} element={element}"),
        Instruction::SimdI8Shr { lanes, element } => format!("SimdI8Shr lanes={lanes} element={element}"),

        Instruction::SimdI64Add { lanes, element } => {
            format!("SimdI64Add lanes={lanes} element={element}")
        }
        Instruction::SimdF64Sub { lanes, element } => {
            format!("SimdF64Sub lanes={lanes} element={element}")
        }
        Instruction::SimdF64Mul { lanes, element } => {
            format!("SimdF64Mul lanes={lanes} element={element}")
        }
        Instruction::SimdF64Div { lanes, element } => {
            format!("SimdF64Div lanes={lanes} element={element}")
        }
        Instruction::SimdReduceAdd { lanes, element } => {
            format!("SimdReduceAdd lanes={lanes} element={element}")
        }
        Instruction::SimdLoad { lanes, element, offset } => {
            format!("SimdLoad lanes={lanes} element={element} offset={offset}")
        }
        Instruction::SimdStore { lanes, element, offset } => {
            format!("SimdStore lanes={lanes} element={element} offset={offset}")
        }
        Instruction::SimdBroadcast { lanes, element } => {
            format!("SimdBroadcast lanes={lanes} element={element}")
        }
        Instruction::ConstStr(s) => format!("ConstStr {}", escape_string(s)),
        Instruction::ConstBool(v) => format!("ConstBool {v}"),
        Instruction::ConstNil => "ConstNil".to_string(),
        Instruction::Pop => "Pop".to_string(),
        Instruction::Dup => "Dup".to_string(),
        Instruction::LoadLocal(i) => format!("LoadLocal {i}"),
        Instruction::StoreLocal(i) => format!("StoreLocal {i}"),
        Instruction::LoadGlobal(i) => format!("LoadGlobal {i}"),
        Instruction::StoreGlobal(i) => format!("StoreGlobal {i}"),
        Instruction::I32Add => "I32Add".to_string(),
        Instruction::I32Sub => "I32Sub".to_string(),
        Instruction::I32Mul => "I32Mul".to_string(),
        Instruction::I32Div => "I32Div".to_string(),
        Instruction::I32Mod => "I32Mod".to_string(),
        Instruction::F64Add => "F64Add".to_string(),
        Instruction::F64Sub => "F64Sub".to_string(),
        Instruction::F64Mul => "F64Mul".to_string(),
        Instruction::F64Div => "F64Div".to_string(),
        Instruction::I32Neg => "I32Neg".to_string(),
        Instruction::F64Neg => "F64Neg".to_string(),
        Instruction::I32Eq => "I32Eq".to_string(),
        Instruction::I32Ne => "I32Ne".to_string(),
        Instruction::I32Lt => "I32Lt".to_string(),
        Instruction::I32Le => "I32Le".to_string(),
        Instruction::I32Gt => "I32Gt".to_string(),
        Instruction::I32Ge => "I32Ge".to_string(),
        Instruction::F64Eq => "F64Eq".to_string(),
        Instruction::F64Ne => "F64Ne".to_string(),
        Instruction::BoolAnd => "BoolAnd".to_string(),
        Instruction::BoolOr => "BoolOr".to_string(),
        Instruction::BoolNot => "BoolNot".to_string(),
        Instruction::Jump(pc) => format!("Jump {pc}"),
        Instruction::JumpIfFalse(pc) => format!("JumpIfFalse {pc}"),
        Instruction::Block => "Block".to_string(),
        Instruction::EndBlock => "EndBlock".to_string(),
        Instruction::Loop => "Loop".to_string(),
        Instruction::EndLoop => "EndLoop".to_string(),
        Instruction::Return => "Return".to_string(),
        Instruction::Call(i) => format!("Call {i}"),
        Instruction::CallHost(i) => format!("CallHost {i}"),
        Instruction::CallSuper(i) => format!("CallSuper {i}"),
        Instruction::CallIndirect(n) => format!("CallIndirect {n}"),
        Instruction::MakeClosure(fn_idx, arity) => format!("MakeClosure {fn_idx} {arity}"),
        Instruction::CallClosure => "CallClosure".to_string(),
        Instruction::ActorRegister => "ActorRegister".to_string(),
        Instruction::ActorCall(i) => format!("ActorCall {i}"),
        Instruction::CallWitness(a, b) => format!("CallWitness {a} {b}"),
        Instruction::StructNew(n) => format!("StructNew {n}"),
        Instruction::MethodCall(i) => format!("MethodCall {i}"),
        Instruction::ArrayNew => "ArrayNew".to_string(),
        Instruction::ArrayGet => "ArrayGet".to_string(),
        Instruction::ArraySet => "ArraySet".to_string(),
        Instruction::ArrSorted => "ArrSorted".to_string(),
        Instruction::GcCollect => "GcCollect".to_string(),
        Instruction::GcPin => "GcPin".to_string(),
        Instruction::GcUnpin => "GcUnpin".to_string(),
        Instruction::HeapCompact => "HeapCompact".to_string(),
        Instruction::BeginTry(pc) => format!("BeginTry {pc}"),
        Instruction::EndTry => "EndTry".to_string(),
        Instruction::Catch => "Catch".to_string(),
        Instruction::JumpIfNil(pc) => format!("JumpIfNil {pc}"),
        Instruction::ThrowError => "ThrowError".to_string(),
        Instruction::BindResult(pc) => format!("BindResult {pc}"),
        Instruction::ReturnErr => "ReturnErr".to_string(),
        Instruction::MakeIterator => "MakeIterator".to_string(),
        Instruction::IteratorNext => "IteratorNext".to_string(),
        Instruction::IteratorDone => "IteratorDone".to_string(),
        Instruction::AwaitPoll => "AwaitPoll".to_string(),
        Instruction::AwaitSuspend { resume_fn, state } => {
            format!("AwaitSuspend resume_fn={resume_fn} state={state}")
        }
        Instruction::ActorSend(i) => format!("ActorSend {i}"),
        Instruction::TaskSpawn(i) => format!("TaskSpawn {i}"),
        Instruction::TaskJoin => "TaskJoin".to_string(),
        Instruction::ArrayBuild(n) => format!("ArrayBuild {n}"),
        Instruction::DictBuild(n) => format!("DictBuild {n}"),
        Instruction::ObjectGet => "ObjectGet".to_string(),
        Instruction::ObjectSet => "ObjectSet".to_string(),
        Instruction::StrEq => "StrEq".to_string(),
        Instruction::StrNe => "StrNe".to_string(),
        Instruction::BoolEq => "BoolEq".to_string(),
        Instruction::BoolNe => "BoolNe".to_string(),
        Instruction::ValueEq => "ValueEq".to_string(),
        Instruction::ValueNe => "ValueNe".to_string(),
        Instruction::RefEq => "RefEq".to_string(),
        Instruction::RefNe => "RefNe".to_string(),
        Instruction::Print => "Print".to_string(),
        Instruction::ConcatStr => "ConcatStr".to_string(),
        Instruction::ToStr => "ToStr".to_string(),
    }
}

/// Format a [`CompiledProgram`] as SYXBC Text v1.
pub fn format_compiled_program(program: &CompiledProgram) -> String {
    let mut out = String::new();
    out.push_str("; syxbc-text v1\n");
    let _ = writeln!(out, "entry {}", program.entry_point);
    out.push('\n');

    if !program.strings.is_empty() {
        out.push_str("[strings]\n");
        for (i, s) in program.strings.iter().enumerate() {
            let _ = writeln!(out, "{i} {}", escape_string(s));
        }
        out.push('\n');
    }

    if !program.globals.is_empty() {
        out.push_str("[globals]\n");
        for (i, g) in program.globals.iter().enumerate() {
            let _ = writeln!(out, "{i} {g}");
        }
        out.push('\n');
    }

    if !program.host_fn_names.is_empty() {
        out.push_str("[host_fns]\n");
        for (i, h) in program.host_fn_names.iter().enumerate() {
            let _ = writeln!(out, "{i} {h}");
        }
        out.push('\n');
    }

    if !program.opaque_witnesses.is_empty() {
        out.push_str("[opaque_witnesses]\n");
        for (fn_name, protocol) in &program.opaque_witnesses {
            let _ = writeln!(out, "{fn_name} {protocol}");
        }
        out.push('\n');
    }

    out.push_str("[functions]\n");
    for (i, f) in program.functions.iter().enumerate() {
        let _ = writeln!(out, "{i} {}", f.name);
    }
    out.push('\n');

    for (i, f) in program.functions.iter().enumerate() {
        let _ = writeln!(out, "[function {i} {}]", f.name);
        let _ = writeln!(out, "params {}", f.param_count);
        let _ = writeln!(out, "locals {}", f.local_count);
        if !f.source_name.is_empty() {
            let _ = writeln!(out, "source {}", f.source_name);
        }
        if !f.local_names.is_empty() {
            let names: Vec<_> = f.local_names.iter().map(|n| n.as_str()).collect();
            let _ = writeln!(out, "local_names {}", names.join(" "));
        }
        if !f.local_types.is_empty() {
            let types: Vec<_> = f.local_types.iter().map(|t| t.as_str()).collect();
            let _ = writeln!(out, "local_types {}", types.join(" "));
        }
        for (pc, ins) in f.instructions.iter().enumerate() {
            let _ = writeln!(out, " {pc} {}", format_instruction(ins));
        }
        if !f.instruction_spans.is_empty() {
            out.push_str("spans\n");
            for (pc, span) in f.instruction_spans.iter().enumerate() {
                match span {
                    Some((off, len)) => {
                        let _ = writeln!(out, " {pc} {off} {len}");
                    }
                    None => {
                        let _ = writeln!(out, " {pc} -");
                    }
                }
            }
        }
        out.push('\n');
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use smol_str::SmolStr;

    #[test]
    fn format_hello_program() {
        let program = CompiledProgram {
            functions: vec![super::super::FunctionUnit {
                name: SmolStr::new("main"),
                param_count: 0,
                local_count: 0,
                instructions: vec![
                    Instruction::ConstStr("Hello, World".into()),
                    Instruction::Print,
                    Instruction::Return,
                ],
                instruction_spans: vec![],
                local_names: vec![],
                local_types: vec![],
                source_name: SmolStr::new("hello.sy"),
                init_body_pc: 0,
                init_wrap_pc: 0,
            }],
            globals: vec![],
            strings: vec!["Hello, World".into()],
            host_fn_names: vec!["print".into()],
            entry_point: SmolStr::new("main"),
            mir_functions: indexmap::IndexMap::new(),
            mir_module: None,
            opaque_witnesses: indexmap::IndexMap::new(),
        };
        let text = format_compiled_program(&program);
        assert!(text.contains("; syxbc-text v1"));
        assert!(text.contains("entry main"));
        assert!(text.contains("ConstStr"));
        assert!(text.contains("Print"));
    }
}
