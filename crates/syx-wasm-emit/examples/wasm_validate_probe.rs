//! R-18 forensic probe for wasm emit validation failures (W001).
//!
//! Usage:
//!   cargo run -p syx-wasm-emit --example wasm_validate_probe -- <file.wasm>
//!   cargo run -p syx-wasm-emit --example wasm_validate_probe -- <file.wasm> --dump-ops <func_index>
//!
//! Why this example exists: R-16 and R-17 burned two windows chasing W001 with
//! hand-written stack simulators, and every conclusion was wrong. The reason was a
//! bug in the auditing code: `Validator::payload` returns `ValidPayload::Func` for
//! each function body, meaning body validation is *deferred*. Dropping that value
//! silently skips every function body, so a broken module "validates cleanly".
//!
//! This tool never drops a payload. It reports the exact failing function and, with
//! `--dump-ops`, every operator offset in that body so stack height can be replayed
//! against real signatures instead of guessed ones.

use wasmparser::{
    CompositeInnerType, Operator, Parser, Payload, TypeRef, ValidPayload, Validator, WasmFeatures,
};

/// One row of the function index space we need for `call` stack effects.
#[derive(Clone, Copy, Default)]
struct Sig {
    params: usize,
    results: usize,
}

fn main() {
    let mut args = std::env::args().skip(1);
    let path = match args.next() {
        Some(p) => p,
        None => {
            eprintln!("usage: wasm_validate_probe <file.wasm> [--dump-ops <func_index>]");
            std::process::exit(2);
        }
    };
    let mut dump_ops: Option<u32> = None;
    while let Some(a) = args.next() {
        if a == "--dump-ops" {
            dump_ops = args.next().and_then(|s| s.parse().ok());
        }
    }

    let bytes = std::fs::read(&path).unwrap_or_else(|e| {
        eprintln!("cannot read {path}: {e}");
        std::process::exit(2);
    });

    // Baseline: this is exactly what the emitter calls. It must fail for a broken
    // module, otherwise there is nothing to forensically locate.
    match wasmparser::validate(&bytes) {
        Ok(_) => {
            println!("wasmparser::validate => OK ({path} is valid)");
            println!("nothing to locate; if a test still fails, the problem is not emit validity");
            if dump_ops.is_none() {
                return;
            }
        }
        Err(e) => println!("wasmparser::validate => FAIL: {e}"),
    }

    let (sigs, type_sigs) = collect_signatures(&bytes);
    let n_import_funcs = import_func_count(&bytes);

    let mut validator = Validator::new_with_features(WasmFeatures::default());
    let mut local_entry: u32 = 0;
    let mut seen_any_func = false;

    for payload in Parser::new(0).parse_all(&bytes) {
        let payload = match payload {
            Ok(p) => p,
            Err(e) => {
                println!("PARSE ERROR: {e}");
                return;
            }
        };

        // `payload(...)` is the only place that hands back deferred function bodies.
        let valid = match validator.payload(&payload) {
            Ok(v) => v,
            Err(e) => {
                println!("SECTION-LEVEL FAIL: {e}");
                return;
            }
        };

        match valid {
            ValidPayload::Func(to_validate, body) => {
                seen_any_func = true;
                let abs = local_entry + n_import_funcs;
                if dump_ops == Some(abs) || dump_ops == Some(local_entry) {
                    dump_body(&body, abs, local_entry, &sigs, &type_sigs);
                }
                let mut func_validator = to_validate.into_validator(Default::default());
                if let Err(e) = func_validator.validate(&body) {
                    println!();
                    println!("=== FAILING FUNCTION ===");
                    println!("  local_entry   : {local_entry}");
                    println!("  absolute_index: {abs}");
                    println!(
                        "  body range    : {}..{} ({} bytes)",
                        body.range().start,
                        body.range().end,
                        body.range().end - body.range().start
                    );
                    println!("  error         : {e}");
                    if let Some(off) = error_offset(&e) {
                        println!("  error offset  : {off:#x}");
                        println!();
                        print_ops_around(&body, abs, &sigs, &type_sigs, off);
                    }
                    return;
                }
                local_entry += 1;
            }
            ValidPayload::End(_) => {}
            ValidPayload::Ok | ValidPayload::Parser(_) => {}
        }
    }

    if !seen_any_func {
        println!("no function bodies were presented for validation (unexpected)");
    } else {
        println!("all {local_entry} function bodies validated individually");
    }
}

/// `BinaryReaderError` stringifies with "(at offset 0x...)" but exposes no accessor,
/// so recover it from the message text.
fn error_offset(e: &wasmparser::BinaryReaderError) -> Option<usize> {
    let msg = e.to_string();
    let idx = msg.rfind("0x")?;
    let hex: String = msg[idx + 2..]
        .chars()
        .take_while(|c| c.is_ascii_hexdigit())
        .collect();
    usize::from_str_radix(&hex, 16).ok()
}

fn import_func_count(bytes: &[u8]) -> u32 {
    let mut n = 0;
    for p in Parser::new(0).parse_all(bytes) {
        let Ok(Payload::ImportSection(r)) = p else {
            continue;
        };
        for imp in r.into_iter().flatten() {
            if matches!(imp.ty, TypeRef::Func(_)) {
                n += 1;
            }
        }
    }
    n
}

/// Function index space -> signature, covering imports first then local functions.
fn collect_signatures(bytes: &[u8]) -> (Vec<Sig>, Vec<Sig>) {
    let mut type_shapes: Vec<Sig> = Vec::new();
    let mut import_types: Vec<u32> = Vec::new();
    let mut local_types: Vec<u32> = Vec::new();

    for p in Parser::new(0).parse_all(bytes) {
        let Ok(payload) = p else { continue };
        match payload {
            Payload::TypeSection(reader) => {
                for group in reader.into_iter().flatten() {
                    let mut shape = Sig::default();
                    for sub in group.types() {
                        if let CompositeInnerType::Func(f) = &sub.composite_type.inner {
                            shape = Sig {
                                params: f.params().len(),
                                results: f.results().len(),
                            };
                            break;
                        }
                    }
                    type_shapes.push(shape);
                }
            }
            Payload::ImportSection(reader) => {
                for imp in reader.into_iter().flatten() {
                    if let TypeRef::Func(t) = imp.ty {
                        import_types.push(t);
                    }
                }
            }
            Payload::FunctionSection(reader) => {
                for t in reader.into_iter().flatten() {
                    local_types.push(t);
                }
            }
            _ => {}
        }
    }

    let mut sigs: Vec<Sig> = Vec::new();
    for t in import_types.iter().chain(local_types.iter()) {
        sigs.push(*type_shapes.get(*t as usize).unwrap_or(&Sig::default()));
    }
    (sigs, type_shapes)
}

fn op_stack_effect(op: &Operator, sigs: &[Sig], type_sigs: &[Sig]) -> (usize, usize) {
    match op {
        Operator::I32Const { .. }
        | Operator::I64Const { .. }
        | Operator::F32Const { .. }
        | Operator::F64Const { .. }
        | Operator::LocalGet { .. }
        | Operator::GlobalGet { .. }
        | Operator::RefNull { .. }
        | Operator::RefFunc { .. } => (0, 1),

        Operator::LocalSet { .. }
        | Operator::LocalTee { .. }
        | Operator::GlobalSet { .. }
        | Operator::Drop
        | Operator::BrIf { .. } => (1, 0),

        Operator::I32Eqz
        | Operator::I64Eqz
        | Operator::I32WrapI64
        | Operator::I64ExtendI32S
        | Operator::I64ExtendI32U
        | Operator::F32ConvertI32S
        | Operator::F64ConvertI32S => (1, 1),

        Operator::I32Eq
        | Operator::I32Ne
        | Operator::I32LtS
        | Operator::I32LtU
        | Operator::I32GtS
        | Operator::I32GtU
        | Operator::I32LeS
        | Operator::I32LeU
        | Operator::I32GeS
        | Operator::I32GeU
        | Operator::I64Eq
        | Operator::I64Ne
        | Operator::I64LtS
        | Operator::I64GtS
        | Operator::I64LeS
        | Operator::I64GeS
        | Operator::F32Eq
        | Operator::F32Ne
        | Operator::F32Lt
        | Operator::F32Gt
        | Operator::F32Le
        | Operator::F32Ge
        | Operator::F64Eq
        | Operator::F64Ne
        | Operator::F64Lt
        | Operator::F64Gt
        | Operator::F64Le
        | Operator::F64Ge
        | Operator::I32Add
        | Operator::I32Sub
        | Operator::I32Mul
        | Operator::I32DivS
        | Operator::I32DivU
        | Operator::I32RemS
        | Operator::I32RemU
        | Operator::I32And
        | Operator::I32Or
        | Operator::I32Xor
        | Operator::I32Shl
        | Operator::I32ShrS
        | Operator::I32ShrU
        | Operator::I32Rotl
        | Operator::I32Rotr
        | Operator::I64Add
        | Operator::I64Sub
        | Operator::I64Mul
        | Operator::I64And
        | Operator::I64Or
        | Operator::I64Xor
        | Operator::I64Shl
        | Operator::I64ShrS
        | Operator::I64ShrU
        | Operator::F32Add
        | Operator::F32Sub
        | Operator::F32Mul
        | Operator::F32Div
        | Operator::F64Add
        | Operator::F64Sub
        | Operator::F64Mul
        | Operator::F64Div => (2, 1),

        Operator::Select => (3, 1),

        Operator::Call { function_index } => {
            let s = sigs
                .get(*function_index as usize)
                .copied()
                .unwrap_or_default();
            (s.params, s.results)
        }
        Operator::CallIndirect { type_index, .. } => {
            // Stack layout is [args..., table_index]. `type_index` indexes the *type*
            // section, not the function index space, so it cannot be looked up in
            // `sigs`; use the separately supplied type table.
            let s = type_sigs
                .get(*type_index as usize)
                .copied()
                .unwrap_or_default();
            (s.params + 1, s.results)
        }

        Operator::Block { .. } | Operator::Loop { .. } => (0, 0),
        Operator::If { .. } => (1, 0),
        Operator::Else | Operator::End => (0, 0),
        Operator::Return | Operator::Br { .. } | Operator::Unreachable | Operator::Nop => (0, 0),

        _ => (0, 0),
    }
}

fn dump_body(
    body: &wasmparser::FunctionBody,
    abs: u32,
    local_entry: u32,
    sigs: &[Sig],
    type_sigs: &[Sig],
) {
    println!();
    println!("=== BODY DUMP func abs={abs} local_entry={local_entry} ===");
    let mut ops = match body.get_operators_reader() {
        Ok(r) => r,
        Err(e) => {
            println!("  cannot read operators: {e}");
            return;
        }
    };
    while !ops.eof() {
        let off = ops.original_position();
        let Ok(op) = ops.read() else { break };
        let (pop, push) = op_stack_effect(&op, sigs, type_sigs);
        println!("  {off:#06x}  -{pop} +{push}  {op:?}");
    }
}

fn print_ops_around(
    body: &wasmparser::FunctionBody,
    abs: u32,
    sigs: &[Sig],
    type_sigs: &[Sig],
    target: usize,
) {
    println!("--- operators near {target:#x} in func abs={abs} ---");
    let mut ops = match body.get_operators_reader() {
        Ok(r) => r,
        Err(e) => {
            println!("  cannot read operators: {e}");
            return;
        }
    };
    let mut all: Vec<(usize, String, usize, usize)> = Vec::new();
    while !ops.eof() {
        let off = ops.original_position();
        let Ok(op) = ops.read() else { break };
        let (pop, push) = op_stack_effect(&op, sigs, type_sigs);
        all.push((off, format!("{op:?}"), pop, push));
    }
    let pos = all
        .iter()
        .position(|(o, ..)| *o >= target)
        .unwrap_or(all.len());
    let start = pos.saturating_sub(12);
    let end = (pos + 6).min(all.len());
    let mut height: i64 = 0;
    for (o, text, pop, push) in &all[..end] {
        height += *push as i64 - *pop as i64;
        let mark = if *o >= target { ">>>" } else { "   " };
        if start <= all.iter().position(|(x, ..)| x == o).unwrap_or(0) {
            println!("  {mark} {o:#06x}  h={height:<3} {text}");
        }
    }
}
