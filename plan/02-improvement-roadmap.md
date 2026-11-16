# SYX 改进增强任务清单（Swift 标准库对齐期 · SA）

> **原则：** 以 [`todo-003.md`](todo-003.md) 为分析底稿；SA-6 后复评残余以 [`todo-005.md`](todo-005.md) 为对照、本表 R-30+ 为活账本。**允许破坏性调整**（无兼容窗口、不保活旧模块名/旧 API）。重大方向先 ADR，再改本表。  
> **状态：** `[ ]` 未开始 · `[~]` 进行中 · `[x]` 完成 · `[-]` 取消/延后  
> **优先级：** P0 阻断对齐 · P1 重组核心 · P2 文档工具 · P3 收口增强  
> **活状态：** [`01-project-status.md`](01-project-status.md)  
> **上期冻结账本：** [`archive/02-improvement-roadmap-credibility-stage-ledger.md`](archive/02-improvement-roadmap-credibility-stage-ledger.md)  
> **最近更新：** 2026-11-16 · **U-16 逗号 `guard let` 已退出**；编号账本（SA-0…R-49）仍收口（无 R-50）；U-0…U-16 账面退出；**诚实：** U-12/U-13 曾纸面，由 U-15/U-14 复落；U-16 对齐书宣称 `guard let …, … else`；残余 = park / 未编号债（JIT Suite-1、simd 159、codegen Break、CST postfix、逗号 while let 等）。  
> **对照说明：** `todo-005` 写于提交 `82f207a` / SA-6 附近；其后 **R-0…R-49** 已消化 Object 切片、wasm 46/46、browser、closure/`super.`、`try` 跨函数 catch、struct/Array 赋值隔离、`Int(String)` Optional、自定义 Iterator for-in、`Array<Shape>` witness、书写阻断三件套、诊断质量、CI fmt/soft-fail、书籍 opt-in `sy,output`、full-native hello 链接可跑、冷启动诚实复测（~2.5–3.1 s）、OrderedSet Sendable façade（inventory 180/10）、simd Object 章程（residual 159 禁 wipe）、擦除基类引用虚分派（embed+wasm `woof`）、Int8 wrap、user Comparable `sorted` 顺序钉、Ch17 Error throw/`catch`/`if !x` 中英 fence 同步、System `Env` enum + `FilePath` struct 值类型 + gen-only 边界、credibility MD-only claim 链瘦身、docs **0.2.0** + ADR `current.md` 短表 + scoped 断链门禁；**U-0** 修绿 `FilePath.joining`；**U-1…U-3** 再删纯 MD claim；**U-4** parser 双接受 `func…async`；**U-5** 清扫余 4 纯 MD；**U-6** parser 双接受 `catch let`；**U-7** MIR `repeat`/`break`/`continue`；**U-8** `if let`/`if case` 关 trailing-closure；**U-9** `parse_block_stmts` 可选 `;`；**U-10** `while let` → While+IfLet+Break；**U-11** `RepeatWhile.label` + MIR loop_stack；**U-12** `while case`（账面纸面；由 U-15 复落）；**U-13** 逗号 `if let`（账面纸面；由 U-14 复落）；**U-14** 复落逗号 `if let`；**U-15** 复落 `while case`；**U-16** 逗号 `guard let` → Seq of GuardLet[+If]。下表只保留 **仍可复现** 或仍挂账的项。

## 进度摘要（本期）

| 阶段 | 主题 | 状态 | 下一步 |
|------|------|------|--------|
| **SA-0…SA-6** | 标准库 Swift 对齐期 | `[x]` 退出 | — |
| **R-0…R-15** | Object 公开面切片 + 并发/测试债 | `[x]` 退出 | — |
| **R-16…R-19** | wasm emit W001 专项 | 未交付，已结案 | 挂账见 [`r-20-known-gaps`](../docs/census/r-20-known-gaps.md) |
| **R-20…R-26** | dataclass ops / path parity / W001→wasm 46/46 / closure | `[x]` 退出 | census r-20…r-26 |
| **R-27** | Collections Array/Dict/Set Object→Sendable（186/11） | `[x]` 退出 | [`r-27`](../docs/census/r-27-collections-object-census.md) |
| **R-28** | Browser 套件收口（22→0） | `[x]` 退出 | [`r-28`](../docs/census/r-28-browser-red-census.md) |
| **R-29** | Lang 诚实钉 + MIR `super.method`→CallSuper | `[x]` 退出 | [`r-29`](../docs/census/r-29-lang-super-census.md) |
| **R-30** | 错误跨函数传播（`do { try g() } catch`） | `[x]` 退出 | [`r-30`](../docs/census/r-30-throw-prop-census.md) |
| **R-31** | 值语义（struct 拷贝 + Array/Dict/Set uniquify） | `[x]` 退出 | [`r-31`](../docs/census/r-31-value-semantics-census.md) |
| **R-32** | `Int(String)` → `Int?` + 值打印 | `[x]` 退出 | [`r-32`](../docs/census/r-32-int-from-string-census.md) |
| **R-33** | 自定义 Iterator `for-in` witness | `[x]` 退出 | [`r-33`](../docs/census/r-33-iterator-witness-census.md) |
| **R-34** | 存在类型 `Array<Shape>` 协议分派 | `[x]` 退出 | [`r-34`](../docs/census/r-34-existential-shape-census.md) |
| **R-35** | 书写阻断三件套 | `[x]` 退出 | [`r-35`](../docs/census/r-35-writing-blockers-census.md) |
| **R-36** | 诊断质量 | `[x]` 退出 | [`r-36`](../docs/census/r-36-diagnostics-census.md) |
| **R-37** | CI 恢复效力 | `[x]` 退出 | [`r-37`](../docs/census/r-37-ci-census.md) |
| **R-38** | 书籍门禁 opt-in run+stdout | `[x]` 退出 | [`r-38`](../docs/census/r-38-book-run-census.md) |
| **R-39** | full-native 链接 fail-closed | `[x]` 退出 | [`r-39`](../docs/census/r-39-native-link-census.md) |
| **R-40** | 冷启动 / 冷执行路径诚实 | `[x]` 退出 | [`r-40`](../docs/census/r-40-coldstart-census.md) |
| **R-41** | OrderedSet Object-shell | `[x]` 退出 | [`r-41`](../docs/census/r-41-orderedset-census.md) |
| **R-42** | simd Object 诚实章程 | `[x]` 退出 | [`r-42`](../docs/census/r-42-simd-object-census.md) |
| **R-43** | polymorphic_base_ref / vtable | `[x]` 退出 | [`r-43`](../docs/census/r-43-poly-base-ref-census.md) |
| **R-44** | Int8 contextual wrap | `[x]` 退出 | [`r-44`](../docs/census/r-44-int8-wrap-census.md) |
| **R-45** | user Comparable sort | `[x]` 退出 | [`r-45`](../docs/census/r-45-comparable-sort-census.md) |
| **R-46** | books Ch17 Error 表面 | `[x]` 退出 | [`r-46`](../docs/census/r-46-books-ch17-census.md) |
| **R-47** | System Env / FilePath 包 API | `[x]` 退出 | [`r-47`](../docs/census/r-47-package-api-census.md) |
| **R-48** | credibility 文本断言瘦身 | `[x]` 退出 | [`r-48`](../docs/census/r-48-credibility-diet-census.md) |
| **R-49** | 文档断链 / ADR 现行集 | `[x]` 退出 | [`r-49`](../docs/census/r-49-docs-adr-census.md) |
| **编号账本** | SA-0…R-49 | **收口** | 无 R-50；残余 park / 未编号债 |
| **U-0** | FilePath joining 返回类型 | `[x]` 退出 | [`u-0`](../docs/census/u-0-filepath-joining-census.md) |
| **U-1** | credibility 纯 MD claim 再瘦身 | `[x]` 退出 | [`u-1`](../docs/census/u-1-credibility-diet-census.md) |
| **U-2** | Stage-3 credibility 纯 MD claim | `[x]` 退出 | [`u-2`](../docs/census/u-2-credibility-diet-census.md) |
| **U-3** | Stage-4 credibility 纯 MD claim | `[x]` 退出 | [`u-3`](../docs/census/u-3-credibility-diet-census.md) |
| **U-4** | `func…async` 双接受 | `[x]` 退出 | [`u-4`](../docs/census/u-4-func-async-census.md) |
| **U-5** | credibility 余 4 纯 MD 清扫 | `[x]` 退出 | [`u-5`](../docs/census/u-5-credibility-remnant-census.md) |
| **U-6** | `catch let` 双接受 | `[x]` 退出 | [`u-6`](../docs/census/u-6-catch-let-census.md) |
| **U-7** | MIR 循环控制 | `[x]` 退出 | [`u-7`](../docs/census/u-7-loop-control-census.md) |
| **U-8** | `if let`/`if case` trailing | `[x]` 退出 | [`u-8`](../docs/census/u-8-if-let-trailing-census.md) |
| **U-9** | 块内语句可选分号 | `[x]` 退出 | [`u-9`](../docs/census/u-9-semicolon-census.md) |
| **U-10** | `while let` 有界解糖 | `[x]` 退出 | [`u-10`](../docs/census/u-10-while-let-census.md) |
| **U-11** | 标签 `repeat`/`break outer` | `[x]` 退出 | [`u-11`](../docs/census/u-11-repeat-label-census.md) |
| **U-12** | `while case` 有界解糖 | `[x]` 账面纸面 | 由 [`u-15`](../docs/census/u-15-while-case-census.md) 复落 |
| **U-13** | 逗号 `if let` 嵌套解糖 | `[x]` 账面纸面 | 由 [`u-14`](../docs/census/u-14-if-let-comma-census.md) 复落 |
| **U-14** | 复落逗号 `if let` | `[x]` 退出 | [`u-14`](../docs/census/u-14-if-let-comma-census.md) |
| **U-15** | 复落 `while case` | `[x]` 退出 | [`u-15`](../docs/census/u-15-while-case-census.md) |
| **U-16** | 逗号 `guard let` | `[x]` 退出 | [`u-16`](../docs/census/u-16-guard-let-comma-census.md) |

```mermaid
flowchart LR
  sa6["SA-6 exited"] --> r0r49["R-0 to R-49 exited"]
  r0r49 --> closed["numbered ledger closed"]
  closed --> u0["U-0 FilePath joining exited"]
  u0 --> u1["U-1 credibility MD diet exited"]
  u1 --> u2["U-2 Stage3 claim diet exited"]
  u2 --> u3["U-3 Stage4 claim diet exited"]
  u3 --> u4["U-4 funcAsync dual exited"]
  u4 --> u5["U-5 remnant MD exited"]
  u5 --> u6["U-6 catchLet dual exited"]
  u6 --> u7["U-7 MIR loopControl exited"]
  u7 --> u8["U-8 ifLet trailing exited"]
  u8 --> u9["U-9 semicolon exited"]
  u9 --> u10["U-10 whileLet exited"]
  u10 --> u11["U-11 repeatLabel exited"]
  u11 --> u12["U-12 whileCase paper"]
  u12 --> u13["U-13 ifLetComma paper"]
  u13 --> u14["U-14 restore ifLetComma"]
  u14 --> u15["U-15 restore whileCase"]
  u15 --> u16["U-16 guardLetComma"]
  u16 --> later["park residual"]
```

---

## todo-005 对照 · 已消化 vs 仍开放（2026-11-12）

底稿：[`todo-005.md`](todo-005.md)（SA-6 后复评）。**原则：** 每条开放项须能复现；修绿靠失败测试驱动，禁止只改 census 叙事。

### 已消化（相对 todo-005 基线）

| todo-005 议题 | 现状 |
|---------------|------|
| Wasm `hello` `[W001]` / 生成期栈不平衡 | **R-23…R-25** → `wasm` **46/46**；`syx run --wasm examples/hello.sy` 打印 Hello |
| 公开 API 大量 `Object` | **R-0…R-13** + **R-27** → inventory **186/11**（simd 159 parked） |
| Browser ~22 红 | **R-28** → **46/0/2** |
| Swift 闭包语句体 P001 / tutorial | **R-26** |
| `struct`/`data class` `==` 缺失 | **R-20** |
| embedded vs 文件路径分歧 | **R-21** |
| `super.method` MIR 洞 / phase 假钉 | **R-29** |
| 无 `full-native` 仍「伪构建」原生 | 现构建 **L006 fail-closed**（诚实；≠ full-native 链接全绿） |
| `let` + `append` 静默 | 早已 **S060**（R-29 element 测改 `var`） |
| 错误跨函数 `do { try g() } catch` | **R-30** → 进 `catch`（SEM-THROW 清） |
| struct / Array 赋值共享突变 | **R-31** → 赋值隔离（SEM-STRUCT / SEM-ARRAY 清） |
| `Int("42")` → `Int {}` / `+1`→`1` | **R-32** → `42` / `43` / `nil`（SEM-INTSTR 清） |
| 自定义 Iterator `for-in` R003 928 | **R-33** → 文件路径求和 `6`（SEM-ITER 清） |
| `Array<Shape>` / `s.area()` 静默 `0` / 误 S014 | **R-34** → total≈`21.56636`；非符合 S014（SEM-EXIST 清） |
| 书籍门禁仅 `check` / 无 stdout 钉 | **R-38** → opt-in `sy,output` + `// prints:`；en/zh **12/12**；冷启动 ≈2.6 s → 不全书默认 run |
| full-native Linux 错 lld / `-Wl,` / 伪 Built native | **R-39** → Linux cc-first；禁 `ld64.lld`；`-Wl,` 规范化；hello 可跑 |
| Stage-3 `hello_cold≈0.08` / `&lt;0.15` 过时 claim | **R-40** → 干净目录复测 ~2.5–3.1 s；census/ADR **superseded**；embed HIT ≠ E1 `&lt;0.3` |
| OrderedSet 公开 Object-shell | **R-41** → `OrderedSet`/`Sendable`；inventory **180/10**；≠ `OrderedSet&lt;T&gt;` GA |
| simd Object 159 无契约 wipe | **R-42** → 分簇 census + ADR 动刀门槛；residual **仍 159**（禁伪装降库存） |
| 擦除基类引用虚分派（wasm 静默 `animal`） | **R-43** → `__vtable_*`+`CallIndirect`；embed+wasm `woof`；≠ 全继承 GA |
| Int8 `<<` / compound `+=`/`<<=` 抬宽不 wrap | **R-44** → 左操作数宽度 + 定宽 compound；`-128`/`-126` |
| user Comparable `sorted` → `2/2/2` | **R-45** → StoreLocal 卫生 + ObjectRef witness；顺序 `1/2/3` |

### 仍可复现（探针 2026-11-12；SEM… / poly / Int8 / Comparable sort 已清）

| ID | 现象 | 期望 | 实测 |
|----|------|------|------|
| **OBJ-SIMD** | 公开 Object inventory | 契约后切片下降 | **180/10**（simd **159**；R-42 charter only） |
| **LANG-POLY** | JIT Suite-1 differential | 分簇翻绿 | R-45 清 Comparable；JIT **park** |

---

## 阶段 R-30+ · 开放账本（按优先级分窗）

**默认施工序：** 语义正确性（todo-005 §4.2 / 阶段 B）→ 原生/CI 工程债 → Object/simd → 书与包 API → 治理瘦身。  
**硬闸（每窗）：** wasm **46/46**；browser failed=0；Object **≤180/10**（除非本窗明确切片）；不 `#[ignore]` 伪装绿；fail-closed validate 不降级。

### P0 · 语言行为正确性（最高优先）

| ID | 任务 | 状态 | 验收（摘要） | 来源 |
|----|------|------|--------------|------|
| **R-30** | **错误跨函数传播** | `[x]` | `try g()` 进 `catch`；VM+MIR 回归；census [`r-30`](../docs/census/r-30-throw-prop-census.md) | todo-005 B3 / SEM-THROW |
| **R-31** | **值语义：struct 拷贝 + Array/Dict 写时复制** | `[x]` | struct/Array/Dict/Set 赋值隔离；census [`r-31`](../docs/census/r-31-value-semantics-census.md) | todo-005 B4 / SEM-STRUCT·ARRAY |
| **R-32** | **`Int(String)` → `Int?` + 值打印** | `[x]` | `Int("42")`→`42`；`+1`→`43`；`Int("abc")`→`nil`；census [`r-32`](../docs/census/r-32-int-from-string-census.md) | todo-005 B5 / SEM-INTSTR |
| **R-33** | **自定义 Iterator `for-in` witness** | `[x]` | 文件路径 Counter 求和 `6`；ConstFnIndex+trim；census [`r-33`](../docs/census/r-33-iterator-witness-census.md) | todo-005 B6 / SEM-ITER / S4-04 residual |
| **R-34** | **存在类型数组 / 协议分派** | `[x]` | `Array<Shape>` witness 求和；S014 诚实；census [`r-34`](../docs/census/r-34-existential-shape-census.md) | todo-005 B6 #8 / SEM-EXIST |
| **R-35** | **书写阻断三件套** | `[x]` | 隐式 `self` 复合赋值；闭包可变捕获写回；`async let` 无假 S020；census [`r-35`](../docs/census/r-35-writing-blockers-census.md) | todo-005 B7 |
| **R-36** | **诊断质量** | `[x]` | 禁 `expected Unknown, found Unknown`；MIR001/R003/C020 带 Span；Error 打印 `E.bad` 非内部 bag；census [`r-36`](../docs/census/r-36-diagnostics-census.md) | todo-005 B8 / §4.7 |

### P1 · 工程与后端

| ID | 任务 | 状态 | 验收（摘要） | 来源 |
|----|------|------|--------------|------|
| **R-37** | **CI 恢复效力** | `[x]` | `cargo fmt` 过关；不可修 job `continue-on-error`+能力表；census [`r-37`](../docs/census/r-37-ci-census.md) | todo-005 A1–A5 |
| **R-38** | **书籍门禁升级 `run`+stdout** | `[x]` | opt-in `sy,output`/`// prints:`；SEM 书内+独立；冷启动先评估；census [`r-38`](../docs/census/r-38-book-run-census.md) | todo-005 B1–B2 / E1 制约 |
| **R-39** | **full-native 链接 fail-closed** | `[x]` | Linux cc-first；禁 ld64；`-Wl,` 规范化；hello 可跑；census [`r-39`](../docs/census/r-39-native-link-census.md) | todo-005 C1–C3 / §4.3 |
| **R-40** | **冷启动 / 冷执行路径诚实** | `[x]` | 干净目录 hello/`fib(30)` 冷热复测；Stage-3 `&lt;0.15` superseded；embed HIT 仍 ~2.5–3 s；census [`r-40`](../docs/census/r-40-coldstart-census.md) | todo-005 E1–E2 / §4.5 |

### P1 · Object / lang 红态续窗

| ID | 任务 | 状态 | 验收（摘要） | 来源 |
|----|------|------|--------------|------|
| **R-41** | **OrderedSet Object-shell→Sendable/Dictionary** | `[x]` | inventory −6→**180/10**；显式切片（≠ simd/json）；≠ `OrderedSet<T>` GA；census [`r-41`](../docs/census/r-41-orderedset-census.md) | R-27 park / todo-005 F4 |
| **R-42** | **simd Object 159 诚实计划** | `[x]` | 分簇 census + ADR 动刀门槛；residual **≥159** 禁 wipe；census [`r-42`](../docs/census/r-42-simd-object-census.md) · [ADR](../docs/adr/r-42-simd-object-charter.md) | R-27/R-29 park |
| **R-43** | **`polymorphic_base_ref` / vtable** | `[x]` | 擦除 `Animal` embed+wasm `woof`；调用点 `__vtable_*`+`CallIndirect`；≠ 全继承 GA；census [`r-43`](../docs/census/r-43-poly-base-ref-census.md) | R-29 still-red |
| **R-44** | **Int8 contextual wrap**（原三簇之 Int8 分簇） | `[x]` | `<<`/`+=`/`<<=` Int8 wrap；census [`r-44`](../docs/census/r-44-int8-wrap-census.md)；≠ Comparable/JIT | R-29 still-red |
| **R-45** | **user Comparable sort** | `[x]` | `struct_with_compare_witness_sorted` 顺序 `1/2/3`；StoreLocal+ObjectRef witness；census [`r-45`](../docs/census/r-45-comparable-sort-census.md)；≠ JIT | R-29 still-red |

### P2 · Swift 对齐与文档治理

| ID | 任务 | 状态 | 验收（摘要） | 来源 |
|----|------|------|--------------|------|
| **R-46** | **书籍表面 Swift 化 · Ch17 Error**（原 R-45） | `[x]` | Ch17 en+zh：`Error` throw / `catch` / `if !x`；fence 反模式 residual **0**；en↔zh sync；`sy,output` **12/12**；census [`r-46`](../docs/census/r-46-books-ch17-census.md)；≠ `func…async` / 全书 F1 | todo-005 F1 / F5 |
| **R-47** | **包 API 形态 · System Env/FilePath**（原 R-46） | `[x]` | `Env` caseless enum；`FilePath` struct 值类型 + String 静态桥；F3 gen-only README；Struct inject；census [`r-47`](../docs/census/r-47-package-api-census.md)；≠ 删树 / HTTPClient / SyxHost 全擦 | todo-005 F2–F3 |
| **R-48** | **credibility 文本断言瘦身**（原 R-47） | `[x]` | 删 sequence_collection / stdlib_cache / structured_concurrency MD-only claim 链（**8**）；S4-04→R-33 lang Iterator；S3-03→R-40 coldstart；census [`r-48`](../docs/census/r-48-credibility-diet-census.md)；≠ D3 / 全清 168 | todo-005 D1–D2 |
| **R-49** | **文档断链 / ADR 现行集**（原 R-48） | `[x]` | `docs/README`(+zh)/books → **0.2.0**；[`adr/current.md`](../docs/adr/current.md) 短表 **15**；scoped 入口+UsageGuide 断链 **0**；census [`r-49`](../docs/census/r-49-docs-adr-census.md)；≠ 274 ADR 全合并 / ≠ 724 清零 | todo-005 D4–D6 |

### 未编号残余（U-*）

| ID | 任务 | 状态 | 验收（摘要） | 来源 |
|----|------|------|--------------|------|
| **U-0** | **FilePath joining 返回类型**（R-47 非声明） | `[x]` | MIR Struct 方法返回登记；语义前 Struct inject；零参字段 `ObjectGet`；chain + 再赋值绿；census [`u-0`](../docs/census/u-0-filepath-joining-census.md)；≠ R-50 / JIT / `func…async` / SyxHost 全擦 | R-47 residual |
| **U-1** | **credibility 纯 MD claim 再瘦身**（R-48 续） | `[x]` | 删 9 纯 MD 模块 + `mod` residual 0；保留 `cst_lower_probe`（Parser 钉）；census [`u-1`](../docs/census/u-1-credibility-diet-census.md)；≠ D1 全清 / D3 / R-50 / JIT | R-48 residual / todo-005 D1 |
| **U-2** | **Stage-3 credibility 纯 MD claim**（init_template / lint_ast / vm_hotspot） | `[x]` | 删 9 纯 MD + `mod` residual 0；保留 prep + `cst_lower`；census [`u-2`](../docs/census/u-2-credibility-diet-census.md)；≠ D1 全清 / D3 / R-50 / JIT / `func…async` | U-1 residual / todo-005 D1 |
| **U-3** | **Stage-4 credibility 纯 MD claim**（stdlib_stability / package_registry+hinge / ecosystem_exit） | `[x]` | 删 9 纯 MD + `mod` residual 0；census [`u-3`](../docs/census/u-3-credibility-diet-census.md)；≠ D1 全清 / D3 / R-50 / JIT / `func…async` | U-2 residual / todo-005 D1 |
| **U-4** | **`func…async` 双接受**（parser 薄刀） | `[x]` | `)` 后可选 `async`；prefix `async func` 不破；census [`u-4`](../docs/census/u-4-func-async-census.md)；≠ 全书改写 / CST / R-50 / JIT | R-46 park / todo-005 |
| **U-5** | **credibility 余 4 纯 MD 清扫** | `[x]` | 删 `dx_exit_open` + lsp_book 三件；保留 `lsp_desugar`；census [`u-5`](../docs/census/u-5-credibility-remnant-census.md)；≠ D3 / R-50 / JIT | U-4 next / todo-005 D1 |
| **U-6** | **`catch let` 双接受**（parser 薄刀） | `[x]` | `Catch` 后可选 `Let`；≡ `catch e`；census [`u-6`](../docs/census/u-6-catch-let-census.md)；≠ 多 catch / CST / fmt / Ch17 fence / R-50 / JIT | U-5 next / todo-005 F1 |
| **U-7** | **MIR 循环控制**（repeat / break / continue） | `[x]` | `loop_stack` + RepeatWhile/Break/Continue 降级；census [`u-7`](../docs/census/u-7-loop-control-census.md)；≠ codegen Break / if-let / R-50 / JIT | U-6 residual |
| **U-8** | **`if let`/`if case` 关 trailing-closure** | `[x]` | RHS/subject 关旗；census [`u-8`](../docs/census/u-8-if-let-trailing-census.md)；≠ while let / R-50 / JIT | U-7 next |
| **U-9** | **块内语句可选分号** | `[x]` | `parse_block_stmts` 吞 0..n `;`；census [`u-9`](../docs/census/u-9-semicolon-census.md)；≠ codegen Break / while let / CST / R-50 / JIT | U-8 next |
| **U-10** | **`while let` 有界解糖** | `[x]` | `parse_while` Let→While+IfLet+Break；census [`u-10`](../docs/census/u-10-while-let-census.md)；≠ while case / 多绑定 / codegen Break / CST / R-50 / JIT | U-9 next |
| **U-11** | **标签 `repeat`/`break outer`** | `[x]` | `RepeatWhile.label` + MIR loop_stack；census [`u-11`](../docs/census/u-11-repeat-label-census.md)；≠ CST postfix / codegen Break / while case / R-50 / JIT | U-10 next |
| **U-12** | **`while case` 有界解糖** | `[x]` | 账面纸面；树交付由 U-15 复落；≠ 逗号链 / CST / codegen Break / R-50 / JIT | U-11 next |
| **U-13** | **逗号 `if let` 嵌套解糖** | `[x]` | 账面纸面；树交付由 U-14 复落；≠ 逗号 while let / guard 链 / CST / R-50 / JIT | U-12 next |
| **U-14** | **复落逗号 `if let`** | `[x]` | Let 臂逗号链→嵌套 IfLet/If；census [`u-14`](../docs/census/u-14-if-let-comma-census.md)；≠ while case 复落 / 逗号 while let / CST / R-50 / JIT | U-13 honesty |
| **U-15** | **复落 `while case`** | `[x]` | Case 臂→While+IfCase+Break；census [`u-15`](../docs/census/u-15-while-case-census.md)；≠ 逗号 while let / guard / CST / R-50 / JIT | U-12 honesty |
| **U-16** | **逗号 `guard let`** | `[x]` | Seq of GuardLet[+trailing If]；census [`u-16`](../docs/census/u-16-guard-let-comma-census.md)；≠ 裸 guard / 逗号 while let / CST / R-50 / JIT | U-15 next / book claim |

**默认下一动作：编号账本仍收口（无 R-50）+ U-0…U-16 已退出** — 残余为 **park** / 未编号债（逗号 `while let`、CST postfix、codegen Break、JIT Suite-1、simd Object 159 章程、SyxHost 等），**不**新开 R-50。

**退出总则：**  
- SA-0 → 核心层清单 + 分层命名 ADR 已接受；manifest / prelude / IMPORTABLE 三处 SSOT 自洽。  
- SA-1 → 自动导入仅含清单内符号；Env/Path/JSON/Fmt 等需 `import`。  
- SA-2 → 包重写所需语法（错误模型、命名空间、去 `data class`/`Object` 用户面）可用。  
- SA-3 → 目标模块名可 `import`；旧名删除或仅存于 archive 说明（**无**兼容别名义务）。  
- SA-4 → 试点包 + 官方随附模块 API 符合 Swift API Design Guidelines 风格；`__syx_*` 不对用户可见。  
- SA-5 → library-guide / examples 与新表面一致；doctest 闸绿。  
- SA-6 → census KPI 绿 + 退出 ADR。

**非目标（本期末默认）：** Apple Foundation 兼容；产品伞 unpark；F-08；HostFn+1 KPI；命名任何模块为 `Foundation`。

---

## 阶段 SA-0 · P0 — 章程与 SSOT 收敛

**目标：** 书面锁定「核心层 / 随附模块 / 独立包」边界与命名规则，并修现有 SSOT 裂缝。

| ID | 任务 | 优先级 | 状态 | 验收（摘要） |
|----|------|--------|------|--------------|
| SA-0-01 | 标准库 Swift 对齐章程 ADR | P0 | [x] | [stdlib-swift-alignment](../docs/adr/stdlib-swift-alignment.md)：破坏性；三层模型；禁 Foundation；`do/catch` + `throws -> T` 默认；kebab `syx-*` / UpperCamelCase import |
| SA-0-02 | 核心层（prelude）清单定稿 | P0 | [x] | [`packages/internal/prelude_core.toml`](../packages/internal/prelude_core.toml) target + `current_injected`；SA-1 执行移出 |
| SA-0-03 | 三处 SSOT 对齐 | P0 | [x] | prelude/importable 三表一致；Fmt 入 prelude；Trace/Metrics/Profiler 入 importable；Text 退出 retired；[census](../docs/census/sa-0-ssot-census.md) + `scripts/sa_0_ssot_census.sh` |
| SA-0-04 | 现名→目标名对照表 | P0 | [x] | [`packages/internal/module_rename_map.toml`](../packages/internal/module_rename_map.toml)（Os→System 等）；SA-3 落地 |
| SA-0-05 | Observation 等未接线包诚实化 | P1 | [~] | **residual**：README 已标非 `importable_modules`；接入或正式降级仍开（R 账） |

**本阶段退出：** SA-0-01…04 `[x]`；SA-0-05 = residual（不阻断）。下一默认曾为 SA-1；**SA-1 已退出** → **SA-2-01**。

---

## 阶段 SA-1 · P0 — Prelude 瘦身与核心层 Swift 化

**目标：** 自动导入范围收敛到 Swift 标准库体量；核心数值/随机/打印表面贴近 Swift。

| ID | 任务 | 优先级 | 状态 | 验收（摘要） |
|----|------|--------|------|--------------|
| SA-1-01 | 移出 IO/环境向 prelude | P0 | [x] | `Time`/`Env`/`Path`/`JSON`/`Fmt` → IMPORTABLE；examples/tools 显式 import |
| SA-1-02 | Collections 非核心移出 | P0 | [x] | `OrderedSet`/`Deque` → IMPORTABLE（SA-3 Collections 改名） |
| SA-1-03 | Math 并入核心数值面 | P0 | [x] | `Double.pi` / `sin`/`sqrt`；Math → IMPORTABLE；stdlib math 测加 `import Math` |
| SA-1-04 | Random 并入核心 | P0 | [x] | `Int.random(in:)`；Random → IMPORTABLE |
| SA-1-05 | Concurrency 核心化 | P1 | [x] | 仍隐式注入；叙事 = 核心层（`prelude_core.toml`） |
| SA-1-06 | 核心文档去过程行话 | P2 | [x] | 薄收口：census/ADR 指针；SIMD4 → IMPORTABLE；[sa-1 census](../docs/census/sa-1-prelude-census.md) |

**本阶段退出：** SA-1-01…04 `[x]`；注入 = `prelude_core.toml` target（26）；下一 = **SA-2-01**。

---

## 阶段 SA-2 · P0/P1 — 语言面补齐（支撑包 API）

**目标：** 让包源码能写成 Swift 风格，而不被语法/语义卡住。破坏性：可直接删除 Kotlin 风格表面。

| ID | 任务 | 优先级 | 状态 | 验收（摘要） |
|----|------|--------|------|--------------|
| SA-2-01 | 错误处理书面对齐 | P0 | [x] | Swift `do/catch`；删语句 `try {`；ADR + 书 + QuickReference |
| SA-2-02 | `enum …: Error` 语义加深 | P0 | [x] | throw 闸；OsError/NetworkError/IOError… 试点 |
| SA-2-03 | caseless enum 命名空间 | P0 | [x] | `ProcessInfo.processorCount` check/run |
| SA-2-04 | 移除 `data class` 用户面 | P0 | [x] | `public data class` 解析拒绝；packages/examples = 0 |
| SA-2-05 | 统一 `throws -> T` | P1 | [x] | packages + 书后置 `-> T throws` = 0 |
| SA-2-06 | 用户 API 禁止裸 `Object` | P1 | [ ] | lint/语义闸：stdlib 公开签名无 `Object`；选项用结构体/泛型/`any Protocol`（→ SA-4 可绑） |
| SA-2-07 | 计算属性 / `static var` | P1 | [x] | 试点 `ProcessInfo.processorCount`（更广面 → SA-4） |

**本阶段退出：** SA-2-01…04 `[x]`（已退出）；下一 = **SA-3-01**。SA-2-06 residual。

---

## 阶段 SA-3 · P1 — 官方模块重组与改名

**目标：** 按 todo-003 对照表落地模块边界。因无兼容要求：**直接改名/合并，不保留旧 `import Os`/`import Net` 等别名**（除非施工需要极短 internally alias，退出前删除）。

| ID | 任务 | 优先级 | 状态 | 验收（摘要） |
|----|------|--------|------|--------------|
| SA-3-01 | `System` 合并 | P0 | [x] | Os+Process+Env+Path+IO+Time+DateTime → `import System`；旧名归零 |
| SA-3-02 | `Networking` | P0 | [x] | Net → `Networking`；旧 `import Net` = 0（HTTP* 深改 → SA-4） |
| SA-3-03 | `Synchronization` | P1 | [x] | Sync → `Synchronization` |
| SA-3-04 | 可观测性改名 | P1 | [x] | Log→Logging；Trace→Tracing；Profiler→Profiling；Metrics keep |
| SA-3-05 | `Collections` 独立 | P1 | [ ] | **defer**：OrderedSet/Deque 仍独立（合并触 S023） |
| SA-3-06 | `JSON` 随附化 | P1 | [x] | 已 IMPORTABLE（SA-1）；keep 名 |
| SA-3-07 | Data 拆分 | P1 | [x] | Sql→`SQL`；ORM 非官方（书面） |
| SA-3-08 | 其余独立包 | P2 | [x] | Linalg→Numerics；Di→DI；Crypto/UI/Testing keep |
| SA-3-09 | Testing API 对齐草案 | P2 | [ ] | **defer** → SA-5 |

**本阶段退出：** SA-3-01…02 `[x]`（已退出）；旧 Os/Net import = 0；下一 = **SA-4-01**（已完成 → 现下一 **SA-5-01**）。Collections/Testing API = residual。

---

## 阶段 SA-4 · P1 — 包内 API Swift 风格重写

**目标：** 包代码本身像 Swift；宿主绑定内藏。

| ID | 任务 | 优先级 | 状态 | 验收（摘要） |
|----|------|--------|------|--------------|
| SA-4-01 | 试点 `System` | P0 | [x] | `ProcessInfo` / `FilePath` / `SystemError: Error`；无 `Os`；无用户可见 `__syx_*` |
| SA-4-02 | `Networking` 强类型 | P0 | [x] | 薄示范：`Http.fetch`/`request`/`post` 去主路径 `options: Object`；深改 residual |
| SA-4-03 | 异步签名统一 | P1 | [ ] | 公开 API `async throws -> T`；禁止 `Object` 句柄返回 |
| SA-4-04 | Host 绑定内聚 | P0 | [x] | `__syx_*` 仅 `packages/internal/*_host.sy`；公开包源 rg 归零 |
| SA-4-05 | 错误枚举化批量 | P1 | [ ] | 去掉 `*Error(code:message:)` 主路径 |
| SA-4-06 | DocC 风格注释 | P2 | [ ] | `- Parameters` / `- Returns` / `- Throws`；无 ADR 行话 |
| SA-4-07 | 其余随附/独立包迁移 | P1 | [ ] | Logging…UI 按稳定性优先级清完 |

**本阶段退出：** SA-4-01 + SA-4-04 `[x]`；Networking 与 System 为示范；其余包迁移率纳入 SA-6 census。

---

## 阶段 SA-5 · P1/P2 — 文档、示例与工具

**目标：** 书与工具跟上新表面；吸收 Stage-4 doctest residual。

| ID | 任务 | 优先级 | 状态 | 验收（摘要） |
|----|------|--------|------|--------------|
| SA-5-01 | `books/library-guide` 全量对齐 | P0 | [x] | 章节按新模块名；示例可 check/run |
| SA-5-02 | language-guide 标准库/错误章 | P1 | [ ] | 与 SA-2 决策一致；去过时 Math/Os 示例（非退出项） |
| SA-5-03 | examples 迁移 | P0 | [x] | `examples/` + VM allowlist 绿 |
| SA-5-04 | tools 真项目检验 | P1 | [~] | soga `Http.post` 薄修已交；sgate 已新 import（非退出硬项） |
| SA-5-05 | `syx doctest` MVP | P1 | [x] | 子命令存在；library-guide `--strict` 闸（承接 S4-01 residual） |
| SA-5-06 | stability ↔ manifest 接线 | P1 | [ ] | `stability.toml` 字段进入 manifest/driver（仍 defer） |
| SA-5-07 | `syx migrate` 或仓内 rewrite | P2 | [ ] | 破坏性期可用脚本一次性改写树内 `.sy`；产品 CLI 可选 |
| SA-5-08 | packages/README 重写 | P2 | [~] | import 名薄扫已交；≠ manifest 接线 |

**本阶段退出：** SA-5-01/03/05 `[x]`（已退出）；下一 = **post-SA Residual / 维护**（经 SA-6 退出）。

---

## 阶段 SA-6 · P2 — 收口与退出

**目标：** 用可机检 KPI 证明对齐完成并退出 SA 期。

| ID | 任务 | 优先级 | 状态 | 验收（摘要） |
|----|------|--------|------|--------------|
| SA-6-01 | census 脚本 | P1 | [x] | prelude/importable 对齐；旧 import=0；公开 `__syx_`=0；`File.stat`/`readAsync` 离 Object；其余 Object inventory 诚实挂账 |
| SA-6-02 | 能力表 / 站点叙事 | P1 | [x] | implementation-status 顶栏 SA 退出 + 现行 import；禁 Foundation；历史 Os/Net 段标冻结账本 |
| SA-6-03 | SA 退出 ADR | P0 | [x] | [sa-exit](../docs/adr/sa-exit.md)；residual；下一 = post-SA Residual / 维护 |
| SA-6-04 | 归档本期窗草稿 | P2 | [x] | `sa-*.plan.md` → `.cursor/plans/archive/sa/`（R-0-03） |

**本阶段退出：** SA-6-01 + SA-6-03 `[x]`（已退出）→ `01` 一句话已更新；下一 = **post-SA Residual / 维护**（R-* 账本）。

---

## 阶段 R · post-SA Residual / 维护

**目标：** 消化 [sa-exit](../docs/adr/sa-exit.md) residual；**≠ SA-7**。ID 前缀 `R-*` 仅为维护账本。

| ID | 任务 | 优先级 | 状态 | 验收（摘要） |
|----|------|--------|------|--------------|
| R-0-01 | residual backlog + census | P0 | [x] | `plan/02` 本表；[`scripts/r0_residual_census.sh`](../scripts/r0_residual_census.sh)；[`docs/census/r-0-residual-census.md`](../docs/census/r-0-residual-census.md)；sa_6 三 KPI 仍绿 |
| R-0-02 | Networking `TcpStream.readAsync` 离 Object | P0 | [x] | `networking`/`net/tcp` + host `tcp_read_async` → `Dictionary`；census `networking_tcp_readasync_object_cleared=1` |
| R-0-03 | plan/01 保鲜 + SA-6-04 归档 | P1 | [x] | `01` 叙事对齐 post-SA；`sa-*.plan.md` → `.cursor/plans/archive/sa/` |
| R-0-04 | 薄跟 Observation / language-guide / withOpen 挂账 | P2 | [x] | Observation 非 importable 诚实；language-guide 顶栏旧名薄扫；withOpen → R-1 |
| R-1-01 | `FileHandle.withOpen` 离 Object | P0 | [x] | 公开 `body: Function`（≠ Object）；gen system；[`r1 census`](../docs/census/r-1-withopen-census.md)；internal host 可留 Object |
| R-1-02 | RAII 硬闸 | P0 | [x] | `filehandle_raii_with` VM/native-mir success/throw/inject 绿；wasm/full-native llvm 诚实 ignore/cfg |
| R-2-01 | Channel `send`/`trySend` Object→Sendable | P0 | [x] | 公开 `value: Sendable`（≠ Object）；[`r2 census`](../docs/census/r-2-channel-census.md)；internal host 可留 Object；≠ Channel 泛型 |
| R-2-02 | Channel 硬闸 | P0 | [x] | parity `channel_send` / `channel_send_recv` 绿；`trySend` Int 烟测绿；credibility `r2_channel_census_contract`；m3_a04 背压/公平/close 绿 |
| R-3-01 | Channel `recv`/`tryRecv` Object→Sendable | P0 | [x] | 公开 `-> Sendable`（≠ Object）；[`r3 census`](../docs/census/r-3-channel-recv-census.md)；internal host 可留 Object；空/关哨兵仍 runtime `nil`；≠ Channel 泛型 |
| R-3-02 | Channel recv 硬闸 | P0 | [x] | parity `channel_send` / `channel_send_recv` 绿；`tryRecv` Int/nil 烟测绿；credibility `r3_channel_recv_census_contract`；m3_a04 三钉绿 |
| R-4-01 | Select 公开面去 Object | P0 | [x] | `Array<Channel>` + `-> Sendable` / `fallback: Sendable`；[`r4 census`](../docs/census/r-4-select-census.md)；host `channel_select*` → `Array`；VM ObjectRef `handle` unwrap；`value_compare` HeapSlot 解析 |
| R-4-02 | Select 硬闸 | P0 | [x] | LSP `channel_select_parse`；Select 烟测；credibility `r4_select_census_contract`；m3_a04 / parity 绿 |
| R-5-01 | Mutex/Semaphore 公开 Object→Int | P0 | [x] | 工厂/`lock`/`wait`… → `Int`；[`r5 census`](../docs/census/r-5-mutex-sem-census.md)；host 本已 Int；≠ handle bag |
| R-5-02 | Mutex/Semaphore RAII 硬闸 | P0 | [x] | `mutex_raii` / `semaphore_raii` 绿；credibility `r5_mutex_sem_census_contract`；产品烟测绿 |
| R-6-01 | `Channel.handle` Object→Int + create 双轨 | P1 | [x] | `handle: Int`；`host_channel_create→Int`；`channel_id_from_value` Int\|bag；[`r6 census`](../docs/census/r-6-channel-handle-census.md)；≠ Channel 泛型 |
| R-6-02 | Channel.handle 硬闸 | P1 | [x] | `channel_close` / `channel_backpressure` / VM Int 直送；credibility `r6_channel_handle_census_contract` |
| R-7-01 | `UdpSocket` 公开 Object→Int/Dictionary | P1 | [x] | bind/socket→`Int`；recvFrom→`Dictionary`；[`r7 census`](../docs/census/r-7-udp-census.md)；gen networking；≠ WS/simd |
| R-7-02 | UdpSocket 硬闸 | P1 | [x] | `p98_driver_udp_bind_host` / `udp_echo`；credibility `r7_udp_census_contract` |
| R-8-01 | `HttpBodyStream.open*` Object→Dictionary | P1 | [x] | open/openWithOptions→`Dictionary`；[`r8 census`](../docs/census/r-8-http-body-stream-census.md)；≠ WS/simd |
| R-8-02 | HttpBodyStream 硬闸 | P1 | [x] | stdlib `http_body_stream` / leak；credibility `r8_http_body_stream_census_contract` |
| R-9-01 | `Url.encodeQuery`/`build` query→Dictionary | P1 | [x] | query:`Dictionary`；SyxHost `url_build` base:`String`；[`r9 census`](../docs/census/r-9-url-query-census.md)；≠ WS/simd |
| R-9-02 | Url query 硬闸 | P1 | [x] | `io_demo` Url.build；credibility `r9_url_query_census_contract` |
| R-10-01 | `WebSocket` 公开 Object→Dictionary | P1 | [x] | connect/send/recv/close；[`r10 census`](../docs/census/r-10-websocket-census.md)；≠ Int dual-track / WSS GA |
| R-10-02 | WebSocket 硬闸 | P1 | [x] | fixture compile / LSP surface；credibility `r10_websocket_census_contract` |
| R-11-01 | `SqlRow` 公开 `row` Object→Dictionary | P1 | [x] | 六访问器；[`r11 census`](../docs/census/r-11-sql-row-census.md)；≠ SqlRow 值类型 GA / Task Int |
| R-11-02 | SqlRow 硬闸 | P1 | [x] | `sql_typed_row` + credibility `r11_sql_row_census_contract` |
| R-12-01 | `Fmt`/`Testing` 公开 Object→Sendable/Function | P1 | [x] | 13 处/3 文件；[`r12 census`](../docs/census/r-12-text-testing-census.md)；≠ simd/collections |
| R-12-02 | Fmt/Testing 硬闸 | P1 | [x] | parity fmt_print · syx_test_* · credibility `r12_text_testing_census_contract` |
| R-13-01 | `Concurrency` 公开任务面去 Object | P1 | [x] | 15 处/1 文件；袋→`Dictionary`、值位→`Sendable`；[`r13 census`](../docs/census/r-13-concurrency-census.md) |
| R-13-02 | Concurrency 硬闸 | P1 | [x] | `--test concurrency` · `examples/task_demo.sy` · credibility `r13_concurrency_census_contract` |
| R-14-01 | 并发测试错误绑定 `catch (err: Object)` | P1 | [x] | 5 处；裸 catch 绑 String（≠ 改语言语义） |
| R-14-02 | actor `HeapSlot` `__actor_id` 丢失 | P1 | [x] | [`value.rs`](../crates/syx-runtime-core/src/value.rs) `register_actor_id` 补 HeapSlot 分支；`actor_counter.sy` → `2` |
| R-14-03 | 并发文档 honesty 锚点 | P1 | [x] | 18-concurrency 应用并发分层；UsageGuide 内存序/Atomic honesty（中英） |
| R-14-04 | Concurrency 套件全绿 | P1 | [x] | `--test concurrency` **37/0**；credibility `r14_concurrency_debt_contract` |
| R-15-01 | `syx-driver --lib` 恢复编译 | P1 | [x] | 内联两处 `include_str!` 幽灵样例；`build_mir_from_graph` → `pub(crate)`；**58/0** |
| R-15-02 | 5 处 count_gate 死路径 | P1 | [x] | 改指子目录真实文件（20/75/46/10/45 全达标） |
| R-15-03 | 空洞断言 + 幽灵输入 | P1 | [x] | `parity_vm` 硬断言；`stdlib_parity_hard` 去 `browser_tier2k` + 3 处软读改 `expect` |
| R-15-04 | 测试基础设施闸 | P1 | [x] | credibility `r15_test_infra_contract`（3/3） |
| R-16…R-19 | wasm emit W001 专项 | P1 | 未交付 | 七假设全错；[`r-20-known-gaps`](../docs/census/r-20-known-gaps.md) 挂账；保留取证工具与栈效应表 |
| R-20-01 | 已知缺口 census | P1 | [x] | W001 七假设表 + 运算符缺口表 |
| R-20-02 | struct/data class `==`/`!=` 合成 | P1 | [x] | `lower_dataclass_eq_shell` / `_ne_shell`；`dataclass_synth` 全绿 |
| R-20-03 | 未定义运算符可操作诊断 | P1 | [x] | `split_type_operator_name`；`Unknown function 'P_+'` → 可操作提示 |
| R-20-04 | R-20 硬闸 | P1 | [x] | credibility `r20_dataclass_ops_contract`（5/5，R-21 已合并回单一路径断言） |
| R-21-01 | 路径分歧取证 | P1 | [x] | `path_parity_probe` 例：embedded `p.x`→`nil` / 文件路径→`3` |
| R-21-02 | `merge_mir_shell_functions` 覆盖同名壳 | P1 | [x] | 通用零参 `{T}_init` 不再压过 memberwise 壳 |
| R-21-03 | 路径一致性硬闸 | P1 | [x] | credibility `r21_path_parity_contract`（3/3，两路径逐行相等） |
| R-21-04 | census 记账 + R-20 契约合并 | P1 | [x] | [`r-20-known-gaps`](../docs/census/r-20-known-gaps.md) §3 |
| R-* | SA defer 指针 | — | — | SA-0-05 · SA-2-06 · SA-3-05/09 · SA-4-03…07 · SA-5-02/06/07 等仍开行 = residual，非新宏阶段 |

**本阶段（R-0）退出：** R-0-01 + R-0-02 `[x]`（已退出）。

**本阶段（R-1）退出：** R-1-01 + R-1-02 `[x]`（已退出）。

**本阶段（R-2）退出：** R-2-01 + R-2-02 `[x]`（已退出）。

**本阶段（R-3）退出：** R-3-01 + R-3-02 `[x]`（已退出）。

**本阶段（R-4）退出：** R-4-01 + R-4-02 `[x]`（已退出）。

**本阶段（R-5）退出：** R-5-01 + R-5-02 `[x]`（已退出）。

**本阶段（R-6）退出：** R-6-01 + R-6-02 `[x]`（已退出）。

**本阶段（R-7）退出：** R-7-01 + R-7-02 `[x]`（已退出）。

**本阶段（R-8）退出：** R-8-01 + R-8-02 `[x]`（已退出）。

**本阶段（R-9）退出：** R-9-01 + R-9-02 `[x]`（已退出）。

**本阶段（R-10）退出：** R-10-01 + R-10-02 `[x]`（已退出）。

**本阶段（R-11）退出：** R-11-01 + R-11-02 `[x]`（已退出）。

**本阶段（R-12）退出：** R-12-01 + R-12-02 `[x]`（已退出）。

**本阶段（R-13）退出：** R-13-01 + R-13-02 `[x]`（已退出）。

**本阶段（R-14）退出：** R-14-01…04 `[x]`（已退出）。

**本阶段（R-15）退出：** R-15-01…04 `[x]`（已退出）。

**本阶段（R-16…R-19）：** W001 专项未交付，已结案挂账（见 [`r-20-known-gaps`](../docs/census/r-20-known-gaps.md)）。

**本阶段（R-20）退出：** R-20-01…04 `[x]`（已退出）。

**本阶段（R-21）退出：** R-21-01…04 `[x]`（已退出）；下一默认 = **R-22+ residual**。

**非目标：** 全库 Object=0 · Foundation · F-08 · 产品伞 · Collections 合并解 S023。

---

## 暂停项（须显式 ADR；非默认）

| ID | 项 | 状态 | 说明 |
|----|----|------|------|
| PARK-F08 | F-08 RT ABI 全清 | park | 沿用 ADR-1375 |
| PARK-PSTAR | P-ABI-v2 / P-FAER / P-SURF | park | soft-close ≠ 产品 GA |
| PARK-PRODUCT | SynUI / Farm·Dist | park | 不随 SA 解锁 |
| PARK-FOUNDATION | 命名/兼容 Apple Foundation | park | **永久默认否**，除非新 ADR |

---

## 施工约定

1. **ID 前缀 `SA-*`**，勿与历史 `S0`–`S4` 混用。  
2. 每窗：`.cursor/plans/` 草稿 → 必要 ADR → 勾本表 → 更新 `01`。  
3. 破坏性删除旧名时，同步：`packages/`、`embedded_std`、`manifest`、`books/`、`examples/`、credibility 中对旧名的产品断言（历史 census 指向 archive 附录即可）。  
4. 不把 HostFn 数量增长当完成度。

---

# 附录 · 可信度阶段 0→4 冻结账本（契约钉）

> 以下全文来自上期活 backlog，**不再施工**。完整副本另见 [`archive/02-improvement-roadmap-credibility-stage-ledger.md`](archive/02-improvement-roadmap-credibility-stage-ledger.md)。  
> 保留本附录是为既有 `crates/syx-driver/tests/credibility/*` 与 census 脚本对 `plan/02` 的字符串钉继续成立。


> **冻结归档（2026-11-09）：** 可信度阶段 0→4 已全部退出。本文件不再作为活 backlog。  
> **活 backlog：** [`../02-improvement-roadmap.md`](../02-improvement-roadmap.md)（Swift 标准库对齐期 · SA）  
> **活状态：** [`../01-project-status.md`](../01-project-status.md)  
> **分析底稿：** [`../todo-003.md`](../todo-003.md)

---

# SYX 改进增强任务清单（可信度阶段）

> **原则：** 先让「已宣称的」成为真的，再扩展新的。重大方向变更先 ADR，再改本表。  
> **状态：** `[ ]` 未开始 · `[~]` 进行中 · `[x]` 完成 · `[-]` 取消/延后  
> **优先级：** P0 阻断可信 · P1 生产核心 · P2 增强 · P3 生态  
> **对照现状：** [`01-project-status.md`](01-project-status.md)  
> **分析底稿：** [`ProjectAnalysisAndImprovementPlan.zh-CN.md`](ProjectAnalysisAndImprovementPlan.zh-CN.md)  
> **历史 Wave A–F（冻结）：** [`archive/02-improvement-roadmap-wave-af-ledger.md`](archive/02-improvement-roadmap-wave-af-ledger.md)  
> **最近更新：** 2026-11-09 · **S3-I109 Stage-4 exit**（[ADR](../docs/adr/stage4-exit.md)；[exit-open](../docs/adr/stage4-exit-open.md)；[exit census](../docs/census/stage4-exit-census.md)；阶段 4 **`[x]` 退出** · **5/5**；下一 = post-exit Residual / 维护）。  




> **当前窗草稿：** [`.cursor/plans/s2-i175_expr_member_1e080def.plan.md`](../.cursor/plans/s2-i175_expr_member_1e080def.plan.md)

## 进度摘要

| 阶段 | 主题 | 状态 | 下一步 |
|------|------|------|--------|
| **0** | 止血与公开仓自洽 | `[x]` 退出 | 维护冒烟与警告闸；不再开阶段 0 窗 |
| **1** | 语言核心正确 + 文档即测试 | `[x]` 退出（S1-I27） | 余项 S1-04/05 post-exit；不抢阶段2 |
| **2** | 架构收敛与精简 | `[x]` 退出（S2-I266） | S2-01…07 scoped `[x]`；post-exit residual 见 [stage2-exit](../docs/adr/stage2-exit.md) |
| **3** | DX 与性能 | `[x]` 退出（S3-I87） · 7/7 | S3-01…07 scoped `[x]`；residual 见 [stage3-exit](../docs/adr/stage3-exit.md) |
| **4** | 生态与语言增强 | `[x]` 退出（S3-I109） · 5/5 | S4-01…05 scoped `[x]`；residual 见 [stage4-exit](../docs/adr/stage4-exit.md) |
| **Hist** | Wave A–F / F-08 / 产品伞 | 归档 · park | 不作为默认施工；须显式 ADR |

```mermaid
flowchart LR
  s0["Stage0 self-host"] --> s1["Stage1 language"]
  s1 --> s2["Stage2 architecture"]
  s2 --> s3["Stage3 DX"]
  s3 --> s4["Stage4 ecosystem"]
```

**退出总则（度量详表见分析底稿 §6）：**  
阶段 0 → 公开仓 `cargo test` 可绿 + CI 自动 + hello native/wasm 通过或明确报错。  
阶段 1 → 书中代码块解析失败归零、通过率 ≥ 85%、examples 可运行率拉满。  
（**S1-I27：阶段1已退出** — [ADR](../docs/adr/stage1-exit.md)。examples 腿口径 = [exit-leg](../docs/adr/stage1-exit-examples-leg.md)（VM 契约 + S0 三后端薄）。**拉满 / allowlist 全三后端仍非宣称范围**。S1-04/05 = **post-exit residual**。）  
阶段 2 → S2-01…S2-07 scoped 验收齐。  
（**S2-I266：阶段2已退出** — [ADR](../docs/adr/stage2-exit.md)。Frontend migrate / docs 117.0 叙事 / fingerprint 盐 / 产品伞 = **post-exit residual**。）  
阶段 3 → S3-01…S3-07 scoped 验收齐。  
（**S3-I87：阶段3已退出** — [ADR](../docs/adr/stage3-exit.md)。fib&lt;0.5 / lint hybrid / LSP rename·fence / Init UsageGuide·crate / parked HostFn·F-08 = **post-exit residual**。）

---

## 阶段 0 · P0 — 止血与公开仓自洽

**目标：** 任何人 `git clone` 公开仓后，默认路径可构建、可测、CI 会跑；后端失败必须 fail-closed。

| ID | 任务 | 优先级 | 状态 | 验收（摘要） |
|----|------|--------|------|--------------|
| S0-01 | 清除对 `plan/` 的编译期依赖 | P0 | [x] | 公开 `include_str!(…/plan/…)` = 0；CI `rg` 闸；窗级断言测已删/迁 |
| S0-02 | 解除对 `../synui` 的硬依赖 | P0 | [x] | `syx-vm`：`synui` → git `rev=87aa558…`（optional / `gui-synui`）；默认不再要求 sibling clone |
| S0-03 | 提交 `Cargo.lock` | P0 | [x] | 已移出 `.gitignore` 并生成入库 |
| S0-04 | CI 改为 push / pull_request | P0 | [x] | `on: [push, pull_request, workflow_dispatch]`；去 phase 字面量门禁；去 synui checkout；合并 windows 重复 checkout |
| S0-05 | Native fail-closed | P0 | [x] | 无 `llvm`/`full-native` 时明确报错；`scripts/s0_backend_smoke.sh`；Darwin：fail-closed 记过 |
| S0-06 | Wasm 校验失败修复 | P0 | [x] | emit 后 `wasmparser::validate`；修 guest cond / ConstStr 栈泄漏；Darwin hello wasm **PASS** |
| S0-07 | 更正能力矩阵诚实状态 | P0 | [x] | appendix-a（en/zh + site）：Native/WASM → **partial**（非 GA） |
| S0-08 | 清零编译警告 | P1 | [x] | 默认 lib 路径 `RUSTFLAGS=-Dwarnings`；CI Clippy 已 `-D warnings`；残面 crate 级 allow + 定点 fix |

**本机注意：** 分析底稿在 Linux 复现；S0-05/06 Darwin/Linux 冒烟见 `plan/01` §8。

---

## 阶段 1 · P0/P1 — 语言核心正确 + 文档即测试

**目标：** 书中教的核心语法能解析、能通过检查，并在后端上行为一致。规格默认**以书（Swift 风格）为准**。

| ID | 任务 | 优先级 | 状态 | 验收（摘要） |
|----|------|--------|------|--------------|
| S1-01 | 语法决策书面化（书为准 + 兼容弃用） | P0 | [x] | [`docs/adr/language-book-ssot.md`](../docs/adr/language-book-ssot.md) |
| S1-02 | 解析器：`struct` 对齐书 | P0 | [x] | 解析 + 语义构造（Point/Counter `check`/`run`）；成员默认/`mutating`；协议 conformances 语义仍薄 |
| S1-03 | 解析器：`enum` / 内联约束 / 闭包 `$0` | P0 | [x] | S1-I3：`<T: P>`、`$0`/`$1` trailing、enum 命名关联/`data` soft-kw、`enum_book_*` check；多 case / 方法→Extension |
| S1-04 | 解析器：protocol/extension / 字典字面量 / switch | P1 | [~] | S1-I18：Ch31 **5/5**；Ch00–34 可检章满不回退；钻石 / Sequence·Collection 全矩阵仍欠。**post-exit residual（S1-I27）；不阻断阶段1退出** |
| S1-05 | 泛型单态符号一致 | P0 | [~] | S1-I15：Ch24 **14/14**、Ch25 **10/10** 不回退；opaque 全矩阵 / 四后端 mono GA 仍欠。**post-exit residual（S1-I27）；不阻断阶段1退出** |
| S1-06 | `let` 值语义与 mutating | P0 | [x] | `let` + `append` / `@mutating` → **S060**（≠ MainActor S020）；`let_append_must_fail.sy` 钉死；S1-I18 examples `var` 修样对齐 |
| S1-07 | 整数溢出默认 trap | P1 | [x] | **scoped `[x]`（S1-I25）**：现行 SSOT = **二补回绕**（非 Swift 默认 trap）；见 [ADR](../docs/adr/stage1-s1-07-overflow-wrap-scope.md)；VM 钉 `Int.max+1` wrap。**trap / R011 / `&+`/`&-`/`&*` 仍 defer**；**≠ trap GA / ≠ 阶段1退出** |
| S1-08 | 错误恢复（多诊断） | P1 | [x] | **scoped `[x]`（S1-I24）**：LSP + 单文件 check（parse+语义薄）+ **项目 check parse 多诊**（钉 `multi_project_syntax` ≥2× P001）；见 [ADR](../docs/adr/stage1-s1-08-cli-multidiag-scope.md)。**build·run / 项目语义全矩阵仍 defer**；**≠ 阶段1退出** |
| S1-09 | 文档即测试基础设施 | P0 | [x] | **scoped `[x]`（S1-I23）**：标注草案 + `book_pass_rate.py` + CI job `book-pass`（en 硬闸：100% + `parse_fail=0`）；见 [ADR](../docs/adr/stage1-s1-09-book-kpi-ci-scope.md)。**zh 矩阵 / doctest 产品化仍 defer**；**≠ 阶段1退出** |
| S1-10 | examples 契约化 | P0 | [x] | **scoped `[x]`（S1-I21）**：VM allowlist **26** + `examples_vm_contract.sh` + CI + OUT 表 = 验收完成；见 [ADR](../docs/adr/stage1-s1-10-vm-inventory-scope.md)。**三后端 / examples 拉满仍 defer**（≠ 拉满宣称 / ≠ 阶段1退出） |

**主 KPI：** 书中代码块通过率（取代测试数量下限）。

---

## 阶段 2 · P1 — 架构收敛与精简

**已退出（S2-I266）** — [stage2-exit](../docs/adr/stage2-exit.md)。S2-01…S2-07 已 scoped `[x]`。默认下一可信度轴 = **Stage-3 open**（S3-01）；**≠ 本窗 Stage-3 施工**。

| ID | 任务 | 优先级 | 状态 | 验收（摘要） |
|----|------|--------|------|--------------|
| S2-01 | 过程治理代码移出产品 `src/` | P1 | [x] | **S2-I135：** census KPI 全 **0**；claim ledger complete；gate peel/accept 齐（[claim ADR](../docs/adr/stage2-s2-01-s2-02-claim.md)）。**≠ 阶段2退出** |
| S2-02 | 测试改为行为驱动 | P1 | [x] | **S2-I135：** soft OUT≈0；KEEP tip policy-kept；行为/`run()`/contract 为主路径（[claim ADR](../docs/adr/stage2-s2-01-s2-02-claim.md)）。**≠ 阶段2退出** |
| S2-03 | VM 消费 MIR（收官 B-01/B-02 诚实） | P1 | [x] | **S2-I142：** plan claim（[ADR](../docs/adr/stage2-s2-03-claim.md)）；尸体轴 I137–I140 + Suite-1 VM⟷MIR 地板 I141。**≠ 阶段2退出 / ≠ 四后端 GA** |
| S2-04 | 统一解析器（CST→AST） | P1 | [x] | **S2-I164：** scoped plan claim（[ADR](../docs/adr/stage2-s2-04-claim.md)；过渡期双轨 SSOT + freeze KPI + lint 诚实）。**≠ 阶段2退出 / ≠ Frontend 已共树 / ≠ 删 syx-cst**；Frontend migrate 后续窗 |
| S2-05 | 拆分巨型文件 | P2 | [x] | **S2-I259：** scoped plan claim（[ADR](../docs/adr/stage2-s2-05-claim.md)）。生产 peel 耗尽；hub/tests-in-lib OK。**≠ 阶段2退出**；下一 = **S2-06 open** |
| S2-06 | 缩减默认 workspace 范围 | P2 | [x] | **S2-I262：** scoped plan claim（[ADR](../docs/adr/stage2-s2-06-claim.md)；施工 [I261 exclude](../docs/adr/stage2-s2-06-exclude-syx-gui.md)）。默认 workspace 无 `syx-gui`；Farm/Dist/HSM/CDN 无独立 crate；opt-in/tests residual OK。**≠ 阶段2退出**；下一 = **S2-07 open** |
| S2-07 | 单一对外版本号 | P2 | [x] | **S2-I265：** scoped plan claim（[ADR](../docs/adr/stage2-s2-07-claim.md)；开窗 [I263](../docs/adr/stage2-s2-07-open.md)；施工 [I264 inject](../docs/adr/stage2-s2-07-inject-cargo-pkg-version.md)）。对外 = crate semver；phase/fingerprint/书叙事 residual OK。**≠ 阶段2退出**；下一 = **Stage-2 exit** |

---

## 阶段 3 · P1/P2 — 开发者体验与性能

**已退出（S3-I87）** — [stage3-exit](../docs/adr/stage3-exit.md)；开窗 [S3-I86](../docs/adr/stage3-exit-open.md)；[exit census](../docs/census/stage3-exit-census.md)。S3-01…07 **`[x]`**（checklist **7/7**；末条 [S3-I85](../docs/adr/stage3-s3-07-claim.md)）；Stage-3 **`[x]` 退出**。Stage-4 open **landed**（[S3-I88](../docs/adr/stage4-open.md)）；S4-01 SSOT **landed**（[S3-I89](../docs/adr/stage4-s4-01-stability-ssot.md)）；residual-scope **landed**（[S3-I90](../docs/adr/stage4-s4-01-residual-scope.md)）；S4-01 claim **landed**（[S3-I91](../docs/adr/stage4-s4-01-claim.md)）；S4-02 open **landed**（[S3-I92](../docs/adr/stage4-s4-02-open.md)）；file-registry hinge **landed**（[S3-I93](../docs/adr/stage4-s4-02-file-registry-hinge.md)）；S4-02 residual-scope **landed**（[S3-I94](../docs/adr/stage4-s4-02-residual-scope.md)）；S4-02 claim **landed**（[S3-I95](../docs/adr/stage4-s4-02-claim.md)）；S4-03 open **landed**（[S3-I96](../docs/adr/stage4-s4-03-open.md)）；cancel/TaskGroup hinge **landed**（[S3-I97](../docs/adr/stage4-s4-03-cancel-taskgroup-hinge.md)）；S4-03 residual-scope **landed**（[S3-I98](../docs/adr/stage4-s4-03-residual-scope.md)）；S4-03 claim **landed**（[S3-I99](../docs/adr/stage4-s4-03-claim.md)）；S4-04 open **landed**（[S3-I100](../docs/adr/stage4-s4-04-open.md)）；Iterator for-in hinge **landed**（[S3-I101](../docs/adr/stage4-s4-04-iterator-forin-hinge.md)）；S4-04 residual-scope **landed**（[S3-I102](../docs/adr/stage4-s4-04-residual-scope.md)）；S4-04 claim **landed**（[S3-I103](../docs/adr/stage4-s4-04-claim.md)）；S4-05 open **landed**（[S3-I104](../docs/adr/stage4-s4-05-open.md)）；doctor-native-sdk hinge **landed**（[S3-I105](../docs/adr/stage4-s4-05-doctor-native-sdk-hinge.md)）；S4-05 residual-scope **landed**（[S3-I106](../docs/adr/stage4-s4-05-residual-scope.md)）；S4-05 claim **landed**（[S3-I107](../docs/adr/stage4-s4-05-claim.md)）；Stage-4 exit-open **landed**（[S3-I108](../docs/adr/stage4-exit-open.md)）；Stage-4 exit **landed**（[S3-I109](../docs/adr/stage4-exit.md)）；下一默认 = **post-exit Residual / 维护**。

| ID | 任务 | 优先级 | 状态 | 验收（摘要） |
|----|------|--------|------|--------------|
| S3-01 | 诊断必须带 Span | P1 | [x] | **S3-I41：** scoped `[x]` claim（[ADR](../docs/adr/stage3-s3-01-claim.md)；[census](../docs/census/stage3-diagnostic-span-census.md)）。①②③ + face gates + I40；**≠** Stage-3 exit / **≠** 零噪声名 / **≠** MIR IR |
| S3-02 | 隐藏内部单态名 | P1 | [x] | **S3-I47：** scoped `[x]` claim（[ADR](../docs/adr/stage3-s3-02-claim.md)；[census](../docs/census/stage3-mono-display-census.md)）。开窗 S3-I42；S3-I43 demangle；S3-I44 debug；S3-I45 runtime；S3-I46 meta；断点/identity OUT；**≠** Stage-3 exit。下一 = **S3-03 open** |
| S3-03 | 标准库预编译/缓存 | P1 | [x] | **S3-I55：** scoped `[x]` claim（[ADR](../docs/adr/stage3-s3-03-claim.md)；[census](../docs/census/stage3-stdlib-cache-census.md)）。开窗 [S3-I48](../docs/adr/stage3-s3-03-open.md)；[S3-I49](../docs/adr/stage3-s3-03-prelude-snapshot.md) prelude-snapshot；[S3-I50](../docs/adr/stage3-s3-03-warm-program-trim.md) warm-program-trim；[S3-I51](../docs/adr/stage3-s3-03-warm-fixed-tax.md) warm-fixed-tax；[S3-I52](../docs/adr/stage3-s3-03-warm-residual.md) warm-residual；[S3-I53](../docs/adr/stage3-s3-03-cache-roundtrip.md) cache-roundtrip；[S3-I54](../docs/adr/stage3-s3-03-prelude-embed.md) prelude-embed；hello cold/warm/prelude_hit &lt;0.15（**wall superseded by R-40** ~2.5–3.1 s）；**≠** Stage-3 exit。下一 = **S3-04 open** |
| S3-04 | VM 热点优化 | P2 | [x] | **S3-I71：** scoped `[x]` claim（[ADR](../docs/adr/stage3-s3-04-claim.md)；[I70](../docs/adr/stage3-s3-04-residual-scope.md)；[I69](../docs/adr/stage3-s3-04-wall-strategy.md)；[I68](../docs/adr/stage3-s3-04-claim-prep.md)；[I67](../docs/adr/stage3-s3-04-can-stack-bind-cache.md)；[I66](../docs/adr/stage3-s3-04-jit-env-gate-cache.md)；[I65](../docs/adr/stage3-s3-04-tryfree-dispatch.md)；[I64](../docs/adr/stage3-s3-04-begintry-cache.md)；[I63](../docs/adr/stage3-s3-04-frame-swap.md)；[I62](../docs/adr/stage3-s3-04-locals-prep.md)；[I61](../docs/adr/stage3-s3-04-tryfree-call.md)；[I60](../docs/adr/stage3-s3-04-call-stack-args.md)；[I59](../docs/adr/stage3-s3-04-loadlocal-fastpath.md)；[I58](../docs/adr/stage3-s3-04-locals-frame-pool.md)；[I57](../docs/adr/stage3-s3-04-call-args-move.md)；开窗 [I56](../docs/adr/stage3-s3-04-open.md)；[census](../docs/census/stage3-vm-hotspot-census.md)）。Hard OUT 下 construct+脚手架齐；`fib30_interp_ceiling_s≈1.5–1.8`；字面 &lt;0.5 **deferred**；`s3_04_claim_helper≥1`；下一 = **S3-I72** S3-05 open。**≠** Stage-3 exit / **≠** 谎报 &lt;0.5 |
| S3-05 | lint 基于 AST/CST | P2 | [x] | **S3-I77：** scoped `[x]` claim（[ADR](../docs/adr/stage3-s3-05-claim.md)；[S3-I76](../docs/adr/stage3-s3-05-residual-scope.md)；[S3-I75](../docs/adr/stage3-s3-05-claim-prep.md)；[S3-I74](../docs/adr/stage3-s3-05-struct-extension-type-member.md)；[S3-I73](../docs/adr/stage3-s3-05-l007-l012-false-positive.md)；开窗 [S3-I72](../docs/adr/stage3-s3-05-open.md)；[census](../docs/census/stage3-lint-ast-census.md)）。Primary L007/L012=0；I73/I74 铰链；hybrid **deferred supersede-only**；`s3_05_claim_helper≥1`；下一 = **S3-I78** S3-06 open **landed**；默认 construct = **S3-I79**。**≠** Stage-3 exit / **≠** 恢复 hybrid 硬门槛 |
| S3-06 | LSP：书中块零误报 | P2 | [x] | **S3-I81：** scoped `[x]` claim（[ADR](../docs/adr/stage3-s3-06-claim.md)；[S3-I80 residual-scope](../docs/adr/stage3-s3-06-residual-scope.md)；[S3-I79 S045](../docs/adr/stage3-s3-06-s045-lsp-desugar.md)；开窗 [S3-I78](../docs/adr/stage3-s3-06-open.md)；[census](../docs/census/stage3-lsp-book-census.md)）。Primary = I79 hinge + sample fixtures；rename/sig/hover/full fence **deferred supersede-only**；`s3_06_claim_helper≥1`；S3-I82…I87 Stage-3 exit **landed**；下一 = **Stage-4 open**。**≠** Stage-4 施工 / **≠** 全书零误报字面 / **≠** rename·sig·hover 产品刀 / **≠** full fence |
| S3-07 | `syx init` 默认 VM 模板 | P2 | [x] | **S3-I85：** scoped `[x]` claim（[ADR](../docs/adr/stage3-s3-07-claim.md)；[S3-I84 residual-scope](../docs/adr/stage3-s3-07-residual-scope.md)；[S3-I83 default-vm](../docs/adr/stage3-s3-07-default-vm.md)；开窗 [S3-I82](../docs/adr/stage3-s3-07-open.md)；[census](../docs/census/stage3-init-template-census.md)）。Primary = I83 hinge + KPI 绿；UsageGuide/crate/CLI 子进程/`_` hard-error **deferred supersede-only**；`s3_07_claim_helper≥1`；S3-I86…I87 exit **landed**；下一 = **Stage-4 open**。**≠** Stage-4 施工 / **≠** UsageGuide 字面 / **≠** 抽 Init crate |

---

## 阶段 4 · P3 — 生态与语言增强（阶段 0–3 退出后）

**已退出（S3-I109）** — [stage4-exit](../docs/adr/stage4-exit.md)；开窗 [S3-I108](../docs/adr/stage4-exit-open.md)；[exit census](../docs/census/stage4-exit-census.md)。S4-01…05 **`[x]`**（checklist **5/5**；末条 [S3-I107](../docs/adr/stage4-s4-05-claim.md)）；Stage-4 **`[x]` 退出**。下一默认 = **post-exit Residual / 维护**。**≠** Stage-5 / **≠** 接线 `doctor --native-sdk` / **≠** 产品刀本窗。

| ID | 任务 | 优先级 | 状态 | 验收（摘要） |
|----|------|--------|------|--------------|
| S4-01 | 标准库稳定性标注 + doctest | P3 | [x] | **S3-I91：** scoped `[x]` claim（[ADR](../docs/adr/stage4-s4-01-claim.md)；[I90 residual](../docs/adr/stage4-s4-01-residual-scope.md)；[I89 SSOT](../docs/adr/stage4-s4-01-stability-ssot.md)；[`stability.toml`](../packages/internal/stability.toml)；[census](../docs/census/stage4-stdlib-stability-doctest-census.md)）。Primary = I89 hinge + I90 scoped acceptance；manifest/doctest **deferred supersede-only**；`s4_01_claim_helper≥1`。下一轴 = S4-02。**≠** Stage-4 exit / **≠** manifest schema / **≠** doctest CLI |
| S4-02 | 包注册表最小可用 | P3 | [x] | **S3-I95：** scoped `[x]` claim（[ADR](../docs/adr/stage4-s4-02-claim.md)；[I94 residual](../docs/adr/stage4-s4-02-residual-scope.md)；[scope-v0](../docs/census/s4-02-registry-residual-scope-v0.md)；[I93 hinge](../docs/adr/stage4-s4-02-file-registry-hinge.md)；[census](../docs/census/stage4-package-registry-census.md)）。Primary = I93 hinge + I94 scoped acceptance；公网 GA/lock trim/CLI UX **deferred supersede-only**；`s4_02_claim_helper≥1`。下一轴 = S4-03。**≠** Stage-4 exit / **≠** `syx-pkg` 产品刀 / **≠** 公网 registry GA |
| S4-03 | 结构化并发加深 | P3 | [x] | **S3-I99：** scoped `[x]` claim（[ADR](../docs/adr/stage4-s4-03-claim.md)；[I98 residual](../docs/adr/stage4-s4-03-residual-scope.md)；[scope-v0](../docs/census/s4-03-concurrency-residual-scope-v0.md)；[I97 hinge](../docs/adr/stage4-s4-03-cancel-taskgroup-hinge.md)；[census](../docs/census/stage4-structured-concurrency-census.md)）。Primary = I97 hinge + I98 scoped acceptance；Sendable/`withTaskGroup`/TSAN **deferred supersede-only**；`s4_03_claim_helper≥1`。下一 = **S3-I100** S4-04 open。**≠** Stage-4 exit / **≠** `syx-async` 产品刀 / **≠** `withTaskGroup` GA |
| S4-04 | Sequence/Collection 协议 | P3 | [x] | **S3-I103：** scoped `[x]` claim（[ADR](../docs/adr/stage4-s4-04-claim.md)；[I102 residual](../docs/adr/stage4-s4-04-residual-scope.md)；[scope-v0](../docs/census/s4-04-sequence-residual-scope-v0.md)；[I101 hinge](../docs/adr/stage4-s4-04-iterator-forin-hinge.md)；[census](../docs/census/stage4-sequence-collection-census.md)）。Primary = I101 hinge + I102 scoped acceptance；`makeIterator`/用户 map·filter/Index **deferred supersede-only**；`s4_04_claim_helper≥1`。下一 = **S3-I104** S4-05 open。**≠** Stage-4 exit / **≠** 语义·stdlib·codegen 刀 / **≠** `makeIterator` GA |
| S4-05 | 预编译发行物 | P3 | [x] | **S3-I107：** scoped `[x]` claim（[ADR](../docs/adr/stage4-s4-05-claim.md)；[I106 residual](../docs/adr/stage4-s4-05-residual-scope.md)；[scope-v0](../docs/census/s4-05-prebuilt-residual-scope-v0.md)；[I105 hinge](../docs/adr/stage4-s4-05-doctor-native-sdk-hinge.md)；[census](../docs/census/stage4-prebuilt-release-census.md)）。Primary = I105 hinge + I106 scoped acceptance；CLI `--native-sdk` / sysroot / marketplace **deferred supersede-only**；`s4_05_claim_helper≥1`。Stage-4 exit **landed**（[S3-I109](../docs/adr/stage4-exit.md)；[I108 open](../docs/adr/stage4-exit-open.md)）。**≠** Stage-5 / **≠** 接线 `doctor --native-sdk` / **≠** CLI·packaging 产品刀 / **≠** 旧 `M2-S4-05` TLS |

---

## 暂停项（须显式 ADR；非默认）

| ID | 项 | 状态 | 说明 |
|----|----|------|------|
| PARK-F08 | F-08 RT ABI 全清 | park | ADR-1375 default-no；`| F-08 |` 仍 `[ ]` |
| PARK-PSTAR | P-ABI-v2 / P-FAER / P-SURF 产品 GA | park | soft-close ≠ 产品 GA；unpark 须章程 |
| PARK-PRODUCT | SynUI / Farm·Dist / 更多产品伞 | park | Wave E 收窄 `[x]` ≠ 产品 GA |
| PARK-WAVE53 | Wave 53/53 | park | 仅余 F-08；禁止伪完成宣称 |

历史详账与窗级 tip 流水见 archive；私有仓 tip/nail 与旧 `02` 指纹段的同步**延后**，不阻断阶段 0。

---

## 附录 · 历史 Wave 收口指针（冻结）

- 软完成：Wave **52/53**；E **6/6**；F **10/11**；HostFn **629**
- `| F-08 |` **`[ ]`**（**W1292 / ADR-1638** census-chain closeout；**W1031 / ADR-1375**）
- 完整轴表与指纹钉子：[`archive/02-improvement-roadmap-wave-af-ledger.md`](archive/02-improvement-roadmap-wave-af-ledger.md)
- 状态页旧版：[`archive/01-project-status-wave-af-ledger.md`](archive/01-project-status-wave-af-ledger.md)
