# 单元测试（`@Test`）

**实现状态：** `@Test` 发现与运行 **已实现**；JUnit 形钩子（`@TestSetup` / `@TestTeardown` / `@TestBefore` / `@TestAfter`）**已实现**；递归 `tests/**/*.sy` 与单文件 `syx test path/to/file.sy` **已实现**；`import Testing` + `assert` / `assertEq` **已实现**（显式 import，**不会**自动注入）；`@Test async` + 体内可完成的 `await`（经 `syx test` JSON pass）**已实现**。没有 `@Test` 的文件若在入口里调用 `test(name:fn:)`，这些闭包会在该入口结束前执行，JSON 用例名为 `{文件}.{name}`。**未做：** `Testing` auto-import。

`syx test` 把测试文件编进 VM 并跑带 `@Test` 的用例；失败非零退出。产品示例见 [examples/test_demo.sy](../../examples/test_demo.sy)；CLI 细节见 [docs/9_UsageGuide.md](../../docs/9_UsageGuide.md)；窄章程与后续片见 [plan/topics/syx-test.md](../../plan/topics/syx-test.md)。

## 最短成例

```syx
import Testing

@Test
func adds() {
    assertEq(1 + 1, 2)
    assert(true)
}
```

把文件放在项目的 `tests/` 下（可嵌套），然后：

```bash
syx test .
```

## 发现规则

| 输入 | 行为 |
|------|------|
| `syx test .`（或项目根） | 递归收集 `tests/**/*.sy`；用例名含相对路径（如 `nested/math.add`） |
| `syx test path/to/file.sy` | 只跑该文件里的 `@Test` |
| `--filter substr` | 按用例名子串过滤；未匹配记 `skipped` |
| `--format human\|json\|tap\|junit` | 报告格式（默认 human） |

每个测试文件独立编译运行。

## 钩子（PascalCase）

| 属性 | 时机 |
|------|------|
| `@TestSetup` | 该文件内所有用例之前，一次 |
| `@TestBefore` | 每个 `@Test` 之前 |
| `@Test` | 用例本身 |
| `@TestAfter` | 每个 `@Test` 之后 |
| `@TestTeardown` | 该文件内所有用例之后，一次 |

```syx
import Log
import Testing

@TestSetup
func boot() {
    Log.info("once before all")
}

@TestBefore
func prep() {
    Log.info("before each")
}

@Test
func first() {
    assertEq(1 + 1, 2)
}

@TestAfter
func clean() {
    Log.info("after each")
}

@TestTeardown
func done() {
    Log.info("once after all")
}
```

## `import Testing`

[`packages/testing/mod.sy`](../../packages/testing/mod.sy) 提供：

| API | 作用 |
|-----|------|
| `assert(condition: Bool)` | 条件为假 → 该用例失败（Host `Err`，非进程 panic） |
| `assertEq(actual: Object, expected: Object)` | 值不相等 → 失败（`value_equals`） |

**必须**写 `import Testing`；`syx test` **不会**自动注入该模块。没有 `@Test` 时，`test(name:fn:)` 登记的闭包会执行并出现在 JSON 里（`{文件}.{name}`）。带 `@Test` 的文件不另开兄弟用例；登记闭包若失败，失败文本并进该 `@Test`。`expectEqual` 仍不是发现路径。`@Test async func …` 体内可 `await`（与 `async func main` 同形 drain）；仍须显式 `import Testing`。**未做：** auto-import。

## 与 `main` 的关系

测试文件通常 **不需要** `func main()`：驱动器按属性发现并调用钩子与用例。普通 `syx run` 入口仍要求 `main`；两者不要混用同一叙事。

## 诚实边界

- Host 能力矩阵里 `TestingAssert` / `TestingAssertEq` 为 **partial**、**仅 VM `real`**（`syx test` 产品路径）；不宣称 mir/llvm/wasm GA。
- ≠ auto-import、≠ 全量 HostFn `status=implemented`、≠ 完整 async test matrix。
