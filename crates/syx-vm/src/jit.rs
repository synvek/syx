use std::collections::HashMap;

use syx_codegen::CompiledProgram;
use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;

/// Phase 36 / ADR-140: tier-2 native entry metadata.
#[derive(Debug, Clone)]
pub struct JitNativeMeta {
    pub code_len: usize,
    pub deopt_version: u32,
    pub native: bool,
    pub code_ptr: usize,
    pub param_count: u32,
}

/// JIT compiler with hot-path counter and optional Cranelift tier-2.
///
/// **ADR-140 / M1 W4:** when `SYX_JIT_TIER2=1`, Cranelift lowers Suite-1 pilot MirOps and the
/// VM **invokes** the native entry. Default remains **off**. Unsupported ops → deopt + reason.
#[derive(Debug, Clone)]
pub struct JitCompiler {
    hot_threshold: u32,
    counters: HashMap<String, u32>,
    compiled: HashMap<String, Vec<u8>>,
    native_meta: HashMap<String, JitNativeMeta>,
    deopt_funcs: HashMap<String, u32>,
    deopt_count: u64,
    native_call_count: u64,
    last_deopt_reason: Option<String>,
}

impl Default for JitCompiler {
    fn default() -> Self {
        let threshold = if Self::fastpath_enabled() || Self::tier2_opt_in_enabled() {
            1
        } else {
            1000
        };
        Self::new(threshold)
    }
}

impl JitCompiler {
    pub fn new(hot_threshold: u32) -> Self {
        Self {
            hot_threshold,
            counters: HashMap::new(),
            compiled: HashMap::new(),
            native_meta: HashMap::new(),
            deopt_funcs: HashMap::new(),
            deopt_count: 0,
            native_call_count: 0,
            last_deopt_reason: None,
        }
    }

    pub fn fastpath_enabled() -> bool {
        std::env::var("SYX_JIT_FASTPATH")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    }

    pub fn record_call(&mut self, func: &str) {
        *self.counters.entry(func.to_string()).or_insert(0) += 1;
    }

    pub fn call_count(&self, func: &str) -> u32 {
        self.counters.get(func).copied().unwrap_or(0)
    }

    pub fn should_jit(&self, func: &str) -> bool {
        self.call_count(func) >= self.effective_hot_threshold()
    }

    pub fn deopt_count(&self) -> u64 {
        self.deopt_count
    }

    pub fn native_call_count(&self) -> u64 {
        self.native_call_count
    }

    pub fn last_deopt_reason(&self) -> Option<&str> {
        self.last_deopt_reason.as_deref()
    }

    pub fn maybe_compile(&mut self, func: &str, program: &CompiledProgram) -> Option<&[u8]> {
        self.record_call(func);
        if !self.should_jit(func) {
            return None;
        }
        if self.deopt_funcs.contains_key(func) {
            return self.compiled.get(func).map(|b| b.as_slice());
        }
        if !self.compiled.contains_key(func) {
            #[cfg(feature = "jit-tier2")]
            if Self::tier2_opt_in_enabled() {
                if let Some(mir) = program.mir_module.as_ref() {
                    match syx_cranelift_jit::compile_hot_function(mir, func) {
                        Ok(entry) if entry.native => {
                            self.native_meta.insert(
                                func.to_string(),
                                JitNativeMeta {
                                    code_len: entry.code_len,
                                    deopt_version: entry.deopt_version,
                                    native: true,
                                    code_ptr: entry.code_ptr,
                                    param_count: entry.param_count,
                                },
                            );
                            self.compiled.insert(
                                func.to_string(),
                                format!("cranelift-jit-native:{func}:{}", entry.code_len)
                                    .into_bytes(),
                            );
                        }
                        Ok(_) => {}
                        Err(e) => {
                            let reason = e.to_string();
                            self.deopt_with_reason(func, &reason);
                        }
                    }
                }
            }
            if Self::fastpath_enabled() && !self.compiled.contains_key(func) {
                if let Some(mir_fn) = program.mir_functions.get(func) {
                    if mir_fn.instructions.len() >= 4 {
                        self.compiled.insert(
                            func.to_string(),
                            format!("mir-fastpath:{func}").into_bytes(),
                        );
                    }
                }
            }
            if !self.compiled.contains_key(func) {
                let stub = format!("jit-stub:{func}:{}", program.functions.len()).into_bytes();
                self.compiled.insert(func.to_string(), stub);
            }
        }
        self.compiled.get(func).map(|b| b.as_slice())
    }

    pub fn fastpath_ready(&self, func: &str) -> bool {
        self.compiled
            .get(func)
            .map(|b| b.starts_with(b"mir-fastpath:") || b.starts_with(b"cranelift-jit-native:"))
            .unwrap_or(false)
    }

    pub fn native_ready(&self, func: &str) -> bool {
        self.native_meta
            .get(func)
            .map(|m| m.native && m.code_ptr != 0 && !self.deopt_funcs.contains_key(func))
            .unwrap_or(false)
    }

    pub fn native_meta(&self, func: &str) -> Option<&JitNativeMeta> {
        self.native_meta.get(func)
    }

    /// Invoke Cranelift native entry; returns `Value::I32` result.
    #[cfg(feature = "jit-tier2")]
    pub fn invoke_native(&mut self, func: &str, args: &[Value]) -> SyxResult<Option<Value>> {
        if !self.native_ready(func) {
            return Ok(None);
        }
        let meta = self.native_meta.get(func).cloned().ok_or_else(|| {
            syx_diagnostics::SyxDiagnostic::error(format!("JIT: missing meta for {func}"))
        })?;
        let iargs: Vec<i32> = args.iter().map(|v| v.as_i32()).collect();
        let entry = syx_cranelift_jit::JitNativeEntry {
            func_name: func.to_string(),
            code_len: meta.code_len,
            deopt_version: meta.deopt_version,
            native: true,
            code_ptr: meta.code_ptr,
            param_count: meta.param_count,
        };
        match syx_cranelift_jit::invoke_native(&entry, &iargs) {
            Ok(n) => {
                self.native_call_count += 1;
                Ok(Some(Value::I32(n)))
            }
            Err(e) => {
                let reason = e.to_string();
                self.deopt_with_reason(func, &reason);
                Ok(None)
            }
        }
    }

    #[cfg(not(feature = "jit-tier2"))]
    pub fn invoke_native(&mut self, _func: &str, _args: &[Value]) -> SyxResult<Option<Value>> {
        Ok(None)
    }

    /// Phase 36 / ADR-140: deopt to interpreter (unsupported op / GC / await / host).
    pub fn deopt(&mut self, func: &str) {
        self.deopt_with_reason(func, "Deopt");
    }

    pub fn deopt_with_reason(&mut self, func: &str, reason: &str) {
        self.deopt_count += 1;
        self.last_deopt_reason = Some(reason.to_string());
        #[cfg(feature = "jit-tier2")]
        syx_cranelift_jit::record_deopt_reason(reason);
        self.deopt_funcs
            .insert(func.to_string(), syx_cranelift_jit_deopt_version());
        self.compiled.remove(func);
        self.native_meta.remove(func);
        #[cfg(feature = "jit-tier2")]
        syx_cranelift_jit::invalidate_jit_cache();
    }

    pub fn invalidate_all(&mut self) {
        self.compiled.clear();
        self.native_meta.clear();
        self.deopt_funcs.clear();
        #[cfg(feature = "jit-tier2")]
        syx_cranelift_jit::invalidate_jit_cache();
    }

    pub fn hotspot_count(&self) -> usize {
        self.counters
            .values()
            .filter(|c| **c >= self.hot_threshold)
            .count()
    }

    pub fn tier2_bench_enabled() -> bool {
        std::env::var("SYX_JIT_TIER2_BENCH")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    }

    pub fn tier2_nightly_default_enabled() -> bool {
        std::env::var("SYX_JIT_TIER2").is_err()
            && std::env::var("SYX_NIGHTLY")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
    }

    /// ADR-140: production default is **off**.
    pub fn tier2_prod_default_enabled() -> bool {
        false
    }

    pub fn tier2_opt_in_enabled() -> bool {
        match std::env::var("SYX_JIT_TIER2") {
            Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
            Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
            Ok(_) => false,
            Err(_) => Self::tier2_prod_default_enabled(),
        }
    }

    pub fn effective_hot_threshold(&self) -> u32 {
        if Self::tier2_opt_in_enabled() {
            1
        } else {
            self.hot_threshold
        }
    }

    pub fn tier2_bench_min_ratio() -> f64 {
        if cfg!(debug_assertions) {
            0.85
        } else {
            2.0
        }
    }
}

#[cfg(feature = "jit-tier2")]
fn syx_cranelift_jit_deopt_version() -> u32 {
    syx_cranelift_jit::cranelift_jit_deopt_version()
}

#[cfg(not(feature = "jit-tier2"))]
fn syx_cranelift_jit_deopt_version() -> u32 {
    140
}

#[cfg(feature = "jit-tier2")]
pub fn cranelift_jit_tier2_prod_ready() -> bool {
    syx_cranelift_jit::cranelift_jit_tier2_prod_ready()
}

#[cfg(not(feature = "jit-tier2"))]
pub fn cranelift_jit_tier2_prod_ready() -> bool {
    false
}

pub fn run_with_jit(program: &CompiledProgram, entry: &str) -> SyxResult<()> {
    let mut jit = JitCompiler::default();
    for func in &program.functions {
        if func.name == entry {
            let _ = jit.maybe_compile(entry, program);
            break;
        }
    }
    Ok(())
}

/// Phase 36 FQ6: global JIT cache invalidation (GC safepoint).
pub fn invalidate_all_jit_caches() {
    #[cfg(feature = "jit-tier2")]
    syx_cranelift_jit::invalidate_jit_cache();
}

#[cfg(test)]
mod tests {
    use super::*;
    use syx_codegen::{FunctionUnit, Instruction};

    #[test]
    fn hot_path_counter_triggers_jit_stub() {
        let mut jit = JitCompiler::new(3);
        let program = CompiledProgram {
            functions: vec![FunctionUnit {
                name: "main".into(),
                param_count: 0,
                local_count: 0,
                instructions: vec![Instruction::Return],
                instruction_spans: vec![],
                local_names: vec![],
                local_types: vec![],
                source_name: Default::default(),
                init_body_pc: 0,
                init_wrap_pc: 0,
            }],
            ..Default::default()
        };
        assert!(!jit.should_jit("main"));
        for _ in 0..3 {
            jit.record_call("main");
        }
        assert!(jit.should_jit("main"));
        assert!(jit.maybe_compile("main", &program).is_some());
    }

    #[test]
    fn fastpath_marker_when_env_enabled() {
        std::env::remove_var("SYX_JIT_TIER2");
        std::env::set_var("SYX_JIT_FASTPATH", "1");
        let mut jit = JitCompiler::new(2);
        let mut program = CompiledProgram {
            functions: vec![FunctionUnit {
                name: "hot".into(),
                param_count: 0,
                local_count: 0,
                instructions: vec![Instruction::Return],
                instruction_spans: vec![],
                local_names: vec![],
                local_types: vec![],
                source_name: Default::default(),
                init_body_pc: 0,
                init_wrap_pc: 0,
            }],
            ..Default::default()
        };
        program.mir_functions.insert(
            smol_str::SmolStr::new("hot"),
            syx_mir::MirFunction {
                name: "hot".into(),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![],
                instructions: vec![
                    syx_mir::MirInstr {
                        dest: None,
                        op: syx_mir::MirOp::Return,
                        span: None,
                    };
                    4
                ],
                local_names: vec![],
                local_types: vec![],
                init_body_pc: 0,
                init_wrap_pc: 0,
            },
        );
        jit.record_call("hot");
        jit.record_call("hot");
        assert!(jit.maybe_compile("hot", &program).is_some());
        assert!(jit.fastpath_ready("hot"));
        std::env::remove_var("SYX_JIT_FASTPATH");
    }

    #[test]
    fn tier2_prod_default_when_unset() {
        std::env::remove_var("SYX_JIT_TIER2");
        std::env::remove_var("SYX_NIGHTLY");
        // Re-read after clear (other tests may race; assert prod default flag itself).
        assert!(!JitCompiler::tier2_prod_default_enabled());
        // If env still polluted by parallel tests, skip opt-in check.
        if std::env::var("SYX_JIT_TIER2").is_err() {
            assert!(!JitCompiler::tier2_opt_in_enabled());
        }
    }

    #[test]
    fn tier2_nightly_default_when_unset() {
        std::env::remove_var("SYX_JIT_TIER2");
        std::env::set_var("SYX_NIGHTLY", "1");
        if std::env::var("SYX_JIT_TIER2").is_err() {
            assert!(JitCompiler::tier2_nightly_default_enabled());
            assert!(!JitCompiler::tier2_opt_in_enabled());
        }
        std::env::remove_var("SYX_NIGHTLY");
    }

    #[test]
    fn tier2_explicit_one_enables() {
        std::env::remove_var("SYX_NIGHTLY");
        std::env::set_var("SYX_JIT_TIER2", "1");
        assert!(JitCompiler::tier2_opt_in_enabled());
        std::env::remove_var("SYX_JIT_TIER2");
    }

    #[test]
    fn tier2_explicit_zero_disables_prod_default() {
        std::env::remove_var("SYX_NIGHTLY");
        std::env::set_var("SYX_JIT_TIER2", "0");
        assert!(!JitCompiler::tier2_opt_in_enabled());
        std::env::remove_var("SYX_JIT_TIER2");
    }

    #[test]
    fn deopt_clears_native_ready() {
        let mut jit = JitCompiler::new(1);
        jit.native_meta.insert(
            "hot".into(),
            JitNativeMeta {
                code_len: 64,
                deopt_version: 140,
                native: true,
                code_ptr: 1,
                param_count: 0,
            },
        );
        assert!(jit.native_ready("hot"));
        jit.deopt_with_reason("hot", "UnsupportedMirOp(Print)");
        assert!(!jit.native_ready("hot"));
        assert_eq!(jit.deopt_count(), 1);
        assert_eq!(jit.last_deopt_reason(), Some("UnsupportedMirOp(Print)"));
    }
}
