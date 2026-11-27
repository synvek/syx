# SYX 改进路线图（收敛期 · C-*）

> **活 backlog。** 编号刀 = **C-0…**（Convergence）。  
> **状态叙事：** [`01-project-status.md`](01-project-status.md)  
> **外部审计输入：** [`syx-status-final-report.md`](syx-status-final-report.md)  
> **上期详账（只读）：** [`archive/02-improvement-roadmap-sa-r-u-ledger.md`](archive/02-improvement-roadmap-sa-r-u-ledger.md)（SA-0…R-49 + U-0…U-40）  
> **最近更新：** 2026-11-27 · **C-0j 方法实参 Unknown sink `[x]`**；C-0 `[x]` 薄闸（C-0a+ 仍 park）；C-3 `[~]`（a…ca）；C-5 `[~]`；C-8 `[~]`；C-7 `[~]`；**C-6 `[x]`**；C-4 `[x]`；默认下一 = **跨轴**（C-0a+ prep / C-0e / subscript_set tip；不宣称 C-3/C-5 `[x]`）。

## 0. 时代切换

| 时代 | 编号 | 状态 | 目标 |
|------|------|------|------|
| 可信度 / Wave / Stage | Stage-0…4 | `[x]` | 证据链与治理 |
| Swift 对齐章程 | SA-0…SA-6 | `[x]` | 对齐边界与 ADR |
| Object / 平台 / 诊断 | R-0…R-49 | `[x]` | 收口；**无 R-50** |
| 语言表面薄刀 | U-0…U-40 | `[x]` | FilePath→字段默认→extension lit |
| **收敛期** | **C-0…** | **活** | 语义 soundness · 堆安全 · 后端诚实 · SSOT |

**默认下一不再是 U-41 / Ch15 MD / HostFn+1。** 新窗用 `C-N`；厚项继续 park。

## 1. 硬 KPI 与硬否

每窗退出必须复跑（U-40 地板）：

| 闸 | 门槛 |
|----|------|
| Wasm | **46/46** |
| Browser | **0** failed |
| Object residual | **≤180 / ≤10** |
| simd residual | **≥159** |
| `sy,output` | **12/12** |

**硬否：** 无 R-50；不以 HostFn+1 / 文档章节数 / 测试条数冒充成熟度；不以 L006 soft-skip 冒充 native 绿；不把 ManagedHeap stdout churn 当成功；不默认开 JIT / SyxHost / CST 深刀 / 真隐式 `CallSuper`。

## 2. 收敛期活轴（C-*）

### C-0 · 类型 soundness（P0） — `[x]` 薄闸完成

**目标：** 未知/错误类型不得 silently 穿过 check；函数 return、call arity、`inout`/`&mut`、narrowing 有负向闸。

**窗序（相对纸面 a→e 的执行序）：** **C-0b → C-0c → C-0d → C-0a**；**C-0e park**；**C-0a+**（完整三分）park。

| 子项 | 状态 | 内容 |
|------|------|------|
| **C-0b** | `[x]` | wrong-type / 裸非 Void **return** → **S004**；方法 mangled `current_function`；census [`c-0b-return-type-census`](../docs/census/c-0b-return-type-census.md) |
| **C-0c** | `[x]` | 非泛型自由函数 **arity** → **S061**（计 default；标签仍 S042；类型错仍 S004）；census [`c-0c-call-arity-census`](../docs/census/c-0c-call-arity-census.md) |
| **C-0d** | `[x]` | `&mut` 裸值传参拒收（AST `Borrow{mutable:true}`；剥 Group；勿改 `deref_type` / is_compatible 臂）；census [`c-0d-mut-ref-arg-census`](../docs/census/c-0d-mut-ref-arg-census.md) |
| **C-0a** | `[x]` 薄闸 | Unknown→地面具体汇点（无注解形参 / 缺失 Member）；**未**关 `(_, Unknown)`；census [`c-0a-unknown-sink-census`](../docs/census/c-0a-unknown-sink-census.md) |
| **C-0f** | `[x]` | 调用实参 Unknown→地面具体形参 → **S004**（复用 `check_unknown_to_ground_sink`；**未**关软臂）；census [`c-0f-call-arg-unknown-sink-census`](../docs/census/c-0f-call-arg-unknown-sink-census.md) |
| **C-0g** | `[x]` | CompoundAssign Unknown→地面具体汇点 → **S004**（复用 `check_unknown_to_ground_sink`；**未**关软臂）；census [`c-0g-compound-assign-unknown-sink-census`](../docs/census/c-0g-compound-assign-unknown-sink-census.md) |
| **C-0h** | `[x]` | MemberAssign：`infer_member_type` + assignable + Unknown→地面具体汇点 → **S004**（复用 `check_unknown_to_ground_sink`；**未**关软臂）；census [`c-0h-member-assign-unknown-sink-census`](../docs/census/c-0h-member-assign-unknown-sink-census.md) |
| **C-0i** | `[x]` | IndexAssign：Array/Dict 地面 value Unknown→地面具体汇点 → **S004**（复用 `check_unknown_to_ground_sink`；**未**关软臂）；census [`c-0i-index-assign-unknown-sink-census`](../docs/census/c-0i-index-assign-unknown-sink-census.md) |
| **C-0j** | `[x]` | 用户 Struct/Class 方法 mangled `function_params` + Member 实参 Unknown→地面形参 → **S004**（复用 `check_unknown_to_ground_sink`；**未**关软臂；≠ host/extension GA）；census [`c-0j-method-arg-unknown-sink-census`](../docs/census/c-0j-method-arg-unknown-sink-census.md) |
| **C-0a+** | `[-]` park | `ErrorType` / `InferenceVar` 三分；关 `is_compatible(_, Unknown)` 软臂 |
| **C-0e** | `[-]` park | integer narrowing / 书 S008 对齐 |

**验收：** 新负向用例进 gate；正向回归不破硬 KPI；census 写清「拒收面」而非「多了多少通过」。  
**验收（C-0f）：** `wrap(_ x) { take(x) }`（`take` 形参 Int）→ **S004**；`Any`/`Object`/字面量不破；硬 KPI 不破；**C-0f `[x]`**；**C-0a+ 仍 park**；默认下一 → **跨轴**（C-3 DefId / 或 P0 余债；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-0g）：** `bump(_ x) { var n: Int = 0; n += x }` → **S004**；字面量/`Any` 不破；软臂保留；硬 KPI 不破；**C-0g `[x]`**；**C-0a+ 仍 park**；默认下一 → **跨轴**（C-0a+ prep / C-0e / 或 MirOp 厚余片；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-0h）：** `put(_ x) { var c = Cell(); c.n = x }`（`n: Int`）→ **S004**；字面量/`Any` 不破；软臂保留；硬 KPI 不破；**C-0h `[x]`**；**C-0a+ 仍 `[-]`**；默认下一 → **跨轴**（IndexAssign tip / C-0a+ prep / C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-0i）：** `put(_ x) { var a: Array<Int> = [0]; a[0] = x }` → **S004**；Dict 同形；字面量/`Any` 不破；软臂保留；硬 KPI 不破；**C-0i `[x]`**；**C-0a+ 仍 `[-]`**；默认下一 → **跨轴**（方法全参 tip / C-0a+ prep / C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-0j）：** `struct S { take(_ n: Int); wrap(_ x) { self.take(x) } }` + `S().wrap(1)` → **S004**；字面量/`Any` 不破；软臂保留；硬 KPI 不破；**C-0j `[x]`**；**C-0a+ 仍 `[-]`**；默认下一 → **跨轴**（C-0a+ prep / C-0e / subscript_set tip；**不**宣称 C-3/C-5 `[x]`）。  
**不做：** 完整 Swift 泛型；整语言 rewrite；C-0b 不做全覆盖 missing-return CFG。

### C-1 · ManagedHeap 安全默认（P0） — `[x]` 默认降级+identity+immutable+叙事

**目标：** 默认路径不再依赖已知危险的 generational/compact；weak / immutable 语义可测；禁 churn=GC 绿叙事。

| 子项 | 状态 | 内容 |
|------|------|------|
| **C-1a** | `[x]` | 默认堆策略降级（v9/v10/v11/v13/compact opt-in；churn stdout 门闩）；census [`c-1a-heap-default-degrade-census`](../docs/census/c-1a-heap-default-degrade-census.md) |
| **C-1b** | `[x]` | weak load 存活保 canonical identity；死后 / mark_dead 仅 Nil；census [`c-1b-weak-identity-census`](../docs/census/c-1b-weak-identity-census.md) |
| **C-1c** | `[x]` | `mutable=false` ObjectRef 写拒收（不 insert、返回原 handle）；census [`c-1c-immutable-write-census`](../docs/census/c-1c-immutable-write-census.md) |
| **C-1d** | `[x]` | 禁「stdout churn = GC 成功」叙事；Host label echo ≠ GC KPI；census [`c-1d-churn-narrative-census`](../docs/census/c-1d-churn-narrative-census.md) |

**验收（C-1d）：** 测例/文档不再把 print/`gc:churn:` 回显冒充 GC 绿；C-1a 门闩不破；硬 KPI 不破。

### C-2 · Backend / CI 支持矩阵诚实（P0） — `[x]` SSOT + mirror + unpaid demote + paid floor + JIT label + ci.job 对齐

**命名：** `-N` = N 个历史 `+`（`C-2b-1`≈`C-2b+`，`C-2b-2`≈`C-2b++`，`C-2b-3`≈旧 `C-2b+++`）。新默认下一禁止写 `+++`。

**目标：** 文档与 CI 对 VM / MIR / Wasm / Native / JIT 的「支持」说法与真实证据一致。  
**SSOT：** [`docs/gates/backend-ci-matrix.md`](../docs/gates/backend-ci-matrix.md)

| 子项 | 状态 | 内容 |
|------|------|------|
| **C-2a** | `[x]` | 默认硬闸 = VM + MIR + Wasm（46/46）+ browser 0；native L006 / JIT = soft·experimental·非默认 KPI；census [`c-2a-backend-ci-matrix-census`](../docs/census/c-2a-backend-ci-matrix-census.md) |
| **C-2b** | `[x]` | 禁 `run_native_strict_or_mirror_vm` 假绿；GA 无 AOT → skip；census [`c-2b-mirror-vm-honesty-census`](../docs/census/c-2b-mirror-vm-honesty-census.md) |
| **C-2b+** / **C-2b-1** | `[x]` | unpaid `native=real`→`partial`；禁 soft_native L006→exit 0；census [`c-2b-plus-native-real-honesty-census`](../docs/census/c-2b-plus-native-real-honesty-census.md) |
| **C-2b++** / **C-2b-2** | `[x]` | required `full-native-linux`（LLVM 21.1.6 + hello `--native`）；census [`c-2b-plusplus-full-native-ci-census`](../docs/census/c-2b-plusplus-full-native-ci-census.md) |
| **C-2b-3** | `[x]` | path_file GA 进 paid job；`io.path` + `io.file.read_all_bytes` → `native=real`；census [`c-2b-3-path-file-raise-census`](../docs/census/c-2b-3-path-file-raise-census.md) |
| **C-2b-4** | `[x]` | tcp GA 进 paid job；`net.tcp` + `net.tcp.listener` → `native=real`；census [`c-2b-4-tcp-raise-census`](../docs/census/c-2b-4-tcp-raise-census.md) |
| **C-2b-5** | `[x]` | sql GA 进 paid job；`data.sqlite` → `native=real`（≠ pool/row）；census [`c-2b-5-sql-raise-census`](../docs/census/c-2b-5-sql-raise-census.md) |
| **C-2b-6** | `[x]` | http GA 进 paid job；`net.http` → `native=real`（≠ server/types/stream/timeout_tls）；census [`c-2b-6-http-raise-census`](../docs/census/c-2b-6-http-raise-census.md) |
| **C-2b-7** | `[x]` | pool/row llvm 进 paid job；`data.sqlite.pool` + `data.sqlite.row` → `native=real`；census [`c-2b-7-pool-row-raise-census`](../docs/census/c-2b-7-pool-row-raise-census.md) |
| **C-2b-8** | `[x]` | types llvm 进 paid job；`net.http.types` → `native=real`（≠ server/stream/timeout_tls）；census [`c-2b-8-http-types-raise-census`](../docs/census/c-2b-8-http-types-raise-census.md) |
| **C-2b-9** | `[x]` | server llvm 进 paid job；`net.http.server` → `native=real`（≠ stream/timeout_tls）；census [`c-2b-9-http-server-raise-census`](../docs/census/c-2b-9-http-server-raise-census.md) |
| **C-2b-10** | `[x]` | stream llvm 进 paid job；`net.http.stream` → `native=real`（≠ timeout_tls）；census [`c-2b-10-http-stream-raise-census`](../docs/census/c-2b-10-http-stream-raise-census.md) |
| **C-2b-11** | `[x]` | timeout llvm 进 paid job；`net.http.timeout_tls` → `native=real`（≠ tls.client）；census [`c-2b-11-timeout-tls-raise-census`](../docs/census/c-2b-11-timeout-tls-raise-census.md) |
| **C-2b-12** | `[x]` | raii/stream/watch llvm 进 paid job；`io.file.raii`+`stream`+`watch` → `native=real`（≠ tls）；census [`c-2b-12-io-file-raise-census`](../docs/census/c-2b-12-io-file-raise-census.md) |
| **C-2b-13** | `[x]` | tls.client aot 进 paid job；`net.tls.client` → `native=real`（≠ server/daemon）；census [`c-2b-13-tls-client-raise-census`](../docs/census/c-2b-13-tls-client-raise-census.md) |
| **C-2c** | `[x]` | JIT 公开 Stub/default-on → experimental-real-subset · default off · non_default_kpi；census [`c-2c-jit-experimental-label-census`](../docs/census/c-2c-jit-experimental-label-census.md) |
| **C-2d** | `[x]` | 8× `ci.job.*` surface 对齐 yaml job；census [`c-2d-ci-job-align-census`](../docs/census/c-2d-ci-job-align-census.md) |

**验收（C-2d）：** surface 与 yaml 对齐；soft/hard 不变；16× `native=real` 不动；C-2 `[x]`；硬 KPI 不破。

### C-3 · MIR 为唯一可验证后端输入（P1） — `[~]`

DefId / 稳定 identity；MIR verifier；删除或降级 quality heuristic 作为正确性闸。子项：

| 子项 | 状态 | 说明 |
|------|------|------|
| **C-3a** | `[x]` | 禁止 codegen 静默 quality 选同名 MIR body → MIR001 fail-closed；同名 overload 上游 arity 唯一化；census [`c-3a-mir-quality-demote-census`](../docs/census/c-3a-mir-quality-demote-census.md) |
| **C-3b** | `[x]` | 薄 verifier 地板：`verify_mir_cfg_floor` → MIR002（entry / OOB jump·successor / missing terminator）；挂钩 unify + codegen；census [`c-3b-mir-cfg-floor-census`](../docs/census/c-3b-mir-cfg-floor-census.md) |
| **C-3c** | `[x]` | 薄 `FuncId` / `MirFnIndex`（按名→稠密 id）→ MIR003；挂钩 unify + codegen；Call 缺名 fail-closed；census [`c-3c-mir-fn-index-census`](../docs/census/c-3c-mir-fn-index-census.md) |
| **C-3d** | `[x]` | 薄 stack-height 地板：`verify_mir_stack_floor` → MIR004（suite1/pilot 子集；CallHost/非 pilot 放弃该流）；挂钩 unify + codegen（CFG+FnIndex 之后）；census [`c-3d-mir-stack-floor-census`](../docs/census/c-3d-mir-stack-floor-census.md) |
| **C-3e** | `[x]` | 薄 type-tag 地板：`verify_mir_type_floor` → MIR005（Const/算术粗 tag；Add 拼接合法；Nil/非数值 Sub·Mul·Div 拒）；挂钩 stack 之后；census [`c-3e-mir-type-floor-census`](../docs/census/c-3e-mir-type-floor-census.md) |
| **C-3f** | `[x]` | 薄 `JumpIfFalse` 条件 tag：`verify_mir_branch_floor` → MIR006（仅 `Bool|Unknown`；比较/逻辑作 Bool 生产者）；挂钩 type 之后；census [`c-3f-mir-branch-floor-census`](../docs/census/c-3f-mir-branch-floor-census.md) |
| **C-3g** | `[x]` | 薄 `JumpIfNil` 条件 tag：`verify_mir_nil_branch_floor` → MIR007（仅 `Nil|Unknown`；Nil pop / Unknown 留栈）；挂钩 branch 之后；census [`c-3g-mir-nil-branch-floor-census`](../docs/census/c-3g-mir-nil-branch-floor-census.md) |
| **C-3h** | `[x]` | 薄 `ModuleId` / `MirModIndex`（`module_names`→稠密 id）→ MIR008；stem 撞路径 fail-closed；挂钩 nil-branch 之后；census [`c-3h-mir-mod-index-census`](../docs/census/c-3h-mir-mod-index-census.md) |
| **C-3i** | `[x]` | 薄 try/catch 配对地板：`verify_mir_try_floor` → **MIR009**（BeginTry/EndTry 深度；orphan/excess Catch）；挂钩 ModuleId 之后；census [`c-3i-mir-try-floor-census`](../docs/census/c-3i-mir-try-floor-census.md) |
| **C-3j** | `[x]` | 可选 `owner_module` + Def 地板：`verify_mir_def_floor` → **MIR010**（OOB owner；同 `(owner,name)` 冲突；全 None OK）；用户路径 populate；挂钩 try 之后；census [`c-3j-mir-def-floor-census`](../docs/census/c-3j-mir-def-floor-census.md) |
| **C-3k** | `[x]` | 薄 locals 槽 OOB：`verify_mir_local_floor` → **MIR011**（LoadLocal/StoreLocal vs `local_count`）；挂钩 Def 之后；census [`c-3k-mir-local-floor-census`](../docs/census/c-3k-mir-local-floor-census.md) |
| **C-3l** | `[x]` | 薄 globals 槽 OOB：`MirModule.global_count` + `verify_mir_global_floor` → **MIR012**（LoadGlobal/StoreGlobal）；lower populate；挂钩 local 之后；census [`c-3l-mir-global-floor-census`](../docs/census/c-3l-mir-global-floor-census.md) |
| **C-3m** | `[x]` | 薄 CallHost 绝对索引：`verify_mir_call_host_floor` → **MIR013**（`host_fn_name_for_index`；≠ trim dense remap）；挂钩 global 之后；census [`c-3m-mir-call-host-floor-census`](../docs/census/c-3m-mir-call-host-floor-census.md) |
| **C-3n** | `[x]` | 薄 param/local 元数据：`verify_mir_param_local_floor` → **MIR014**（`param_count ≤ local_count`；≠ locals 类型/φ）；挂钩 CallHost 之后；census [`c-3n-mir-param-local-floor-census`](../docs/census/c-3n-mir-param-local-floor-census.md) |
| **C-3o** | `[x]` | 薄 fn-index 字面量 OOB：`verify_mir_fn_lit_floor` → **MIR015**（`CallWitness` / `AwaitSuspend.resume_fn` vs `functions.len()`；≠ ConstFnIndex R-33；≠ CallIndirect arity）；挂钩 param-local 之后；census [`c-3o-mir-fn-lit-floor-census`](../docs/census/c-3o-mir-fn-lit-floor-census.md) |
| **C-3p** | `[x]` | 薄命名 callee 解析：`verify_mir_named_callee_floor` → **MIR016**（Call/CallSuper/ActorCall/ActorSend/MakeClosure/TaskSpawn；≠ ConstFnIndex R-33）；挂钩 fn-lit 之后；census [`c-3p-mir-named-callee-floor-census`](../docs/census/c-3p-mir-named-callee-floor-census.md) |
| **C-3q** | `[x]` | 薄 Build 栈高：`verify_mir_build_stack_floor` → **MIR017**（Array/Dict/SetBuild vs 栈；≠ 改 pilot 全集；≠ MIR004 扩义）；挂钩 named-callee 之后；census [`c-3q-mir-build-stack-floor-census`](../docs/census/c-3q-mir-build-stack-floor-census.md) |
| **C-3r** | `[x]` | 薄 local debug 元数据：`verify_mir_local_debug_floor` → **MIR018**（非空 `local_names`/`local_types` 须 `len == local_count`；空 OK）；挂钩 build-stack 之后；census [`c-3r-mir-local-debug-floor-census`](../docs/census/c-3r-mir-local-debug-floor-census.md) |
| **C-3s** | `[x]` | 薄 aot_export_roots 名解析：`verify_mir_aot_export_floor` → **MIR019**（空 OK；未知 root fail-closed）；挂钩 local-debug 之后；census [`c-3s-mir-aot-export-floor-census`](../docs/census/c-3s-mir-aot-export-floor-census.md) |
| **C-3t** | `[x]` | 薄 init_pc 有界：`verify_mir_init_pc_floor` → **MIR020**（双 0 OK；否则 `body ≤ wrap ≤ instructions.len()`）；挂钩 aot-export 之后；census [`c-3t-mir-init-pc-floor-census`](../docs/census/c-3t-mir-init-pc-floor-census.md) |
| **C-3u** | `[x]` | 薄 MirBlock.id 唯一：`verify_mir_block_id_floor` → **MIR021**（空 blocks OK；重复 id fail-closed）；挂钩 init-pc 之后；census [`c-3u-mir-block-id-floor-census`](../docs/census/c-3u-mir-block-id-floor-census.md) |
| **C-3v** | `[x]` | 薄 flat↔blocks 条数：`verify_mir_flat_blocks_floor` → **MIR022**（任一侧空 OK；双非空须 `sum(block.instr)==instructions.len()`）；挂钩 block-id 之后；census [`c-3v-mir-flat-blocks-floor-census`](../docs/census/c-3v-mir-flat-blocks-floor-census.md) |
| **C-3w** | `[x]` | 薄 successor⊇terminator：`verify_mir_succ_term_floor` → **MIR023**（Jump* 目标须 ∈ successors；Return/Throw 多余边 OK）；挂钩 flat-blocks 之后；census [`c-3w-mir-succ-term-floor-census`](../docs/census/c-3w-mir-succ-term-floor-census.md) |
| **C-3x** | `[x]` | 薄 Pop 栈高：`verify_mir_pop_stack_floor` → **MIR024**（pilot∪Pop；仅 Pop underflow；≠ 改 pilot）；挂钩 succ-term 之后；census [`c-3x-mir-pop-stack-floor-census`](../docs/census/c-3x-mir-pop-stack-floor-census.md) |
| **C-3y** | `[x]` | 薄空函数名：`verify_mir_fn_name_floor` → **MIR025**（`functions[].name` 非空；空模块 OK；≠ `source_name`）；挂钩 pop-stack 之后；census [`c-3y-mir-fn-name-floor-census`](../docs/census/c-3y-mir-fn-name-floor-census.md) |
| **C-3z** | `[x]` | 薄空指令块：`verify_mir_block_instr_floor` → **MIR026**（非空 blocks 每块 ≥1 指令；空 blocks OK；补 MIR002 洞）；挂钩 fn-name 之后；census [`c-3z-mir-block-instr-floor-census`](../docs/census/c-3z-mir-block-instr-floor-census.md) |
| **C-3aa** | `[x]` | 薄 `module_names` 空串：`verify_mir_module_names_floor` → **MIR027**（空表 OK；条目 `""` fail-closed；≠ MIR008/Call(ModuleId)）；挂钩 block-instr 之后；census [`c-3aa-mir-module-names-floor-census`](../docs/census/c-3aa-mir-module-names-floor-census.md) |
| **C-3ab** | `[x]` | 薄 successors 去重：`verify_mir_succ_unique_floor` → **MIR028**（块内无重复目标；空 OK；≠ succ==term）；挂钩 module-names 之后；census [`c-3ab-mir-succ-unique-floor-census`](../docs/census/c-3ab-mir-succ-unique-floor-census.md) |
| **C-3ac** | `[x]` | 薄 aot_export_roots 去重：`verify_mir_aot_export_unique_floor` → **MIR029**（空表 OK；重复名 fail-closed；≠ MIR019 解析）；挂钩 succ-unique 之后；census [`c-3ac-mir-aot-export-unique-floor-census`](../docs/census/c-3ac-mir-aot-export-unique-floor-census.md) |
| **C-3ad** | `[x]` | 薄 local_types 空串槽：`verify_mir_local_types_empty_floor` → **MIR030**（空向量 OK；非空禁 `""`；≠ MIR018 / locals 类型语义）；挂钩 aot-export-unique 之后；census [`c-3ad-mir-local-types-empty-floor-census`](../docs/census/c-3ad-mir-local-types-empty-floor-census.md) |
| **C-3ae** | `[x]` | 薄 local_names 空串槽：`verify_mir_local_names_empty_floor` → **MIR031**（空向量 OK；非空禁 `""`；≠ MIR018 / locals 类型语义）；挂钩 local-types-empty 之后；census [`c-3ae-mir-local-names-empty-floor-census`](../docs/census/c-3ae-mir-local-names-empty-floor-census.md) |
| **C-3af** | `[x]` | 薄 Dup 栈：`verify_mir_dup_stack_floor` → **MIR032**（pilot∪Dup；仅已证空栈 Dup；≠ 改 pilot / 扩 MIR004 / MakeClosure·ToStr）；挂钩 local-names-empty 之后；census [`c-3af-mir-dup-stack-floor-census`](../docs/census/c-3af-mir-dup-stack-floor-census.md) |
| **C-3ag** | `[x]` | 薄 ToStr 栈：`verify_mir_tostr_stack_floor` → **MIR033**（pilot∪ToStr；仅已证空栈 ToStr；≠ 改 pilot / 扩 MIR004 / MakeClosure）；挂钩 dup-stack 之后；census [`c-3ag-mir-tostr-stack-floor-census`](../docs/census/c-3ag-mir-tostr-stack-floor-census.md) |
| **C-3ah** | `[x]` | 薄 Not 栈：`verify_mir_not_stack_floor` → **MIR034**（pilot∪Not；仅已证空栈 Not；≠ 改 pilot / 扩 MIR004 / Neg·MakeClosure）；挂钩 tostr-stack 之后；census [`c-3ah-mir-not-stack-floor-census`](../docs/census/c-3ah-mir-not-stack-floor-census.md) |
| **C-3ai** | `[x]` | 薄 Neg 栈：`verify_mir_neg_stack_floor` → **MIR035**（pilot∪Neg；仅已证空栈 Neg；≠ 改 pilot / 扩 MIR004 / BitNot·MakeClosure）；挂钩 not-stack 之后；census [`c-3ai-mir-neg-stack-floor-census`](../docs/census/c-3ai-mir-neg-stack-floor-census.md) |
| **C-3aj** | `[x]` | 薄 BitNot 栈：`verify_mir_bitnot_stack_floor` → **MIR036**（pilot∪BitNot；仅已证空栈 BitNot；≠ 改 pilot / 扩 MIR004 / NumericCast·MakeClosure）；挂钩 neg-stack 之后；census [`c-3aj-mir-bitnot-stack-floor-census`](../docs/census/c-3aj-mir-bitnot-stack-floor-census.md) |
| **C-3ak** | `[x]` | 薄 NumericCast 栈：`verify_mir_numeric_cast_stack_floor` → **MIR037**（pilot∪NumericCast；仅已证空栈 NumericCast；≠ 改 pilot / 扩 MIR004 / MakeClosure）；挂钩 bitnot-stack 之后；census [`c-3ak-mir-numeric-cast-stack-floor-census`](../docs/census/c-3ak-mir-numeric-cast-stack-floor-census.md) |
| **C-3al** | `[x]` | 薄 MakeClosure 栈：`verify_mir_make_closure_stack_floor` → **MIR038**（pilot∪MakeClosure；Δ=(cap,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / MIR016 名 / BitAnd）；挂钩 numeric-cast 之后；census [`c-3al-mir-make-closure-stack-floor-census`](../docs/census/c-3al-mir-make-closure-stack-floor-census.md) |
| **C-3am** | `[x]` | 薄 BitAnd 栈：`verify_mir_bitand_stack_floor` → **MIR039**（pilot∪BitAnd；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / BitOr·Xor·Shl·Shr）；挂钩 make-closure 之后；census [`c-3am-mir-bitand-stack-floor-census`](../docs/census/c-3am-mir-bitand-stack-floor-census.md) |
| **C-3an** | `[x]` | 薄 BitOr 栈：`verify_mir_bitor_stack_floor` → **MIR040**（pilot∪BitOr；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / BitXor·Shl·Shr）；挂钩 bitand-stack 之后；census [`c-3an-mir-bitor-stack-floor-census`](../docs/census/c-3an-mir-bitor-stack-floor-census.md) |
| **C-3ao** | `[x]` | 薄 BitXor 栈：`verify_mir_bitxor_stack_floor` → **MIR041**（pilot∪BitXor；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / Shl·Shr）；挂钩 bitor-stack 之后；census [`c-3ao-mir-bitxor-stack-floor-census`](../docs/census/c-3ao-mir-bitxor-stack-floor-census.md) |
| **C-3ap** | `[x]` | 薄 Shl 栈：`verify_mir_shl_stack_floor` → **MIR042**（pilot∪Shl；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / Shr）；挂钩 bitxor-stack 之后；census [`c-3ap-mir-shl-stack-floor-census`](../docs/census/c-3ap-mir-shl-stack-floor-census.md) |
| **C-3aq** | `[x]` | 薄 Shr 栈：`verify_mir_shr_stack_floor` → **MIR043**（pilot∪Shr；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / Mod·Eq）；挂钩 shl-stack 之后；census [`c-3aq-mir-shr-stack-floor-census`](../docs/census/c-3aq-mir-shr-stack-floor-census.md) |
| **C-3ar** | `[x]` | 薄 Mod 栈：`verify_mir_mod_stack_floor` → **MIR044**（pilot∪Mod；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / Eq·比较）；挂钩 shr-stack 之后；census [`c-3ar-mir-mod-stack-floor-census`](../docs/census/c-3ar-mir-mod-stack-floor-census.md) |
| **C-3as** | `[x]` | 薄 Eq 栈：`verify_mir_eq_stack_floor` → **MIR045**（pilot∪Eq；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / Ne·Lt·比较）；挂钩 mod-stack 之后；census [`c-3as-mir-eq-stack-floor-census`](../docs/census/c-3as-mir-eq-stack-floor-census.md) |
| **C-3at** | `[x]` | 薄 Ne 栈：`verify_mir_ne_stack_floor` → **MIR046**（pilot∪Ne；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / Lt·比较）；挂钩 eq-stack 之后；census [`c-3at-mir-ne-stack-floor-census`](../docs/census/c-3at-mir-ne-stack-floor-census.md) |
| **C-3au** | `[x]` | 薄 Lt 栈：`verify_mir_lt_stack_floor` → **MIR047**（pilot∪Lt；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / Le·比较）；挂钩 ne-stack 之后；census [`c-3au-mir-lt-stack-floor-census`](../docs/census/c-3au-mir-lt-stack-floor-census.md) |
| **C-3av** | `[x]` | 薄 Le 栈：`verify_mir_le_stack_floor` → **MIR048**（pilot∪Le；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / Gt·比较）；挂钩 lt-stack 之后；census [`c-3av-mir-le-stack-floor-census`](../docs/census/c-3av-mir-le-stack-floor-census.md) |
| **C-3aw** | `[x]` | 薄 Gt 栈：`verify_mir_gt_stack_floor` → **MIR049**（pilot∪Gt；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / Ge·比较）；挂钩 le-stack 之后；census [`c-3aw-mir-gt-stack-floor-census`](../docs/census/c-3aw-mir-gt-stack-floor-census.md) |
| **C-3ax** | `[x]` | 薄 Ge 栈：`verify_mir_ge_stack_floor` → **MIR050**（pilot∪Ge；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / And·逻辑）；挂钩 gt-stack 之后；census [`c-3ax-mir-ge-stack-floor-census`](../docs/census/c-3ax-mir-ge-stack-floor-census.md) |
| **C-3ay** | `[x]` | 薄 And 栈：`verify_mir_and_stack_floor` → **MIR051**（pilot∪And；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / Or）；挂钩 ge-stack 之后；census [`c-3ay-mir-and-stack-floor-census`](../docs/census/c-3ay-mir-and-stack-floor-census.md) |
| **C-3az** | `[x]` | 薄 Or 栈：`verify_mir_or_stack_floor` → **MIR052**（pilot∪Or；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / ConcatStr）；挂钩 and-stack 之后；census [`c-3az-mir-or-stack-floor-census`](../docs/census/c-3az-mir-or-stack-floor-census.md) |
| **C-3ba** | `[x]` | 薄 ConcatStr 栈：`verify_mir_concat_str_stack_floor` → **MIR053**（pilot∪ConcatStr；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004）；挂钩 or-stack 之后；census [`c-3ba-mir-concat-str-stack-floor-census`](../docs/census/c-3ba-mir-concat-str-stack-floor-census.md) |
| **C-3bb** | `[x]` | 薄 ValueEq 栈：`verify_mir_value_eq_stack_floor` → **MIR054**（pilot∪ValueEq；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / ValueNe）；挂钩 concat-str 之后；census [`c-3bb-mir-value-eq-stack-floor-census`](../docs/census/c-3bb-mir-value-eq-stack-floor-census.md) |
| **C-3bc** | `[x]` | 薄 ValueNe 栈：`verify_mir_value_ne_stack_floor` → **MIR055**（pilot∪ValueNe；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / RefEq）；挂钩 value-eq 之后；census [`c-3bc-mir-value-ne-stack-floor-census`](../docs/census/c-3bc-mir-value-ne-stack-floor-census.md) |
| **C-3bd** | `[x]` | 薄 RefEq 栈：`verify_mir_ref_eq_stack_floor` → **MIR056**（pilot∪RefEq；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / RefNe）；挂钩 value-ne 之后；census [`c-3bd-mir-ref-eq-stack-floor-census`](../docs/census/c-3bd-mir-ref-eq-stack-floor-census.md) |
| **C-3be** | `[x]` | 薄 RefNe 栈：`verify_mir_ref_ne_stack_floor` → **MIR057**（pilot∪RefNe；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / ArrayGet）；挂钩 ref-eq 之后；census [`c-3be-mir-ref-ne-stack-floor-census`](../docs/census/c-3be-mir-ref-ne-stack-floor-census.md) |
| **C-3bf** | `[x]` | 薄 ArrayGet 栈：`verify_mir_array_get_stack_floor` → **MIR058**（pilot∪ArrayGet；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / ObjectGet）；挂钩 ref-ne 之后；census [`c-3bf-mir-array-get-stack-floor-census`](../docs/census/c-3bf-mir-array-get-stack-floor-census.md) |
| **C-3bg** | `[x]` | 薄 ObjectGet 栈：`verify_mir_object_get_stack_floor` → **MIR059**（pilot∪ObjectGet；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / ArraySet）；挂钩 array-get 之后；census [`c-3bg-mir-object-get-stack-floor-census`](../docs/census/c-3bg-mir-object-get-stack-floor-census.md) |
| **C-3bh** | `[x]` | 薄 ArraySet 栈：`verify_mir_array_set_stack_floor` → **MIR060**（pilot∪ArraySet；Δ=(3,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / ObjectSet）；挂钩 object-get 之后；census [`c-3bh-mir-array-set-stack-floor-census`](../docs/census/c-3bh-mir-array-set-stack-floor-census.md) |
| **C-3bi** | `[x]` | 薄 ObjectSet 栈：`verify_mir_object_set_stack_floor` → **MIR061**（pilot∪ObjectSet；Δ=(3,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / CallIndirect）；挂钩 array-set 之后；census [`c-3bi-mir-object-set-stack-floor-census`](../docs/census/c-3bi-mir-object-set-stack-floor-census.md) |
| **C-3bj** | `[x]` | 薄 OpaqueValueEq 栈：`verify_mir_opaque_value_eq_stack_floor` → **MIR062**（pilot∪OpaqueValueEq；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / CallWitness 语义）；挂钩 object-set 之后；census [`c-3bj-mir-opaque-value-eq-stack-floor-census`](../docs/census/c-3bj-mir-opaque-value-eq-stack-floor-census.md) |
| **C-3bk** | `[x]` | 薄 StoreGlobal 栈：`verify_mir_store_global_stack_floor` → **MIR063**（pilot∪StoreGlobal；Δ=(1,0)；仅已证高度不足；≠ MIR012 槽 OOB）；挂钩 opaque-eq 之后；census [`c-3bk-mir-store-global-stack-floor-census`](../docs/census/c-3bk-mir-store-global-stack-floor-census.md) |
| **C-3bl** | `[x]` | 薄 ArrSorted 栈：`verify_mir_arr_sorted_stack_floor` → **MIR064**（pilot∪ArrSorted；Δ=(1,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004）；挂钩 store-global 之后；census [`c-3bl-mir-arr-sorted-stack-floor-census`](../docs/census/c-3bl-mir-arr-sorted-stack-floor-census.md) |
| **C-3bm** | `[x]` | 薄 CallIndirect 栈：`verify_mir_call_indirect_stack_floor` → **MIR065**（pilot∪CallIndirect；Δ=(n+1,1)；仅已证高度不足；≠ 完整 arity / 改 pilot / 扩 MIR004）；挂钩 arr-sorted 之后；census [`c-3bm-mir-call-indirect-stack-floor-census`](../docs/census/c-3bm-mir-call-indirect-stack-floor-census.md) |
| **C-3bn** | `[x]` | 薄 ActorRegister 栈：`verify_mir_actor_register_stack_floor` → **MIR066**（pilot∪ActorRegister；Δ=(1,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004）；挂钩 call-indirect 之后；census [`c-3bn-mir-actor-register-stack-floor-census`](../docs/census/c-3bn-mir-actor-register-stack-floor-census.md) |
| **C-3bo** | `[x]` | 薄 MakeIterator 栈：`verify_mir_make_iterator_stack_floor` → **MIR067**（pilot∪MakeIterator；Δ=(1,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004）；挂钩 actor-register 之后；census [`c-3bo-mir-make-iterator-stack-floor-census`](../docs/census/c-3bo-mir-make-iterator-stack-floor-census.md) |
| **C-3bp** | `[x]` | 薄 TaskJoin 栈：`verify_mir_task_join_stack_floor` → **MIR068**（pilot∪TaskJoin；Δ=(1,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004）；挂钩 make-iterator 之后；census [`c-3bp-mir-task-join-stack-floor-census`](../docs/census/c-3bp-mir-task-join-stack-floor-census.md) |
| **C-3bq** | `[x]` | 薄 AwaitSuspend 栈：`verify_mir_await_suspend_stack_floor` → **MIR069**（pilot∪AwaitSuspend；Δ=(1,1)；仅已证高度不足；≠ MIR015）；挂钩 task-join 之后；census [`c-3bq-mir-await-suspend-stack-floor-census`](../docs/census/c-3bq-mir-await-suspend-stack-floor-census.md) |
| **C-3br** | `[x]` | 薄 Throw 栈：`verify_mir_throw_stack_floor` → **MIR070**（pilot∪Throw；Δ=(1,0)；仅已证高度不足；≠ MIR009 / 完整 exception）；挂钩 await-suspend 之后；census [`c-3br-mir-throw-stack-floor-census`](../docs/census/c-3br-mir-throw-stack-floor-census.md) |
| **C-3bs** | `[x]` | 薄 ReturnErr 栈：`verify_mir_return_err_stack_floor` → **MIR071**（pilot∪ReturnErr；Δ=(1,0)；仅已证高度不足；≠ MIR009）；挂钩 throw 之后；census [`c-3bs-mir-return-err-stack-floor-census`](../docs/census/c-3bs-mir-return-err-stack-floor-census.md) |
| **C-3bt** | `[x]` | 薄 ActorSend 栈：`verify_mir_actor_send_stack_floor` → **MIR072**（pilot∪ActorSend；Δ=(n,1) via callee；仅已证高度不足；≠ 完整 arity）；挂钩 return-err 之后；census [`c-3bt-mir-actor-send-stack-floor-census`](../docs/census/c-3bt-mir-actor-send-stack-floor-census.md) |
| **C-3bu** | `[x]` | 薄 TaskSpawn 栈：`verify_mir_task_spawn_stack_floor` → **MIR073**（pilot∪TaskSpawn；Δ=(n,1) via callee；仅已证高度不足；≠ JumpIfNil 高度）；挂钩 actor-send 之后；census [`c-3bu-mir-task-spawn-stack-floor-census`](../docs/census/c-3bu-mir-task-spawn-stack-floor-census.md) |
| **C-3bv** | `[x]` | 薄 JumpIfNil 栈：`verify_mir_jump_if_nil_stack_floor` → **MIR074**（pilot∪JumpIfNil；Δ=(1,0)；仅已证高度不足；≠ MIR007 tag / 路径敏感高度）；挂钩 task-spawn 之后；census [`c-3bv-mir-jump-if-nil-stack-floor-census`](../docs/census/c-3bv-mir-jump-if-nil-stack-floor-census.md) |
| **C-3bw** | `[x]` | 薄 ActorCall 栈：`verify_mir_actor_call_stack_floor` → **MIR075**（pilot∪ActorCall；Δ=(n,1) via callee；仅已证高度不足；≠ MIR016 / 完整 arity）；挂钩 jump-if-nil 之后；census [`c-3bw-mir-actor-call-stack-floor-census`](../docs/census/c-3bw-mir-actor-call-stack-floor-census.md) |
| **C-3bx** | `[x]` | 薄 IteratorNext 栈：`verify_mir_iterator_next_stack_floor` → **MIR076**（pilot∪IteratorNext；Δ=(1,1)；仅已证高度不足；≠ 成功 +3 / 耗尽 +1）；挂钩 actor-call 之后；census [`c-3bx-mir-iterator-next-stack-floor-census`](../docs/census/c-3bx-mir-iterator-next-stack-floor-census.md) |
| **C-3by** | `[x]` | 薄 CallWitness 栈：`verify_mir_call_witness_stack_floor` → **MIR077**（pilot∪CallWitness；Δ=(n,1) via method 索引；仅已证高度不足；≠ MIR015 / 完整 arity）；挂钩 iterator-next 之后；census [`c-3by-mir-call-witness-stack-floor-census`](../docs/census/c-3by-mir-call-witness-stack-floor-census.md) |
| **C-3bz** | `[x]` | 薄 CallSuper 栈：`verify_mir_call_super_stack_floor` → **MIR078**（pilot∪CallSuper；Δ=(n,1) via callee；仅已证高度不足；≠ `_init` self 路径 / MIR016）；挂钩 call-witness 之后；census [`c-3bz-mir-call-super-stack-floor-census`](../docs/census/c-3bz-mir-call-super-stack-floor-census.md) |
| **C-3ca** | `[x]` | 薄 CallClosure 栈：`verify_mir_call_closure_stack_floor` → **MIR079**（pilot∪CallClosure；Δ=(1,1)；仅已证高度不足；≠ VM (1+n,1) 完整 arity）；挂钩 call-super 之后；census [`c-3ca-mir-call-closure-stack-floor-census`](../docs/census/c-3ca-mir-call-closure-stack-floor-census.md) |

**验收（C-3h）：** dup/unknown/stem 撞名 → MIR008；空单文件不破；负向钉绿；硬 KPI 不破；**不**宣称 C-3 `[x]`。全量 MirOp / exception / DefId / Call(ModuleId) IR 继续 park。默认下一 → **C-5c+**。  
**验收（C-3i）：** 失衡 EndTry / 未闭合 BeginTry / orphan·excess Catch → **MIR009**；合法 try/`?`/传播 Throw 不破；硬 KPI 不破；**C-3i `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（P0 Unknown 实参 / 或 DefId 薄窗；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3j）：** OOB / 同 module 重名 owner → **MIR010**；全 `None` 与空单文件不破；硬 KPI 不破；**C-3j `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3k）：** LoadLocal/StoreLocal 槽 ≥ `local_count` → **MIR011**；合法槽与空函数不破；硬 KPI 不破；**C-3k `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3l）：** LoadGlobal/StoreGlobal 槽 ≥ `global_count` → **MIR012**；合法程序与空模块不破；硬 KPI 不破；**C-3l `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3m）：** 不可解析 `CallHost(i)` → **MIR013**；合法绝对索引与无 CallHost 不破；硬 KPI 不破；**C-3m `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3n）：** `param_count > local_count` → **MIR014**；合法元数据与空函数不破；硬 KPI 不破；**C-3n `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3o）：** `CallWitness` / `AwaitSuspend.resume_fn` 索引 ≥ `functions.len()` → **MIR015**；合法字面量与无此类 op 不破；硬 KPI 不破；**C-3o `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3p）：** 未知名 `Call`/`CallSuper`/`ActorCall`/`ActorSend`/`MakeClosure`/`TaskSpawn` → **MIR016**；合法名与无此类 op 不破；ConstFnIndex 缺名路径不破；硬 KPI 不破；**C-3p `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3q）：** Build 立即数导致栈 underflow → **MIR017**；合法 Build 流与无 Build 不破；硬 KPI 不破；**C-3q `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3r）：** 非空 `local_names`/`local_types` 与 `local_count` 不等长 → **MIR018**；空与等长不破；硬 KPI 不破；**C-3r `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3s）：** 未知 `aot_export_roots` 名 → **MIR019**；空与可解析名不破；硬 KPI 不破；**C-3s `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3t）：** `body > wrap` 或 `wrap > instructions.len()` → **MIR020**；双 0 与合法有序不破；硬 KPI 不破；**C-3t `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3u）：** 重复 `MirBlock.id` → **MIR021**；空 blocks 与唯一 id 不破；硬 KPI 不破；**C-3u `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3v）：** 双非空且条数不等 → **MIR022**；单侧空与等长不破；硬 KPI 不破；**C-3v `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3w）：** terminator Jump* 目标 ∉ successors → **MIR023**；空 blocks / 合法 link / Return+多余边不破；硬 KPI 不破；**C-3w `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3x）：** 空栈 `Pop` → **MIR024**；合法 Pop / 无 Pop / 非 trackable abandon 不破；硬 KPI 不破；**C-3x `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3y）：** `name=""` → **MIR025**；合法名 / 空模块不破；硬 KPI 不破；**C-3y `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3z）：** 空指令 `MirBlock` → **MIR026**；空 `blocks` / 合法非空块不破；硬 KPI 不破；**C-3z `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3aa）：** `module_names` 含 `""` → **MIR027**；空表 / 合法名不破；硬 KPI 不破；**C-3aa `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3ab）：** `successors` 含重复目标 → **MIR028**；空 / 互异不破；硬 KPI 不破；**C-3ab `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3ac）：** `aot_export_roots` 含重复名 → **MIR029**；空表 / 互异不破；硬 KPI 不破；**C-3ac `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3ad）：** 非空 `local_types` 含 `""` → **MIR030**；空向量 / 合法哨兵不破；硬 KPI 不破；**C-3ad `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3ae）：** 非空 `local_names` 含 `""` → **MIR031**；空向量 / 合法名不破；硬 KPI 不破；**C-3ae `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3af）：** 已证空栈 `Dup` → **MIR032**；合法 Dup / 无 Dup / abandon 不破；硬 KPI 不破；**C-3af `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3ag）：** 已证空栈 `ToStr` → **MIR033**；合法 ToStr / 无 ToStr / abandon 不破；硬 KPI 不破；**C-3ag `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3ah）：** 已证空栈 `Not` → **MIR034**；合法 Not / 无 Not / abandon 不破；硬 KPI 不破；**C-3ah `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。  
**验收（C-3ai）：** 已证空栈 `Neg` → **MIR035**；合法 Neg / 无 Neg / abandon 不破；硬 KPI 不破；**C-3ai `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3aj）：** 已证空栈 `BitNot` → **MIR036**；合法 BitNot / 无 BitNot / abandon 不破；硬 KPI 不破；**C-3aj `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3ak）：** 已证空栈 `NumericCast` → **MIR037**；合法 NumericCast / 无 NumericCast / abandon 不破；硬 KPI 不破；**C-3ak `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3al）：** 已证高度下 `MakeClosure(_, N)` 且 `height < N` → **MIR038**；cap=0 / 合法 / 无 MakeClosure / abandon 不破；硬 KPI 不破；**C-3al `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3am）：** 已证高度下 `BitAnd` 且 `height < 2` → **MIR039**；合法双值 / 无 BitAnd / abandon 不破；硬 KPI 不破；**C-3am `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3an）：** 已证高度下 `BitOr` 且 `height < 2` → **MIR040**；合法双值 / 无 BitOr / abandon 不破；硬 KPI 不破；**C-3an `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3ao）：** 已证高度下 `BitXor` 且 `height < 2` → **MIR041**；合法双值 / 无 BitXor / abandon 不破；硬 KPI 不破；**C-3ao `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3ap）：** 已证高度下 `Shl` 且 `height < 2` → **MIR042**；合法双值 / 无 Shl / abandon 不破；硬 KPI 不破；**C-3ap `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3aq）：** 已证高度下 `Shr` 且 `height < 2` → **MIR043**；合法双值 / 无 Shr / abandon 不破；硬 KPI 不破；**C-3aq `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3ar）：** 已证高度下 `Mod` 且 `height < 2` → **MIR044**；合法双值 / 无 Mod / abandon 不破；硬 KPI 不破；**C-3ar `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3as）：** 已证高度下 `Eq` 且 `height < 2` → **MIR045**；合法双值 / 无 Eq / abandon 不破；硬 KPI 不破；**C-3as `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3at）：** 已证高度下 `Ne` 且 `height < 2` → **MIR046**；合法双值 / 无 Ne / abandon 不破；硬 KPI 不破；**C-3at `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3au）：** 已证高度下 `Lt` 且 `height < 2` → **MIR047**；合法双值 / 无 Lt / abandon 不破；硬 KPI 不破；**C-3au `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3av）：** 已证高度下 `Le` 且 `height < 2` → **MIR048**；合法双值 / 无 Le / abandon 不破；硬 KPI 不破；**C-3av `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3aw）：** 已证高度下 `Gt` 且 `height < 2` → **MIR049**；合法双值 / 无 Gt / abandon 不破；硬 KPI 不破；**C-3aw `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **C-3ax**（同窗）。
**验收（C-3ax）：** 已证高度下 `Ge` 且 `height < 2` → **MIR050**；合法双值 / 无 Ge / abandon 不破；硬 KPI 不破；**C-3ax `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3ay）：** 已证高度下 `And` 且 `height < 2` → **MIR051**；合法双值 / 无 And / abandon 不破；硬 KPI 不破；**C-3ay `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3az**。
**验收（C-3az）：** 已证高度下 `Or` 且 `height < 2` → **MIR052**；合法双值 / 无 Or / abandon 不破；硬 KPI 不破；**C-3az `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3ba**。
**验收（C-3ba）：** 已证高度下 `ConcatStr` 且 `height < 2` → **MIR053**；合法双值 / 无 ConcatStr / abandon 不破；硬 KPI 不破；**C-3ba `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3bb）：** 已证高度下 `ValueEq` 且 `height < 2` → **MIR054**；合法双值 / 无 ValueEq / abandon 不破；硬 KPI 不破；**C-3bb `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3bc**。
**验收（C-3bc）：** 已证高度下 `ValueNe` 且 `height < 2` → **MIR055**；合法双值 / 无 ValueNe / abandon 不破；硬 KPI 不破；**C-3bc `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3bd**。
**验收（C-3bd）：** 已证高度下 `RefEq` 且 `height < 2` → **MIR056**；合法双值 / 无 RefEq / abandon 不破；硬 KPI 不破；**C-3bd `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3be）：** 已证高度下 `RefNe` 且 `height < 2` → **MIR057**；合法双值 / 无 RefNe / abandon 不破；硬 KPI 不破；**C-3be `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3bf**。
**验收（C-3bf）：** 已证高度下 `ArrayGet` 且 `height < 2` → **MIR058**；合法双值 / 无 ArrayGet / abandon 不破；硬 KPI 不破；**C-3bf `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3bg**。
**验收（C-3bg）：** 已证高度下 `ObjectGet` 且 `height < 2` → **MIR059**；合法双值 / 无 ObjectGet / abandon 不破；硬 KPI 不破；**C-3bg `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3bh）：** 已证高度下 `ArraySet` 且 `height < 3` → **MIR060**；合法三值 / 无 ArraySet / abandon 不破；硬 KPI 不破；**C-3bh `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3bi**。
**验收（C-3bi）：** 已证高度下 `ObjectSet` 且 `height < 3` → **MIR061**；合法三值 / 无 ObjectSet / abandon 不破；硬 KPI 不破；**C-3bi `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3bj）：** 已证高度下 `OpaqueValueEq` 且 `height < 2` → **MIR062**；合法双值 / 无 OpaqueValueEq / abandon 不破；硬 KPI 不破；**C-3bj `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3bk**。
**验收（C-3bk）：** 已证空栈 `StoreGlobal` → **MIR063**；合法写 / 无 StoreGlobal / abandon 不破；硬 KPI 不破；**C-3bk `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3bl**。
**验收（C-3bl）：** 已证空栈 `ArrSorted` → **MIR064**；合法流 / 无 ArrSorted / abandon 不破；硬 KPI 不破；**C-3bl `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3bm）：** 已证高度下 `CallIndirect(n)` 且 `height < n+1` → **MIR065**；合法流 / 无 CallIndirect / abandon 不破；硬 KPI 不破；**C-3bm `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3bn**。
**验收（C-3bn）：** 已证空栈 `ActorRegister` → **MIR066**；合法流 / 无 ActorRegister / abandon 不破；硬 KPI 不破；**C-3bn `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3bo**。
**验收（C-3bo）：** 已证空栈 `MakeIterator` → **MIR067**；合法流 / 无 MakeIterator / abandon 不破；硬 KPI 不破；**C-3bo `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3bp）：** 已证空栈 `TaskJoin` → **MIR068**；合法流 / 无 TaskJoin / abandon 不破；硬 KPI 不破；**C-3bp `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3bq**。
**验收（C-3bq）：** 已证空栈 `AwaitSuspend` → **MIR069**；合法流 / 无 AwaitSuspend / abandon 不破；硬 KPI 不破；**C-3bq `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3br**。
**验收（C-3br）：** 已证空栈 `Throw` → **MIR070**；合法流 / 无 Throw / abandon 不破；硬 KPI 不破；**C-3br `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3bs）：** 已证空栈 `ReturnErr` → **MIR071**；合法流 / 无 ReturnErr / abandon 不破；硬 KPI 不破；**C-3bs `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3bt**。
**验收（C-3bt）：** 已证 `ActorSend` 且 `height < n` → **MIR072**；合法流 / 无 ActorSend / abandon 不破；硬 KPI 不破；**C-3bt `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3bu**。
**验收（C-3bu）：** 已证 `TaskSpawn` 且 `height < n` → **MIR073**；合法流 / 无 TaskSpawn / abandon 不破；硬 KPI 不破；**C-3bu `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3bv）：** 已证空栈 `JumpIfNil` → **MIR074**；合法流 / 无 JumpIfNil / abandon 不破；硬 KPI 不破；**C-3bv `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3bw**。
**验收（C-3bw）：** 已证 `ActorCall` 且 `height < n` → **MIR075**；合法流 / 无 ActorCall / abandon 不破；硬 KPI 不破；**C-3bw `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3bx**。
**验收（C-3bx）：** 已证空栈 `IteratorNext` → **MIR076**；合法流 / 无 IteratorNext / abandon 不破；硬 KPI 不破；**C-3bx `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。
**验收（C-3by）：** 已证 `CallWitness` 且 `height < n` → **MIR077**；合法流 / 无 CallWitness / abandon 不破；硬 KPI 不破；**C-3by `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3bz**。
**验收（C-3bz）：** 已证 `CallSuper` 且 `height < n` → **MIR078**；合法流 / 无 CallSuper / abandon 不破；硬 KPI 不破；**C-3bz `[x]`**；**C-3 仍 `[~]`**。默认下一 → **C-3ca**。
**验收（C-3ca）：** 已证空栈 `CallClosure` → **MIR079**；合法流 / 无 CallClosure / abandon 不破；硬 KPI 不破；**C-3ca `[x]`**；**C-3 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（全量 MirOp 余片 / 或 P0 C-0a+/C-0e；**不**宣称 C-3/C-5 `[x]`）。

### C-4 · Package 默认可复现（P1） — `[x]`

lockfile / git / path 依赖默认钉死；publish 与测试顺序一致；失败路径可诊断。清单 **C-4a…C-4e** 收齐。

| 子项 | 状态 | 说明 |
|------|------|------|
| **C-4a** | `[x]` | Manifest git **强制 `rev`**（缺 → **P043**）；`SYX_INSTALL_LOCKED` 下 Git 从 `syx.lock` 钉死（缺/错 pin → **P014**；不重写 lock）；census [`c-4a-git-rev-pin-census`](../docs/census/c-4a-git-rev-pin-census.md) |
| **C-4b** | `[x]` | 有 `syx.lock` 时 `install` **默认锁定**；`InstallMode::{Auto,Locked,Unlocked}`；CLI `syx install --locked`；`update`/`add` → Unlocked；census [`c-4b-locked-default-census`](../docs/census/c-4b-locked-default-census.md) |
| **C-4c** | `[x]` | `syx publish` 强制可用 `syx.lock`（缺 → **P014**；`enforce_lock_v16`）；CLI `--dry-run` 同闸；census [`c-4c-publish-lock-census`](../docs/census/c-4c-publish-lock-census.md) |
| **C-4d** | `[x]` | `graph_to_lockfile` 默认停写 CDN/farm/billing/marketplace env 快照（保留 schema+enforce）；census [`c-4d-lock-trim-census`](../docs/census/c-4d-lock-trim-census.md) |
| **C-4e** | `[x]` | path 依赖 **content hash**（`PackageSource::Path.checksum`）；Locked 缺/错 → **P014**；census [`c-4e-path-content-hash-census`](../docs/census/c-4e-path-content-hash-census.md) |

**验收（C-4e）：** 新 path lock 含 `sha256:`；Locked 改文件或旧无 checksum → **P014**；Unlocked 可重写；硬 KPI 不破；**C-4 `[x]`**。manifest↔lock 全量对账 / 删 infra 字段仍 park。默认下一 → **C-5**。

### C-5 · 宏 / embed 隔离诚实（P1） — `[~]`

去掉虚假「sandbox」安全承诺，或落地真隔离边界；文档降级到 threat model。**不**宣称 C-5 `[x]`。

| 子项 | 状态 | 说明 |
|------|------|------|
| **C-5a** | `[x]` | threat-model ADR + 公开面假 sandbox 降级；`macro_sandbox_mode` → `seccomp-label-unwired`；census [`c-5a-threat-model-census`](../docs/census/c-5a-threat-model-census.md) |
| **C-5b** | `[x]` | `SYX_EMBED_WASM_SANDBOX=1` 时 wasm 失败/不可用 → `syx_embed_v2_run` **null**（禁静默 in-process）；census [`c-5b-embed-wasm-fail-closed-census`](../docs/census/c-5b-embed-wasm-fail-closed-census.md) |
| **C-5c-1** | `[x]` | embed/macro **timeout 硬闸**：env∪CLI、wasmtime kill、`SYX_MACRO_TIMEOUT_MS`→M040；census [`c-5c-1-embed-macro-timeout-census`](../docs/census/c-5c-1-embed-macro-timeout-census.md) |
| **C-5c-2** | `[x]` | Linux **net-deny seccomp-BPF**（macro child + embed worker）；非 Linux fail-closed；census [`c-5c-2-seccomp-wire-census`](../docs/census/c-5c-2-seccomp-wire-census.md) |
| **C-5c+** | `[~]` | 伞：timeout+netdeny+allowlist+RSS+Seatbelt+permission default-on 子集 done；真全量 syscall GA / C-5 `[x]` 仍 park |
| **C-5d** | `[x]` | Linux **`prlimit(RLIMIT_AS)`** 硬杀；MEMORY env∪CLI；census [`c-5d-embed-rss-hard-census`](../docs/census/c-5d-embed-rss-hard-census.md) |
| **C-5e** | `[x]` | Windows **Job Object `PROCESS_MEMORY_LIMIT`**；census [`c-5e-jobobject-thin-census`](../docs/census/c-5e-jobobject-thin-census.md) |
| **C-5f** | `[x]` | macOS **Seatbelt net-deny**；census [`c-5f-macos-sandbox-thin-census`](../docs/census/c-5f-macos-sandbox-thin-census.md) |
| **C-5g** | `[x]` | Linux **default-deny seccomp allowlist**（v11+补项；`SYX_*_SECCOMP_ALLOWLIST`；与 SECCOMP 互斥）；census [`c-5g-seccomp-allowlist-census`](../docs/census/c-5g-seccomp-allowlist-census.md) |
| **C-5h** | `[x]` | permission-sandbox **default-on** + `--no-permission-sandbox` / `SYX_PERMISSION_SANDBOX=0` opt-out；census [`c-5h-permission-default-on-census`](../docs/census/c-5h-permission-default-on-census.md) |

**验收（C-5b…C-5f）：** 见各子窗；**不**宣称 C-5 `[x]`。  
**验收（C-5g）：** Linux + ALLOWLIST → default-deny 装闸；与 SECCOMP 同开 fail-closed；非 Linux + env → null/M040；net-deny 路径不破；硬 KPI 不破；**不**宣称 C-5 `[x]`。  
**验收（C-5h）：** 默认无 grant → net/fs HostFn **S024**；`--no-permission-sandbox` / `=0` 恢复可编；硬 KPI 不破（allow 注入）；**C-5h `[x]`**；**C-5 仍 `[~]`**（不宣称 `[x]`）。默认下一 → **跨轴**（C-3 全量 MirOp / 或 P0 余债；**不**自动 C-5 `[x]`）。

### C-6 · Version / capability 单一 SSOT（P1） — `[x]`

消除多源漂移；`capability` / banner / crate version 同源。清单 a+b 收齐 → **宣称 C-6 `[x]`**。

| 子项 | 状态 | 说明 |
|------|------|------|
| **C-6a** | `[x]` | Cargo `[workspace.package].version` = 公开 SSOT；对齐 banner / plan/01 / capability `syx_version`；credibility+R-49 跟 `V`；census [`c-6a-version-ssot-census`](../docs/census/c-6a-version-ssot-census.md) |
| **C-6b** | `[x]` | `gen_capabilities.py` / `gen_ir_capabilities.py` 从 Cargo 注入 `syx_version`；TOML 就地改写；`--check` TOML≠`V` fail-closed；去 `"117.0"` 缺省；census [`c-6b-gen-from-cargo-census`](../docs/census/c-6b-gen-from-cargo-census.md) |

**验收（C-6a）：** 公开 banner / capability / plan/01 / manifest = Cargo `V`；旧 0.2.x 反闸已改；硬 KPI 不破。  
**验收（C-6b）：** 改 Cargo 后跑 gen → TOML+generated 跟新 `V`；`--check` 在 TOML/生成物漂移时 fail；无 `117.0` 缺省；硬 KPI 不破；**C-6 `[x]`**。默认下一 → **C-7**。

### C-7 · 测试信号与维护成本（P2） — `[~]`

矩阵分 shard；fuzz/benchmark 真失败信号；巨型模块拆分；实验 scaffold 移出 stable。**不**宣称 C-7 `[x]`。

| 子项 | 状态 | 说明 |
|------|------|------|
| **C-7a** | `[x]` | `syx_smoke` 经 `syx-lexer` 真消费；required CI `fuzz-smoke` `-runs=1`；UsageGuide CI met 对齐；census [`c-7a-fuzz-honesty-census`](../docs/census/c-7a-fuzz-honesty-census.md) |
| **C-7b** | `[x]` | soft job 内吞错 `exit 0` → fail-closed（job 仍 soft）；白名单：空 plugin / 无 book JSON / domain notice；census [`c-7b-soft-exit0-census`](../docs/census/c-7b-soft-exit0-census.md) |
| **C-7c** | `[x]` | count-gate SSOT **2600/1700**：脚本默认 = CI = nightly；活公开面 LSP ≥1700；census [`c-7c-count-gate-census`](../docs/census/c-7c-count-gate-census.md) |
| **C-7c+** | `[-]` park | Farm/CDN scaffold 外移；多 fuzz target；soft→required |

**验收（C-7a）：** target 消费输入；`ci.yml` 含 `cargo fuzz` + `syx_smoke` + `-runs=1`；UsageGuide「CI met」与 yaml 一致；硬 KPI 不破。  
**验收（C-7b）：** gui/jit/extended/native-shake+diff/browser/windows-plugin/book-rate 测败传播真实 exit；白名单三处仍 `exit 0`；不升 required；硬 KPI 不破。  
**验收（C-7c）：** 脚本/CI/nightly 同为 2600/1700；活公开面无 LSP ≥3050；anti-delete only；硬 KPI 不破；**不**宣称 C-7 `[x]`（C-7c+ park）。默认下一 → **C-8**。

### C-8 · 反馈质量 / 文档诚实（P2–P3） — `[~]`

formatter CST lossless；Ch15 隐式 `super.init` MD 诚实；语言书与 parser 分叉清单。**不**宣称 C-8 `[x]`。

| 子项 | 状态 | 说明 |
|------|------|------|
| **C-8a** | `[x]` | Ch15 en+zh：省略 `super.init` = U-38 字段垫 **≠ CallSuper**；真隐式 CallSuper **未实现**；census [`c-8a-ch15-implicit-super-md-census`](../docs/census/c-8a-ch15-implicit-super-md-census.md) |
| **C-8b** | `[x]` | format 默认拒 CST001→AST 丢注释写盘（`F003`）；显式 `--allow-lossy` 才放行；LSP 硬拒；census [`c-8b-fmt-refuse-lossy-census`](../docs/census/c-8b-fmt-refuse-lossy-census.md) |
| **C-8c** | `[x]` | 语言书 ↔ parser 分叉活清单开账（≥20 行 + class）；census [`c-8c-book-parser-fork-census`](../docs/census/c-8c-book-parser-fork-census.md) |
| **C-8d** | `[x]` | 清 F20–F23 `doc_debt`/`book_ahead` en+zh MD 诚实对齐；census [`c-8d-book-md-honesty-census`](../docs/census/c-8d-book-md-honesty-census.md) |
| **C-8e** | `[x]` | F18/F19 Ch10 en+zh Compatibility 注；inventory 无活 `legacy_accept`；census [`c-8e-legacy-compat-notes-census`](../docs/census/c-8e-legacy-compat-notes-census.md) |
| **C-8f** | `[x]` | UsageGuide en+zh：refuse-lossy **≠** format lossless GA；census [`c-8f-cst-residual-note-census`](../docs/census/c-8f-cst-residual-note-census.md) |

**验收（C-8a）：** en+zh 无「compiler inserts / 自动插入」现行宣称；有未实现 / `≠ CallSuper`；薄钉绿；硬 KPI 不破；**不**宣称 C-8 `[x]`。  
**验收（C-8b）：** 默认 write/`--check` 在 CST001→AST 时 fail-closed（`F003`）；`--allow-lossy` 显式放行；LSP 拒 lossy；UsageGuide 对齐；硬 KPI 不破；**不**宣称 C-8 `[x]`。  
**验收（C-8c）：** 活清单 ≥20 行且 class/evidence 完整；开账时四 class 均出现；薄钉 + census 绿；硬 KPI 不破；**不**宣称 C-8 `[x]` / 分叉已收敛。  
**验收（C-8d）：** F20–F23 en+zh 状态词一致 + Honesty；inventory 无活 `doc_debt`/`book_ahead`；硬 KPI 不破；**不**宣称 C-8 `[x]`。  
**验收（C-8e）：** Ch10 en+zh Compatibility（C-8e）×2；inventory 无活 `legacy_accept`；硬 KPI 不破；**不**宣称 C-8 `[x]` / 去 parser legacy。  
**验收（C-8f）：** UsageGuide en+zh Honesty ≠ lossless GA；薄钉绿；硬 KPI 不破；**不**宣称 C-8 `[x]` / CST 扩面。Soft park：CST lossless 实现 · 真 CallSuper · 去 parser legacy。

## 3. 显式 park（非默认下一）

| 项 | 原因 |
|----|------|
| U-41+ 新语法表面 | 收敛期优先 soundness；表面增长冻结除非 unblock P0 |
| **C-0e** integer narrowing / 书 S008 | 与实现 S008（actor）冲突；独立窗 |
| Ch15 隐式 `super.init` / 真 `CallSuper` | 厚语义；需独立 ADR |
| CST postfix 深刀 / codegen Break | 高成本；非 P0 |
| JIT Suite-1 扩面 | pilot only |
| simd Object wipe / residual 降 | 易破 KPI；禁默认 |
| SyxHost / HostFn+1 | 伞与膨胀风险 |
| 产品伞 GA（Farm / SynUI / marketplace） | 冻结至 ADR |
| 官方模块名 `Foundation` | 永久禁 |

## 4. 窗口工作法

1. **只开一窗**；编号 `C-N`（或报告批准的子项 `C-Na`）。  
2. 先写负向/支持矩阵验收，再改实现。  
3. 退出：硬 KPI + 本窗 census；更新本文件勾选与 `01` 一句。  
4. 厚项 → park + ADR，不塞进默认下一。  
5. 施工草稿可落 `.cursor/plans/`；**勾选 SSOT = 本文件**。

## 5. 与外部报告映射

| 报告优先级 | 收敛刀 |
|------------|--------|
| P0 语义 soundness | **C-0** |
| P0 ManagedHeap | **C-1** |
| P0 Native/CI 矩阵 | **C-2** |
| P1 MIR / lock / sandbox / version | **C-3…C-6** |
| P2/P3 测试·文档·维护 | **C-7…C-8** |

报告全文只读；**活勾选只在本文件**。

## 6. 上期账本指针

完整 SA / R / U 条目、验收句与历史「默认下一」叙事：

→ [`archive/02-improvement-roadmap-sa-r-u-ledger.md`](archive/02-improvement-roadmap-sa-r-u-ledger.md)

勿在收敛期重新打开 R-50 或把 U-41 设为默认下一，除非 `01` 显式改成熟度并 ADR 解锁。
