# comprehensive_demo — syx 综合测试范例

独立 syx 项目，用大量小用例覆盖语言指南、标准库与宿主能力。同一份源码面向三个后端
（VM / native / wasm），每个用例打印 `[PASS]` / `[FAIL]` / `[SKIP]`，末尾输出汇总。

syx 处于开发期：**FAIL 与 SKIP 是有效覆盖**（暴露编译器/标准库缺口），默认退出码 0；
只有编译期错误才需要处理（不允许阻塞整个项目）。

## 运行

```bash
# 在仓库根目录（stdlib.time_path 的 FilePath.exists 用例以此为前提）
./target/release/syx run examples/comprehensive_demo

# 三后端一键验证（VM 完整报告 + native staging 构建验证 + wasm 失败记录）
bash examples/comprehensive_demo/run_all.sh
```

### CLI 参数（`syx run . -- <参数>`）

| 参数 | 说明 |
|---|---|
| （无） | 跑全部 25 个套件 |
| `--only a,b` | 仅跑指定套件；支持全名 `lang.basics`、短名 `basics`、组名 `lang` |
| `--skip a,b` | 跳过指定套件（打印 `[SKIP] … (user skip)`） |
| `--list` | 列出套件与后端标注后退出 |
| `--backend vm\|native\|wasm` | 声明当前后端（默认 vm）；与套件标注不符的整套件 SKIP 并附原因 |
| `--force` | 忽略后端标注强制运行 |
| `--strict` | 有 FAIL 时以退出码 1 结束（throw SuiteError.strictFailure） |

示例：

```bash
syx run examples/comprehensive_demo -- --only basics,strings
syx run examples/comprehensive_demo -- --skip host.net
syx run examples/comprehensive_demo -- --backend native   # 全部 SKIP（见后端矩阵）
syx run examples/comprehensive_demo -- --list
```

## 目录结构

```
comprehensive_demo/
├── syx.toml              # name/version/main，无外部依赖
├── main.sy               # 唯一 func main()：参数解析、套件注册表、调度、汇总
├── harness.sy            # 全局计数 + 打印性断言（beginSuite/check/checkMsg/skipCase/endSuite）
├── ac_helper.sy          # access_control 套件的跨文件辅助模块
├── run_all.sh            # 三后端便捷脚本（含 native staging 逻辑）
└── suite_*.sy            # 每文件一个套件，入口 runXxx() -> (Int, Int, Int)，无 main
```

注意：`syx run <dir>` 只收集 main.sy 同目录的 `.sy`（非递归），同包所有文件共享一个
命名空间，因此套件平铺为 `suite_*.sy`，文件间无需 import，入口函数名全局唯一。

## 套件与后端矩阵（按实测标注）

当前编译器 build（0.4.0 开发期）**仅 VM 可完整运行**；native/wasm 的控制流 lowering
存在根本缺陷，全部套件标注 `vm`。标注是 main.sy 注册表中的数据（`backendsFor`/`reasonFor`），
编译器修复后改回对应字符串即可，无需改调度逻辑。

| 组 | 套件（17 + 3 + 5 = 25） | vm | native | wasm |
|---|---|---|---|---|
| lang | basics / operators / strings / control_flow / functions / enums / structs_classes / properties / inheritance / errors / generics / protocols / extensions / macros / access_control / memory / collections | ✅ | ⛔ 运行时 | ⛔ 运行时 |
| stdlib | math / text_json / time_path | ✅ | ⛔ 编译+运行时 | ⛔ 运行时 |
| host | concurrency / simd / io / net / services | ✅ | ⛔ 编译+运行时 | ⛔ 运行时 |

VM 全量基准：**25 套件，371 用例（296 pass / 19 fail / 56 skip）**。

### native 阻塞明细（实测）

- 编译期（整项目构建失败，run_all.sh 用 staging 剔除后 17 个 lang 套件可构建）：
  - `stdlib.math`：MIR001 Double lowering 未支持
  - `stdlib.text_json`：C020 Int `hash(into:)` 未生成
  - `stdlib.time_path`：MIR001 FilePath lowering 未支持
  - `host.concurrency`：MIR001 Channel；其余 host 套件依赖 VM 宿主（host.io 宿主函数返回 nil）
- 运行时（所有套件，含可构建的 lang 组）：
  - `if Bool变量` / `if String==String` 条件：条件体不执行（静默跳过）
  - `if Bool==Bool`、`while Bool`：死循环（break 无效）
  - `switch Bool`：返回 scrutinee 而非分支值
  - `String !=` 返回 nil（`==` 正常）；调度器已用 `!(x == y)` 等价写法规避
  - 仅 `if true/false` 字面量与 `if Int比较` 正常

### wasm 阻塞明细（实测）

- 编译期：MIR001 Unknown variable 'Channel'（整项目）
- 运行时：W001 If underflow；最小 probe 证实 if 语句体不执行

## Harness 约定（harness.sy）

采用模块级全局计数 + 函数（而非 class 实例）：当前编译器对多存储属性 class 的 init
参数赋值存在缺陷，且 class 实例跨函数按值传递触发 move（S011）。

```syx
func runXxx() -> (Int, Int, Int) {
    beginSuite("lang.xxx")          // 重置计数并打印套件头
    check("case_name", expr)        // 单表达式断言
    checkMsg("case2", ok, "期望 …，实际 …")  // FAIL 时附详情
    skipCase("case3", "特性未 GA")  // 主动跳过单用例
    return endSuite()               // 打印小计并返回 (pass, fail, skip)
}
```

用例写法注意（开发期编译器约束）：

- 多语句用例封装为套件内私有 `func caseXxx() -> Bool`（内部 do/catch 捕获异常返回 false）。
- String/class/enum 实例是 move 类型：传参后原变量不可再用；复制用 `"\(s)"`。
- 泛型返回值/字段非 Equatable：先 `let` 绑定再经插值转字符串比较；插值内不要直接调
  泛型函数（误推断 Int64）或嵌套字符串字面量（L001）。
- `check` 的 String 实参可能被相邻 String 变量污染：可疑时先绑定 Bool 结果再传入。
- 不依赖 `import Testing` / `@Test`（那是 VM-only 的 `syx test` 路径）。

## 扩展指南：新增一个套件

1. 新建 `suite_xxx.sy`，定义唯一入口 `func runXxx() -> (Int, Int, Int)`（按上面 harness 约定）。
2. 在 `main.sy` 注册表加三处（各占一行，便于 run_all.sh 按行过滤）：
   - `allSuites()` 数组加 `"组名.xxx"`；
   - `dispatch()` 加 `if name == "组名.xxx" { return runXxx() }`；
   - 如非全后端支持，在 `backendsFor()` / `reasonFor()` 登记（当前默认全部 `vm`）。
3. 验证：`syx run examples/comprehensive_demo -- --only xxx`。
4. 若该套件导致 native 整项目构建失败，把文件名加入 run_all.sh 的 `NATIVE_EXCLUDE_FILES`、
   套件名加入 `NATIVE_EXCLUDE_NAMES`（staging 会自动剔除并过滤注册表行）。

## 已知 FAIL 清单（VM，编译器/标准库缺口，随演进修正）

basics 科学计数法/块级 shadow、operators 短路求值与 Int8 溢出、functions 闭包捕获可变变量、
access_control 跨文件 private 未强制、memory weak 不自动置 nil、stdlib Random 恒 0 /
SHA-512 空 / Hasher combine 无效 / DateTime.format 返回 ms、concurrency await 结果未传回、
simd 归约恒 0、net Object 响应体未序列化 / Url.build 丢 query、services Metrics.inc 不计数 /
gauge 截断小数。详见各套件头部注释与 `[FAIL]` 输出详情。