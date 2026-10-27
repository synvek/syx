# 闭包

**实现状态：** 闭包表达式 / 捕获 / HOF **已实现**（三后端 parity，测试 `fixtures/parity/closures_capture.sy` / `fixtures/parity/hof.sy`）；`@escaping` / `@autoclosure` / 闭包简写 `$0` **部分实现**；尾随闭包语法 **支持**

闭包是 **自包含的可调用代码块**，可捕获周围变量。SYX 闭包与 Swift 风格一致：`{ params in body }`。

## 基础语法

```syx
let inc = { x in x + 1 }
print(inc(41))                          // 42

let sum = { (a, b) in a + b }
print(sum(1, 2))                        // 3
```

| 形式 | 说明 |
|------|------|
| `{ x in x + 1 }` | 单参数；`x` 为参数名 |
| `{ (a, b) in a + b }` | 多参数 |
| `{ print("hi") }` | 无参数；可省 `() in` |
| `{ x, y in … }` | 无类型标注（推断） |
| `{ (x: Int) -> Int in x + 1 }` | 显式类型 |

## 类型标注

闭包作为变量 / 参数 / 返回值时，可显式标注类型：

```syx
let f: (Int) -> Int = { x in x * 2 }
let g: (Int, Int) -> Int = { (a, b) in a + b }
let h: () -> Void = { print("hi") }
```

> **类型推断：** 作为参数传入时，参数类型由函数签名推断；如推断失败，标注函数签名。

## 高阶函数（HOF）

闭包最常用于 HOF（`map` / `filter` / `reduce`）：

```syx
let nums = [1, 2, 3, 4, 5]

let doubled = nums.map({ x in x * 2 })               // [2, 4, 6, 8, 10]
let evens   = nums.filter({ x in x % 2 == 0 })       // [2, 4]
let sum     = nums.reduce(0, { acc, x in acc + x })  // 15
let flat    = nums.flatMap({ x in [x, -x] })         // [1, -1, 2, -2, ...]
```

HOF 走 `__syx_array_map` / `__syx_array_filter` / `__syx_array_reduce` Host 函数（[第 32 章 运算符 vs Core Host](../../docs/adr/architecture-decisions.md)）；闭包参数对应 `Array<Element>` 的元素。

> **三后端 parity：** `map/filter/reduce` 在 VM / 统一 MIR / WASM 行为一致（测试 `fixtures/parity/hof.sy`）。

## 尾随闭包

参数列表中的最后一个闭包可移到 `()` 之外：

```syx
let evens = nums.filter { x in x % 2 == 0 }           // 等价 nums.filter({ x in x % 2 == 0 })
let dbl   = nums.map { x in x * 2 }                   // 尾随
```

如果闭包是**唯一**参数，`()` 也可省：

```syx
let dbl = nums.map { x in x * 2 }                     // 尾随
```

> **多闭包：** 多个闭包参数时，**只有最后一个**可尾随；其它仍写 `()` 内：
>
> ```syx
> arr.reduce(0, combine: { acc, x in acc + x })
> //                           ^ 仍带 label
> ```

## 捕获

闭包可捕获外层 `let` / `var`：

```syx
func makeCounter() -> () -> Int {
    var n = 0
    return { n = n + 1; n }
}

let c = makeCounter()
print(c())         // 1
print(c())         // 2
print(c())         // 3
```

| 捕获 | 行为 |
|------|------|
| `let` 捕获 | 按值捕获（闭包内只读） |
| `var` 捕获 | 按引用捕获（闭包可修改） |
| 跨函数返回 | 闭包持有捕获对象（避免过早释放） |

```syx
let name = "SYX"
let greet = { print("Hello, \(name)") }
greet()                        // "Hello, SYX"
```

> **生命周期：** 闭包持有捕获对象到自身被释放。多次调用同一闭包修改捕获 `var` 会影响后续调用（与 Swift 一致）。

## 闭包作为参数

```syx
func twice(f: () -> Int) -> Int {
    return f() + f()
}

let r = twice(f: { 21 })
print(r)                                    // 42
```

## 闭包作为返回值

```syx
func makeAdder(n: Int) -> (Int) -> Int {
    return { x in x + n }
}

let add5 = makeAdder(n: 5)
print(add5(3))                              // 8
```

## 闭包与异步

```syx
Task {
    print("in task")
}

Task.detached {
    print("detached")
}
```

`Task { … }` 与 `Task.detached { … }` 接受闭包；详见 [第 18 章 并发](../../docs/adr/architecture-decisions.md)。

## 闭包与 Channel

```syx
let ch = Channel<Int>()
ch.send { v in print("got: \(v)") }         // 接收闭包（待实现语义；Phase 132 行为）
```

详见 [第 18 章 并发](18-concurrency.md#channel--select)。

## @resultBuilder 中的闭包

```syx
@resultBuilder
struct ArrayBuilder {
    static func buildBlock<T>(_ items: T...) -> Array<T> { return items }
}

@ArrayBuilder
func makeList() -> Array<Int> {
    1
    2
    3
}
let arr = makeList()                        // [1, 2, 3]
```

详见 [第 19 章 宏](19-macros.md#resultbuilder-phase-119--engine-管线--phase-123-hostcallclosure-修栈)。

## 闭包简写 `$0` / `$1`

```syx
let doubled = nums.map { $0 * 2 }          // 等价 { x in x * 2 }
let sum     = nums.reduce(0) { $0 + $1 }   // acc + element
```

| 简写 | 含义 |
|------|------|
| `$0` | 第 0 个参数 |
| `$1` | 第 1 个参数 |
| `$N` | 第 N 个参数 |

> **状态：** **部分实现**。推荐用 `{ x in … }` 显式写法以保可读性。

## 闭包作为存储

```syx
class Handler {
    var callback: (Int) -> Void = { _ in }
}

let h = Handler()
h.callback = { x in print("got \(x)") }
h.callback(42)                              // "got 42"
```

闭包存到 class 属性时持有 `self` 引用；注意避免循环引用（**未来** 计划支持 `[weak self]` 捕获列表）。

## 闭包与生命周期

```syx
func callLater(f: () -> String) {
    let s = f()
    print(s)
}

callLater(f: { return "later" })
```

闭包可作为参数跨函数传递；捕获的对象在闭包被释放前不会被回收。

## 性能

- 闭包是 heap 分配的 `Closure` 对象（与 Swift 类似）
- 简单闭包可被内联（`makeAdder` 等单次调用场景，Phase 30+ 内联）
- 循环内的闭包分配是已知热点；可用 `arr.forEach` 替代 `for-in` 闭包

## 完整示例

```syx
func main() {
    // HOF
    let nums = [1, 2, 3, 4, 5]
    let stats = (
        sum: nums.reduce(0) { $0 + $1 },                    // 15
        avg: nums.reduce(0) { $0 + $1 } / nums.count,      // 3
        max: nums.reduce(Int.min) { $0 > $1 ? $0 : $1 }      // 5
    )
    print("sum=\(stats.sum) avg=\(stats.avg) max=\(stats.max)")

    // 捕获
    func makeCounter(start: Int) -> () -> Int {
        var n = start
        return { n = n + 1; n }
    }
    let c = makeCounter(start: 10)
    print(c())      // 11
    print(c())      // 12

    // 尾随
    let evenSum = nums.filter { $0 % 2 == 0 }.reduce(0) { $0 + $1 }
    print(evenSum)  // 6
}
```

## 诊断

| 代码 | 触发 | 修复 |
|------|------|------|
| **S004** | 闭包参数/返回类型不匹配 | 修正类型标注 |
| **S010** | 闭包传给不满足协议约束的 HOF | 检查协议一致性 |
| **P001** | 闭包语法错误 | 检查 `{ params in body }` 结构 |

## 与 Swift 的差异

| Swift | SYX |
|-------|-----|
| `$0` / `$1` 闭包简写 | **部分实现** |
| 尾随闭包 | **支持** |
| `@escaping` | **未实现**（闭包默认 escaping） |
| `@autoclosure` | **未实现** |
| 捕获列表 `[weak self]` / `[unowned self]` | **未实现**（规划中） |
| 多尾随闭包 `arr.sorted(by:) { $0 < $1 }` | 合法 |
| `()` 单参数可省 | 合法 |

---

**下一章** → [枚举](../../docs/adr/architecture-decisions.md)

**参见** → [语言参考：表达式](../language-reference/03-expressions.md) · [第 7 章 函数](../../docs/adr/architecture-decisions.md) · [第 24 章 泛型](../../docs/adr/architecture-decisions.md) · [第 19 章 宏](../../docs/adr/architecture-decisions.md)
