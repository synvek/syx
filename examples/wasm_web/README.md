# wasm_web — 浏览器内编一点 SYX 再跑

演示页：加载同目录下的 `compiler.wasm`（**C ABI cdylib**，**不要**经 wasm-bindgen）→ 编译 textarea 源码 → `WebAssembly.instantiate` 跑 guest。

**边界：** 这是浏览器 **编译子集 + AOT** 路径，**远小于**本机 `syx run` / `syx run --wasm`（CLI MIR + wasmtime/`mir_exec`，近完整 host）。产品说明见 [docs/9_UsageGuide.md](../../docs/9_UsageGuide.md)「浏览器边界」。

## 页面如何跑 guest

1. `fetch('./compiler.wasm')` 裸 `instantiate`（空 import object）。
2. 调用 `syx_compile_source_to_wasm_bytes` 得到 guest bytes；同时编译器内安装 `MirRuntime`。
3. 实例化 guest 时，把 `syx.*` AOT 导入（如 `__syx_aot_push_rt_numeric`）经 Proxy 转到 `syx_guest_mir_exec` / `syx_guest_end_fn` / `syx_guest_take_catch_pc`。
4. 调用 guest `main()`（AOT 出口码为 **0**），再用 `syx_guest_get_return` 读 RT 返回值并显示为 `main() => …`。

**能力地板（诚实）：** 无项目 / `syx.toml` / 包依赖；无完整 File/HTTP/TCP；另无 SQL/ORM、本机 TLS/WSS、动态插件、产品 GUI、对等 Task/Actor；`getrandom` 为 custom 确定性填充。

## 准备 `compiler.wasm`

与 CI（`.github/workflows/ci.yml`）同源：

```bash
# 仓库根目录（改完编译器后必须重新 cp，否则页面仍是旧产物）
rustup target add wasm32-unknown-unknown
cargo build -p syx-compiler-wasm --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/syx_compiler_wasm.wasm examples/wasm_web/compiler.wasm
```

若出现 `Import "__wbindgen_placeholder__"`：说明 `compiler.wasm` 仍是带 wasm-bindgen 的旧构建；按上面命令重编并复制即可（当前路径用 `getrandom/custom`，无 wbindgen）。

编译器 crate 说明：[crates/syx-compiler-wasm/README.md](../../crates/syx-compiler-wasm/README.md)。

## 打开页面

- Playwright 规格使用 `file://` + `index.html`。
- 手动打开时若浏览器拦截 `fetch('./compiler.wasm')`，在本目录起本地 HTTP，例如：`python3 -m http.server`，再访问对应 URL。

## 可选脚本

```bash
npm i
npm run test:gui   # playwright test -g tier2w；≠ Playwright 产品 GA / 全矩阵
```
