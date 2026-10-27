# WASM / Native / VM Tier Opcode Matrix (Phase 78)

| Opcode / feature | VM bytecode | WASM tier-1 | Native LLVM | Heap v7 | Notes |
|------------------|-------------|-------------|-------------|---------|-------|
| HeapRelocate | yes | yes | yes | yes | Phase 78 `relocate_global` + `syx_rt_heap_relocate` |
| DictGet / DictSet | yes | yes | yes | yes | compile + MIR/WASM four-end (VM stdout GA deferred) |

**Phase 78 gates:**
- Heap v7 relocate: `cargo test -p syx-driver --test vm_heap_relocate_phase78`
- Cache v50: `cargo test -p syx-driver --test cache_v50_inkwell_heap_rt_lock_invalidation`
- Lock v12: `cargo test -p syx-driver --test registry_lock_v12_phase78`
- Farm worker: `cargo test -p syx-driver --test build_farm_worker_hit_phase78`
- Inkwell module v4: `cargo test -p syx-driver --test inkwell_module_real_phase78`
- RT runtime: `cargo test -p syx-driver --test rt_zero_mir_exec_runtime_phase78`
- Dict four-end: `cargo test -p syx-driver --test heap_dict_four_end_phase78`

**Phase 77 gates:**
- Heap v6 host: `cargo test -p syx-driver --test vm_heap_host_materialize_phase77`
- Cache v49: `cargo test -p syx-driver --test cache_v49_inkwell_heap_rt_lock_invalidation`
- Lock v11: `cargo test -p syx-driver --test registry_lock_v11_phase77`
- Farm queue: `cargo test -p syx-driver --test build_farm_queue_phase77`
- Inkwell llvm: `cargo test -p syx-driver --test inkwell_llvm_real_phase77`
- RT zero exec: `cargo test -p syx-driver --test rt_zero_internal_exec_main_async_phase77`
- main.sy SIMD: `cargo test -p syx-driver --test examples_main_simd_phase77`

**Phase 76 gates:**
- Four-end heap: `cargo test -p syx-driver --test vm_heap_slot_parity_phase76`
- Cache push: `cargo test -p syx-driver --test cache_remote_push_phase76`
- Lock v10: `cargo test -p syx-driver --test registry_lock_v10_phase76`
- Inkwell crate: `cargo test -p syx-driver --test inkwell_crate_parity_phase76`

**Phase 74 gates:**
- Native strict: `cargo test -p syx-driver --test native_exec_op_zero_phase74 --test native_tier2_full_matrix`
- Heap v3: `cargo test -p syx-driver --test memory_unified_heap --test borrow_phase74_heap`
- Actor export: `cargo test -p syx-driver --test actor_cluster_phase74`
- Behavioral audit: `bash scripts/audit_behavioral_tests.sh --min-driver 900 --min-lsp 1400`
