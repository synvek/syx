# Doctest / book code-block annotations (S1-09 draft)

Stage-1 draft for marking fences in `books/language-guide`. The baseline runner is [`scripts/book_pass_rate.py`](../scripts/book_pass_rate.py). Full `syx doctest` / CI productization deepens later; CI may start with a **check-only** subset.

## Fence tags

| Tag | Meaning |
|-----|---------|
| `sy` / `syx` | Default: run `syx check` on the block as a full program |
| `sy,check` | Explicit check-only (same as `sy` in this draft) |
| `sy,fragment` | Snippet without `main`; runner wraps in `func main() { … }` |
| `sy,error=CODE` | Expect failure whose diagnostics contain `CODE` (e.g. `P001`, `S020`) |

Historical ` ```swift ` fences are treated as `sy,check` by the baseline tool.

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
````

## CI policy (first slice)

- Publish pass-rate numbers on `plan/01` after each Stage-1 window.
- Prefer check-only; do not require native/wasm for book blocks until S1-10 contracts land.
- Primary KPI remains **pass rate**, not absolute test counts.
