---
title: Install SYX
description: Install the SYX toolchain — download prebuilt syx binaries from GitHub Releases, or build from source with Rust 1.85+. Native AOT needs LLVM.
lastUpdated: 2026-11-13
---
# Install SYX

You do **not** need Rust just to *run* SYX programs. Releases ship prebuilt `syx` binaries. Building the toolchain from source needs **Rust 1.85+**; the full native (LLVM) toolchain also needs a local LLVM install.

## Option A — prebuilt release (recommended)

1. Open the latest release: [github.com/synvek/syx/releases/latest](https://github.com/synvek/syx/releases/latest)
2. Download the archive for your OS / architecture
3. Put the `syx` binary on your `PATH`
4. Verify:

```bash
syx --version
syx run path/to/hello.sy
```

## Option B — build from source

```bash
git clone https://github.com/synvek/syx.git
cd syx
cargo build --release -p syx-cli
./target/release/syx run examples/hello.sy
```

Optional install into Cargo’s bin directory:

```bash
cargo install --path crates/syx-cli
```

Native AOT (`syx build --native`) requires the full-native / LLVM feature set — details in the [Quick Guide](/quick-guide/).

## Editor support

Install the **SYX Language** extension for VS Code (TextMate + LSP + DAP). See [Editors](/editors/) and [VS Code](/editors/vscode).

## Next steps

- [File extension (.sy)](./file-extension) and `syx.toml`
- [Backends](./backends) — VM, LLVM, Wasm
- [Getting Started](/guide/01-getting-started) in the Language Guide
