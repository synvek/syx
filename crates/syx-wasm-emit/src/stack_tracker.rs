//! R-23: emit-time wasm operand-stack height tracker.
//!
//! W001 (`values remaining on stack at end of block`) is reported by wasmparser only
//! after a full module is assembled. This tracker mirrors control frames and stack
//! height while MIR emit runs, so the first offending site can be named by
//! function / MIR block / instruction index / `MirOp` instead of a byte offset in an
//! 89 KB module.
//!
//! Scope: wasm operand stack only (same layer as [`crate::stack_effect`]). The RT
//! value stack is separate.
//!
//! Disable with `SYX_WASM_STACK_CHECK=0` (emergency only; tests must not rely on it).

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Write as _;

use wasm_encoder::{BlockType, Instruction as WasmInstr};

use crate::exec::aot;
use crate::stack_effect::{import_effect, StackEffect};

/// Result arity for `block` / `loop` / `if` labels (`Empty` → 0, `Result(_)` → 1).
/// `FunctionType` multi-value is not used by current emit; treat as 0 (fail at End if wrong).
fn block_result_arity(bt: &BlockType) -> u32 {
    match bt {
        BlockType::Empty => 0,
        BlockType::Result(_) => 1,
        BlockType::FunctionType(_) => 0,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FrameKind {
    Block,
    Loop,
    If,
    Else,
}

#[derive(Debug, Clone)]
struct ControlFrame {
    kind: FrameKind,
    /// Operand-stack height when this frame was entered.
    height_on_enter: i32,
    /// Expected result arity at End (`Empty` → 0).
    result_arity: u32,
}

/// Emit-time operand-stack tracker.
#[derive(Debug, Clone)]
pub struct StackTracker {
    height: i32,
    frames: Vec<ControlFrame>,
    /// After `Br`/`Return`/`Unreachable`, stack is polymorphic until we `End` back to
    /// this many remaining frames (the branch target's enclosing depth).
    unreachable_until_frames: Option<usize>,
    pub fn_name: String,
    pub mir_block: u32,
    pub mir_idx: usize,
    pub mir_op: String,
}

impl StackTracker {
    pub fn new(fn_name: impl Into<String>) -> Self {
        Self {
            height: 0,
            frames: Vec::new(),
            unreachable_until_frames: None,
            fn_name: fn_name.into(),
            mir_block: 0,
            mir_idx: 0,
            mir_op: String::new(),
        }
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn set_mir_context(&mut self, block: u32, idx: usize, mir_op: impl Into<String>) {
        self.mir_block = block;
        self.mir_idx = idx;
        self.mir_op = mir_op.into();
    }

    fn push_frame(&mut self, kind: FrameKind, result_arity: u32) {
        self.frames.push(ControlFrame {
            kind,
            height_on_enter: self.height,
            result_arity,
        });
    }

    fn diagnostic(&self, detail: &str) -> String {
        format!(
            "W001 stack tracker: {detail} (fn={}, mir_block={}, mir_idx={}, mir_op={}, height={}, frames={})",
            self.fn_name,
            self.mir_block,
            self.mir_idx,
            self.mir_op,
            self.height,
            self.frames.len()
        )
    }

    fn end_frame(&mut self) -> Result<(), String> {
        let frame = self.frames.pop().ok_or_else(|| {
            self.diagnostic("End with no open control frame")
        })?;
        let expected = frame.height_on_enter + frame.result_arity as i32;
        if self.height != expected {
            return Err(self.diagnostic(&format!(
                "values remaining on stack at end of {:?} (expected height {expected}, actual {})",
                frame.kind, self.height
            )));
        }
        Ok(())
    }

    /// Apply one wasm instruction's operand-stack effect.
    ///
    /// `call_effect` resolves `Call(func_idx)` to an import stack effect when known.
    pub fn apply(
        &mut self,
        instr: &WasmInstr<'_>,
        call_effect: Option<StackEffect>,
    ) -> Result<(), String> {
        // Dead code after Br/Return: keep control structure balanced, ignore values.
        // Without skipping value ops, SimdI8Extract lane fallthrough double-counts.
        if self.unreachable_until_frames.is_some() {
            match instr {
                WasmInstr::End | WasmInstr::Else => {}
                WasmInstr::Block(bt) => {
                    self.push_frame(FrameKind::Block, block_result_arity(bt));
                    return Ok(());
                }
                WasmInstr::Loop(bt) => {
                    self.push_frame(FrameKind::Loop, block_result_arity(bt));
                    return Ok(());
                }
                WasmInstr::If(bt) => {
                    // Condition ops were skipped; do not pop a phantom i32.
                    self.push_frame(FrameKind::If, block_result_arity(bt));
                    return Ok(());
                }
                _ => return Ok(()),
            }
        }
        match instr {
            WasmInstr::Block(bt) => {
                // R-24-03: `Block(Result(I32))` (SimdI8Extract pilot) must keep arity 1;
                // treating every block as Empty false-flagged SyxHost_simd_i8_extract.
                self.push_frame(FrameKind::Block, block_result_arity(bt));
            }
            WasmInstr::Loop(bt) => {
                self.push_frame(FrameKind::Loop, block_result_arity(bt));
            }
            WasmInstr::If(bt) => {
                // `If` consumes the i32 condition before the frame body runs.
                self.height -= 1;
                if self.height < 0 {
                    return Err(self.diagnostic("If underflow (missing condition i32)"));
                }
                self.push_frame(FrameKind::If, block_result_arity(bt));
            }
            WasmInstr::Else => {
                // Else re-enters a reachable path even after a then-branch `Br`.
                self.unreachable_until_frames = None;
                let (kind, height_on_enter, result_arity) = {
                    let frame = self.frames.last().ok_or_else(|| {
                        self.diagnostic("Else with no open If frame")
                    })?;
                    (frame.kind, frame.height_on_enter, frame.result_arity)
                };
                if kind != FrameKind::If {
                    return Err(self.diagnostic(&format!(
                        "Else under {kind:?} frame (expected If)"
                    )));
                }
                // Then-branch must leave exactly result_arity values above enter height.
                let expected = height_on_enter + result_arity as i32;
                if self.height != expected {
                    return Err(self.diagnostic(&format!(
                        "If/then stack mismatch before Else (expected height {expected}, actual {})",
                        self.height
                    )));
                }
                // Reset to enter height for the else branch; mark as Else.
                self.height = height_on_enter;
                if let Some(frame) = self.frames.last_mut() {
                    frame.kind = FrameKind::Else;
                }
            }
            WasmInstr::End => {
                if let Some(remain) = self.unreachable_until_frames {
                    // Polymorphic stack after Br/Return: pop frame without height check.
                    if self.frames.is_empty() {
                        // Function-body terminator after Return/Unreachable (remain==0).
                        if remain == 0 {
                            self.unreachable_until_frames = None;
                            self.height = 0;
                            return Ok(());
                        }
                        return Err(self.diagnostic(
                            "End with no open control frame (unreachable)",
                        ));
                    }
                    let frame = self.frames.pop().expect("frames non-empty");
                    if self.frames.len() == remain {
                        // Finished the branch target: its results appear to the parent.
                        self.unreachable_until_frames = None;
                        self.height =
                            frame.height_on_enter + frame.result_arity as i32;
                    }
                    return Ok(());
                }
                if self.frames.is_empty() {
                    // Function-body terminator (wasm_encoder requires a trailing End).
                    if self.height != 0 {
                        return Err(self.diagnostic(&format!(
                            "values remaining on stack at end of function (height {})",
                            self.height
                        )));
                    }
                    return Ok(());
                }
                self.end_frame()?;
            }
            WasmInstr::Br(depth) => {
                self.br_unreachable(*depth)?;
            }
            WasmInstr::BrIf(depth) => {
                self.height -= 1;
                if self.height < 0 {
                    return Err(self.diagnostic("BrIf underflow"));
                }
                // Fallthrough path only; taken path is not modeled here.
                let _ = depth;
            }
            WasmInstr::Return | WasmInstr::Unreachable => {
                // Unconditional exit / trap: polymorphic until function End.
                self.height = 0;
                self.unreachable_until_frames = Some(0);
            }
            WasmInstr::Drop => {
                self.height -= 1;
                if self.height < 0 {
                    return Err(self.diagnostic("Drop underflow"));
                }
            }
            WasmInstr::I32Const(_)
            | WasmInstr::I64Const(_)
            | WasmInstr::F32Const(_)
            | WasmInstr::F64Const(_)
            | WasmInstr::V128Const(_) => {
                self.height += 1;
            }
            WasmInstr::LocalGet(_) | WasmInstr::GlobalGet(_) => {
                self.height += 1;
            }
            WasmInstr::LocalSet(_) | WasmInstr::GlobalSet(_) => {
                self.height -= 1;
                if self.height < 0 {
                    return Err(self.diagnostic("LocalSet/GlobalSet underflow"));
                }
            }
            WasmInstr::LocalTee(_) => {
                // pop + push same value → net 0
            }
            WasmInstr::I32Eq
            | WasmInstr::I32Ne
            | WasmInstr::I32LtS
            | WasmInstr::I32LtU
            | WasmInstr::I32GtS
            | WasmInstr::I32GtU
            | WasmInstr::I32LeS
            | WasmInstr::I32LeU
            | WasmInstr::I32GeS
            | WasmInstr::I32GeU
            | WasmInstr::I32Add
            | WasmInstr::I32Sub
            | WasmInstr::I32Mul
            | WasmInstr::I32DivS
            | WasmInstr::I32DivU
            | WasmInstr::I32RemS
            | WasmInstr::I32RemU
            | WasmInstr::I32And
            | WasmInstr::I32Or
            | WasmInstr::I32Xor
            | WasmInstr::I32Shl
            | WasmInstr::I32ShrS
            | WasmInstr::I32ShrU => {
                self.height -= 1; // pop2 push1
                if self.height < 0 {
                    return Err(self.diagnostic("i32 binop underflow"));
                }
            }
            WasmInstr::I32Eqz | WasmInstr::I32Clz | WasmInstr::I32Ctz | WasmInstr::I32Popcnt => {
                // pop1 push1 → net 0
            }
            WasmInstr::Call(_) => {
                let effect = call_effect.unwrap_or(StackEffect::new(0, 0));
                self.height -= effect.pops as i32;
                if self.height < 0 {
                    return Err(self.diagnostic(&format!(
                        "Call underflow (pops={}, height after would be {})",
                        effect.pops, self.height
                    )));
                }
                self.height += effect.pushes as i32;
            }
            WasmInstr::Select => {
                // pop3 push1
                self.height -= 2;
                if self.height < 0 {
                    return Err(self.diagnostic("Select underflow"));
                }
            }
            // Unmodeled ops: leave height unchanged (coverage gap is recorded if
            // validate still fails while the tracker stays silent).
            _ => {}
        }
        Ok(())
    }

    fn br_unreachable(&mut self, depth: u32) -> Result<(), String> {
        let idx = self
            .frames
            .len()
            .checked_sub(1 + depth as usize)
            .ok_or_else(|| self.diagnostic(&format!("Br({depth}) out of range")))?;
        let frame = &self.frames[idx];
        let expected = frame.height_on_enter + frame.result_arity as i32;
        // Soft-adjust height to the target label. Hard failures for leftover values are
        // reported at reachable `End` (the W001 shape), not here — Br sites often sit
        // after host/Call sequences whose guest effects are only partially modeled, and
        // a false Br fault would block the End-site diagnosis the census asks for.
        self.height = expected;
        // Ancestors remaining after End of the branch target (exclude target itself).
        // Br(1) out of `block (result i32)` must stay dead until that block's End —
        // not re-enter mid-block (SimdI8Extract lane loop fallthrough).
        self.unreachable_until_frames = Some(idx);
        Ok(())
    }
}

struct StackCtx {
    tracker: StackTracker,
    enabled: bool,
    /// Import function index → stack effect (args already pushed as separate instrs).
    import_effects: HashMap<u32, StackEffect>,
}

thread_local! {
    static STACK_CTX: RefCell<Option<StackCtx>> = const { RefCell::new(None) };
}

/// True unless `SYX_WASM_STACK_CHECK=0`.
pub fn stack_check_enabled() -> bool {
    match std::env::var("SYX_WASM_STACK_CHECK") {
        Ok(v) => v != "0" && !v.eq_ignore_ascii_case("false") && v != "off",
        Err(_) => true,
    }
}

/// Begin tracking for one MIR function body.
pub fn begin_function(
    fn_name: &str,
    aot_import_map: &HashMap<aot::Import, u32>,
    host_import_map: &HashMap<u32, u32>,
) {
    let enabled = stack_check_enabled();
    let mut import_effects = HashMap::new();
    for (kind, idx) in aot_import_map {
        import_effects.insert(*idx, import_effect(*kind));
    }
    // Host imports in the current ABI are overwhelmingly `() -> i32` (TYPE_FUNC_I32):
    // the emitter calls them with no guest args; the i32 stub is then run through
    // take_catch / Drop. Modeling them as `(i32)->i32` falsely underflows.
    for &wasm_idx in host_import_map.values() {
        import_effects
            .entry(wasm_idx)
            .or_insert_with(|| StackEffect::new(0, 1));
    }
    STACK_CTX.with(|c| {
        *c.borrow_mut() = Some(StackCtx {
            tracker: StackTracker::new(fn_name),
            enabled,
            import_effects,
        });
    });
}

pub fn end_function() {
    STACK_CTX.with(|c| {
        *c.borrow_mut() = None;
    });
}

pub fn set_mir_context(block: u32, idx: usize, mir_op: &str) {
    STACK_CTX.with(|c| {
        if let Some(ctx) = c.borrow_mut().as_mut() {
            ctx.tracker.set_mir_context(block, idx, mir_op);
        }
    });
}

/// Apply tracker to `instr` when an active context exists. Returns the first error.
pub fn track_instruction(instr: &WasmInstr<'_>, func_idx_hint: Option<u32>) -> Result<(), String> {
    STACK_CTX.with(|c| {
        let mut slot = c.borrow_mut();
        let Some(ctx) = slot.as_mut() else {
            return Ok(());
        };
        if !ctx.enabled {
            return Ok(());
        }
        let call_effect = match (instr, func_idx_hint) {
            (WasmInstr::Call(idx), _) => ctx.import_effects.get(idx).copied().or_else(|| {
                func_idx_hint
                    .and_then(|h| ctx.import_effects.get(&h).copied())
            }),
            _ => None,
        };
        // Prefer the Call's own index from the instruction.
        let call_effect = if let WasmInstr::Call(idx) = instr {
            ctx.import_effects.get(idx).copied().or(call_effect)
        } else {
            call_effect
        };
        ctx.tracker.apply(instr, call_effect)
    })
}

/// Format helper used by unit tests / credibility (stable keywords).
pub fn format_leak_diagnostic(
    fn_name: &str,
    mir_block: u32,
    mir_idx: usize,
    mir_op: &str,
    expected: i32,
    actual: i32,
) -> String {
    let mut out = String::new();
    let _ = write!(
        out,
        "W001 stack tracker: values remaining on stack at end of Block \
         (expected height {expected}, actual {actual}) \
         (fn={fn_name}, mir_block={mir_block}, mir_idx={mir_idx}, mir_op={mir_op}, height={actual}, frames=1)"
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_encoder::BlockType;

    #[test]
    fn r23_intentional_leak_diagnoses_mir_site() {
        let mut t = StackTracker::new("main");
        t.set_mir_context(2, 7, "ConstI32(1)");
        t.apply(&WasmInstr::Block(BlockType::Empty), None).unwrap();
        t.apply(&WasmInstr::I32Const(1), None).unwrap(); // leak
        let err = t.apply(&WasmInstr::End, None).expect_err("must detect leak");
        assert!(
            err.contains("W001 stack tracker"),
            "diagnostic must carry stable keyword: {err}"
        );
        assert!(err.contains("values remaining on stack"), "got: {err}");
        assert!(err.contains("fn=main"), "got: {err}");
        assert!(err.contains("mir_block=2"), "got: {err}");
        assert!(err.contains("mir_idx=7"), "got: {err}");
        assert!(err.contains("mir_op=ConstI32(1)"), "got: {err}");
        assert!(err.contains("expected height 0"), "got: {err}");
        assert!(err.contains("actual 1"), "got: {err}");
    }

    #[test]
    fn r23_balanced_block_ok() {
        let mut t = StackTracker::new("ok");
        t.apply(&WasmInstr::Block(BlockType::Empty), None).unwrap();
        t.apply(&WasmInstr::I32Const(1), None).unwrap();
        t.apply(&WasmInstr::Drop, None).unwrap();
        t.apply(&WasmInstr::End, None).unwrap();
        assert_eq!(t.height(), 0);
    }

    #[test]
    fn r24_block_result_i32_keeps_arity() {
        // SimdI8Extract pilot: `block (result i32)` … `end` must leave height +1.
        let mut t = StackTracker::new("SyxHost_simd_i8_extract");
        t.set_mir_context(0, 2, "SimdI8Extract");
        t.apply(
            &WasmInstr::Block(BlockType::Result(wasm_encoder::ValType::I32)),
            None,
        )
        .unwrap();
        t.apply(&WasmInstr::I32Const(7), None).unwrap();
        t.apply(&WasmInstr::End, None).unwrap();
        assert_eq!(t.height(), 1, "result i32 must remain after End");
        t.apply(&WasmInstr::Drop, None).unwrap();
        assert_eq!(t.height(), 0);
    }

    #[test]
    fn r23_if_else_empty_ok() {
        let mut t = StackTracker::new("ife");
        t.apply(&WasmInstr::I32Const(1), None).unwrap(); // condition
        t.apply(&WasmInstr::If(BlockType::Empty), None).unwrap();
        t.apply(&WasmInstr::Else, None).unwrap();
        t.apply(&WasmInstr::End, None).unwrap();
        assert_eq!(t.height(), 0);
    }

    #[test]
    fn r23_format_leak_diagnostic_keywords() {
        let s = format_leak_diagnostic("main", 0, 1, "Print", 0, 1);
        assert!(s.contains("W001 stack tracker"));
        assert!(s.contains("mir_op=Print"));
    }
}
