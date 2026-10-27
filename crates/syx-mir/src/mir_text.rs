//! SYX MIR Text format v1 — human-readable dump of [`MirModule`].

use std::fmt::Write as FmtWrite;

use super::{MirInstr, MirModule, MirOp};

fn escape_mir_string(s: &str) -> String {
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

fn format_mir_op(op: &MirOp) -> String {
    match op {
        MirOp::ConstI32(v) => format!("ConstI32 {v}"),
        MirOp::ConstNumeric { kind, bits } => format!("ConstNumeric kind={kind} bits={bits}"),
        MirOp::ConstNumeric128 { kind, lo, hi } => {
            format!("ConstNumeric128 kind={kind} lo={lo} hi={hi}")
        }
        MirOp::ConstStr(s) => format!("ConstStr {}", escape_mir_string(s)),
        MirOp::ConstBool(v) => format!("ConstBool {v}"),
        MirOp::ConstNil => "ConstNil".to_string(),
        MirOp::LoadLocal(i) => format!("LoadLocal {i}"),
        MirOp::StoreLocal(i) => format!("StoreLocal {i}"),
        MirOp::LoadGlobal(i) => format!("LoadGlobal {i}"),
        MirOp::StoreGlobal(i) => format!("StoreGlobal {i}"),
        MirOp::Add { kind } => format!("Add kind={kind}"),
        MirOp::SimdAdd => "SimdAdd".to_string(),
        MirOp::SimdSub => "SimdSub".to_string(),
        MirOp::SimdMul => "SimdMul".to_string(),
        MirOp::SimdDiv => "SimdDiv".to_string(),
        MirOp::SimdEq => "SimdEq".to_string(),
        MirOp::SimdLt => "SimdLt".to_string(),
        MirOp::SimdGt => "SimdGt".to_string(),
        MirOp::SimdNe => "SimdNe".to_string(),
        MirOp::SimdLe => "SimdLe".to_string(),
        MirOp::SimdGe => "SimdGe".to_string(),
        MirOp::SimdAnd => "SimdAnd".to_string(),
        MirOp::SimdOr => "SimdOr".to_string(),
        MirOp::SimdXor => "SimdXor".to_string(),
        MirOp::SimdNot => "SimdNot".to_string(),
        MirOp::SimdShl => "SimdShl".to_string(),
        MirOp::SimdShr => "SimdShr".to_string(),
        MirOp::SimdMin => "SimdMin".to_string(),
        MirOp::SimdMax => "SimdMax".to_string(),
        MirOp::SimdF32Add => "SimdF32Add".to_string(),
        MirOp::SimdF32Sub => "SimdF32Sub".to_string(),
        MirOp::SimdF32Mul => "SimdF32Mul".to_string(),
        MirOp::SimdF32Div => "SimdF32Div".to_string(),
        MirOp::SimdF64Add => "SimdF64Add".to_string(),
        MirOp::SimdF64Sub => "SimdF64Sub".to_string(),
        MirOp::SimdF64Mul => "SimdF64Mul".to_string(),
        MirOp::SimdF64Div => "SimdF64Div".to_string(),
        MirOp::SimdI16Add => "SimdI16Add".to_string(),
        MirOp::SimdI16Sub => "SimdI16Sub".to_string(),
        MirOp::SimdI16Mul => "SimdI16Mul".to_string(),
        MirOp::SimdI8Add => "SimdI8Add".to_string(),
        MirOp::SimdI8Sub => "SimdI8Sub".to_string(),
        MirOp::SimdI8Mul => "SimdI8Mul".to_string(),
        MirOp::SimdI8And => "SimdI8And".to_string(),
        MirOp::SimdI8Or => "SimdI8Or".to_string(),
        MirOp::SimdI8Xor => "SimdI8Xor".to_string(),
        MirOp::SimdI8Not => "SimdI8Not".to_string(),
        MirOp::SimdI8Min => "SimdI8Min".to_string(),
        MirOp::SimdI8Max => "SimdI8Max".to_string(),
        MirOp::SimdI8UMin => "SimdI8UMin".to_string(),
        MirOp::SimdI8UMax => "SimdI8UMax".to_string(),
        MirOp::SimdI8Abs => "SimdI8Abs".to_string(),
        MirOp::SimdI8Neg => "SimdI8Neg".to_string(),
        MirOp::SimdI8Extract => "SimdI8Extract".to_string(),
        MirOp::SimdI8Replace => "SimdI8Replace".to_string(),
        MirOp::SimdI8Eq => "SimdI8Eq".to_string(),
        MirOp::SimdI8Ne => "SimdI8Ne".to_string(),
        MirOp::SimdI8Le => "SimdI8Le".to_string(),
        MirOp::SimdI8Ge => "SimdI8Ge".to_string(),
        MirOp::SimdI8AvgrU => "SimdI8AvgrU".to_string(),
        MirOp::SimdI8Popcnt => "SimdI8Popcnt".to_string(),
        MirOp::SimdI8Lt => "SimdI8Lt".to_string(),
        MirOp::SimdI8Gt => "SimdI8Gt".to_string(),
        MirOp::SimdI8AddSat => "SimdI8AddSat".to_string(),
        MirOp::SimdI8SubSat => "SimdI8SubSat".to_string(),
        MirOp::SimdI8Shl => "SimdI8Shl".to_string(),
        MirOp::SimdI8Shr => "SimdI8Shr".to_string(),
        MirOp::SimdI8Shuffle { mode } => format!("SimdI8Shuffle mode={mode}"),
        MirOp::SimdI64Add => "SimdI64Add".to_string(),
        MirOp::SimdReduceAdd { lanes, element } => {
            format!("SimdReduceAdd lanes={lanes} element={element}")
        }
        MirOp::SimdShuffle { mode } => format!("SimdShuffle mode={mode}"),
        MirOp::SimdLoad { lanes, element } => format!("SimdLoad {lanes} {element}"),
        MirOp::SimdStore { lanes, element } => format!("SimdStore {lanes} {element}"),
        MirOp::Sub { kind } => format!("Sub kind={kind}"),
        MirOp::Mul { kind } => format!("Mul kind={kind}"),
        MirOp::Div { kind } => format!("Div kind={kind}"),
        MirOp::Mod { kind } => format!("Mod kind={kind}"),
        MirOp::BitAnd { kind } => format!("BitAnd kind={kind}"),
        MirOp::BitOr { kind } => format!("BitOr kind={kind}"),
        MirOp::BitXor { kind } => format!("BitXor kind={kind}"),
        MirOp::Shl { kind } => format!("Shl kind={kind}"),
        MirOp::Shr { kind } => format!("Shr kind={kind}"),
        MirOp::Eq => "Eq".to_string(),
        MirOp::Ne => "Ne".to_string(),
        MirOp::Lt { kind } => format!("Lt kind={kind}"),
        MirOp::Le { kind } => format!("Le kind={kind}"),
        MirOp::Gt { kind } => format!("Gt kind={kind}"),
        MirOp::Ge { kind } => format!("Ge kind={kind}"),
        MirOp::And => "And".to_string(),
        MirOp::Or => "Or".to_string(),
        MirOp::Not => "Not".to_string(),
        MirOp::Neg { kind } => format!("Neg kind={kind}"),
        MirOp::BitNot { kind } => format!("BitNot kind={kind}"),
        MirOp::Print => "Print".to_string(),
        MirOp::Pop => "Pop".to_string(),
        MirOp::ConcatStr => "ConcatStr".to_string(),
        MirOp::ToStr => "ToStr".to_string(),
        MirOp::Jump(t) => format!("Jump block {t}"),
        MirOp::JumpIfFalse(t) => format!("JumpIfFalse block {t}"),
        MirOp::Call(name) => format!("Call {name}"),
        MirOp::CallIndirect(n) => format!("CallIndirect {n}"),
        MirOp::ConstFnIndex(name) => format!("ConstFnIndex {name}"),
        MirOp::Return => "Return".to_string(),
        MirOp::Dup => "Dup".to_string(),
        MirOp::JumpIfNil(t) => format!("JumpIfNil block {t}"),
        MirOp::CallHost(i) => format!("CallHost {i}"),
        MirOp::MakeClosure(name, n) => format!("MakeClosure {name} {n}"),
        MirOp::CallClosure => "CallClosure".to_string(),
        MirOp::ActorCall(name) => format!("ActorCall {name}"),
        MirOp::ActorRegister => "ActorRegister".to_string(),
        MirOp::CallWitness(a, b) => format!("CallWitness {a} {b}"),
        MirOp::MakeIterator => "MakeIterator".to_string(),
        MirOp::IteratorNext => "IteratorNext".to_string(),
        MirOp::AwaitPoll => "AwaitPoll".to_string(),
        MirOp::AwaitSuspend { resume_fn, state } => {
            format!("AwaitSuspend resume_fn={resume_fn} state={state}")
        }
        MirOp::ActorSend(name) => format!("ActorSend {name}"),
        MirOp::DictBuild(n) => format!("DictBuild {n}"),
        MirOp::ArrayBuild(n) => format!("ArrayBuild {n}"),
        MirOp::SetBuild(n) => format!("SetBuild {n}"),
        MirOp::ObjectGet => "ObjectGet".to_string(),
        MirOp::ObjectSet => "ObjectSet".to_string(),
        MirOp::HeapCompact => "HeapCompact".to_string(),
        MirOp::ArrayGet => "ArrayGet".to_string(),
        MirOp::ArraySet => "ArraySet".to_string(),
        MirOp::ArrSorted => "ArrSorted".to_string(),
        MirOp::NumericCast { from, to } => format!("NumericCast from={from} to={to}"),
        MirOp::ValueEq => "ValueEq".to_string(),
        MirOp::ValueNe => "ValueNe".to_string(),
        MirOp::OpaqueValueEq => "OpaqueValueEq".to_string(),
        MirOp::RefEq => "RefEq".to_string(),
        MirOp::RefNe => "RefNe".to_string(),
        MirOp::BeginTry(t) => format!("BeginTry block {t}"),
        MirOp::EndTry => "EndTry".to_string(),
        MirOp::Catch => "Catch".to_string(),
        MirOp::Throw => "Throw".to_string(),
        MirOp::BindResult(t) => format!("BindResult block {t}"),
        MirOp::ReturnErr => "ReturnErr".to_string(),
        MirOp::TaskSpawn(name) => format!("TaskSpawn {name}"),
        MirOp::TaskJoin => "TaskJoin".to_string(),
        MirOp::CallSuper(name) => format!("CallSuper {name}"),
        MirOp::SimdI8Bitmask => "SimdI8Bitmask".to_string(),
        MirOp::SimdI8AllTrue => "SimdI8AllTrue".to_string(),
        MirOp::SimdI8AnyTrue => "SimdI8AnyTrue".to_string(),
    }
}

fn write_instr(out: &mut String, instr: &MirInstr) {
    match instr.dest {
        Some(r) => {
            let _ = writeln!(out, "  r{r} = {}", format_mir_op(&instr.op));
        }
        None => {
            let _ = writeln!(out, "  {}", format_mir_op(&instr.op));
        }
    }
}

/// Format a [`MirModule`] as SYX MIR Text v1.
pub fn format_mir_module(module: &MirModule) -> String {
    let mut out = String::new();
    out.push_str("; syx-mir-text v1\n\n");

    for func in &module.functions {
        let _ = writeln!(out, "[function {}]", func.name);
        let _ = writeln!(
            out,
            "params {} locals {} entry {}",
            func.param_count,
            func.local_count,
            func.entry
        );

        if !func.blocks.is_empty() {
            for block in &func.blocks {
                let _ = writeln!(out, "block {}:", block.id);
                if !block.successors.is_empty() {
                    let succ: Vec<_> = block.successors.iter().map(|s| s.to_string()).collect();
                    let _ = writeln!(out, "  successors {}", succ.join(" "));
                }
                for instr in &block.instructions {
                    write_instr(&mut out, instr);
                }
            }
        }

        if !func.instructions.is_empty() {
            let _ = writeln!(out, "flat:");
            for instr in &func.instructions {
                write_instr(&mut out, instr);
            }
        }

        out.push('\n');
    }

    out
}
