# Doctest / book code-block annotations (S1-09 / R-38)

Fence annotations for `books/language-guide` and (SA-5-05) `books/library-guide`. Runner: [`scripts/book_pass_rate.py`](../scripts/book_pass_rate.py) · CLI: `syx doctest` (default `--book library-guide --strict`).  
**S1-09 scoped `[x]`:** en language-guide CI gate — see [ADR](adr/stage1-s1-09-book-kpi-ci-scope.md). **SA-5-05:** library-guide gate via `syx doctest` / `scripts/doctest_library_guide.py` (independent of language-guide KPI).  
**R-38:** opt-in `sy,output` + `// prints:` run+stdout compare (default remains check-only).

## Fence tags

| Tag | Meaning |
|-----|---------|
| `sy` / `syx` | Default: run `syx check` on the block as a full program |
| `sy,check` | Explicit check-only (same as `sy`) |
| `sy,fragment` | Snippet without `main`; runner wraps in `func main() { … }` |
| `sy,error=CODE` | Expect failure whose diagnostics contain `CODE` (e.g. `P001`, `S020`) |
| `sy,output` | Opt-in: run `syx run` and compare stdout to the first `// prints: …` line in the fence |

Historical ` ```swift ` fences are treated as `sy,check` by the baseline tool.

### `sy,output` / `// prints:`

- Only fences tagged `output` execute `syx run` (alone or with `fragment`).
- Expected stdout comes from the first line matching `// prints: <expected>` inside the fence body.
- Escape sequences in `<expected>`: `\n`, `\t`, `\\`.
- Comparison normalizes CRLF and strips a single trailing newline from both sides; otherwise exact match.
- Missing `// prints:` on an `output` fence → `run_fail`.

Default `sy` / `syx` blocks stay **check-only**. Full-book run is not the CI default (cold start ≈ 3 s/block; see [`r-38-book-run-census.md`](census/r-38-book-run-census.md)).  
Runner flag `--only-output` evaluates only `sy,output` fences (census / local pin check).

## Examples

````markdown
```sy
struct Point {
    var x: Int
    var y: Int
}
func main() {
    let p = Point(x: 1, y: 2)
    print(p.x)
}
```

```sy,fragment
let xs = [1, 2, 3]
print(xs.count)
```

```sy,error=S020
func main() {
    let a = [1, 2]
    a.append(3)
}
```

```sy,output
// prints: 6
protocol Iterator {
    func next() -> Int?
}
class Counter: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? {
        self.n = self.n + 1
        if self.n > 3 { return nil }
        return self.n
    }
}
func main() {
    var s: Int = 0
    for x in Counter() { s = s + x }
    print(s)
}
```

```sy,output
// prints: 1\n9
struct Point {
    var x: Int
}
func main() {
    var p = Point(x: 1)
    var q = p
    q.x = 9
    print(p.x)
    print(q.x)
}
```
````

## CI policy

- Publish pass-rate numbers on `plan/01` after each Stage-1 window.
- Default fence path remains **check**; `output` is opt-in (R-38). Do not require native/wasm for book blocks until a later contract lands.
- Primary KPI remains **pass rate**, not absolute test counts.
- `book-pass` job may stay soft-fail while check residuals or unpaid matrix jobs remain (R-37); reclaiming a hard gate is a later window.
