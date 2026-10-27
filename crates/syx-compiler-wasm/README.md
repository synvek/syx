# syx-compiler-wasm

浏览器目标的 SYX 编译器（FrontendPipeline → MIR → WASM bytes），以 `wasm32-unknown-unknown` **C ABI cdylib** 产出，供页面 **直接** `WebAssembly.instantiate`（**无** wasm-bindgen）。

**能力：** 编译子集 + AOT（注入 `browser_prelude`），**远小于**本机 `syx run` / CLI `syx run --wasm`。不是完整 stdlib / File / HTTP host 面。

**能力地板（诚实）：** 无项目 / `syx.toml` / 包依赖；无完整 File/HTTP/TCP；另无 SQL/ORM、本机 TLS/WSS、动态插件、产品 GUI、对等 Task/Actor。`getrandom` 使用 **`custom`**（确定性填充），避免 `js`/`wasm-bindgen` 导入。详见 [docs/9_UsageGuide.md](../../docs/9_UsageGuide.md)「浏览器边界」。

## 主要导出（C ABI）

- `alloc` / `dealloc` — 页面写入源码与 out-len 槽
- `syx_compile_source_to_wasm_bytes` — 源码 → guest WASM bytes（失败时诊断经 JSON 侧通道）
- `syx_last_compile_diagnostics_json` — 最近一次编译诊断 JSON
- `syx_guest_mir_exec` / `syx_guest_end_fn` / `syx_guest_take_catch_pc` / `syx_guest_get_return` — 页面把 guest 的 `syx.*` AOT 导入接到编译器内 `MirRuntime`（AOT `main` 出口码为 0，展示值用 `get_return`）

## 构建

```bash
# 仓库根目录（改完后必须重新 cp，否则页面仍是旧产物）
rustup target add wasm32-unknown-unknown
cargo build -p syx-compiler-wasm --target wasm32-unknown-unknown --release
# 产物：target/wasm32-unknown-unknown/release/syx_compiler_wasm.wasm
```

演示页约定把该文件复制为 `examples/wasm_web/compiler.wasm`，见 [examples/wasm_web/README.md](../../examples/wasm_web/README.md)。若页面报 `Import "__wbindgen_placeholder__"`，按上式重编并复制（当前路径无 wbindgen）。

产品边界与 how-to：[docs/9_UsageGuide.md](../../docs/9_UsageGuide.md)「浏览器边界」。
