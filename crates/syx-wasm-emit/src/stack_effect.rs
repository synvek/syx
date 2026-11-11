//! R-17: wasm operand-stack contract for AOT host imports.
//!
//! Before this module, every `MirOp` arm in `mir_emit` hand-coded whether it left an
//! i32 on the wasm operand stack. Miss one and `wasmparser` rejects the module with
//! `values remaining on stack at end of block` (W001). The authoritative signature is
//! already encoded in [`aot::Import::type_index`], so we derive the stack effect from
//! it instead of duplicating the knowledge 40+ times.
//!
//! Scope: this models the *wasm operand stack* only. The SYX runtime stack (the RT
//! stack manipulated by `push_rt_*` / `take_rt_*`) is separate and unaffected.

use crate::exec::{aot, host};

/// Wasm-level stack effect of calling a host import.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackEffect {
    /// Values popped from the operand stack by the call.
    pub pops: u32,
    /// Values pushed onto the operand stack by the call.
    pub pushes: u32,
}

impl StackEffect {
    pub const fn new(pops: u32, pushes: u32) -> Self {
        Self { pops, pushes }
    }

    /// Net change to the operand stack height.
    pub fn delta(self) -> i32 {
        self.pushes as i32 - self.pops as i32
    }

    /// True when the call leaves no value behind (safe to emit with no cleanup).
    pub fn is_void(self) -> bool {
        self.pushes == 0
    }
}

/// Stack effect for a host import, derived from its wasm signature.
///
/// Keep this in sync with [`aot::Import::type_index`]; the parity test at the bottom
/// of this module fails if a signature is mapped to the wrong shape.
pub fn import_effect(import: aot::Import) -> StackEffect {
    use host::*;
    match import.type_index() {
        TYPE_MIR_EXEC => StackEffect::new(3, 1),
        TYPE_FUNC_I32 => StackEffect::new(0, 1),
        TYPE_I32_TO_I32 => StackEffect::new(1, 1),
        TYPE_I32_VOID => StackEffect::new(1, 0),
        TYPE_I32_I32_TO_I32 => StackEffect::new(2, 1),
        TYPE_VOID => StackEffect::new(0, 0),
        TYPE_I32_I32_VOID => StackEffect::new(2, 0),
        TYPE_CONST_NUMERIC128 => StackEffect::new(5, 1),
        TYPE_FUNC_F64 => StackEffect::new(0, 1),
        TYPE_F64_VOID => StackEffect::new(1, 0),
        TYPE_FUNC_F32 => StackEffect::new(0, 1),
        TYPE_F32_VOID => StackEffect::new(1, 0),
        TYPE_FUNC_V128 => StackEffect::new(0, 1),
        TYPE_I32_V128_VOID => StackEffect::new(2, 0),
        TYPE_FUNC_I64 => StackEffect::new(0, 1),
        TYPE_I64_VOID => StackEffect::new(1, 0),
        // Unknown signature: assume it leaves nothing behind. Callers that need a value
        // take it explicitly, so a conservative `(0, 0)` never invents a phantom value.
        _ => StackEffect::new(0, 0),
    }
}

/// Number of i32 operands an instruction pushes *before* calling `import`.
///
/// This is the argument marshalling the emitter performs immediately prior to the call
/// (e.g. `i32.const slot` for `load_local`). Derived from the same signature table.
pub fn import_arg_count(import: aot::Import) -> u32 {
    import_effect(import).pops
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r17_void_imports_push_nothing() {
        // These imports are pure RT-stack side effects; they must not leave a value.
        for import in [
            aot::Import::StoreLocal,
            aot::Import::StoreGlobal,
            aot::Import::LoadLocal,
            aot::Import::PushRtI32,
            aot::Import::PushRtBool,
            aot::Import::PushRtNil,
            aot::Import::StackOp,
            aot::Import::EndFn,
        ] {
            let e = import_effect(import);
            assert!(
                e.is_void(),
                "{import:?} must not leave a wasm operand-stack value (got {e:?})"
            );
        }
    }

    /// `load_global` is `(i32) -> i32`: it takes a global slot and returns the value.
    /// This is NOT void — the emitter must consume the result.
    #[test]
    fn r17_load_global_leaves_one_value() {
        let e = import_effect(aot::Import::LoadGlobal);
        assert_eq!(e, StackEffect::new(1, 1));
    }

    #[test]
    fn r17_value_imports_push_one() {
        for import in [
            aot::Import::TakeRtI32,
            aot::Import::GetReturn,
            aot::Import::CallClosure,
            aot::Import::CallHost,
            aot::Import::DictGet,
            aot::Import::ObjectGet,
        ] {
            let e = import_effect(import);
            assert_eq!(
                e.pushes, 1,
                "{import:?} must leave exactly one value (got {e:?})"
            );
        }
    }

    #[test]
    fn r17_effect_delta_matches_signature_shape() {
        // Delta is what the call alone does to the stack; the emitter pairs it with its
        // own argument pushes (e.g. `i32.const slot` before `store_local`), netting 0.
        assert_eq!(import_effect(aot::Import::TakeRtI32).delta(), 1);
        assert_eq!(import_effect(aot::Import::StoreLocal).delta(), -1);
        // const_str is `(i32, i32) -> i32`: two args in, one discarded i32 out.
        assert_eq!(import_effect(aot::Import::ConstStr).delta(), -1);
        // Paired with its single argument push, store_local nets zero stack change.
        assert_eq!(import_arg_count(aot::Import::StoreLocal), 1);
        assert_eq!(import_effect(aot::Import::StoreLocal).delta() + 1, 0);
    }
}
